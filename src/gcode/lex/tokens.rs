use std::fmt;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f\r]+")]
#[logos(subpattern float = r"[-+]?([0-9]+)?\.?([0-9]+)?")]
#[logos(subpattern letter = r"[A-Z]")]
#[logos(subpattern integer = r"[0-9]+")]
#[logos(subpattern comment = r"\(.*\)")]
pub enum Token {
    #[token(r"%", priority = 3)]
    StartBlock,
    #[token(";\n", priority = 3)]
    #[token("\n", priority = 3)]
    #[token(r";", priority = 3)]
    EndOfBlock,
    #[regex(r"X(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    XPoint(f64),
    #[regex(r"Y(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    YPoint(f64),
    #[regex(r"Z(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    ZPoint(f64),
    #[regex(r"I(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    IPoint(f64),
    #[regex(r"J(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    JPoint(f64),
    #[regex(r"K(?&float)", |lex| lex.slice()[1..].parse::<f64>().map_err(|_| ()), priority = 3)]
    KPoint(f64),
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
    #[token("G90", priority = 3)]
    AbsoluteDistanceMode,
    #[token("G91", priority = 3)]
    IncrementalDistanceMode,
    #[token("G17", priority = 3)]
    XYPlaneSelection,
    #[token("G18", priority = 3)]
    XZPlaneSelection,
    #[token("G19", priority = 3)]
    YZPlaneSelection
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

