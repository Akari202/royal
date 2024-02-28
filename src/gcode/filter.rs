use crate::gcode::points::Program;

mod linear;
mod arc;

const LINEAR_TOLERANCE: f64 = 0.0001;
const ARC_TOLERANCE: f64 = 0.0001;

pub fn filter(program: &mut Program) {
    if program.points.len() < 3 {
        println!("Program too short to filter");
        return;
    }
    println!("Filtering program");
    println!("Initial program length: {}", program.points.len());

    println!("Filtering collinear lines and duplicate arcs");
    let perf_start = std::time::Instant::now();
    arc::filter_duplicate_arcs(program);
    linear::filter_collinear_lines(program);
    println!("Filtering took: {:?}", perf_start.elapsed());
    println!("Filtered length: {}", program.points.len());
}
