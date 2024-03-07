use log::{debug, info, warn};
use crate::gcode::points::Program;

mod linear;
mod arc;

const LINEAR_TOLERANCE: f64 = 0.0001;
const ARC_TOLERANCE: f64 = 0.0001;
const ARC_MIN_RADIUS: f64 = 0.0001;
const ARC_MAX_RADIUS: f64 = 500.0;


pub fn filter(program: &mut Program) {
    if program.points.len() < 3 {
        warn!("Program too short to filter");
        return;
    }
    let initial_length = program.points.len();
    info!("Initial program length: {}", initial_length);

    linear::filter_collinear_lines(program);
    linear::filter_zero_length_lines(program);
    arc::filter_duplicate_arcs(program);
    arc::filter_zero_length_arcs(program);
    // arc::fit_arcs(program).unwrap();
    // arc::filter_duplicate_arcs(program);

    let filtered_length = program.points.len();
    info!("Filtered length: {} or a {:.2}% reduction", filtered_length, (1.0 - (filtered_length as f64 / initial_length as f64)) * 100.0);
}

fn remove_indices(program: &mut Program, indices: &Vec<usize>) {
    if indices.is_empty() {
        return;
    }
    let mut points = Vec::new();
    let mut i = 0;
    let mut end: bool = false;
    for (j, point) in program.points.iter().enumerate() {
        if !end && indices[i] == j {
            i += 1;
            if i == indices.len() {
                end = true;
            }
            debug!("Removing point: {}", point);
        } else {
            points.push(*point);
        }
    }
    program.points = points;
}
