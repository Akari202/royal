use log::info;
use rayon::prelude::*;
use vec_utils::angle::AngleRadians;
use vec_utils::quat::Quat;
use vec_utils::vec3d::Vec3d;
use crate::gcode::points::{Point, Program};

pub fn rotate_program(program: &mut Program, angle: impl Into<AngleRadians>, axis: Vec3d, center: Vec3d) {
    let perf_start = std::time::Instant::now();
    let angle: AngleRadians = angle.into();
    let rotation_quat = Quat::from_axis_angle(&axis, angle);
    let rotation_quat_conjugate = rotation_quat.conjugate();
    program.points.par_iter_mut().for_each(|i| {
        let point_quat = Quat {
            w: 0.0,
            i: i.x - center.x,
            j: i.y - center.y,
            k: i.z - center.z
        };
        let rotated_point_quat = rotation_quat_conjugate * point_quat * rotation_quat;
        i.x = rotated_point_quat.i + center.x;
        i.y = rotated_point_quat.j + center.y;
        i.z = rotated_point_quat.k + center.z;
        if i.is_arc() {
            let center_quat = Quat { w: 0.0, i: i.i.unwrap(), j: i.j.unwrap(), k: i.k.unwrap() };
            let rotated_center_quat = rotation_quat_conjugate * center_quat * rotation_quat;
            i.i = Some(rotated_center_quat.i);
            i.j = Some(rotated_center_quat.j);
            i.k = Some(rotated_center_quat.k);
        }
    });
    info!("Rotation took: {:?}", perf_start.elapsed());
}

pub fn translate_program(program: &mut Program, translation: Vec3d) {
    let perf_start = std::time::Instant::now();
    program.points.iter_mut().for_each(|i| {
        i.x = i.x + translation.x;
        i.y = i.y + translation.y;
        i.z = i.z + translation.z;
    });
    info!("Translation took: {:?}", perf_start.elapsed());
}

pub fn scale_program(program: &mut Program, scale_factor: f64) {
    let perf_start = std::time::Instant::now();
    program.points.iter_mut().for_each(|i| {
        i.x = i.x * scale_factor;
        i.y = i.y * scale_factor;
        i.z = i.z * scale_factor;
        if i.is_arc() {
            i.i = Some(i.i.unwrap() * scale_factor);
            i.j = Some(i.j.unwrap() * scale_factor);
            i.k = Some(i.k.unwrap() * scale_factor);
        }
    });
    info!("Scaling took: {:?}", perf_start.elapsed());
}
