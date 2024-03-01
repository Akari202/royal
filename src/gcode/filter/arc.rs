use std::error::Error;
use log::info;
use vec_utils::matrix::matrix4x4;
use vec_utils::quat::Quat;
use vec_utils::vec3d::Vec3d;
use crate::gcode::filter::{ARC_TOLERANCE, linear, remove_indices};
use crate::gcode::points::{scalar_triple_product, Point, PointType, Program, vec_from_point};

pub fn filter_duplicate_arcs(program: &mut Program) {
    let perf_start = std::time::Instant::now();
    let mut pop_indices: Vec<usize> = Vec::new();
    let mut previous_point = program.points[0];
    let mut point_iter = program.points.iter_mut().enumerate();
    point_iter.next();
    for (i, point) in point_iter {
        if point.point_type == previous_point.point_type {
            match point.point_type {
                PointType::ArcCW | PointType::ArcCCW => {
                    if similar_circles(&previous_point, point) {
                        pop_indices.push(i - 1);
                        continue;
                    }
                },
                _ => { }
            }
        }
        previous_point = *point;
    }
    remove_indices(program, &pop_indices);
    info!("Duplicate arc filtering took: {:?} and removed {} blocks", perf_start.elapsed(), pop_indices.len());
}

fn similar_circles(p1: &Point, p2: &Point) -> bool {
    let radius_diff = p1.arc_radius().unwrap() - p2.arc_radius().unwrap();
    let center_diff = p1.arc_center_distance(p2).unwrap();
    (radius_diff.abs() + center_diff.abs()) < ARC_TOLERANCE
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

fn fit_arc(p1: &Point, p2: &Point, p3: &Point) -> Result<(Point), Box<dyn Error>> {
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
    let planar_v1 = rotation_quat.rotate(&v1);
    let planar_v2 = rotation_quat.rotate(&v2);
    let planar_v3 = rotation_quat.rotate(&v3);
    // calculate center and radius
    let matrix: [[f64; 4]; 4] = [
        [0.0, 0.0, 0.0, 0.0],
        [planar_v1.x.powf(2.0) + planar_v1.y.powf(2.0), planar_v1.x, planar_v1.y, 1.0],
        [planar_v2.x.powf(2.0) + planar_v2.y.powf(2.0), planar_v2.x, planar_v2.y, 1.0],
        [planar_v3.x.powf(2.0) + planar_v3.y.powf(2.0), planar_v3.x, planar_v3.y, 1.0]
    ];
    let planar_center = Vec3d::new(
        -0.5 * matrix4x4::minor(&matrix, 0, 1) / matrix4x4::minor(&matrix, 0, 0),
        0.5 * matrix4x4::minor(&matrix, 0, 2) / matrix4x4::minor(&matrix, 0, 0),
        0.0
    );
    // dont need the radius but here is the math i had to figure out
    // let radius = planar_center.x.powf(2.0) +
    //     planar_center.y.powf(2.0) +
    //     minor(&matrix, 0, 3) /
    //         minor(&matrix, 0, 0);
    // rotate everything back
    let center = undo_rotation_quat.rotate(&planar_center);
    point.i = Some(center.x);
    point.j = Some(center.y);
    point.k = Some(center.z);
    Ok(point)
}


