#[derive(Debug, Default, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

//#[allow(dead_code)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // Identifiers
    Identifier,

    // Whitespace
    Whitespace,

    // Values
    ValueNumber, // Can hold both integers and floats
    // ValueBool,
    ValueString,
    ValueChar,

    // Symbols
    SymAt,
    SymDot,
    SymHash,
    SymComma,
    SymColon,
    SymColcol,
    // Maybe we can get rid of this in favor of it being an identifier?
    // SymDollar,
    SymQuestion,
    SymSemiColon,

    // Brackets
    BracketLcurly,
    BracketRcurly,
    BracketLparent,
    BracketRparent,
    BracketRsquared,
    BracketLsquared,

    // Operators
    OpMul,
    OpMod,
    OpDiv,
    OpPlus,
    OpMinus,
    OpMuleq, // Holds the value for assignment
    OpDiveq,
    OpModEq,
    OpPluseq,
    OpMinuseq,
    OpGt,
    OpLt,
    OpArrow,
    OpEqeq,
    OpGteq,
    OpLteq,
    OpNoteq,
    OpEq,
    OpNot,
    OpAnd,
    OpOr,
    OpBitNot,
    OpBitOr,
    OpBitAnd,
    OpBitXor,
    OpBitOrEq, // Holds the value for assignment (assuming bitwise OR)
    OpBitRshift,
    OpBitLshift,
    OpBitAndEq,
    OpBitXorEq,
    OpBitRshiftEq,
    OpBitLshiftEq,

    // Keywords
    KWordTrue, // *
    KWordFalse,
    //KwordStartPoint,
    KwordIf,
    KwordVar,
    KwordNew,
    KwordThrow,
    KwordFor,
    KwordEnum,
    KwordFunc,
    KwordOperator,
    KwordMacro,
    KwordElse,
    KwordBreak,
    KwordConst,
    KwordSuper,
    KwordWhile,
    KwordExtern,
    KwordVirtual,
    KwordOverride,
    KwordClass,
    KwordAs,
    KwordImport,
    KwordUnsafe,
    KwordConstexpr,
    KwordTypedef,
    KwordMutable,
    KwordDo,
    KwordNamespace,
    KwordStruct,
    KwordPublic,
    KwordCase,
    KwordSwitch,
    KwordStatic,
    KwordReturn,
    KwordPrivate,
    KwordDefault,
    KwordTry,
    KwordCatch,
    KwordContinue,
    KwordInter,
    KwordExtends,
    KwordImplements,
    KwordAbstract,
    KwordFinal,
    KwordInline,
    KwordDelete,
    //KwordEndingPoint,

    // Special markers
    Eof,
    #[default]
    Unknown,
}

/// Representing a token in the source code
///
/// To get the actual value of the token, you can access its [`Span`] field and use it to get a slice of the source code
#[derive(Debug, Default, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new() -> Self {
        Token {
            kind: TokenKind::Unknown,
            span: Span { start: 0, end: 0 },
        }
    }
}
