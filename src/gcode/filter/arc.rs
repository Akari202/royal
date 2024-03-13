use std::error::Error;
use log::{debug, info};
use vec_utils::matrix::matrix4x4;
use vec_utils::quat::Quat;
use vec_utils::vec3d::Vec3d;
use crate::gcode::filter::{ARC_MAX_RADIUS, ARC_MIN_RADIUS, ARC_TOLERANCE, linear, remove_and_update_indices, remove_indices};
use crate::gcode::points::{scalar_triple_product, Point, PointType, Program, vec_from_point, vec_from_point_center, Plane};

pub fn filter_duplicate_arcs(program: &mut Program) {
    let perf_start = std::time::Instant::now();
    let mut pop_indices: Vec<usize> = Vec::new();
    let mut previous_point = program.points[0];
    for (i, point) in program.points.iter().enumerate().skip(1) {
        if point.point_type == previous_point.point_type && point.plane == previous_point.plane {
            if point.is_arc() {
                if similar_circles(&previous_point, point) {
                    pop_indices.push(i - 1);
                    continue;
                }
            }
        }
        previous_point = *point;
    }
    remove_indices(program, &pop_indices);
    info!("Duplicate arc filtering took: {:?} and removed {} blocks", perf_start.elapsed(), pop_indices.len());
}

pub fn fit_arcs(program: &mut Program) -> Result<(), Box<dyn Error>> {
    let perf_start = std::time::Instant::now();
    let mut pop_indices: Vec<usize> = Vec::new();
    let mut previous_points = (program.points[0], program.points[1]);
    let mut fitted_flag: bool = false;
    for (i, point) in program.points.iter_mut().enumerate().skip(2) {
        if fitted_flag {
            fitted_flag = false;
            previous_points.1 = *point;
            continue;
        }
        if previous_points.1.point_type == point.point_type {
            if point.is_linear() {
                // let fitted_arc = fit_arc_3d(&previous_points.0, &previous_points.1, point);
                let mut fitted_arc = fit_arc_planar(&previous_points.0, &previous_points.1, point);
                match fitted_arc {
                    Ok(fitted_arc) => {
                        let radius = fitted_arc.arc_radius()?;
                        if radius < ARC_MIN_RADIUS || radius > ARC_MAX_RADIUS {
                            continue;
                        }
                        let v1 = vec_from_point(&previous_points.0);
                        let v2 = vec_from_point(&previous_points.1);
                        let v3 = vec_from_point(point);
                        let center = vec_from_point_center(&fitted_arc)?;
                        let error1 = radius - point_to_line_distance(&center, &v1, &v2);
                        let error2 = radius - point_to_line_distance(&center, &v2, &v3);
                        if error1 < ARC_TOLERANCE && error2 < ARC_TOLERANCE{
                            fitted_flag = true;
                            pop_indices.push(i - 1);
                            // pop_indices.push(i);
                            *point = fitted_arc;
                            previous_points.0 = fitted_arc;
                        }
                    },
                    Err(_) => { }
                }
            }
        }
        previous_points = (previous_points.1, *point);
    }
    remove_indices(program, &pop_indices);
    info!("Arc fitting took: {:?} and removed {} blocks", perf_start.elapsed(), pop_indices.len());
    Ok(())
}

// NOTE: i realize now that this is basically the same as duplicate arc filtering
// im keeping it for now just in case
pub fn filter_zero_length_arcs(program: &mut Program) {
    let perf_start = std::time::Instant::now();
    let mut pop_indices: Vec<usize> = Vec::new();
    let mut previous_point = &program.points[0];
    for (i, point) in program.points.iter().enumerate().skip(1) {
        if point.is_arc() && previous_point.point_type == point.point_type {
            if previous_point.distance(&point) < ARC_TOLERANCE {
                pop_indices.push(i - 1);
                continue;
            }
        }
        previous_point = point;
    }
    remove_indices(program, &pop_indices);
    info!("Zero length arc filtering took: {:?} and removed {} blocks", perf_start.elapsed(), pop_indices.len());
}

