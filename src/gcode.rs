use std::time::Duration;

pub mod parser;
pub mod util;

trait GCode {
    fn to_string(&self) -> String;

    fn to_royal_string(&self) -> String;
}

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
    a: Option<f32>
}

pub type Incremental = Position;
pub type Absolute = Position;

impl GCode for Position {
    fn to_string(&self) -> String {
        if self.a.is_some() {
            format!("X{} Y{} Z{} A{}", self.x, self.y, self.z, self.a.unwrap())
        } else {
            format!("X{} Y{} Z{}", self.x, self.y, self.z)
        }
    }

    fn to_royal_string(&self) -> String {
        if self.a.is_some() {
            format!("({}, {}, {}, {})", self.x, self.y, self.z, self.a.unwrap())
        } else {
            format!("({}, {}, {})", self.x, self.y, self.z)
        }
    }
}

#[derive(Debug)]
pub enum MotionType {
    Rapid {
        spindle_speed: f32
    },
    Linear {
        feed_rate: f32,
        spindle_speed: f32
    },
    ArcClockwise {
        feed_rate: f32,
        spindle_speed: f32
    },
    ArcCounterClockwise {
        feed_rate: f32,
        spindle_speed: f32
    },
    Dwell {
        spindle_speed: f32,
        duration: Duration
    },
    Drill {
        feed_rate: f32,
        spindle_speed: f32,
        retract: f32,
        depth: f32,
        clean_rpm: Option<f32>
    },
    PeckDrill {
        feed_rate: f32,
        spindle_speed: f32,
        retract: f32,
        depth: f32,
        clean_rpm: Option<f32>,
        first_peck: Option<f32>,
        peck_reduction: Option<f32>,
        dwell: Option<Duration>,
        peck_depth: Option<f32>
    },
    Tap {
        feed_rate: f32,
        spindle_speed: f32,
        retract: f32,
        retract_multiplier: f32,
        depth: f32,
        clean_rpm: Option<f32>
    },
    Engrave {
        feed_rate: f32,
        spindle_speed: f32,
        depth: f32,
        retract: f32,
        text: String,
        angle: f32,
        plunge_rate: f32,
        smoothing: Smoothness,
        corner_rounding: f32
    },
    None
}

#[derive(Debug)]
pub enum Smoothness {
    Rough,
    Medium,
    Finish
}

#[derive(Debug)]
pub enum Units {
    Millimeter,
    Inch
}

#[derive(Debug)]
pub struct Config {
    units: Units,
    plane: Plane,
    cutter_compensation: CutterCompensation,
    coolant: bool,
    tool: u32,
    tool_offset: u32
}

#[derive(Debug)]
pub enum Plane {
    XY,
    XZ,
    YZ
}

#[derive(Debug)]
pub enum CutterCompensation {
    Off,
    Left,
    Right
}

#[derive(Debug)]
pub struct Motion {
    motion_type: MotionType,
    config: Config,
    positions: Vec<Position>
}

#[derive(Debug)]
pub struct Program {
    title: String,
    o: u32,
    // tools: Vec<Tool>,
    motions: Vec<Motion>
}

impl Program {
    fn new() -> Program {
        Program {
            title: String::new(),
            o: 0,
            // tools: Vec::new(),
            motions: Vec::new()
        }
    }
}




