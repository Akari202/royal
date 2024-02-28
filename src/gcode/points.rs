use std::error::Error;
use std::fmt::Debug;
use crate::gcode::lex::Lexer;
use crate::gcode::lex::tokens::Token::*;

#[derive(Debug, Clone)]
pub struct Program {
    points: Vec<Box<dyn PointList>>
}

trait PointList {}
trait Point {
    fn set_x(&mut self, x: f64) -> Result<(), Box<dyn Error>>;
    fn set_y(&mut self, y: f64) -> Result<(), Box<dyn Error>>;
    fn set_z(&mut self, z: f64) -> Result<(), Box<dyn Error>>;
    fn set_i(&mut self, i: f64) -> Result<(), Box<dyn Error>>;
    fn set_j(&mut self, j: f64) -> Result<(), Box<dyn Error>>;
    fn set_k(&mut self, k: f64) -> Result<(), Box<dyn Error>>;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn i(&self) -> Result<f64, Box<dyn Error>>;
    fn j(&self) -> Result<f64, Box<dyn Error>>;
    fn k(&self) -> Result<f64, Box<dyn Error>>;
    fn from_linear_point(&mut self, point: LinearPoint);
    fn from_arc_point(&mut self, point: ArcPoint);
}

#[derive(Debug, Clone)]
struct LinearPointList {
    pub points: Vec<LinearPoint>,
    linear_type: LinearInterpolation
}

#[derive(Debug, Clone)]
enum LinearInterpolation {
    Rapid,
    Feed
}

#[derive(Clone)]
struct LinearPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Clone)]
struct ArcPointList {
    pub points: Vec<ArcPoint>,
    sense: ArcSense
}

#[derive(Clone)]
struct ArcPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub i: f64,
    pub j: f64,
    pub k: f64
}

#[derive(Debug, Clone)]
enum ArcSense {
    CW,
    CCW
}

#[derive(Debug, Clone)]
enum DistanceMode {
    Absolute,
    Incremental
}

#[derive(Debug, Clone)]
enum PointType {
    Linear,
    Arc
}

