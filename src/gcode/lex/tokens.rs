use std::cmp::PartialEq;
use std::fmt;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f\r]+")]
#[logos(subpattern float = r"[-+]?([0-9]+)?\.?([0-9]+)?")]
#[logos(subpattern letter = r"[A-Z]")]
#[logos(subpattern integer = r"[0-9]+")]
#[logos(subpattern comment = r"\(.*\)")]
pub enum Token {
    #[token("%", priority = 3)]
    StartBlock,
    #[token(";\n", priority = 3)]
    #[token("\n", priority = 3)]
    #[token(r";", priority = 3)]
    EndOfBlock,
    #[regex(r"\(([^)]+)\)")]
    Comment,

    #[regex(r"A(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    AAxisPoint(f64),
    #[regex(r"B(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    BAxisPoint(f64),
    #[regex(r"C(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    CAxisPoint(f64),
    #[regex(r"D(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    DValue(f64),
    #[regex(r"E(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    EValue(f64),
    #[regex(r"F(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    Feedrate(f64),
    #[regex(r"H(?&integer)", |lex| lex.slice()[1..].parse::<usize>().map_err(|_| ()), priority = 3)]
    ToolOffsetNumber(usize),
    #[regex(r"I(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    IValue(f64),
    #[regex(r"J(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    JValue(f64),
    #[regex(r"K(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    KValue(f64),
    #[regex(r"L(?&integer)", |lex| lex.slice()[1..].parse::<usize>().map_err(|_| ()), priority = 3)]
    LValue(usize),
    #[regex(r"N(?&integer)", |lex| lex.slice()[1..].parse::<usize>().map_err(|_| ()), priority = 3)]
    LineNumber(usize),
    #[regex(r"O(?&integer)", |lex| lex.slice()[1..].parse::<usize>().map_err(|_| ()), priority = 3)]
    ONumber(usize),
    #[regex(r"P(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    PValue(f64),
    #[regex(r"Q(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    QValue(f64),
    #[regex(r"R(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    RValue(f64),
    #[regex(r"S(?&integer)", |lex| lex.slice()[1..].parse::<isize>().map_err(|_| ()), priority = 3)]
    SpindleSpeed(isize),
    #[regex(r"T(?&integer)", |lex| lex.slice()[1..].parse::<usize>().map_err(|_| ()), priority = 3)]
    ToolNumber(usize),
    #[regex(r"X(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    XPoint(f64),
    #[regex(r"Y(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    YPoint(f64),
    #[regex(r"Z(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    ZPoint(f64),

    #[token("G0", priority = 3)]
    #[token("G00", priority = 3)]
    RapidPositioning,
    #[token("G1", priority = 3)]
    #[token("G01", priority = 3)]
    LinearInterpolation,
    #[token("G2", priority = 3)]
    #[token("G02", priority = 3)]
    CWCircularInterpolation,
    #[token("G3", priority = 3)]
    #[token("G03" , priority = 3)]
    CCWCircularInterpolation,
    #[token("G4", priority = 3)]
    #[token("G04", priority = 3)]
    Dwell,
    #[token("G9", priority = 3)]
    #[token("G09", priority = 3)]
    ExactStop,
    #[token("G17", priority = 3)]
    XYPlaneSelection,
    #[token("G18", priority = 3)]
    XZPlaneSelection,
    #[token("G19", priority = 3)]
    YZPlaneSelection,
    #[token("G20", priority = 3)]
    UnitsInches,
    #[token("G21", priority = 3)]
    UnitsMetric,
    #[token("G28", priority = 3)]
    ReturnMachineZero,
    #[token("G40", priority = 3)]
    CancelCutterCompensation,
    #[token("G43", priority = 3)]
    ToolLengthCompensation,
    #[token("G49", priority = 3)]
    CancelToolLengthCompensation,
    #[token("G54", priority = 3)]
    G54WorkCoordinateSystem,
    #[token("G55", priority = 3)]
    G55WorkCoordinateSystem,
    #[token("G56", priority = 3)]
    G56WorkCoordinateSystem,
    #[token("G57", priority = 3)]
    G57WorkCoordinateSystem,
    #[token("G58", priority = 3)]
    G58WorkCoordinateSystem,
    #[token("G59", priority = 3)]
    G59WorkCoordinateSystem,
    #[token("G80", priority = 3)]
    CancelCannedCycle,
    #[token("G81", priority = 3)]
    DrillCannedCycle,
    #[token("G83", priority = 3)]
    PeckDrillCannedCycle,
    #[token("G90", priority = 3)]
    AbsoluteDistanceMode,
    #[token("G91", priority = 3)]
    IncrementalDistanceMode,
    #[token("G98", priority = 3)]
    InitialPointReturn,
    #[token("G99", priority = 3)]
    RPlaneReturn,

    #[token("M0", priority = 3)]
    #[token("M00", priority = 3)]
    ProgramStop,
    #[token("M1", priority = 3)]
    #[token("M01", priority = 3)]
    OptionalStop,
    #[token("M2", priority = 3)]
    #[token("M02", priority = 3)]
    ProgramEnd,
    #[token("M3", priority = 3)]
    #[token("M03", priority = 3)]
    SpindleOnCW,
    #[token("M4", priority = 3)]
    #[token("M04", priority = 3)]
    SpindleOnCCW,
    #[token("M5", priority = 3)]
    #[token("M05", priority = 3)]
    SpindleStop,
    #[token("M6", priority = 3)]
    #[token("M06", priority = 3)]
    Toolchange,
    #[token("M8", priority = 3)]
    #[token("M08", priority = 3)]
    CoolantOn,
    #[token("M9", priority = 3)]
    #[token("M09", priority = 3)]
    CoolantOff,
    #[token("M30", priority = 3)]
    ProgramEndReset
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


