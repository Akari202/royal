use std::error::Error;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use log::{debug, info};
use logos::Logos;
use nestify::nest;
use vec_utils::vec3d::Vec3d;
use crate::gcode::lex::tokens::Token;
use crate::gcode::lex::tokens::Token::*;
use thiserror::Error;
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



nest! {
    #[derive(Debug, Clone)]*
    pub struct Program {
        pub operation_number: usize,
        pub toolpaths: Vec<pub struct Toolpath {
            #>[derive(Copy, PartialEq)]
            pub toolpath_type: pub enum ToolpathType {
                Drill,
                Mill,
                None
            },
            #>[derive(Copy, PartialEq)]
            pub spindle: pub struct Spindle(isize),
            #>[derive(Copy, PartialEq)]
            pub coolant: pub struct Coolant(bool),
            pub tool_number: usize,
            pub tool_offset_number: usize,
            #>[derive(Copy, PartialEq)]
            pub work_coordinate_system: pub enum WorkCoordinateSystem {
                G54,
                G55,
                G56,
                G57,
                G58,
                G59,
                G110,
                G111,
                G112,
                G113,
                G114,
                G115,
                G116,
                G117,
                G118,
                G119,
                G120,
                G121,
                G122,
                G123,
                G124,
                G125,
                G126,
                G127,
                G128,
                G129
            },
            #>[derive(Copy, PartialEq)]
            pub points: Vec<pub struct Point {
                #>[derive(Copy, PartialEq)]
                pub point_type: pub enum PointType {
                    ArcCW,
                    ArcCCW,
                    Feed,
                    Rapid,
                    None,
                    Drill,
                    Dwell
                },
                pub point: Option<Vec3d>,
                pub multiaxis_point: Option<Vec3d>,
                pub center: Option<Vec3d>,
                pub feedrate: Option<f64>,
                pub dwell_time: Option<f64>,
                pub line_number: Option<usize>,
                #>[derive(Copy, PartialEq)]
                pub plane: Option<pub enum Plane {
                    XY,
                    YZ,
                    XZ
                }>
            }>
        }>
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum DistanceMode {
    Absolute,
    Incremental
}

impl Program {
    pub fn new() -> Self {
        Self {
            operation_number: 0,
            toolpaths: vec![]
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let mut nc = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut nc)?;
        Self::from_str(nc.as_str())
    }

    pub fn from_str(nc: &str) -> Result<Self, Box<dyn Error>> {
        let perf_start = std::time::Instant::now();
        let mut lex = Token::lexer(nc);
        let mut program = Program::new();
        'ONumber: loop {
            if let Some(token) = lex.next() {
                if let Ok(token) = token {
                    debug!("Token: {:?}", token);
                    match token {
                        // EndOfBlock => { }
                        StartBlock => { }
                        ONumber(num) => {
                            program.operation_number = num;
                            break 'ONumber;
                        }
                        _ => {
                            return Err(ProgramError::MissingONumber.into());
                        }
                    }
                } else {
                    return Err(ProgramError::UnknownToken(format!("{}", lex.slice())).into());
                }
            }
        }

        let mut selected_plane: Plane;
        let mut toolpath = Toolpath::new();
        let mut point = Point::new();
        let mut distance_mode: DistanceMode;
        'Toolpath: loop {
            if let Some(token) = lex.next() {
                if let Ok(token) = token {
                    debug!("Token: {:?}", token);
                    match token {
                        StartBlock => {
                            break 'Toolpath;
                        }
                        EndOfBlock => {}
                        Comment => {}
                        AAxisPoint(a) => {
                            match point.multiaxis_point {
                                Some(mut value) => {
                                    value.x = a;
                                }
                                None => {
                                    point.multiaxis_point = Some(Vec3d::new(a, 0.0, 0.0));
                                }
                            }
                        }
                        BAxisPoint(b) => {
                            match point.multiaxis_point {
                                Some(mut value) => {
                                    value.y = b;
                                }
                                None => {
                                    point.multiaxis_point = Some(Vec3d::new(0.0, b, 0.0));
                                }
                            }
                        }
                        CAxisPoint(c) => {
                            match point.multiaxis_point {
                                Some(mut value) => {
                                    value.z = c;
                                }
                                None => {
                                    point.multiaxis_point = Some(Vec3d::new(0.0, 0.0, c));
                                }
                            }
                        }
                        DValue(_) => {}
                        EValue(_) => {}
                        Feedrate(_) => {}
                        ToolOffsetNumber(_) => {}
                        IValue(_) => {}
                        JValue(_) => {}
                        KValue(_) => {}
                        LValue(_) => {}
                        LineNumber(n) => {
                            point.line_number = Some(n);
                        }
                        // ONumber(_) => {}
                        PValue(_) => {}
                        QValue(_) => {}
                        RValue(_) => {}
                        SpindleSpeed(_) => {}
                        ToolNumber(t) => {}
                        XPoint(_) => {}
                        YPoint(_) => {}
                        ZPoint(_) => {}
                        RapidPositioning => {}
                        LinearInterpolation => {}
                        CWCircularInterpolation => {}
                        CCWCircularInterpolation => {}
                        Dwell => {}
                        ExactStop => {}
                        XYPlaneSelection => {}
                        XZPlaneSelection => {}
                        YZPlaneSelection => {}
                        UnitsInches => {}
                        UnitsMetric => {}
                        ReturnMachineZero => {}
                        CancelCutterCompensation => {}
                        ToolLengthCompensation => {}
                        CancelToolLengthCompensation => {}
                        G54WorkCoordinateSystem => {}
                        G55WorkCoordinateSystem => {}
                        G56WorkCoordinateSystem => {}
                        G57WorkCoordinateSystem => {}
                        G58WorkCoordinateSystem => {}
                        G59WorkCoordinateSystem => {}
                        CancelCannedCycle => {}
                        DrillCannedCycle => {}
                        PeckDrillCannedCycle => {}
                        AbsoluteDistanceMode => {
                            distance_mode = DistanceMode::Absolute;
                        }
                        IncrementalDistanceMode => {
                            distance_mode = DistanceMode::Incremental;
                        }
                        InitialPointReturn => {}
                        RPlaneReturn => {}
                        ProgramStop => {}
                        OptionalStop => {}
                        ProgramEnd => {
                            break 'Toolpath;
                        }
                        SpindleOnCW => {}
                        SpindleOnCCW => {}
                        SpindleStop => {}
                        Toolchange => {}
                        CoolantOn => {}
                        CoolantOff => {}
                        ProgramEndReset => {}
                        _ => {
                            return Err(ProgramError::SomethingWeirdHappened.into())
                        }
                    }
                } else {
                    return Err(ProgramError::UnknownToken(format!("{}", lex.slice())).into());
                }
            }
        }
        Ok(program)
    }
}