impl Program {
    pub fn from_lex(mut lex: &Lexer) -> Result<Self, Box<dyn Error>> {
        let mut points = Vec::new();
        let mut point_list: Option<Box<dyn PointList>> = None;
        let mut point: Option<Box<dyn Point>> = None;
        let mut new_line = false;
        let mut distance_mode = DistanceMode::Absolute;
        let mut point_type = PointType::Linear;
        loop {
            if let Some(token) = lex.next() {
                if let Ok(token) = token {
                    match token.1 {
                        RapidPositioning => {
                            if let Some(point_list) = point_list {
                                points.push(point_list);
                            }
                            point_list = Some(Box::new(LinearPointList::new(LinearInterpolation::Rapid)));
                        },
                        LinearInterpolation => {
                            if let Some(point_list) = point_list {
                                points.push(point_list);
                            }
                            point_list = Some(Box::new(LinearPointList::new(LinearInterpolation::Feed)));
                        },
                        CWCircularInterpolation => {
                            if let Some(point_list) = point_list {
                                points.push(point_list);
                            }
                            point_list = Some(Box::new(ArcPointList::new(ArcSense::CW)));
                        },
                        CCWCircularInterpolation => {
                            if let Some(point_list) = point_list {
                                points.push(point_list);
                            }
                            point_list = Some(Box::new(ArcPointList::new(ArcSense::CCW)));
                        },
                        EndOfBlock => {
                            if let Some(mut point) = point {
                                if let Some(point_list) = point_list {
                                    point_list.push(point.clone());
                                }
                            }
                        },
                        XPoint(x) => {
                            if let Some(mut point) = point {
                                match distance_mode {
                                    DistanceMode::Absolute => point.set_x(x)?,
                                    DistanceMode::Incremental => point.set_x(point.x() + x)?
                                }
                            } else {
                                match point_type {
                                    PointType::Linear => point = Some(Box::new(LinearPoint::new(x, 0.0, 0.0))),
                                    PointType::Arc => point = Some(Box::new(ArcPoint::new(x, 0.0, 0.0, 0.0, 0.0, 0.0)))
                                }
                            }
                        },
                        YPoint(y) => {
                            if let Some(mut point) = point {
                                match distance_mode {
                                    DistanceMode::Absolute => point.set_y(y)?,
                                    DistanceMode::Incremental => point.set_y(point.y() + y)?
                                }
                            } else {
                                match point_type {
                                    PointType::Linear => point = Some(Box::new(LinearPoint::new(0.0, y, 0.0))),
                                    PointType::Arc => point = Some(Box::new(ArcPoint::new(0.0, y, 0.0, 0.0, 0.0, 0.0)))
                                }
                            }
                        },
                        ZPoint(z) => {
                            if let Some(mut point) = point {
                                match distance_mode {
                                    DistanceMode::Absolute => point.set_z(z)?,
                                    DistanceMode::Incremental => point.set_z(point.z() + z)?
                                }
                            } else {
                                match point_type {
                                    PointType::Linear => point = Some(Box::new(LinearPoint::new(0.0, 0.0, z))),
                                    PointType::Arc => point = Some(Box::new(ArcPoint::new(0.0, 0.0, z, 0.0, 0.0, 0.0)))
                                }
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }
        Ok(Self { points })
    }
}

impl LinearPointList {
    pub fn new(interpolation: LinearInterpolation) -> Self {
        Self { points: Vec::new(), linear_type: interpolation }
    }
}

impl ArcPointList {
    pub fn new(sense: ArcSense) -> Self {
        Self { points: Vec::new(), sense }
    }
}

impl Point for LinearPoint {
    fn set_x(&mut self, x: f64) -> Result<(), Box<dyn Error>> {
        self.x = x;
        Ok(())
    }

    fn set_y(&mut self, y: f64) -> Result<(), Box<dyn Error>> {
        self.y = y;
        Ok(())
    }

    fn set_z(&mut self, z: f64) -> Result<(), Box<dyn Error>> {
        self.z = z;
        Ok(())
    }

    fn set_i(&mut self, i: f64) -> Result<(), Box<dyn Error>> {
        Err("LinearPoint does not have an i value")?
    }

    fn set_j(&mut self, j: f64) -> Result<(), Box<dyn Error>> {
        Err("LinearPoint does not have a j value")?
    }

    fn set_k(&mut self, k: f64) -> Result<(), Box<dyn Error>> {
        Err("LinearPoint does not have a k value")?
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn i(&self) -> Result<f64, Box<dyn Error>> {
        Err("LinearPoint does not have an i value")?
    }

    fn j(&self) -> Result<f64, Box<dyn Error>> {
        Err("LinearPoint does not have a j value")?
    }

    fn k(&self) -> Result<f64, Box<dyn Error>> {
        Err("LinearPoint does not have a k value")?
    }

    fn from_linear_point(&mut self, point: LinearPoint) {
        self.x = point.x;
        self.y = point.y;
        self.z = point.z;
    }

    fn from_arc_point(&mut self, point: ArcPoint) {
        self.x = point.x;
        self.y = point.y;
        self.z = point.z;
    }
}

impl Point for ArcPoint {
    fn set_x(&mut self, x: f64) -> Result<(), Box<dyn Error>> {
        self.x = x;
        Ok(())
    }

    fn set_y(&mut self, y: f64) -> Result<(), Box<dyn Error>> {
        self.y = y;
        Ok(())
    }

    fn set_z(&mut self, z: f64) -> Result<(), Box<dyn Error>> {
        self.z = z;
        Ok(())
    }

    fn set_i(&mut self, i: f64) -> Result<(), Box<dyn Error>> {
        self.i = i;
        Ok(())
    }

    fn set_j(&mut self, j: f64) -> Result<(), Box<dyn Error>> {
        self.j = j;
        Ok(())
    }

    fn set_k(&mut self, k: f64) -> Result<(), Box<dyn Error>> {
        self.k = k;
        Ok(())
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn i(&self) -> Result<f64, Box<dyn Error>> {
        Ok(self.i)
    }

    fn j(&self) -> Result<f64, Box<dyn Error>> {
        Ok(self.j)
    }

    fn k(&self) -> Result<f64, Box<dyn Error>> {
        Ok(self.k)
    }

    fn from_linear_point(&mut self, point: LinearPoint) {
        self.x = point.x;
        self.y = point.y;
        self.z = point.z;
        self.i = 0.0;
        self.j = 0.0;
        self.k = 0.0;
    }

    fn from_arc_point(&mut self, point: ArcPoint) {
        self.x = point.x;
        self.y = point.y;
        self.z = point.z;
        self.i = point.i;
        self.j = point.j;
        self.k = point.k;
    }
}

impl PointList for LinearPointList {}

impl PointList for ArcPointList {}

impl Debug for LinearPointList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LinearPointList {{ points: {:?}, linear_type: {:?} }}", self.points, self.linear_type)
    }
}

impl Debug for ArcPointList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArcPointList {{ points: {:?}, sense: {:?} }}", self.points, self.sense)
    }
}

impl Debug for LinearPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LinearPoint {{ x: {}, y: {}, z: {} }}", self.x, self.y, self.z)
    }
}

impl Debug for ArcPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArcPoint {{ x: {}, y: {}, z: {}, i: {}, j: {}, k: {} }}", self.x, self.y, self.z, self.i, self.j, self.k)
    }
}
