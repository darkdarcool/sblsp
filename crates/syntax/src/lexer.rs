// We use the `BYTE_HANDLERS` table for our parsing, which is just a table of functions
// that correspond to a specific ascii byte value.
//
// Current challenges:
// - For generics, you would need to be able to handle `>>` as `OpBitLshift`, this complicates
//   things, and we may just have to insert two `OpGt` tokens instead. This would just be messier
//   and I would hope to just have the generics part of the parsing be able to just "figure it out"

use crate::ast::{Token, TokenKind};
use crate::source::Source;

/// Function that handles a specific byte value
pub type ByteHandler = Option<for<'alloc> fn(&mut Lexer)>;

/// List of byte handlers for each byte value.
/// Ref: <https://www.freecodecamp.org/news/ascii-table-hex-to-ascii-value-character-code-chart-2/>
#[rustfmt::skip]
pub static BYTE_HANDLERS: [ByteHandler; 256] = [
//   0    1    2    3    4    5    6    7    8    9    A    B    C    D    E    F   //
    EOF, ___, ___, ___, ___, ___, ___, ___, ___, SPS, LNN, ___, ___, ___, ___, ___, // 0
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 1
    SPS, OEM, STR, SHT, IDN, OMD, OAD, CHR, SLP, SRP, OSR, OPS, SCM, OMS, SDT, ODV, // 2
    NUM, NUM, NUM, NUM, NUM, NUM, NUM, NUM, NUM, NUM, SAC, SBC, OLT, OEQ, OGT, SQM, // 3
    SAT, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, // 4
    IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, SLB, SRB, ___, ___, ___, // 5
    ___, LLA, LLB, LLC, LLD, LLE, LLF, IDN, IDN, LLI, IDN, IDN, LLL, LLM, LLN, LLO, // 6
    LLP, IDN, LLR, LLS, LLT, LLU, LLV, LLW, IDN, IDN, IDN, SLC, OVB, SRC, OTE, ___, // 7
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, OCT, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // 8
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // 9
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // A
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // B
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // C
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // D
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // E
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // F
];

/// Identifiers, and special characters like `$` and `_`
pub const IDN: ByteHandler = Some(|lex| {
    lex.identifier_handler();
    // println!("identifier");

    lex.token.kind = TokenKind::Identifier;
});

/// Line new (\n)
pub const LNN: ByteHandler = Some(|lex| {
    lex.bump();
    lex.token.kind = TokenKind::Newline;
});

pub const NUM: ByteHandler = Some(|lex| {
    lex.number_handler();
    lex.token.kind = TokenKind::ValueNumber;
});

pub const STR: ByteHandler = Some(|lex| {
    // let denoter_kind = StrDenoter::from(lex.read_byte() as char);
    lex.string_handler();
    lex.token.kind = TokenKind::ValueString;
});

pub const CHR: ByteHandler = Some(|lex| {
    // let denoter_kind = StrDenoter::from(lex.read_byte() as char);
    lex.char_handler();
    lex.token.kind = TokenKind::ValueChar;
});

/// Whitespace
pub const SPS: ByteHandler = Some(|lex| {
    lex.whitespace_handler();
    lex.token.kind = TokenKind::Whitespace;
});

/// The... end... of... the.. file?
pub const EOF: ByteHandler = Some(|_lex| {
    println!("End of file reached");
});

/// Symbol `#`
pub const SHT: ByteHandler = Some(|lex| {
    lex.token.kind = TokenKind::SymHash;
    lex.bump();
});

/// Symbol `(`
pub const SLP: ByteHandler = Some(|lex| {
    lex.token.kind = TokenKind::BracketLparent;
    lex.bump();
});

/// Symbol `)`
pub const SRP: ByteHandler = Some(|lex| {
    lex.token.kind = TokenKind::BracketRparent;
    lex.bump();
});

/// Symbol `,`
pub const SCM: ByteHandler = Some(|lex| {
    lex.token.kind = TokenKind::SymComma;
    lex.bump();
});

/// Symbol `.`
pub const SDT: ByteHandler = Some(|lex| {
    lex.token.kind = TokenKind::SymDot;
    lex.bump();
});