pub fn filter_collinear_arcs(program: &mut Program) {
    let perf_start = std::time::Instant::now();
    let mut pop_indices: Vec<usize> = Vec::new();
    let mut previous_points = (program.points[0], program.points[1]);
    for (i, point) in program.points.iter().enumerate().skip(2) {
        if previous_points.1.is_arc() {
            let center = vec_from_point_center(&previous_points.1).unwrap();
            let radius = previous_points.1.arc_radius().unwrap();
            // WARN: there are cases where this is incorrect
            if point.is_arc() {
                let first_error = radius - point_to_line_distance(&center, &vec_from_point(&previous_points.1), &vec_from_point(&point));
                let second_error = point.arc_radius().unwrap() -
                    point_to_line_distance(
                        &vec_from_point_center(&point).unwrap(),
                        &vec_from_point(&previous_points.1),
                        &vec_from_point(&point)
                    );
                let error = first_error - second_error;
                debug!("Collinear arc error: {}", error);
                if error < ARC_TOLERANCE {
                    pop_indices.push(i);
                    continue;
                }
            }
        }
        previous_points.0 = previous_points.1;
        previous_points.1 = *point;
    }
    remove_and_update_indices(program, &pop_indices);
    info!("Collinear arc filtering took: {:?} and removed {} blocks", perf_start.elapsed(), pop_indices.len());
}

fn similar_circles(p1: &Point, p2: &Point) -> bool {
    let radius_diff = (p1.arc_radius().unwrap() - p2.arc_radius().unwrap()).abs();
    let center_diff = p1.arc_center_distance(p2).unwrap().abs();
    (radius_diff + center_diff) < ARC_TOLERANCE
}

fn min_max_radius(program: &Program) -> (f64, f64) {
    let mut min_radius = f64::MAX;
    let mut max_radius = f64::MIN;
    for point in program.points.iter() {
        if let Some(i) = point.i {
            let radius = point.arc_radius().unwrap();
            if radius < min_radius {
                min_radius = radius;
            }
            if radius > max_radius {
                max_radius = radius;
            }
        }
    }
    (min_radius, max_radius)
}

// NOTE: this might have been a lot of unnecessary work
fn fit_arc_3d(p1: &Point, p2: &Point, p3: &Point) -> Result<Point, Box<dyn Error>> {
    if p2.point_type != PointType::Rapid && p2.point_type != PointType::Feed ||
        p3.point_type != PointType::Rapid && p3.point_type != PointType::Feed {
        Err("Middle and end points must be a rapid or feed motion")?
    }
    if linear::collinear(p1, p2, p3) {
        Err("Points are collinear")?
    }
    // the arc endpoint will be the same as p3
    let mut point = Point::new();
    point.x = p3.x;
    point.y = p3.y;
    point.z = p3.z;
    // calculate if arc is CW or CCW
    // WARN: no clue if this will do what i want
    let stp = scalar_triple_product(p1, p2, p3);
    if stp > 0.0 {
        point.set_type(PointType::ArcCW);
    } else {
        point.set_type(PointType::ArcCCW);
    }
    // build vectors
    let v1 = vec_from_point(p1);
    let v2 = vec_from_point(p2);
    let v3 = vec_from_point(p3);
    let vk = Vec3d::k();
    let v12 = Vec3d::new_from_to(&v1, &v2);
    let v23 = Vec3d::new_from_to(&v2, &v3);
    let normal = v12.cross(&v23).normalize();
    let angle = vk.angle_to(&normal);
    let rotation_axis = vk.cross(&normal).normalize();
    let rotation_quat = Quat::from_axis_angle(&rotation_axis, angle);
    let undo_rotation_quat = rotation_quat.conjugate();
    let (_, planar_center) = fit_circle_to_points_in_xy_plane(
        &rotation_quat.rotate(&v1),
        &rotation_quat.rotate(&v2),
        &rotation_quat.rotate(&v3)
    );
    // rotate everything back
    let center = undo_rotation_quat.rotate(&planar_center);
    point.i = Some(center.x - point.x);
    point.j = Some(center.y - point.y);
    point.k = Some(center.z - point.z);
    Ok(point)
}

