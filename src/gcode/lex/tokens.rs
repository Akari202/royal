use std::fmt;
use std::ops::Range;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f\r]+")]
#[logos(subpattern float = r"[-+]?([0-9]+)?\.?([0-9]+)?")]
#[logos(subpattern letter = r"[A-Z]")]
#[logos(subpattern integer = r"[0-9]+")]
#[logos(subpattern comment = r"\(.*\)")]
pub enum Token {
    #[token(r"%")]
    StartBlock,
    #[token(r";")]
    EndOfBlock,
    // #[token(r"/")]
    // BlockDelete,
    #[regex(r"O(?&integer)", |lex| lex.slice()[1..].parse::<usize>().map_err(|_| ()))]
    ONumber(usize),
    #[regex(r"N(?&integer)", |lex| lex.slice()[1..].parse::<usize>().map_err(|_| ()))]
    LineNumber(usize),
    #[regex(r"(?&letter)(?&float)", |lex| lex.span())]
    #[regex(r"(?&letter)(?:\#(?&integer))", |lex| lex.span())]
    LetterCode(Range<usize>),
    #[regex(r"(?&integer)", |lex| lex.slice().parse::<usize>().map_err(|_| ()))]
    Integer(usize),
    #[regex(r"\#(?&integer)", |lex| lex.slice()[1..].parse::<usize>().map_err(|_| ()))]
    Variable(usize),
    #[regex(r"\([^)]*\)")]
    Comment,
    #[token(r"[")]
    LeftBracket,
    #[token(r"]")]
    RightBracket,
    #[token(r"{")]
    LeftCurly,
    #[token(r"}")]
    RightCurly,
    #[token(r"EQ")]
    LogicalEquals,
    #[token(r"NE")]
    LogicalNotEquals,
    #[token(r"AND")]
    LogicalAnd,
    #[token(r"OR")]
    LogicalOr,
    #[token(r"XOR")]
    LogicalXor,
    #[token(r"LT")]
    LogicalLessThan,
    #[token(r"GT")]
    LogicalGreaterThan,
    #[token(r"LE")]
    LogicalLessThanOrEqual,
    #[token(r"GE")]
    LogicalGreaterThanOrEqual,
    #[token(r"+")]
    MathematicalAdd,
    #[token(r"-")]
    MathematicalSubtract,
    #[token(r"*")]
    MathematicalMultiply,
    #[token(r"/")]
    MathematicalDivide,
    #[token(r"MOD")]
    MathematicalModulo,
    #[token(r"**")]
    MathematicalPower,
    #[token(r"ABS")]
    AbsoluteValue,
    #[token(r"EXISTS")]
    Exists,
    #[token(r"TAN")]
    Tangent,
    #[token(r"ATAN")]
    ArcTangent,
    #[token(r"ACOS")]
    ArcCosine,
    #[token(r"ASIN")]
    ArcSine,
    #[token(r"COS")]
    Cosine,
    #[token(r"SIN")]
    Sine,
    #[token(r"SQRT")]
    SquareRoot,
    #[token(r"LN")]
    NaturalLogarithm,
    #[token(r"ROUND")]
    Round,
    #[token(r"FIX")]
    Floor,
    #[token(r"FUP")]
    Ceiling,
    #[token(r"EXP")]
    Exponential,
    #[token(r"GOTO")]
    GoTo,
    #[token(r"IF")]
    If,
    #[token(r"WHILE")]
    While,
    #[regex(r"DO(?&integer)", |lex| lex.slice()[2..].parse::<u8>().map_err(|_| ()))]
    Do(u8),
    #[regex(r"END(?&integer)", |lex| lex.slice()[3..].parse::<u8>().map_err(|_| ()))]
    End(u8),
    #[token(r"=")]
    Assign
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Token::LetterCode(range) = self {
            write!(f, "Letter Code: {:?}", &range)
        } else {
            write!(f, "{:?}", self)
        }
    }
}