/// Symbol ALPHA `:` (alpha as in alpha male)
pub const SAC: ByteHandler = Some(|lex| {
    // lex.token.kind = TokenKind::SymColon;
    if lex.read_byte() == b':' {
        lex.token.kind = TokenKind::SymColcol;
        lex.bump();
    } else {
        lex.token.kind = TokenKind::SymColon;
    }
    lex.bump();
});

/// Symbol BETA `;` (beta as in beta male)
pub const SBC: ByteHandler = Some(|lex| {
    lex.token.kind = TokenKind::SymSemiColon;
    lex.bump();
});

/// Symbol `?`
pub const SQM: ByteHandler = Some(|lex| {
    lex.token.kind = TokenKind::SymQuestion;
    lex.bump();
});

/// Symbol `@`
pub const SAT: ByteHandler = Some(|lex| {
    lex.token.kind = TokenKind::SymAt;
    lex.bump();
});

/// Symbol `{`
pub const SLC: ByteHandler = Some(|lex| {
    lex.token.kind = TokenKind::BracketLcurly;
    lex.bump();
});

/// Symbol `}`
pub const SRC: ByteHandler = Some(|lex| {
    lex.token.kind = TokenKind::BracketRcurly;
    lex.bump();
});

/// Symbol `[`
pub const SLB: ByteHandler = Some(|lex| {
    lex.token.kind = TokenKind::BracketLsquared;
    lex.bump();
});

/// Symbol `]`
pub const SRB: ByteHandler = Some(|lex| {
    lex.token.kind = TokenKind::BracketRsquared;
    lex.bump();
});

/// Operator `!` (exclamation mark)
pub const OEM: ByteHandler = Some(|lex| {
    //lex.token.kind = TokenKind::OpNot;
    if lex.read_byte() == b'=' {
        lex.token.kind = TokenKind::OpNoteq;
        lex.bump();
    } else {
        lex.token.kind = TokenKind::OpNot;
    }
    lex.bump();
});

/// Operator `*` (asterisk/star)
pub const OSR: ByteHandler = Some(|lex| {
    //lex.token.kind = TokenKind::OpMul;
    if lex.read_byte() == b'=' {
        lex.token.kind = TokenKind::OpMuleq;
        lex.bump();
    } else {
        lex.token.kind = TokenKind::OpMul;
    }
    lex.bump();
});

/// Operator `+` (plus)
pub const OPS: ByteHandler = Some(|lex| {
    //lex.token.kind = TokenKind::OpPlus;
    if lex.read_byte() == b'=' {
        lex.token.kind = TokenKind::OpPluseq;
        lex.bump();
    } else {
        lex.token.kind = TokenKind::OpPlus;
    }
    lex.bump();
});

/// Operator `-` (minus)
pub const OMS: ByteHandler = Some(|lex| {
    //lex.token.kind = TokenKind::OpMinus;
    if lex.read_byte() == b'=' {
        lex.token.kind = TokenKind::OpMinuseq;
        lex.bump();
    } else if lex.read_byte() == b'>' {
        lex.token.kind = TokenKind::OpArrow;
        lex.bump();
    } else {
        lex.token.kind = TokenKind::OpMinus;
    }
    lex.bump();
});

/// Operator `%` (percent/mod)
pub const OMD: ByteHandler = Some(|lex| {
    //lex.token.kind = TokenKind::OpMod;
    if lex.read_byte() == b'=' {
        lex.token.kind = TokenKind::OpModEq;
        lex.bump();
    } else {
        lex.token.kind = TokenKind::OpMod;
    }
    lex.bump();
});

/// Operator `&` (ampersand/and)
pub const OAD: ByteHandler = Some(|lex| {
    //lex.token.kind = TokenKind::OpAnd;
    if lex.read_byte() == b'&' {
        lex.token.kind = TokenKind::OpAnd;
        lex.bump();
    } else if lex.read_byte() == b'=' {
        lex.token.kind = TokenKind::OpBitAndEq;
        lex.bump();
    } else {
        lex.token.kind = TokenKind::OpBitAnd;
    }
    lex.bump();
});

/// Operator `/` (slash/div)
pub const ODV: ByteHandler = Some(|lex| {
    //lex.token.kind = TokenKind::OpDiv;
    if lex.read_byte() == b'=' {
        lex.token.kind = TokenKind::OpDiveq;
        lex.bump();
    } else {
        lex.token.kind = TokenKind::OpDiv;
    }
    lex.bump();
});

