use std::error::Error;
use std::fmt::{Debug, Display};
use log::{debug, info};
use logos::Logos;
use vec_utils::vec3d::Vec3d;
use crate::gcode::lex::tokens::Token;
use crate::gcode::lex::tokens::Token::*;

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
    pub point_type: PointType
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

impl Program {
    pub fn from_file(input: &str) -> Result<Self, Box<dyn Error>> {
        let perf_start = std::time::Instant::now();
        let mut lex = Token::lexer(input);
        let mut program: Program = Program::new();
        let mut point: Point = Point::new();
        let mut distance_mode = DistanceMode::Absolute;
        loop {
            if let Some(token) = lex.next() {
                if let Ok(token) = token {
                    debug!("Token: {}", token);
                    match token {
                        StartBlock => { },
                        EndOfBlock => {
                            if point.is_initialized() {
                                point.check()?;
                                program.push(point);
                            }
                        },
                        XPoint(x) => {
                            if distance_mode == DistanceMode::Absolute {
                                point.x = x;
                            } else {
                                point.x += x;
                            }
                        },
                        YPoint(y) => {
                            if distance_mode == DistanceMode::Absolute {
                                point.y = y;
                            } else {
                                point.y += y;
                            }
                        },
                        ZPoint(z) => {
                            if distance_mode == DistanceMode::Absolute {
                                point.z = z;
                            } else {
                                point.z += z;
                            }
                        },
                        IPoint(i) => { point.i = Some(i); },
                        JPoint(j) => { point.j = Some(j); },
                        KPoint(k) => { point.k = Some(k); },
                        RapidPositioning => { point.set_type(PointType::Rapid); },
                        LinearInterpolation => { point.set_type(PointType::Feed); },
                        CWCircularInterpolation => { point.set_type(PointType::ArcCW); },
                        CCWCircularInterpolation => { point.set_type(PointType::ArcCCW); },
                        AbsoluteDistanceMode => { distance_mode = DistanceMode::Absolute; },
                        IncrementalDistanceMode => { distance_mode = DistanceMode::Incremental; }
                    }
                }
            } else {
                break;
            }
        }
        info!("Parsing program took: {:?}", perf_start.elapsed());
        Ok(program)
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
            point_type: PointType::None
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
            point_type: PointType::None
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

    fn check(&mut self) -> Result<(), Box<dyn Error>> {
        if self.i == None || self.j == None || self.k == None {
            if self.point_type == PointType::ArcCW || self.point_type == PointType::ArcCCW {
                Err("Arc point missing I, J, or K value")?
            } else if self.point_type == PointType::Rapid || self.point_type == PointType::Feed {
                Ok(())
            } else {
                Err("Point type not set")?
            }
        } else {
            self.i = Some(self.x + self.i.unwrap());
            self.j = Some(self.y + self.j.unwrap());
            self.k = Some(self.z + self.k.unwrap());
            Ok(())
        }
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
                (self.x - self.i.unwrap()).powi(2) +
                    (self.y - self.j.unwrap()).powi(2) +
                    (self.z - self.k.unwrap()).powi(2)
            ).sqrt())
        }
    }

    pub fn arc_center_distance(&self, other: &Point) -> Result<f64, Box<dyn Error>> {
        if self.i == None || self.j == None || self.k == None ||
            other.i == None || other.j == None || other.k == None {
            Err("Arc point missing I, J, or K value")?
        } else {
            Ok((
                (self.i.unwrap() - other.i.unwrap()).powi(2) +
                    (self.j.unwrap() - other.j.unwrap()).powi(2) +
                    (self.k.unwrap() - other.k.unwrap()).powi(2)
            ).sqrt())
        }
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

