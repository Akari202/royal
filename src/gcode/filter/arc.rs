use crate::gcode::filter::ARC_TOLERANCE;
use crate::gcode::points::{Point, PointType, Program};

pub fn filter_duplicate_arcs(program: &mut Program) {
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
    for i in pop_indices.iter().rev() {
        program.points.remove(*i);
    }
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