/// Operator `<` (less than)
pub const OLT: ByteHandler = Some(|lex| {
    //lex.token.kind = TokenKind::OpLt;
    if lex.read_byte() == b'=' {
        lex.token.kind = TokenKind::OpLteq;
        lex.bump();
    } else if lex.read_byte() == b'<' {
        // lex.token.kind = TokenKind::OpBitRshift;
        //lex.bump();
        if lex.read_byte() == b'=' {
            lex.token.kind = TokenKind::OpBitLshiftEq;
            lex.bump();
        } else {
            lex.token.kind = TokenKind::OpBitLshift;
        }
    } else {
        lex.token.kind = TokenKind::OpLt;
    }
    lex.bump();
});

/// Operator `=` (equals)
pub const OEQ: ByteHandler = Some(|lex| {
    //lex.token.kind = TokenKind::OpEq;
    if lex.read_byte() == b'=' {
        lex.token.kind = TokenKind::OpEqeq;
        lex.bump();
    } else {
        lex.token.kind = TokenKind::OpEq;
    }
    lex.bump();
});

/// Operator `>` (greater than)
pub const OGT: ByteHandler = Some(|lex| {
    //lex.token.kind = TokenKind::OpGt;
    if lex.read_byte() == b'=' {
        lex.token.kind = TokenKind::OpGteq;
        lex.bump();
    } else if lex.read_byte() == b'>' {
        // lex.token.kind = TokenKind::OpBitRshift;
        lex.bump();
        if lex.read_byte() == b'=' {
            lex.token.kind = TokenKind::OpBitRshiftEq;
            lex.bump();
        } else {
            lex.token.kind = TokenKind::OpBitRshift;
        }
    } else {
        lex.token.kind = TokenKind::OpGt;
    }
    lex.bump();
});

/// Operator `|` (vertical bar)
pub const OVB: ByteHandler = Some(|lex| {
    //lex.token.kind = TokenKind::OpBitOr;
    if lex.read_byte() == b'|' {
        lex.token.kind = TokenKind::OpOr;
        lex.bump();
    } else if lex.read_byte() == b'=' {
        lex.token.kind = TokenKind::OpBitOrEq;
        lex.bump();
    } else {
        lex.token.kind = TokenKind::OpBitOr;
    }
    lex.bump();
});

/// Operator `~` (tilde)
pub const OTE: ByteHandler = Some(|lex| {
    lex.token.kind = TokenKind::OpBitNot;
    lex.bump();
});

/// Operator `^` (caret)
pub const OCT: ByteHandler = Some(|lex| {
    //lex.token.kind = TokenKind::OpBitXor;
    if lex.read_byte() == b'=' {
        lex.token.kind = TokenKind::OpBitXorEq;
        lex.bump();
    } else {
        lex.token.kind = TokenKind::OpBitXor;
    }
    lex.bump();
});

/// Literal lowercase `a`
pub const LLA: ByteHandler = Some(|lex| {
    lex.token.kind = match lex.identifier_handler() {
        "as" => TokenKind::KwordAs,
        "abstract" => TokenKind::KwordAbstract,
        _ => TokenKind::Identifier,
    };
});

/// Literal lowercase `b`
pub const LLB: ByteHandler = Some(|lex| {
    lex.token.kind = match lex.identifier_handler() {
        "break" => TokenKind::KwordBreak,
        _ => TokenKind::Identifier,
    };
});

/// Literal lowercase `c`
pub const LLC: ByteHandler = Some(|lex| {
    lex.token.kind = match lex.identifier_handler() {
        "case" => TokenKind::KwordCase,
        "const" => TokenKind::KwordConst,
        "continue" => TokenKind::KwordContinue,
        "class" => TokenKind::KwordClass,
        "catch" => TokenKind::KwordCatch,
        "constexpr" => TokenKind::KwordConstexpr,
        _ => TokenKind::Identifier,
    };
});

/// Literal lowercase `d`
pub const LLD: ByteHandler = Some(|lex| {
    lex.token.kind = match lex.identifier_handler() {
        "do" => TokenKind::KwordDo,
        "default" => TokenKind::KwordDefault,
        "delete" => TokenKind::KwordDelete,
        _ => TokenKind::Identifier,
    };
});