impl Toolpath {
    pub fn new() -> Self {
        Self {
            toolpath_type: ToolpathType::None,
            spindle: Spindle(0),
            coolant: Coolant(false),
            tool_number: 0,
            tool_offset_number: 0,
            work_coordinate_system: WorkCoordinateSystem::G54,
            points: vec![]
        }
    }
}

impl Point {
    pub fn new() -> Self {
        Self {
            point_type: PointType::None,
            point: None,
            multiaxis_point: None,
            center: None,
            feedrate: None,
            dwell_time: None,
            line_number: None,
            plane: None
        }
    }

    pub fn arc_radius(&self) -> Result<f64, PointError> {
        if self.point_type != PointType::ArcCW && self.point_type != PointType::ArcCCW {
            Err(PointError::NotAnArc)
        } else {
            Self::inplane_distance(
                &self.point.ok_or(PointError::NoCenter)?,
                &self.center.ok_or(PointError::NoPoint)?,
                self.plane.ok_or(PointError::NoPlane)?
            )
        }
    }

    pub fn inplane_arc_center_distance(&self, other: &Point) -> Result<f64, PointError> {
        if self.plane != other.plane {
            Err(PointError::MismatchedPlanes)
        } else {
            Self::inplane_distance(
                &self.center.ok_or(PointError::NoCenter)?,
                &other.center.ok_or(PointError::NoCenter)?,
                self.plane.ok_or(PointError::NoPlane)?
            )
        }
    }

    // TODO: move to vec-utils as Vec3d member function or maybe plane, that might make more sense
    pub fn inplane_distance(point1: &Vec3d, point2: &Vec3d, plane: Plane) -> Result<f64, PointError> {
        let normal = plane.normal();
        Ok(point1.project_onto_plane(&normal).distance_to(&point2.project_onto_plane(&normal)))
    }
}

impl Plane {
    pub fn normal(&self) -> Vec3d {
        match self {
            Plane::XY => { Vec3d::k() }
            Plane::YZ => { Vec3d::i() }
            Plane::XZ => { Vec3d::j() }
        }
    }
}