fn fit_arc_planar(p1: &Point, p2: &Point, p3: &Point) -> Result<Point, Box<dyn Error>> {
    if p2.is_linear() || p3.is_linear() {
        Err("Middle and end points must be a rapid or feed motion")?
    }
    if linear::collinear(p1, p2, p3) {
        Err("Points are collinear")?
    }
    if p1.plane != p2.plane || p2.plane != p3.plane {
        Err("Points dont have the same plane selected")?
    }
    // the arc endpoint will be the same as p3
    let plane = p1.plane;
    let mut point = Point::new();
    point.x = p3.x;
    point.y = p3.y;
    point.z = p3.z;
    point.plane = plane;
    // build vectors
    let v1 = collapse_vec_to_xy_plane(&vec_from_point(p1), &plane);
    let v2 = collapse_vec_to_xy_plane(&vec_from_point(p2), &plane);
    let v3 = collapse_vec_to_xy_plane(&vec_from_point(p3), &plane);
    // calculate if arc is CW or CCW
    // WARN: might be backwards
    let cross = (v2 - v1).cross(&(v3 - v2));
    if cross.x + cross.y + cross.z < 0.0 {
        point.set_type(PointType::ArcCCW);
    } else {
        point.set_type(PointType::ArcCW);
    }
    // fit circle
    let (_, center): (f64, Vec3d) = fit_circle_to_points_in_xy_plane(&v1, &v2, &v3);
    match plane {
        Plane::XY => {
            point.i = Some(center.x - point.x);
            point.j = Some(center.y - point.y);
            point.k = Some(0.0);
        },
        Plane::XZ => {
            point.i = Some(center.x - point.x);
            point.j = Some(0.0);
            point.k = Some(center.y - point.z);
        },
        Plane::YZ => {
            point.i = Some(0.0);
            point.j = Some(center.x - point.y);
            point.k = Some(center.y - point.z);
        },
        Plane::None => { Err("No plane selected")? }
    }
    Ok(point)
}

// WARN: does not check if the points are in the same plane or collinear
fn fit_circle_to_points_in_xy_plane(v1: &Vec3d, v2: &Vec3d, v3: &Vec3d) -> (f64, Vec3d) {
    let matrix: [[f64; 4]; 4] = [
        [0.0, 0.0, 0.0, 0.0],
        [v1.x.powf(2.0) + v1.y.powf(2.0), v1.x, v1.y, 1.0],
        [v2.x.powf(2.0) + v2.y.powf(2.0), v2.x, v2.y, 1.0],
        [v3.x.powf(2.0) + v3.y.powf(2.0), v3.x, v3.y, 1.0]
    ];
    let center = Vec3d::new(
        -0.5 * matrix4x4::minor(&matrix, 0, 1) / matrix4x4::minor(&matrix, 0, 0),
        0.5 * matrix4x4::minor(&matrix, 0, 2) / matrix4x4::minor(&matrix, 0, 0),
        0.0
    );
    // I dont need the radius but here is the math i had to figure out
    let radius = center.x.powf(2.0) +
        center.y.powf(2.0) +
        matrix4x4::minor(&matrix, 0, 3) /
            matrix4x4::minor(&matrix, 0, 0);
    (radius, center)
}

fn point_to_line_distance(point: &Vec3d, l1: &Vec3d, l2: &Vec3d) -> f64 {
    let d = (l2 - l1).normalize();
    let v = point - l1;
    let t = v.dot(&d);
    let p = l1 + t * d;
    p.distance_to(point)
}

fn collapse_vec_to_xy_plane(v: &Vec3d, plane: &Plane) -> Vec3d {
    match plane {
        Plane::XY => Vec3d::new(v.x, v.y, 0.0),
        Plane::XZ => Vec3d::new(v.x, v.z, 0.0),
        Plane::YZ => Vec3d::new(v.y, v.z, 0.0),
        Plane::None => v.clone()
    }
}