/// Literal lowercase `e`
pub const LLE: ByteHandler = Some(|lex| {
    lex.token.kind = match lex.identifier_handler() {
        "enum" => TokenKind::KwordEnum,
        "else" => TokenKind::KwordElse,
        "extends" => TokenKind::KwordExtends,
        "external" => TokenKind::KwordExtern,
        _ => TokenKind::Identifier,
    };
});

/// Literal lowercase `f`
pub const LLF: ByteHandler = Some(|lex| {
    lex.token.kind = match lex.identifier_handler() {
        "for" => TokenKind::KwordFor,
        "false" => TokenKind::KWordFalse,
        "final" => TokenKind::KwordFinal,
        "func" => TokenKind::KwordFunc,
        _ => TokenKind::Identifier,
    };
});

/// Literal lowercase `i`
pub const LLI: ByteHandler = Some(|lex| {
    lex.token.kind = match lex.identifier_handler() {
        "if" => TokenKind::KwordIf,
        "interface" => TokenKind::KwordInter,
        "implements" => TokenKind::KwordImplements,
        "import" => TokenKind::KwordImport,
        "inline" => TokenKind::KwordInline,
        _ => TokenKind::Identifier,
    };
});

/// Literal lowercase `l`
pub const LLL: ByteHandler = Some(|lex| {
    lex.token.kind = match lex.identifier_handler() {
        "let" => TokenKind::KwordVar,
        _ => TokenKind::Identifier,
    };
});

/// Literal lowercase `m`
pub const LLM: ByteHandler = Some(|lex| {
    lex.token.kind = match lex.identifier_handler() {
        "macro" => TokenKind::KwordMacro,
        "mut" => TokenKind::KwordMutable,
        _ => TokenKind::Identifier,
    };
});

/// Literal lowercase `n`
pub const LLN: ByteHandler = Some(|lex| {
    lex.token.kind = match lex.identifier_handler() {
        "namespace" => TokenKind::KwordNamespace,
        "new" => TokenKind::KwordNew,
        _ => TokenKind::Identifier,
    };
});

/// Literal lowercase `o`
pub const LLO: ByteHandler = Some(|lex| {
    lex.token.kind = match lex.identifier_handler() {
        "operator" => TokenKind::KwordOperator,
        "override" => TokenKind::KwordOverride,
        _ => TokenKind::Identifier,
    };
});

/// Literal lowercase `p`
pub const LLP: ByteHandler = Some(|lex| {
    lex.token.kind = match lex.identifier_handler() {
        "public" => TokenKind::KwordPublic,
        "private" => TokenKind::KwordPrivate,
        _ => TokenKind::Identifier,
    };
});

/// Literal lowercase `r`
pub const LLR: ByteHandler = Some(|lex| {
    lex.token.kind = match lex.identifier_handler() {
        "return" => TokenKind::KwordReturn,
        _ => TokenKind::Identifier,
    };
});

/// Literal lowercase `s`
pub const LLS: ByteHandler = Some(|lex| {
    lex.token.kind = match lex.identifier_handler() {
        "super" => TokenKind::KwordSuper,
        "static" => TokenKind::KwordStatic,
        "struct" => TokenKind::KwordStruct,
        "switch" => TokenKind::KwordSwitch,
        _ => TokenKind::Identifier,
    };
});

/// Literal lowercase `t`
pub const LLT: ByteHandler = Some(|lex| {
    lex.token.kind = match lex.identifier_handler() {
        "true" => TokenKind::KWordTrue,
        "type" => TokenKind::KwordTypedef,
        "throw" => TokenKind::KwordThrow,
        "try" => TokenKind::KwordTry,
        _ => TokenKind::Identifier,
    };
});

/// Literal lowercase `u`
pub const LLU: ByteHandler = Some(|lex| {
    lex.token.kind = match lex.identifier_handler() {
        "unsafe" => TokenKind::KwordUnsafe,
        _ => TokenKind::Identifier,
    };
});

/// Literal lowercase `v`
pub const LLV: ByteHandler = Some(|lex| {
    lex.token.kind = match lex.identifier_handler() {
        "virtual" => TokenKind::KwordVirtual,
        _ => TokenKind::Identifier,
    };
});

