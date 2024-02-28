use crate::gcode::filter::LINEAR_TOLERANCE;
use crate::gcode::points::{Point, PointType, Program};


pub fn filter_collinear_lines(program: &mut Program) {
    let mut pop_indices: Vec<usize> = Vec::new();
    let mut previous_points = (program.points[0], program.points[1]);
    let mut point_iter = program.points.iter_mut().enumerate();
    point_iter.next();
    point_iter.next();
    for (i, point) in point_iter {
        if point.point_type == previous_points.0.point_type && point.point_type == previous_points.1.point_type {
            match point.point_type {
                PointType::Rapid | PointType::Feed => {
                    if collinear(&previous_points.0, &previous_points.1, point) {
                        pop_indices.push(i - 1);
                        previous_points.1 = *point;
                        continue;
                    }

                },
                _ => { }
            }
        }
        previous_points.0 = previous_points.1;
        previous_points.1 = *point;
    }
    for i in pop_indices.iter().rev() {
        program.points.remove(*i);
    }
}

// check if three points are collinear
fn collinear(p1: &Point, p2: &Point, p3: &Point) -> bool {
    let det = p1.x * (p2.y * p3.z - p3.y * p2.z) -
        p1.y * (p2.x * p3.z - p3.x * p2.z) +
        p1.z * (p2.x * p3.y - p3.x * p2.y);
    det.abs() < LINEAR_TOLERANCE
}
