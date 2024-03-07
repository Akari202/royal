use log::info;
use crate::gcode::filter::{LINEAR_TOLERANCE, remove_indices};
use crate::gcode::points::{scalar_triple_product, Point, PointType, Program};


pub fn filter_collinear_lines(program: &mut Program) {
    let perf_start = std::time::Instant::now();
    let mut pop_indices: Vec<usize> = Vec::new();
    let mut previous_points = (program.points[0], program.points[1]);
    for (i, point) in program.points.iter().enumerate().skip(2) {
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
    remove_indices(program, &pop_indices);
    info!("Collinear filtering took: {:?} and removed {} blocks", perf_start.elapsed(), pop_indices.len());
}

pub fn filter_zero_length_lines(program: &mut Program) {
    let perf_start = std::time::Instant::now();
    let mut pop_indices: Vec<usize> = Vec::new();
    let mut previous_point = &program.points[0];
    for (i, point) in program.points.iter().enumerate().skip(1) {
        if point.point_type == PointType::Rapid || point.point_type == PointType::Feed {
            if previous_point.point_type == point.point_type {
                if previous_point.distance(&point) < LINEAR_TOLERANCE {
                    pop_indices.push(i - 1);
                    continue;
                }
            }
        }
        previous_point = point;
    }
    remove_indices(program, &pop_indices);
    info!("Zero length line filtering took: {:?} and removed {} blocks", perf_start.elapsed(), pop_indices.len());
}

// check if three points are collinear
pub fn collinear(p1: &Point, p2: &Point, p3: &Point) -> bool {
    let stp = scalar_triple_product(p1, p2, p3);
    stp.abs() < LINEAR_TOLERANCE
}