/// Literal lowercase `w`
pub const LLW: ByteHandler = Some(|lex| {
    lex.token.kind = match lex.identifier_handler() {
        "while" => TokenKind::KwordWhile,
        _ => TokenKind::Identifier,
    };
});

pub const UNI: ByteHandler = Some(|lex| {
    // lex.source.advance(1);
    lex.bump();
    // TODO: Whatever this is
});

pub const ___: ByteHandler = None;

pub struct Lexer {
    // allocator: &'alloc Allocator,
    source: Source,
    token: Token,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            source: Source::new(input),
            token: Token::default(),
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.source.is_at_end()
    }

    pub fn next_token(&mut self) -> Token {
        let next_byte = self.read_byte();

        if let Some(handler) = self.handler_from_byte(next_byte) {
            self.token.span.start = self.source.current_pos();
            handler(self);
        } else {
            println!("Unexpected byte: {} [{}]", next_byte as char, next_byte);
            self.bump();
        }

        self.token.span.end = self.source.current_pos();

        let tok = self.token;
        self.token = Token::default();

        tok
    }

    fn read_byte(&self) -> u8 {
        self.source.current()
    }

    fn handler_from_byte(&self, byte: u8) -> ByteHandler {
        unsafe { *(&BYTE_HANDLERS as *const ByteHandler).offset(byte as isize) }
    }

    #[inline]
    fn bump(&mut self) {
        self.source.advance_ptr();
    }
}

// Identifiers
impl Lexer {
    pub(super) fn identifier_handler<'a>(&mut self) -> &'a str {
        let start = self.source.current_pos();

        while !self.is_at_end() {
            let byte = self.read_byte();

            if byte.is_ascii_alphanumeric() && (byte as char) != ' ' || byte == b'_' {
                self.bump();
            } else {
                break;
            }
        }

        &self.source.get_slice(start, self.source.get_current_pos())
    }
}

// Whitespace
impl Lexer {
    pub(super) fn whitespace_handler<'a>(&mut self) -> &'a str {
        let start = self.source.current_pos();

        while !self.is_at_end() {
            let byte = self.read_byte();

            if self.is_whitespace(byte) {
                self.bump();
            } else {
                break;
            }
        }

        &self.source.get_slice(start, self.source.get_current_pos())
    }

    fn is_whitespace(&self, byte: u8) -> bool {
        byte == 32 || byte == 9
    }
}

// Numbers
impl Lexer {
    pub(super) fn number_handler<'a>(&mut self) -> &'a str {
        let start = self.source.current_pos();
        let mut has_had_dot = false;

        while !self.is_at_end() {
            let byte = self.read_byte();

            //if byte.is_ascii_digit() {
            //    self.bump()
            //} else if byte == b'.' && has_had_dot {
            //    self.bump();
            //    has_had_dot = true;
            //}

            if byte.is_ascii_digit() || (byte == b'.' && !has_had_dot) {
                self.bump();
                if byte == b'.' {
                    has_had_dot = true;
                }
            } else {
                break;
            }
        }

        &self.source.get_slice(start, self.source.get_current_pos())
    }
}

// Strings
impl Lexer {
    pub(super) fn string_handler<'a>(&mut self) -> &'a str {
        let start = self.source.current_pos();

        while !self.is_at_end() {
            let byte = self.read_byte();

            if byte == b'\\' {
                self.bump(); // move past the backslash
                             // TODO: Make an actual escape sequence handler - this is just a place holder
                self.bump(); // move past the escaped character
            }

            if byte != b'"' {
                self.bump();
            } else {
                break;
            }
        }

        // move past the last quote
        self.bump();

        &self.source.get_slice(start, self.source.get_current_pos())
    }

    pub(super) fn char_handler<'a>(&mut self) -> &'a str {
        let start = self.source.current_pos();

        self.bump(); // move past the first quote

        let next_byte = self.read_byte();

        if next_byte == b'\\' {
            self.bump(); // move past the backslash
            self.bump();
        } else {
            self.bump();
        }

        // move past the last quote
        self.bump();

        &self.source.get_slice(start, self.source.get_current_pos())
    }
}
