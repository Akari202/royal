use log::{debug, info, warn};
use vec_utils::angle::AngleRadians;
use vec_utils::vec3d::Vec3d;
use crate::gcode::transform::{rotate_program, scale_program, translate_program};
use crate::gcode::program::{Point, Program};

// mod linear;
// mod arc;

const LINEAR_TOLERANCE: f64 = 0.0001;
const ARC_TOLERANCE: f64 = 0.0001;
const ARC_MIN_RADIUS: f64 = 0.0001;
const ARC_MAX_RADIUS: f64 = 500.0;


pub fn filter(program: &mut Program) {
    // if program.points.len() < 3 {
    //     warn!("Program too short to filter");
    //     return;
    // }
    // let initial_length = program.points.len();
    // info!("Initial program length: {}", initial_length);
    //
    // linear::filter_zero_length_lines(program);
    // linear::filter_collinear_lines(program);
    // arc::filter_duplicate_arcs(program);
    // arc::filter_collinear_arcs(program);
    // arc::filter_zero_length_arcs(program);
    //
    // rotate_program(program, AngleRadians::quarter_pi(), Vec3d::k(), Vec3d::new(3.0, 4.0, 0.0));
    // translate_program(program, Vec3d { x: 3.0, y: 0.0, z: 5.0 });
    // scale_program(program, 2.0);
    //
    // // arc::fit_arcs(program).unwrap();
    // // arc::filter_duplicate_arcs(program);
    //
    // let filtered_length = program.points.len();
    // info!("Filtered length: {} or a {:.2}% reduction", filtered_length, (1.0 - (filtered_length as f64 / initial_length as f64)) * 100.0);
}

// The reaason this does not use filter is because i didnt know about that
fn remove_indices(program: &mut Program, indices: &Vec<usize>) {
    // if indices.is_empty() {
    //     return;
    // }
    // let mut points: Vec<Point> = Vec::new();
    // let mut i = 0;
    // let mut end: bool = false;
    // for (j, point) in program.points.iter().enumerate() {
    //     if !end && indices[i] == j {
    //         i += 1;
    //         if i == indices.len() {
    //             end = true;
    //         }
    //         debug!("Removing point: {}", point);
    //     } else {
    //         points.push(*point);
    //     }
    // }
    // program.points = points;
}

fn remove_and_update_indices(program: &mut Program, indices: &Vec<usize>) {
    // if indices.is_empty() {
    //     return;
    // }
    // let mut points: Vec<Point> = Vec::new();
    // let mut i = 0;
    // let mut end: bool = false;
    // for (j, point) in program.points.iter().enumerate() {
    //     if !end && indices[i] == j {
    //         points.last_mut().unwrap().update_xyz(point);
    //         i += 1;
    //         if i == indices.len() {
    //             end = true;
    //         }
    //         debug!("Removing point: {}", point);
    //     } else {
    //         points.push(*point);
    //     }
    // }
    // program.points = points;
}