#[derive(Error, Debug)]
pub enum PointError {
    #[error("the provided point is not an arc")]
    NotAnArc,
    #[error("the provided point does not have a plane specified")]
    NoPlane,
    #[error("the provided point is missing a point value")]
    NoPoint,
    #[error("the provided point is missing a center of rotation")]
    NoCenter,
    #[error("the provided point does not have a type specified")]
    NoPointType,
    #[error("the provided points arent in the same plane")]
    MismatchedPlanes
}

#[derive(Error, Debug)]
pub enum ProgramError {
    #[error("the provided program does not contain an O number")]
    MissingONumber,
    #[error("unrecognised token found: {0}")]
    UnknownToken(String),
    #[error("i dont know")]
    SomethingWeirdHappened
}

mod gcode {
    use thiserror::Error;
    use crate::gcode::program::{Coolant, Plane, PointType, Spindle, WorkCoordinateSystem};

    pub trait GCodeAble {
        fn gcode(&self) -> Result<String, GCodeError>;
    }

    #[derive(Error, Debug)]
    pub enum GCodeError {
        #[error("the point type is set to none")]
        PointNoneType
    }

    impl GCodeAble for Plane {
        fn gcode(&self) -> Result<String, GCodeError> {
            match self {
                Plane::XY => { Ok("G17".into()) }
                Plane::YZ => { Ok("G19".into()) }
                Plane::XZ => { Ok("G18".into()) }
            }
        }
    }

    impl GCodeAble for PointType {
        fn gcode(&self) -> Result<String, GCodeError> {
            match self {
                PointType::ArcCW => { Ok("G2".into()) }
                PointType::ArcCCW => { Ok("G3".into()) }
                PointType::Feed => { Ok("G1".into()) }
                PointType::Rapid => { Ok("G0".into()) }
                PointType::None => { Err(GCodeError::PointNoneType) }
                PointType::Drill => { Ok("G81".into()) }
                PointType::Dwell => { Ok("G4".into()) }
            }
        }
    }

    impl GCodeAble for WorkCoordinateSystem {
        fn gcode(&self) -> Result<String, GCodeError> {
            match self {
                WorkCoordinateSystem::G54 => { Ok("G54".into()) }
                WorkCoordinateSystem::G55 => { Ok("G55".into()) }
                WorkCoordinateSystem::G56 => { Ok("G56".into()) }
                WorkCoordinateSystem::G57 => { Ok("G57".into()) }
                WorkCoordinateSystem::G58 => { Ok("G58".into()) }
                WorkCoordinateSystem::G59 => { Ok("G59".into()) }
                WorkCoordinateSystem::G110 => { Ok("G110".into()) }
                WorkCoordinateSystem::G111 => { Ok("G111".into()) }
                WorkCoordinateSystem::G112 => { Ok("G112".into()) }
                WorkCoordinateSystem::G113 => { Ok("G113".into()) }
                WorkCoordinateSystem::G114 => { Ok("G114".into()) }
                WorkCoordinateSystem::G115 => { Ok("G115".into()) }
                WorkCoordinateSystem::G116 => { Ok("G116".into()) }
                WorkCoordinateSystem::G117 => { Ok("G117".into()) }
                WorkCoordinateSystem::G118 => { Ok("G118".into()) }
                WorkCoordinateSystem::G119 => { Ok("G119".into()) }
                WorkCoordinateSystem::G120 => { Ok("G120".into()) }
                WorkCoordinateSystem::G121 => { Ok("G121".into()) }
                WorkCoordinateSystem::G122 => { Ok("G122".into()) }
                WorkCoordinateSystem::G123 => { Ok("G123".into()) }
                WorkCoordinateSystem::G124 => { Ok("G124".into()) }
                WorkCoordinateSystem::G125 => { Ok("G125".into()) }
                WorkCoordinateSystem::G126 => { Ok("G126".into()) }
                WorkCoordinateSystem::G127 => { Ok("G127".into()) }
                WorkCoordinateSystem::G128 => { Ok("G128".into()) }
                WorkCoordinateSystem::G129 => { Ok("G129".into()) }
            }
        }
    }

    impl GCodeAble for Coolant {
        fn gcode(&self) -> Result<String, GCodeError> {
            match self.0 {
                true => { Ok("M8".into()) }
                false => { Ok("M9".into()) }
            }
        }
    }

    impl GCodeAble for Spindle {
        fn gcode(&self) -> Result<String, GCodeError> {
            if self.0 == 0 {
                Ok("M5".into())
            } else if self.0 > 0 {
                Ok(format!("M4 S{}", self.0))
            } else {
                Ok(format!("M3 S{}", self.0.abs()))
            }
        }
    }
}
