use std::error::Error;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::Write;
use log::{debug, info};
use logos::Logos;
use vec_utils::vec3d::Vec3d;
use crate::gcode::lex::tokens::Token;
use crate::gcode::lex::tokens::Token::*;

// Plan:
// Change program to being a collection of toolpaths
// a toolpath contains what is currently a program and some additional information
// i cant decide, should a toolpath have a fixed spindle speed or should i accomodate elijah's
// wierd programing habits. spindle speed will be stored as a single i32 with CCW rotations taken to be +
// toolpaths will have  a type of drilling, milling or whatever
//
// Output will only be touched later
//
// progress
//
#[derive(Debug, Clone)]
pub struct Program {
    pub points: Vec<Point>
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub i: Option<f64>,
    pub j: Option<f64>,
    pub k: Option<f64>,
    pub point_type: PointType,
    pub plane: Plane
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PointType {
    ArcCW,
    ArcCCW,
    Rapid,
    Feed,
    None
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum DistanceMode {
    Absolute,
    Incremental
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Plane {
    XY,
    YZ,
    XZ,
    None
}

impl Program {
    // TODO: make this accept a stream input
    pub fn from_file(input: &str) -> Result<Self, Box<dyn Error>> {
        let perf_start = std::time::Instant::now();
        let mut lex = Token::lexer(input);
        let mut program: Program = Program::new();
        let mut point: Point = Point::new();
        let mut distance_mode = DistanceMode::Absolute;
        let mut plane_selection = Plane::None;
        loop {
            if let Some(token) = lex.next() {
                if let Ok(token) = token {
                    debug!("Token: {}", token);
                    match token {
                        StartBlock => { }
                        EndOfBlock => {
                            if point.is_initialized() {
                                point.check(plane_selection)?;
                                program.push(point);
                            }
                        }
                        XPoint(x) => {
                            if distance_mode == DistanceMode::Absolute {
                                point.x = x;
                            } else {
                                point.x += x;
                            }
                        }
                        YPoint(y) => {
                            if distance_mode == DistanceMode::Absolute {
                                point.y = y;
                            } else {
                                point.y += y;
                            }
                        }
                        ZPoint(z) => {
                            if distance_mode == DistanceMode::Absolute {
                                point.z = z;
                            } else {
                                point.z += z;
                            }
                        }
                        IPoint(i) => { point.i = Some(i); }
                        JPoint(j) => { point.j = Some(j); }
                        KPoint(k) => { point.k = Some(k); }
                        RapidPositioning => { point.set_type(PointType::Rapid); }
                        LinearInterpolation => { point.set_type(PointType::Feed); }
                        CWCircularInterpolation => { point.set_type(PointType::ArcCW); }
                        CCWCircularInterpolation => { point.set_type(PointType::ArcCCW); }
                        AbsoluteDistanceMode => { distance_mode = DistanceMode::Absolute; }
                        IncrementalDistanceMode => { distance_mode = DistanceMode::Incremental; }
                        XYPlaneSelection => { plane_selection = Plane::XY; }
                        XZPlaneSelection => { plane_selection = Plane::XZ; }
                        YZPlaneSelection => { plane_selection = Plane::YZ; }
                    }
                }
            } else {
                break;
            }
        }
        info!("Parsing program took: {:?}", perf_start.elapsed());
        Ok(program)
    }

    pub fn to_file(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(filename)?;
        let mut previous_point = self.points[0];
        // write the first point
        writeln!(file, "%")?;
        writeln!(file, "G90")?;
        write!(
            file,
            "{} {} X{} Y{} Z{} ",
            previous_point.plane_gcode(),
            previous_point.type_gcode(),
            previous_point.x,
            previous_point.y,
            previous_point.z
        )?;
        if previous_point.i.is_some() && previous_point.i != Some(0.0) {
            write!(file, "I{} ", previous_point.i.unwrap())?;
        }
        if previous_point.j.is_some() && previous_point.j != Some(0.0) {
            write!(file, "J{} ", previous_point.j.unwrap())?;
        }
        if previous_point.k.is_some() && previous_point.k != Some(0.0) {
            write!(file, "K{} ", previous_point.k.unwrap())?;
        }
        for point in self.points.iter().skip(1) {
            if point.plane != previous_point.plane {
                write!(file, "{} ", point.plane_gcode())?;
            }
            if point.point_type != previous_point.point_type {
                write!(file, "{} ", point.type_gcode())?;
            }
            if point.x != previous_point.x {
                write!(file, "X{} ", point.x)?;
            }
            if point.y != previous_point.y {
                write!(file, "Y{} ", point.y)?;
            }
            if point.z != previous_point.z {
                write!(file, "Z{} ", point.z)?;
            }
            if point.point_type == PointType::ArcCW || point.point_type == PointType::ArcCCW {
                if point.i != Some(0.0) {
                    write!(file, "I{} ", point.i.unwrap())?;
                }
                if point.j != Some(0.0) {
                    write!(file, "J{} ", point.j.unwrap())?;
                }
                if point.k != Some(0.0) {
                    write!(file, "K{} ", point.k.unwrap())?;
                }
            }
            writeln!(file)?;
            previous_point = *point;
        }
        Ok(())
    }
}

impl Program {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub fn push(&mut self, point: Point) {
        self.points.push(point);
    }
}

impl Point {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            i: None,
            j: None,
            k: None,
            point_type: PointType::None,
            plane: Plane::None
        }
    }

    pub fn from_vec3d(v: Vec3d) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            i: None,
            j: None,
            k: None,
            point_type: PointType::None,
            plane: Plane::None
        }
    }

    pub(crate) fn set_type(&mut self, point_type: PointType) {
        match point_type {
            PointType::ArcCW | PointType::ArcCCW => {
                self.point_type = point_type;
                self.i = Some(0.0);
                self.j = Some(0.0);
                self.k = Some(0.0);
            },
            PointType::Rapid | PointType::Feed | PointType::None => {
                self.point_type = point_type;
                self.i = None;
                self.j = None;
                self.k = None;
            }
        }
    }

    fn check(&mut self, plane: Plane) -> Result<(), Box<dyn Error>> {
        if plane == Plane::None {
            Err("Plane not set")?
        }
        self.plane = plane;
        if self.i == None || self.j == None || self.k == None {
            match self.point_type {
                PointType::ArcCW | PointType::ArcCCW => { Err("Arc point missing I, J, or K value")? },
                PointType::Rapid | PointType::Feed => { return Ok(()); },
                PointType::None => { Err("Point type not set")? }
            }
        }
        Ok(())
    }

    fn is_initialized(&self) -> bool {
        self.x != 0.0 || self.y != 0.0 || self.z != 0.0 ||
            self.i != None || self.j != None || self.k != None ||
            self.point_type != PointType::None
    }

    pub fn distance(&self, other: &Point) -> f64 {
        (
            (self.x - other.x).powi(2) +
                (self.y - other.y).powi(2) +
                (self.z - other.z).powi(2)
        ).sqrt()
    }

    pub fn arc_radius(&self) -> Result<f64, Box<dyn Error>> {
        if self.i == None || self.j == None || self.k == None {
            Err("Arc point missing I, J, or K value")?
        } else {
            Ok((
                self.i.unwrap().powi(2) +
                    self.j.unwrap().powi(2) +
                    self.k.unwrap().powi(2)
            ).sqrt())
        }
    }

    pub fn arc_center_distance(&self, other: &Point) -> Result<f64, Box<dyn Error>> {
        if self.i == None || self.j == None || self.k == None ||
            other.i == None || other.j == None || other.k == None {
            Err("Arc point missing I, J, or K value")?
        } else {
            match self.plane {
                Plane::XY => {
                    Ok((
                        (self.i.unwrap() - other.i.unwrap()).powi(2) +
                            (self.j.unwrap() - other.j.unwrap()).powi(2)
                    ).sqrt())
                },
                Plane::XZ => {
                    Ok((
                        (self.i.unwrap() - other.i.unwrap()).powi(2) +
                            (self.k.unwrap() - other.k.unwrap()).powi(2)
                    ).sqrt())
                },
                Plane::YZ => {
                    Ok((
                        (self.j.unwrap() - other.j.unwrap()).powi(2) +
                            (self.k.unwrap() - other.k.unwrap()).powi(2)
                    ).sqrt())
                },
                Plane::None => { Err("No plane selected")? }
            }
        }
    }

    fn type_gcode(&self) -> &str {
        match self.point_type {
            PointType::ArcCW => "G2",
            PointType::ArcCCW => "G3",
            PointType::Rapid => "G0",
            PointType::Feed => "G1",
            PointType::None => ""
        }
    }

    fn plane_gcode(&self) -> &str {
        match self.plane {
            Plane::XY => "G17",
            Plane::XZ => "G18",
            Plane::YZ => "G19",
            Plane::None => ""
        }
    }

    pub fn is_arc(&self) -> bool {
        self.point_type == PointType::ArcCW || self.point_type == PointType::ArcCCW
    }

    pub fn is_linear(&self) -> bool {
        self.point_type == PointType::Rapid || self.point_type == PointType::Feed
    }

    pub fn update_xyz(&mut self, other: &Point) {
        self.x = other.x;
        self.y = other.y;
        self.z = other.z;
    }
}

pub fn scalar_triple_product(p1: &Point, p2: &Point, p3: &Point) -> f64 {
    p1.x * (p2.y * p3.z - p3.y * p2.z) -
        p1.y * (p2.x * p3.z - p3.x * p2.z) +
        p1.z * (p2.x * p3.y - p3.x * p2.y)
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for point in &self.points {
            write!(f, "{}\n", point)?;
        }
        Ok(())
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "X: {}, Y: {}, Z: {}, I: {:?}, J: {:?}, K: {:?}, Type: {:?}",
            self.x, self.y, self.z, self.i, self.j, self.k, self.point_type)
    }
}

pub fn vec_from_point(p1: &Point) -> Vec3d {
    Vec3d::new(p1.x, p1.y, p1.z)
}

pub fn vec_from_point_center(p1: &Point) -> Result<Vec3d, Box<dyn Error>> {
    if p1.i == None || p1.j == None || p1.k == None {
        Err("Arc point missing I, J, or K value")?
    } else {
        Ok(Vec3d::new(p1.i.unwrap(), p1.j.unwrap(), p1.k.unwrap()))
    }
}


