use crate::{
    ast::{self, Located, Location},
    error::{LexicalError, ParseErrorType},
    lexer::LexResult,
    mode::Mode,
    token::{StringKind, Tok},
};
use num_bigint::BigInt;

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<PegTok>,
    locations: Vec<(Location, Location)>,
    names: Vec<String>,
    ints: Vec<BigInt>,
    floats: Vec<f64>,
    complexes: Vec<(f64, f64)>,
    strings: Vec<(String, StringKind, bool)>,
    // comments: Vec<String>,
}

impl Parser {
    pub fn from(lexer: impl IntoIterator<Item = LexResult>) -> Result<Self, LexicalError> {
        let mut tokens = vec![];
        let mut locations = vec![];
        let mut names = vec![];
        let mut ints = vec![];
        let mut floats = vec![];
        let mut complexes = vec![];
        let mut strings = vec![];
        // let mut comments = vec![];

        for tok in lexer {
            let (begin, tok, end) = tok?;
            let tok = match tok {
                Tok::Name { name } => {
                    names.push(name);
                    PegTok::Name((names.len() - 1) as u32)
                }
                Tok::Int { value } => {
                    ints.push(value);
                    PegTok::Int((ints.len() - 1) as u32)
                }
                Tok::Float { value } => {
                    floats.push(value);
                    PegTok::Float((floats.len() - 1) as u32)
                }
                Tok::Complex { real, imag } => {
                    complexes.push((real, imag));
                    PegTok::Complex((complexes.len() - 1) as u32)
                }
                Tok::String {
                    value,
                    kind,
                    triple_quoted,
                } => {
                    strings.push((value, kind, triple_quoted));
                    PegTok::String((strings.len() - 1) as u32)
                }
                Tok::Newline => PegTok::Newline,
                Tok::Indent => PegTok::Indent,
                Tok::Dedent => PegTok::Dedent,
                Tok::EndOfFile => PegTok::EndOfFile,
                Tok::Lpar => PegTok::Lpar,
                Tok::Rpar => PegTok::Rpar,
                Tok::Lsqb => PegTok::Lsqb,
                Tok::Rsqb => PegTok::Rsqb,
                Tok::Colon => PegTok::Colon,
                Tok::Comma => PegTok::Comma,
                Tok::Semi => PegTok::Semi,
                Tok::Plus => PegTok::Plus,
                Tok::Minus => PegTok::Minus,
                Tok::Star => PegTok::Star,
                Tok::Slash => PegTok::Slash,
                Tok::Vbar => PegTok::Vbar,
                Tok::Amper => PegTok::Amper,
                Tok::Less => PegTok::Less,
                Tok::Greater => PegTok::Greater,
                Tok::Equal => PegTok::Equal,
                Tok::Dot => PegTok::Dot,
                Tok::Percent => PegTok::Percent,
                Tok::Lbrace => PegTok::Lbrace,
                Tok::Rbrace => PegTok::Rbrace,
                Tok::EqEqual => PegTok::EqEqual,
                Tok::NotEqual => PegTok::NotEqual,
                Tok::LessEqual => PegTok::LessEqual,
                Tok::GreaterEqual => PegTok::GreaterEqual,
                Tok::Tilde => PegTok::Tilde,
                Tok::CircumFlex => PegTok::CircumFlex,
                Tok::LeftShift => PegTok::LeftShift,
                Tok::RightShift => PegTok::RightShift,
                Tok::DoubleStar => PegTok::DoubleStar,
                Tok::DoubleStarEqual => PegTok::DoubleStarEqual,
                Tok::PlusEqual => PegTok::PlusEqual,
                Tok::MinusEqual => PegTok::MinusEqual,
                Tok::StarEqual => PegTok::StarEqual,
                Tok::SlashEqual => PegTok::SlashEqual,
                Tok::PercentEqual => PegTok::PercentEqual,
                Tok::AmperEqual => PegTok::AmperEqual,
                Tok::VbarEqual => PegTok::VbarEqual,
                Tok::CircumflexEqual => PegTok::CircumflexEqual,
                Tok::LeftShiftEqual => PegTok::LeftShiftEqual,
                Tok::RightShiftEqual => PegTok::RightShiftEqual,
                Tok::DoubleSlash => PegTok::DoubleSlash,
                Tok::DoubleSlashEqual => PegTok::DoubleSlashEqual,
                Tok::ColonEqual => PegTok::ColonEqual,
                Tok::At => PegTok::At,
                Tok::AtEqual => PegTok::AtEqual,
                Tok::Rarrow => PegTok::Rarrow,
                Tok::Ellipsis => PegTok::Ellipsis,
                Tok::False => PegTok::False,
                Tok::None => PegTok::None,
                Tok::True => PegTok::True,
                Tok::And => PegTok::And,
                Tok::As => PegTok::As,
                Tok::Assert => PegTok::Assert,
                Tok::Async => PegTok::Async,
                Tok::Await => PegTok::Await,
                Tok::Break => PegTok::Break,
                Tok::Class => PegTok::Class,
                Tok::Continue => PegTok::Continue,
                Tok::Def => PegTok::Def,
                Tok::Del => PegTok::Del,
                Tok::Elif => PegTok::Elif,
                Tok::Else => PegTok::Else,
                Tok::Except => PegTok::Except,
                Tok::Finally => PegTok::Finally,
                Tok::For => PegTok::For,
                Tok::From => PegTok::From,
                Tok::Global => PegTok::Global,
                Tok::If => PegTok::If,
                Tok::Import => PegTok::Import,
                Tok::In => PegTok::In,
                Tok::Is => PegTok::Is,
                Tok::Lambda => PegTok::Lambda,
                Tok::Nonlocal => PegTok::Nonlocal,
                Tok::Not => PegTok::Not,
                Tok::Or => PegTok::Or,
                Tok::Pass => PegTok::Pass,
                Tok::Raise => PegTok::Raise,
                Tok::Return => PegTok::Return,
                Tok::Try => PegTok::Try,
                Tok::While => PegTok::While,
                Tok::With => PegTok::With,
                Tok::Yield => PegTok::Yield,
                _ => continue,
            };
            tokens.push(tok);
            locations.push((begin, end));
        }

        Ok(Self {
            tokens,
            locations,
            names,
            ints,
            floats,
            complexes,
            strings,
            // comments,
        })
    }

    fn _parse(&self, mode: Mode) -> Result<ast::Mod, peg::error::ParseError<PegParseError>> {
        match mode {
            Mode::Module => python_parser::file(self, self),
            Mode::Interactive => python_parser::interactive(self, self),
            Mode::Expression => python_parser::eval(self, self),
        }
    }

    pub fn parse(
        &self,
        mode: Mode,
        source_path: &str,
    ) -> Result<ast::Mod, crate::error::ParseError> {
        self._parse(mode).map_err(|e| crate::error::ParseError {
            error: e.location.error,
            location: e.location.location,
            source_path: source_path.to_owned(),
        })
    }

    fn new_located<T>(&self, begin: usize, end: usize, node: T) -> Located<T> {
        assert!(begin < end);
        let location = self.locations[begin].0;
        let end_location = self.locations[end - 1].1;
        Located::new(location, end_location, node)
    }

    fn new_located_single<T>(&self, tok_pos: usize, node: T) -> Located<T> {
        let loc = self.locations[tok_pos];
        Located::new(loc.0, loc.1, node)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PegTok {
    Name(u32),
    Int(u32),
    Float(u32),
    Complex(u32),
    String(u32),
    Newline,
    // NonLogicalNewline,
    Indent,
    Dedent,
    // StartModule,
    // StartInteractive,
    // StartExpression,
    EndOfFile,
    Lpar,
    Rpar,
    Lsqb,
    Rsqb,
    Colon,
    Comma,
    // Comment(u32),
    Semi,
    Plus,
    Minus,
    Star,
    Slash,
    Vbar,  // '|'
    Amper, // '&'
    Less,
    Greater,
    Equal,
    Dot,
    Percent,
    Lbrace,
    Rbrace,
    EqEqual,
    NotEqual,
    LessEqual,
    GreaterEqual,
    Tilde,
    CircumFlex,
    LeftShift,
    RightShift,
    DoubleStar,
    DoubleStarEqual, // '**='
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    AmperEqual, // '&='
    VbarEqual,
    CircumflexEqual, // '^='
    LeftShiftEqual,
    RightShiftEqual,
    DoubleSlash, // '//'
    DoubleSlashEqual,
    ColonEqual,
    At,
    AtEqual,
    Rarrow,
    Ellipsis,

    // Keywords (alphabetically):
    False,
    None,
    True,

    And,
    As,
    Assert,
    Async,
    Await,
    Break,
    Class,
    Continue,
    Def,
    Del,
    Elif,
    Else,
    Except,
    Finally,
    For,
    From,
    Global,
    If,
    Import,
    In,
    Is,
    Lambda,
    Nonlocal,
    Not,
    Or,
    Pass,
    Raise,
    Return,
    Try,
    While,
    With,
    Yield,
}

pub struct PegParseError {
    location: Location,
    error: ParseErrorType,
}

impl std::fmt::Display for PegParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.location.fmt_with(f, &self.error)
    }
}

impl peg::Parse for Parser {
    type PositionRepr = PegParseError;

    fn start<'input>(&'input self) -> usize {
        0
    }

    fn is_eof<'input>(&'input self, p: usize) -> bool {
        p >= self.tokens.len()
    }

    fn position_repr<'input>(&'input self, p: usize) -> Self::PositionRepr {
        // format!("source: {}, p: {}, loc: {:?}, curr: {}", &self.source_path, p, self.locations[p], self.tokens[p])
        if self.is_eof(p) {
            PegParseError {
                location: Default::default(),
                error: ParseErrorType::Eof,
            }
        } else {
            // TODO: UnrecognizedToken
            // ParseErrorType::InvalidToken
            let tok = self.tokens[p];
            let tok = match tok {
                PegTok::Name(id) => Tok::Name {
                    name: self.names[id as usize].clone(),
                },
                PegTok::Int(id) => Tok::Int {
                    value: self.ints[id as usize].clone(),
                },
                PegTok::Float(id) => Tok::Float {
                    value: self.floats[id as usize].clone(),
                },
                PegTok::Complex(id) => {
                    let value = self.complexes[id as usize];
                    Tok::Complex {
                        real: value.0,
                        imag: value.1,
                    }
                }
                PegTok::String(id) => {
                    let value = self.strings[id as usize].clone();
                    Tok::String {
                        value: value.0,
                        kind: value.1,
                        triple_quoted: value.2,
                    }
                }
                PegTok::Newline => Tok::Newline,
                PegTok::Indent => Tok::Indent,
                PegTok::Dedent => Tok::Dedent,
                PegTok::EndOfFile => Tok::EndOfFile,
                PegTok::Lpar => Tok::Lpar,
                PegTok::Rpar => Tok::Rpar,
                PegTok::Lsqb => Tok::Lsqb,
                PegTok::Rsqb => Tok::Rsqb,
                PegTok::Colon => Tok::Colon,
                PegTok::Comma => Tok::Comma,
                PegTok::Semi => Tok::Semi,
                PegTok::Plus => Tok::Plus,
                PegTok::Minus => Tok::Minus,
                PegTok::Star => Tok::Star,
                PegTok::Slash => Tok::Slash,
                PegTok::Vbar => Tok::Vbar,
                PegTok::Amper => Tok::Amper,
                PegTok::Less => Tok::Less,
                PegTok::Greater => Tok::Greater,
                PegTok::Equal => Tok::Equal,
                PegTok::Dot => Tok::Dot,
                PegTok::Percent => Tok::Percent,
                PegTok::Lbrace => Tok::Lbrace,
                PegTok::Rbrace => Tok::Rbrace,
                PegTok::EqEqual => Tok::EqEqual,
                PegTok::NotEqual => Tok::NotEqual,
                PegTok::LessEqual => Tok::LessEqual,
                PegTok::GreaterEqual => Tok::GreaterEqual,
                PegTok::Tilde => Tok::Tilde,
                PegTok::CircumFlex => Tok::CircumFlex,
                PegTok::LeftShift => Tok::LeftShift,
                PegTok::RightShift => Tok::RightShift,
                PegTok::DoubleStar => Tok::DoubleStar,
                PegTok::DoubleStarEqual => Tok::DoubleStarEqual,
                PegTok::PlusEqual => Tok::PlusEqual,
                PegTok::MinusEqual => Tok::MinusEqual,
                PegTok::StarEqual => Tok::StarEqual,
                PegTok::SlashEqual => Tok::SlashEqual,
                PegTok::PercentEqual => Tok::PercentEqual,
                PegTok::AmperEqual => Tok::AmperEqual,
                PegTok::VbarEqual => Tok::VbarEqual,
                PegTok::CircumflexEqual => Tok::CircumflexEqual,
                PegTok::LeftShiftEqual => Tok::LeftShiftEqual,
                PegTok::RightShiftEqual => Tok::RightShiftEqual,
                PegTok::DoubleSlash => Tok::DoubleSlash,
                PegTok::DoubleSlashEqual => Tok::DoubleSlashEqual,
                PegTok::ColonEqual => Tok::ColonEqual,
                PegTok::At => Tok::At,
                PegTok::AtEqual => Tok::AtEqual,
                PegTok::Rarrow => Tok::Rarrow,
                PegTok::Ellipsis => Tok::Ellipsis,
                PegTok::False => Tok::False,
                PegTok::None => Tok::None,
                PegTok::True => Tok::True,
                PegTok::And => Tok::And,
                PegTok::As => Tok::As,
                PegTok::Assert => Tok::Assert,
                PegTok::Async => Tok::Async,
                PegTok::Await => Tok::Await,
                PegTok::Break => Tok::Break,
                PegTok::Class => Tok::Class,
                PegTok::Continue => Tok::Continue,
                PegTok::Def => Tok::Def,
                PegTok::Del => Tok::Del,
                PegTok::Elif => Tok::Elif,
                PegTok::Else => Tok::Else,
                PegTok::Except => Tok::Except,
                PegTok::Finally => Tok::Finally,
                PegTok::For => Tok::For,
                PegTok::From => Tok::From,
                PegTok::Global => Tok::Global,
                PegTok::If => Tok::If,
                PegTok::Import => Tok::Import,
                PegTok::In => Tok::In,
                PegTok::Is => Tok::Is,
                PegTok::Lambda => Tok::Lambda,
                PegTok::Nonlocal => Tok::Nonlocal,
                PegTok::Not => Tok::Not,
                PegTok::Or => Tok::Or,
                PegTok::Pass => Tok::Pass,
                PegTok::Raise => Tok::Raise,
                PegTok::Return => Tok::Return,
                PegTok::Try => Tok::Try,
                PegTok::While => Tok::While,
                PegTok::With => Tok::With,
                PegTok::Yield => Tok::Yield,
            };
            PegParseError {
                location: self.locations[p].0,
                error: ParseErrorType::UnrecognizedToken(tok, None),
            }
        }
    }
}

impl<'input> peg::ParseElem<'input> for Parser {
    type Element = PegTok;

    fn parse_elem(&'input self, pos: usize) -> peg::RuleResult<Self::Element> {
        match self.tokens.get(pos) {
            Some(tok) => peg::RuleResult::Matched(pos + 1, *tok),
            None => peg::RuleResult::Failed,
        }
    }
}

impl<'input> peg::ParseSlice<'input> for Parser {
    type Slice = &'input [PegTok];

    fn parse_slice(&'input self, p1: usize, p2: usize) -> Self::Slice {
        &self.tokens[p1..p2]
    }
}

peg::parser! { grammar python_parser(zelf: &Parser) for Parser {

use PegTok::*;
use crate::token::StringKind;
use ast::{
    Expr, Stmt, ExprKind, StmtKind, ExprContext, Withitem, Cmpop, Keyword, KeywordData, Comprehension,
    Operator, Excepthandler, ExcepthandlerKind, Arguments, Arg, ArgData
};
use std::option::Option::{Some, None};
use std::string::String;

pub rule file() -> ast::Mod
  = a:(statements() / {vec![]}) { ast::Mod::Module { body: a, type_ignores: vec![] } }

pub rule interactive() -> ast::Mod
  = a:statement() { ast::Mod::Interactive { body: a } }

pub rule eval() -> ast::Mod
  = a:expressions() [Newline]* { ast::Mod::Expression { body: Box::new(a) } }

// TODO:
// pub rule func_type() -> ast::Mod
//     = [Lpar] a:type_expressions()

pub rule fstring() -> Expr = star_expressions()

rule statements() -> Vec<Stmt> = a:statement()+ { a.into_iter().flatten().collect() }

rule statement() -> Vec<Stmt>
  = a:compound_stmt() { vec![a] }
  / simple_stmts()

rule statement_newline() -> Vec<Stmt>
  = a:compound_stmt() [Newline] { vec![a] }
  / simple_stmts()
  / begin:position!() [Newline] { vec![zelf.new_located_single(begin, StmtKind::Pass)] }
  / [EndOfFile] {? Err("unexpected EOF") }

rule simple_stmts() -> Vec<Stmt>
  = a:simple_stmt() ![Semi] [Newline] { vec![a] }
  / a:simple_stmt() ++ [Semi] [Semi]? [Newline] {a}

#[cache]
rule simple_stmt() -> Stmt
  = assignment()
  / loc(<a:star_expressions() { StmtKind::Expr { value: Box::new(a) } }>)
  / &[Return] a:return_stmt() {a}
  / &[Import | From] a:import_stmt() {a}
  / &[Raise] a:raise_stmt() {a}
  / loc(<[Pass] { StmtKind::Pass }>)
  / &[Del] a:del_stmt() {a}
  / &[Yield] a:yield_stmt() {a}
  / &[Assert] a:assert_stmt() {a}
  / loc(<[Break] { StmtKind::Break }>)
  / loc(<[Continue] { StmtKind::Continue }>)
  / &[Global] a:global_stmt() {a}
  / &[Nonlocal] a:nonlocal_stmt() {a}

rule compound_stmt() -> Stmt =
    &[Def | At | Async] a:function_def() {a} /
    &[If] a:if_stmt() {a} /
    &[Class | At] a:class_def() {a} /
    &[With | Async] a:with_stmt() {a} /
    &[For | Async] a:for_stmt() {a} /
    &[Try] a:try_stmt() {a} /
    &[While] a:while_stmt() {a}
    // TODO:
    // match_stmt()

rule assignment() -> Stmt =
    loc(<a:name_expr(ExprContext::Store) [Colon] b:expression() c:([Equal] z:annotated_rhs() {z})? {
        StmtKind::AnnAssign { target: Box::new(a), annotation: Box::new(b), value: option_box(c), simple: 1, }
    }>) /
    loc(<a:(par(<single_target()>) / single_subscript_attribute_target(ExprContext::Store))
        [Colon] b:expression() c:([Equal] z:annotated_rhs() {z})? {
            StmtKind::AnnAssign { target: Box::new(a), annotation: Box::new(b), value: option_box(c), simple: 0 }
    }>) /
    loc(<a:(z:star_targets() [Equal] {z})+ b:(yield_expr() / star_expressions()) ![Equal] tc:type_comment() {
        StmtKind::Assign { targets: a, value: Box::new(b), type_comment: tc }
    }>) /
    loc(<a:single_target() b:augassign() c:(yield_expr() / star_expressions()) {
        StmtKind::AugAssign { target: Box::new(a), op: b, value: Box::new(c) }
    }>)

rule annotated_rhs() -> Expr = yield_expr() / star_expressions()

rule augassign() -> Operator =
    [PlusEqual] { Operator::Add } /
    [MinusEqual] { Operator::Sub } /
    [StarEqual] { Operator::Mult } /
    [AtEqual] { Operator::MatMult } /
    [SlashEqual] { Operator::Div } /
    [PercentEqual] { Operator::Mod } /
    [AmperEqual] { Operator::BitAnd } /
    [VbarEqual] { Operator::BitOr } /
    [CircumflexEqual] { Operator::BitXor } /
    [LeftShiftEqual] { Operator::LShift } /
    [RightShiftEqual] { Operator::RShift } /
    [DoubleStarEqual] { Operator::Pow } /
    [DoubleSlashEqual] { Operator::FloorDiv }

rule return_stmt() -> Stmt = loc(<[Return] a:star_expressions()? {
    StmtKind::Return { value: option_box(a) }
}>)

rule raise_stmt() -> Stmt =
    loc(<[Raise] a:expression() b:([From] z:expression() {z})? {
        StmtKind::Raise { exc: Some(Box::new(a)), cause: option_box(b) }
    }>) /
    loc(<[Raise] {
        StmtKind::Raise { exc: None, cause: None }
    }>)

rule global_stmt() -> Stmt = loc(<[Global] names:name() ++ [Comma] {
    StmtKind::Global { names }
}>)

rule nonlocal_stmt() -> Stmt = loc(<[Nonlocal] names:name() ++ [Comma] {
    StmtKind::Nonlocal { names }
}>)

rule del_stmt() -> Stmt = loc(<[Del] a:del_targets() &[Semi | Newline] {
    StmtKind::Delete { targets: a }
}>)

rule yield_stmt() -> Stmt = loc(<a:yield_expr() {
    StmtKind::Expr { value: Box::new(a) }
}>)

rule assert_stmt() -> Stmt = loc(<[Assert] a:expression() b:([Comma] z:expression() {z})? {
    StmtKind::Assert { test: Box::new(a), msg: option_box(b) }
}>)

rule import_stmt() -> Stmt = import_name() / import_from()

rule import_name() -> Stmt = loc(<[Import] a:dotted_as_names() {
    StmtKind::Import { names: a }
}>)
rule import_from() -> Stmt =
    loc(<[From] a:[Dot | Ellipsis]* b:dotted_name() [Import] c:import_from_targets() {
        StmtKind::ImportFrom { module: Some(b), names: c, level: count_dots(a) }
    }>) /
    loc(<[From] a:[Dot | Ellipsis]+ [Import] b:import_from_targets() {
        StmtKind::ImportFrom { module: None, names: b, level: count_dots(a) }
    }>)
rule import_from_targets() -> Vec<ast::Alias> =
    par(<a:import_from_as_names() [Comma]? {a}>) /
    a:import_from_as_names() ![Comma] {a} /
    a:loc(<[Star] {
        ast::AliasData { name: "*".to_owned(), asname: None }
    }>) { vec![a] }
rule import_from_as_names() -> Vec<ast::Alias> = import_from_as_name() ++ [Comma]
rule import_from_as_name() -> ast::Alias = loc(<a:name() b:([As] z:name() {z})? {
    ast::AliasData { name: a, asname: b }
}>)
rule dotted_as_names() -> Vec<ast::Alias> = dotted_as_name() ++ [Comma]
rule dotted_as_name() -> ast::Alias = loc(<a:dotted_name() b:([As] z:name() {z})? {
    ast::AliasData { name: a, asname: b }
}>)

#[cache_left_rec]
rule dotted_name() -> String =
    a:dotted_name() [Dot] b:name() {
        format!("{}.{}", a, b)
    } /
    name()

#[cache]
rule block() -> Vec<Stmt>
    = [Newline] [Indent] a:statements() [Dedent] {a}
    / simple_stmts()

rule decorator() -> Expr = [At] f:named_expression() [Newline] {f}

rule class_def() -> Stmt =
    dec:decorator()* begin:position!() [Class] name:name() arg:par(<arguments()?>)? [Colon] b:block() end:block_end() {
        let (bases, keywords) = arg.flatten().unwrap_or_default();
        let stmt = StmtKind::ClassDef { name, bases, keywords, body: b, decorator_list: dec };
        zelf.new_located(begin, end, stmt)
    }

rule function_def() -> Stmt =
    dec:decorator()* begin:position!() is_async:[Async]? [Def] name:name() p:par(<params()?>)
    r:([Rarrow] z:expression() {z})? [Colon] tc:func_type_comment() b:block() end:block_end() {
        let stmt = if is_async.is_none() {
            StmtKind::FunctionDef { name, args: Box::new(p.unwrap_or_else(make_empty_arguments)), body: b, decorator_list: dec, returns: option_box(r), type_comment: tc }
        } else {
            StmtKind::AsyncFunctionDef { name, args: Box::new(p.unwrap_or_else(make_empty_arguments)), body: b, decorator_list: dec, returns: option_box(r), type_comment: tc }
        };
        zelf.new_located(begin, end, stmt)
    }

rule params() -> Arguments = parameters()

rule parameters() -> Arguments =
    a:slash_no_default() c:param_no_default()* d:param_with_default()* e:star_etc()? {
        make_arguments(a, Default::default(), c, d, e)
    } /
    b:slash_with_default() d:param_with_default()* e:star_etc()? {
        make_arguments(vec![], b, vec![], d, e)
    } /
    c:param_no_default()+ d:param_with_default()* e:star_etc()? {
        make_arguments(vec![], Default::default(), c, d, e)
    } /
    d:param_with_default()+ e:star_etc()? {
        make_arguments(vec![], Default::default(), vec![], d, e)
    } /
    e:star_etc() {
        make_arguments(vec![], Default::default(), vec![], vec![], Some(e))
    }

rule slash_no_default() -> Vec<Arg> = a:param_no_default()+ [Slash] param_split() {a}
rule slash_with_default() -> (Vec<Arg>, Vec<(Arg, Expr)>) =
    a:param_no_default()* b:param_with_default()+ [Slash] param_split() {(a, b)}

rule star_etc() -> (Option<Arg>, Vec<(Arg, Option<Expr>)>, Option<Arg>) =
    [Star] a:param_no_default() b:param_maybe_default()* c:kwds()? {
        (Some(a), b, c)
    } /
    [Star] a:param_no_default_star_annotation() b:param_maybe_default()* c:kwds()? {
        (Some(a), b, c)
    } /
    [Star] [Comma] b:param_maybe_default()+ c:kwds()? {
        (None, b, c)
    } /
    c:kwds() {
        (None, vec![], Some(c))
    }

rule kwds() -> Arg = [DoubleStar] a:param_no_default() {a}

// TODO: type_comment
rule param_no_default() -> Arg = a:param() param_split() {a}
rule param_no_default_star_annotation() -> Arg = a:param_star_annotation() param_split() {a}
rule param_with_default() -> (Arg, Expr) = a:param() c:default() param_split() {(a, c)}
rule param_maybe_default() -> (Arg, Option<Expr>) = a:param() c:default()? param_split() {(a, c)}
rule param() -> Arg =
    loc(<a:name() b:annotation()? {
        ArgData { arg: a, annotation: option_box(b), type_comment: None }
    }>)
rule param_star_annotation() -> Arg =
    loc(<a:name() b:star_annotation() {
        ArgData { arg: a, annotation: Some(Box::new(b)), type_comment: None }
    }>)
rule annotation() -> Expr = [Colon] a:expression() {a}
rule star_annotation() -> Expr = [Colon] a:star_annotation() {a}
rule default() -> Expr = [Equal] a:expression() {a}
rule param_split() = [Comma] / &[Rpar]

rule if_stmt() -> Stmt
    = begin:position!() [If] a:named_expression() [Colon] b:block() c:(
        z:elif_stmt() {vec![z]} / else_block() / {vec![]}
    ) end:block_end() {
        zelf.new_located(begin, end, StmtKind::If { test: Box::new(a), body: b, orelse: c })
    }

rule elif_stmt() -> Stmt
    = begin:position!() [Elif] a:named_expression() [Colon] b:block() end:block_end() c:(
        z:elif_stmt() {vec![z]} / else_block() / {vec![]}
    ) {
        zelf.new_located(begin, end, StmtKind::If { test: Box::new(a), body: b, orelse: c })
    }

rule else_block() -> Vec<Stmt> = [Else] [Colon] b:block() {b}
rule else_block_opt() -> Vec<Stmt> = else_block() / {vec![]}

rule while_stmt() -> Stmt =
    loc_block_end(<[While] a:named_expression() [Colon] b:block() c:else_block_opt() {
        StmtKind::While { test: Box::new(a), body: b, orelse: c }
    }>)

rule for_stmt() -> Stmt =
    loc_block_end(<is_async:[Async]? [For] t:star_targets() [In] ex:star_expressions() [Colon] tc:type_comment() b:block() el:else_block_opt() {
        if is_async.is_none() {
            StmtKind::For { target: Box::new(t), iter: Box::new(ex), body: b, orelse: el, type_comment: tc }
        } else {
            StmtKind::AsyncFor { target: Box::new(t), iter: Box::new(ex), body: b, orelse: el, type_comment: tc }
        }
    }>)

rule with_stmt() -> Stmt =
    loc_block_end(<is_async:[Async]? [With] a:par(<z:with_item() ++ [Comma] [Comma]? {z}>) [Colon] b:block() {
        if is_async.is_none() {
            StmtKind::With { items: a, body: b, type_comment: None }
        } else {
            StmtKind::AsyncWith { items: a, body: b, type_comment: None }
        }
    }>) /
    loc_block_end(<is_async:[Async]? [With] a:with_item() ++ [Comma] [Colon] tc:type_comment() b:block() {
        if is_async.is_none() {
            StmtKind::With { items: a, body: b, type_comment: tc }
        } else {
            StmtKind::AsyncWith { items: a, body: b, type_comment: tc }
        }
    }>)

rule with_item() -> Withitem =
    e:expression() vars:([As] t:star_target() &[Comma | Rpar | Colon] {t})? {
        Withitem { context_expr: e, optional_vars: option_box(vars) }
    }

rule try_stmt() -> Stmt
    = loc_block_end(<[Try] [Colon] b:block() x:(
        f:finally_block() {(vec![], vec![], f)}
        / ex:except_block()+ el:else_block_opt() f:(finally_block() / {vec![]}) {(ex, el, f)}
    ) {
        StmtKind::Try { body: b, handlers: x.0, orelse: x.1, finalbody: x.2 }
    }>)
    // TODO: except star
    // loc(<[Try] [Colon] b:block() ex:except_star_block()+ el:else_block_opt() f:finally_block() {
    //     StmtKind::{ body: b, handlers: ex, orelse: el, finalbody: f }
    // }>)

rule except_block() -> Excepthandler =
    loc_block_end(<[Except] e:expression() t:([As] z:name() {z})? [Colon] b:block() {
        ExcepthandlerKind::ExceptHandler { type_: Some(Box::new(e)), name: t, body: b }
    }>) /
    loc_block_end(<[Except] [Colon] b:block() {
        ExcepthandlerKind::ExceptHandler { type_: None, name: None, body: b }
    }>)
rule except_star_block() -> Excepthandler =
    loc_block_end(<[Except] [Star] e:expression() t:([As] z:name() {z})? [Colon] b:block() {
        ExcepthandlerKind::ExceptHandler { type_: Some(Box::new(e)), name: t, body: b }
    }>)
rule finally_block() -> Vec<Stmt> = [Finally] [Colon] b:block() {b}

// rule match_stmt() -> Stmt =
//     [Match]

rule expressions() -> Expr = pack_tuple_expr(<expression()>, ExprContext::Load)

rule expression() -> Expr =
    loc(<a:disjunction() [If] b:disjunction() [Else] c:expression() {
        ExprKind::IfExp { test: Box::new(b), body: Box::new(a), orelse: Box::new(c) }
    }>) /
    disjunction() /
    lambdef()

rule yield_expr() -> Expr =
    loc(<[Yield] [From] a:expression() {
        ExprKind::YieldFrom { value: Box::new(a) }
    }>) /
    loc(<[Yield] a:star_expressions()? {
        ExprKind::Yield { value: option_box(a) }
    }>)

rule star_expressions() -> Expr = pack_tuple_expr(<star_expression()>, ExprContext::Load)

rule star_expression() -> Expr =
    loc(<[Star] a:bitwise_or() {
        ExprKind::Starred { value: Box::new(a), ctx: ExprContext::Load }
    }>) /
    expression()

rule star_named_expressions() -> Vec<Expr> =
    a:star_named_expression() ++ [Comma] [Comma]? {a}

rule star_named_expression() -> Expr =
    loc(<[Star] a:bitwise_or() {
        ExprKind::Starred { value: Box::new(a), ctx: ExprContext::Load }
    }>) /
    named_expression()

rule assignment_expression() -> Expr =
    loc(<a:name_expr(ExprContext::Store) [ColonEqual] b:expression() {
        ExprKind::NamedExpr { target: Box::new(a), value: Box::new(b) }
    }>)

rule named_expression() -> Expr =
    assignment_expression() /
    a:expression() ![ColonEqual] {a}

// #[cache]
// rule disjunction() -> Expr =
//     loc(<a:conjunction() **<2,> [Or] {
//         ExprKind::BoolOp { op: ast::Boolop::Or, values: a }
//     }>) /
//     conjunction()
#[cache]
rule disjunction() -> Expr
    = begin:position!() a:conjunction() v:([Or] z:conjunction() ++ [Or] {z})? end:position!() {
        if let Some(v) = v {
            zelf.new_located(begin, end, ExprKind::BoolOp { op: ast::Boolop::Or, values: insert_front(v, a) })
        } else { a }
    }

#[cache]
rule conjunction() -> Expr
    = begin:position!() a:inversion() v:([And] z:inversion() ++ [And] {z})? end:position!() {
        if let Some(v) = v {
            zelf.new_located(begin, end, ExprKind::BoolOp { op: ast::Boolop::And, values: insert_front(v, a) })
        } else { a }
    }

#[cache]
rule inversion() -> Expr =
    loc(<[Not] a:inversion() {
        ExprKind::UnaryOp { op: ast::Unaryop::Not, operand: Box::new(a) }
    }>) /
    comparison()

#[cache]
rule comparison() -> Expr
    = begin:position!() a:bitwise_or() b:compare_op_bitwise_or_pair()* end:position!() {
        if b.is_empty() { return a; }
        let (ops, comparators) = comparison_ops_comparators(b);
        zelf.new_located(begin, end, ExprKind::Compare { left: Box::new(a), ops, comparators })
    }

// TODO: simplify
#[cache]
rule compare_op_bitwise_or_pair() -> (Cmpop, Expr) =
    eq_bitwise_or() /
    noteq_bitwise_or() /
    lte_bitwise_or() /
    lt_bitwise_or() /
    gte_bitwise_or() /
    gt_bitwise_or() /
    notin_bitwise_or() /
    in_bitwise_or() /
    isnot_bitwise_or() /
    is_bitwise_or()

rule eq_bitwise_or() -> (Cmpop, Expr) = [EqEqual] a:bitwise_or() { (Cmpop::Eq, a) }
rule noteq_bitwise_or() -> (Cmpop, Expr) = [NotEqual] a:bitwise_or() { (Cmpop::NotEq, a) }
rule lte_bitwise_or() -> (Cmpop, Expr) = [LessEqual] a:bitwise_or() { (Cmpop::LtE, a) }
rule lt_bitwise_or() -> (Cmpop, Expr) = [Less] a:bitwise_or() { (Cmpop::Lt, a) }
rule gte_bitwise_or() -> (Cmpop, Expr) = [GreaterEqual] a:bitwise_or() { (Cmpop::GtE, a) }
rule gt_bitwise_or() -> (Cmpop, Expr) = [Greater] a:bitwise_or() { (Cmpop::Gt, a) }
rule notin_bitwise_or() -> (Cmpop, Expr) = [Not] [In] a:bitwise_or() { (Cmpop::NotIn, a) }
rule in_bitwise_or() -> (Cmpop, Expr) = [In] a:bitwise_or() { (Cmpop::In, a) }
rule isnot_bitwise_or() -> (Cmpop, Expr) = [Is] [Not] a:bitwise_or() { (Cmpop::IsNot, a) }
rule is_bitwise_or() -> (Cmpop, Expr) = [Is] a:bitwise_or() { (Cmpop::Is, a) }

#[cache_left_rec]
rule bitwise_or() -> Expr =
    loc(<a:bitwise_or() [Vbar] b:bitwise_xor() {
        ExprKind::BinOp { left: Box::new(a), op: Operator::BitOr, right: Box::new(b) }
    }>) /
    bitwise_xor()

#[cache_left_rec]
rule bitwise_xor() -> Expr =
    loc(<a:bitwise_xor() [CircumFlex] b:bitwise_and() {
        ExprKind::BinOp { left: Box::new(a), op: Operator::BitXor, right: Box::new(b) }
    }>) /
    bitwise_and()

#[cache_left_rec]
rule bitwise_and() -> Expr =
    loc(<a:bitwise_and() [Amper] b:shift_expr() {
        ExprKind::BinOp { left: Box::new(a), op: Operator::BitAnd, right: Box::new(b) }
    }>) /
    shift_expr()

#[cache_left_rec]
rule shift_expr() -> Expr
    = loc(<a:shift_expr() op:shift_op() b:sum() {
        ExprKind::BinOp { left: Box::new(a), op, right: Box::new(b) }
    }>)
    / sum()
rule shift_op() -> Operator
    = [LeftShift] { Operator::LShift }
    / [RightShift] { Operator::RShift }

#[cache_left_rec]
rule sum() -> Expr =
    loc(<a:sum() op:sum_op() b:term() {
        ExprKind::BinOp { left: Box::new(a), op, right: Box::new(b) }
    }>)
    / term()
rule sum_op() -> Operator
    = [Plus] { Operator::Add }
    / [Minus] { Operator::Sub }

#[cache_left_rec]
rule term() -> Expr
    = loc(<a:term() op:term_op() b:factor() {
        ExprKind::BinOp { left: Box::new(a), op, right: Box::new(b) }
    }>)
    / factor()
rule term_op() -> Operator
    = [Star] { Operator::Mult }
    / [Slash] { Operator::Div }
    / [DoubleSlash] { Operator::FloorDiv }
    / [Percent] { Operator::Mod }
    / [At] { Operator::MatMult }

#[cache]
rule factor() -> Expr =
    loc(<op:factor_op() a:factor() {
        ExprKind::UnaryOp { op, operand: Box::new(a) }
    }>) /
    power()
rule factor_op() -> ast::Unaryop
    = [Plus] { ast::Unaryop::UAdd }
    / [Minus] { ast::Unaryop::USub }
    / [Tilde] { ast::Unaryop::Invert }

rule power() -> Expr
    = begin:position!() a:await_primary() b:([DoubleStar] z:factor() {z})? end:position!() {
        if let Some(b) = b {
            zelf.new_located(begin, end, ExprKind::BinOp { left: Box::new(a), op: Operator::Pow, right: Box::new(b) })
        } else { a }
    }

#[cache]
rule await_primary() -> Expr
    = loc(<[Await] a:primary() {
        ExprKind::Await { value: Box::new(a) }
    }>)
    / primary()

#[cache_left_rec]
rule primary() -> Expr
    = loc(<a:primary() [Dot] b:name() {
        ExprKind::Attribute { value: Box::new(a), attr: b, ctx: ExprContext::Load }
    }>)
    / loc(<a:primary() b:genexp() {
        ExprKind::Call { func: Box::new(a), args: vec![b], keywords: vec![] }
    }>)
    / loc(<a:primary() b:par(<arguments() / {Default::default()}>) {
        ExprKind::Call { func: Box::new(a), args: b.0, keywords: b.1 }
    }>)
    / loc(<a:primary() b:sqb(<slices()>) {
        ExprKind::Subscript { value: Box::new(a), slice: Box::new(b), ctx: ExprContext::Load }
    }>)
    / atom()

rule slices() -> Expr =
    a:slice() ![Comma] {a} /
    loc(<a:(slice() / starred_expression()) ++ [Comma] [Comma]? {
        ExprKind::Tuple { elts: a, ctx: ExprContext::Load }
    }>)

rule slice() -> Expr =
    loc(<a:expression()? [Colon] b:expression()? c:([Colon] z:expression()? {z})? {
        ExprKind::Slice { lower: option_box(a), upper: option_box(b), step: option_box(c.flatten()) }
    }>) /
    named_expression()

rule atom() -> Expr
    = name_expr(ExprContext::Load)
    / loc(<
        [True] { ExprKind::Constant { value: ast::Constant::Bool(true), kind: None } }
        / [False] { ExprKind::Constant { value: ast::Constant::Bool(false), kind: None } }
        / [PegTok::None] { ExprKind::Constant { value: ast::Constant::None, kind: None } }
        / [Int(id)] { ExprKind::Constant { value: ast::Constant::Int(zelf.ints[id as usize].clone()), kind: None } }
        / [Float(id)] { ExprKind::Constant { value: ast::Constant::Float(zelf.floats[id as usize]), kind: None } }
        / [Complex(id)] {
            let (real, imag) = zelf.complexes[id as usize];
            ExprKind::Constant { value: ast::Constant::Complex { real, imag }, kind: None }
        }
        / [Ellipsis] { ExprKind::Constant { value: ast::Constant::Ellipsis, kind: None } }
    >)
    / strings()
    / &[Lpar] a:(tuple() / group() / genexp()) {a}
    / &[Lsqb] a:(list() / listcomp()) {a}
    / &[Lbrace] a:(dict() / set() / dictcomp() / setcomp()) {a}

rule group() -> Expr = par(<yield_expr() / named_expression()>)

rule lambdef() -> Expr =
    loc(<[Lambda] a:lambda_params()? [Colon] b:expression() {
        ExprKind::Lambda { args: Box::new(a.unwrap_or_else(make_empty_arguments)), body: Box::new(b) }
    }>)

rule lambda_params() -> Arguments = lambda_parameters()

rule lambda_parameters() -> Arguments =
    a:lambda_slash_no_default() c:lambda_param_no_default()* d:lambda_param_with_default()* e:lambda_star_etc()? {
        make_arguments(a, Default::default(), c, d, e)
    } /
    b:lambda_slash_with_default() d:lambda_param_with_default()* e:lambda_star_etc()? {
        make_arguments(vec![], b, vec![], d, e)
    } /
    c:lambda_param_no_default()+ d:lambda_param_with_default()* e:lambda_star_etc()? {
        make_arguments(vec![], Default::default(), c, d, e)
    } /
    d:lambda_param_with_default()+ e:lambda_star_etc()? {
        make_arguments(vec![], Default::default(), vec![], d, e)
    } /
    e:lambda_star_etc() {
        make_arguments(vec![], Default::default(), vec![], vec![], Some(e))
    }

rule lambda_slash_no_default() -> Vec<Arg> =
    a:lambda_param_no_default()+ [Slash] lambda_param_split() {a}

rule lambda_slash_with_default() -> (Vec<Arg>, Vec<(Arg, Expr)>) =
    a:lambda_param_no_default()* b:lambda_param_with_default()+ [Slash] lambda_param_split() {(a, b)}

rule lambda_star_etc() -> (Option<Arg>, Vec<(Arg, Option<Expr>)>, Option<Arg>) =
    [Star] a:lambda_param_no_default() b:lambda_param_maybe_default()* c:lambda_kwds()? {
        (Some(a), b, c)
    } /
    [Star] [Comma] b:lambda_param_maybe_default()+ c:lambda_kwds()? {
        (None, b, c)
    } /
    c:lambda_kwds() {
        (None, vec![], Some(c))
    }

rule lambda_kwds() -> Arg =
    [DoubleStar] a:lambda_param_no_default() {a}

rule lambda_param_no_default() -> Arg = a:lambda_param() lambda_param_split() {a}
rule lambda_param_with_default() -> (Arg, Expr) = a:lambda_param() c:default() lambda_param_split() {(a, c)}
rule lambda_param_maybe_default() -> (Arg, Option<Expr>) = a:lambda_param() c:default()? lambda_param_split() {(a, c)}
rule lambda_param() -> Arg =
    loc(<a:name() {
        ArgData { arg: a, annotation: None, type_comment: None }
    }>)
rule lambda_param_split() = [Comma] / &[Colon]

#[cache]
rule strings() -> Expr = a:string()+ {?
    // TODO: error handling
    crate::string::parse_strings(a).map_err(|_| "string format error")
}

rule string() -> (Location, (String, StringKind, bool), Location) =
    begin:position!() [PegTok::String(id)] end:position!() {
        (zelf.locations[begin].0, zelf.strings[id as usize].clone(), zelf.locations[end - 1].1)
    }

rule list() -> Expr =
    loc(<a:sqb(<star_named_expressions() / {vec![]}>) {
        ExprKind::List { elts: a, ctx: ExprContext::Load }
    }>)

rule tuple() -> Expr =
    loc(<a:par(<a:star_named_expression() [Comma] v:(star_named_expressions() / {vec![]}) {
        insert_front(v, a)
    } / {vec![]}>) {
        ExprKind::Tuple { elts: a, ctx: ExprContext::Load }
    }>)

rule set() -> Expr =
    loc(<a:brace(<star_named_expressions()>) {
        ExprKind::Set { elts: a }
    }>)

rule dict() -> Expr =
    loc(<a:brace(<double_starred_kvpairs() / {vec![]}>) {
        let (keys, values) = dict_kvpairs(a);
        ExprKind::Dict { keys, values }
    }>)

rule double_starred_kvpairs() -> Vec<(Option<Expr>, Expr)> =
    a:double_starred_kvpair() ++ [Comma] [Comma]? {a}

rule double_starred_kvpair() -> (Option<Expr>, Expr) =
    [DoubleStar] a:bitwise_or() { (None, a) } /
    a:kvpair() { (Some(a.0), a.1) }

rule kvpair() -> (Expr, Expr) =
    a:expression() [Colon] b:expression() { (a, b) }

rule for_if_clauses() -> Vec<Comprehension> = for_if_clause()+

rule for_if_clause() -> Comprehension =
    is_async:[Async]? [For] a:star_targets() [In] b:disjunction() c:([If] z:disjunction() { z })* {
        Comprehension { target: a, iter: b, ifs: c, is_async: if is_async.is_some() {1} else {0} }
    }

rule listcomp() -> Expr =
    loc(<sqb(<a:named_expression() b:for_if_clauses() {
        ExprKind::ListComp { elt: Box::new(a), generators: b }
    }>)>)

rule setcomp() -> Expr =
    loc(<brace(<a:named_expression() b:for_if_clauses() {
        ExprKind::SetComp { elt: Box::new(a), generators: b }
    }>)>)

rule genexp() -> Expr =
    loc(<par(<a:(assignment_expression() / z:expression() ![ColonEqual] {z}) b:for_if_clauses() {
        ExprKind::GeneratorExp { elt: Box::new(a), generators: b }
    }>)>)

rule dictcomp() -> Expr =
    loc(<brace(<a:kvpair() b:for_if_clauses() {
        ExprKind::DictComp { key: Box::new(a.0), value: Box::new(a.1), generators: b }
    }>)>)

#[cache]
rule arguments() -> (Vec<Expr>, Vec<Keyword>) = a:args() [Comma]? &[Rpar] {a}

rule args() -> (Vec<Expr>, Vec<Keyword>) =
    a:(starred_expression() / z:(assignment_expression() / z:expression() ![ColonEqual] {z}) ![Equal] {z}) ++ [Comma]
    b:([Comma] k:kwargs() {k} / {vec![]}) {
        let (mut ex, kw) = keyword_or_starred_partition(b);
        let mut a = a;
        a.append(&mut ex);
        (a, kw)
    } /
    a:kwargs() {
        keyword_or_starred_partition(a)
    }

rule kwargs() -> Vec<KeywordOrStarred> =
    a:kwarg_or_starred() ++ [Comma] [Comma] b:kwarg_or_double_starred() ++ [Comma] {
        let mut a = a;
        let mut b = b;
        a.append(&mut b);
        a
    } /
    kwarg_or_starred() ++ [Comma] /
    kwarg_or_double_starred() ++ [Comma]

rule starred_expression() -> Expr =
    loc(<[Star] a:expression() {
        ExprKind::Starred { value: Box::new(a), ctx: ExprContext::Load }
    }>)

rule kwarg_or_starred() -> KeywordOrStarred =
    a:loc(<a:name() [Equal] b:expression() {
        KeywordData { arg: Some(a), value: b }
    }>) { KeywordOrStarred::Keyword(a) } /
    a:starred_expression() {
        KeywordOrStarred::Starred(a)
    }

rule kwarg_or_double_starred() -> KeywordOrStarred =
    a:loc(<a:name() [Equal] b:expression() {
        KeywordData { arg: Some(a), value: b }
    }>) { KeywordOrStarred::Keyword(a) } /
    a:loc(<[DoubleStar] a:expression() {
        KeywordData { arg: None, value: a }
    }>) { KeywordOrStarred::Keyword(a) }

rule star_targets() -> Expr =
    a:star_target() ![Comma] {a} /
    loc(<a:star_target() v:([Comma] z:star_target() {z})* [Comma]? {
        ExprKind::Tuple { elts: insert_front(v, a), ctx: ExprContext::Store }
    }>)

rule star_targets_list() -> Vec<Expr> = a:star_target() ++ [Comma] [Comma]? {a}

rule star_targets_tuple() -> Vec<Expr>
    = a:star_target() [Comma] v:star_target() ** [Comma] tail:[Comma]? {?
        if tail.is_some() && v.is_empty() {
            Err("invalid token ','")
        } else {
            Ok(insert_front(v, a))
        }
    }
    // a:star_target() ([Comma] v:star_target())+ [Comma]? {}
    // begin:position!() a:star_target() v:([Comma z:star_target() {z}])+ [Comma]? {a} /
    // a:star_target() [Comma] { vec![a] }

#[cache]
rule star_target() -> Expr =
    loc(<[Star] ![Star] a:star_target() {
        ExprKind::Starred { value: Box::new(a), ctx: ExprContext::Store }
    }>) /
    target_with_star_atom()

#[cache]
rule target_with_star_atom() -> Expr =
    single_subscript_attribute_target(ExprContext::Store) /
    star_atom()

rule star_atom() -> Expr
    = name_expr(ExprContext::Store)
    / par(<target_with_star_atom()>)
    / loc(<a:par(<star_targets_tuple() / {vec![]}>) {
        ExprKind::Tuple { elts: a, ctx: ExprContext::Store }
    }>)
    / loc(<a:sqb(<star_targets_list() / {vec![]}>) {
        ExprKind::List { elts: a, ctx: ExprContext::Store }
    }>)

rule single_target() -> Expr =
    single_subscript_attribute_target(ExprContext::Store) /
    name_expr(ExprContext::Store) /
    par(<single_target()>)

rule single_subscript_attribute_target(ctx: ExprContext) -> Expr = loc(<
    a:t_primary() [Dot] attr:name() !t_lookahead() {
        ExprKind::Attribute { value: Box::new(a), attr, ctx: ctx.clone() }
    }
  / a:t_primary() b:sqb(<slices()>) !t_lookahead() {
        ExprKind::Subscript { value: Box::new(a), slice: Box::new(b), ctx: ctx.clone() }
    }>)

#[cache_left_rec]
rule t_primary() -> Expr =
    loc(<a:t_primary() [Dot] attr:name() &t_lookahead() {
        ExprKind::Attribute { value: Box::new(a), attr, ctx: ExprContext::Load }
    }>) /
    loc(<a:t_primary() b:sqb(<slices()>) &t_lookahead() {
        ExprKind::Subscript { value: Box::new(a), slice: Box::new(b), ctx: ExprContext::Load }
    }>) /
    loc(<a:t_primary() b:genexp() &t_lookahead() {
        ExprKind::Call { func: Box::new(a), args: vec![b], keywords: vec![] }
    }>) /
    loc(<a:t_primary() b:par(<arguments()?>) &t_lookahead() {
        let (ex, kw) = b.unwrap_or_default();
        ExprKind::Call { func: Box::new(a), args: ex, keywords: kw }
    }>) /
    a:atom() &t_lookahead() {a}

rule t_lookahead() = [Lpar] / [Lsqb] / [Dot]

rule del_targets() -> Vec<Expr> = a:del_target() ++ [Comma] [Comma]? {a}

#[cache]
rule del_target() -> Expr =
    single_subscript_attribute_target(ExprContext::Del) /
    del_t_atom()

rule del_t_atom() -> Expr =
    name_expr(ExprContext::Del) /
    par(<del_target()>) /
    loc(<a:par(<del_targets()>) {
        ExprKind::Tuple { elts: a, ctx: ExprContext::Del }
    }>) /
    loc(<a:sqb(<del_targets()>) {
        ExprKind::List { elts: a, ctx: ExprContext::Del }
    }>)

rule loc<T>(r: rule<T>) -> Located<T> = begin:position!() z:r() end:position!() {
    zelf.new_located(begin, end, z)
}

rule loc_block_end<T>(r: rule<T>) -> Located<T> = begin:position!() z:r() end:block_end() {
    zelf.new_located(begin, end, z)
}

rule block_end() -> usize = p:position!() {
    let mut p = p;
    while zelf.tokens[p - 1] == Newline || zelf.tokens[p - 1] == Dedent {
        p -= 1;
    }
    p
}

rule name() -> String = [Name(id)] { zelf.names[id as usize].clone() }
rule name_expr(ctx: ExprContext) -> Expr =
    loc(<id:name() {
        ExprKind::Name { id, ctx: ctx.clone() }
    }>)

rule par<T>(r: rule<T>) -> T = [Lpar] z:r() [Rpar] {z}
rule sqb<T>(r: rule<T>) -> T = [Lsqb] z:r() [Rsqb] {z}
rule brace<T>(r: rule<T>) -> T = [Lbrace] z:r() [Rbrace] {z}

// not yet supported by lexer
rule type_comment() -> Option<String> = { None }
// not yet supported by lexer
rule func_type_comment() -> Option<String> = { None }

// TODO: optimize
rule pack_tuple_expr(r: rule<Expr>, ctx: ExprContext) -> Expr
  = begin:position!() a:r() v:([Comma] z:r() {z})* tail:[Comma]? end:position!() {
        if tail.is_none() && v.is_empty() {
            a
        } else {
            zelf.new_located(begin, end, ExprKind::Tuple { elts: insert_front(v, a), ctx: ctx.clone() })
        }
    }
    // loc(<z:r() **<2,> [Comma] [Comma]? {
    //     ExprKind::Tuple { elts: z, ctx: ctx.clone() }
    // }>) /
    // loc(<z:r() [Comma] {
    //     ExprKind::Tuple { elts: vec![z], ctx: ctx.clone() }
    // }>) /
    // r()

}}

#[cold]
#[inline(always)]
fn insert_front<T>(mut v: Vec<T>, a: T) -> Vec<T> {
    v.insert(0, a);
    v
}

fn count_dots(toks: Vec<PegTok>) -> Option<usize> {
    if toks.is_empty() {
        return None;
    }

    let mut count = 0;
    for tok in toks {
        count += match tok {
            PegTok::Dot => 1,
            PegTok::Ellipsis => 3,
            _ => unreachable!(),
        };
    }
    Some(count)
}

fn option_box<T>(val: Option<T>) -> Option<Box<T>> {
    val.map(|x| Box::new(x))
}

enum KeywordOrStarred {
    Keyword(ast::Keyword),
    Starred(ast::Expr),
}

fn keyword_or_starred_partition(v: Vec<KeywordOrStarred>) -> (Vec<ast::Expr>, Vec<ast::Keyword>) {
    let mut ex_vec = vec![];
    let mut kw_vec = vec![];
    for x in v {
        match x {
            KeywordOrStarred::Keyword(kw) => kw_vec.push(kw),
            KeywordOrStarred::Starred(ex) => ex_vec.push(ex),
        }
    }
    (ex_vec, kw_vec)
}

fn dict_kvpairs(
    v: Vec<(Option<ast::Expr>, ast::Expr)>,
) -> (Vec<Option<ast::Expr>>, Vec<ast::Expr>) {
    let mut keys = Vec::with_capacity(v.len());
    let mut values = Vec::with_capacity(v.len());

    for (key, value) in v {
        keys.push(key);
        values.push(value);
    }

    // let (packed, unpacked) = v.into_iter().partition::<Vec<_>, _>(|x| x.0.is_some());
    // for x in packed {
    //     keys.push(x.0.unwrap());
    //     values.push(x.1);
    // }
    // for x in unpacked {
    //     values.push(x.1);
    // }
    (keys, values)
}

fn comparison_ops_comparators(
    v: Vec<(ast::Cmpop, ast::Expr)>,
) -> (Vec<ast::Cmpop>, Vec<ast::Expr>) {
    let mut ops = Vec::with_capacity(v.len());
    let mut comparators = Vec::with_capacity(v.len());

    for x in v {
        ops.push(x.0);
        comparators.push(x.1);
    }
    (ops, comparators)
}

fn make_arguments(
    slash_no_default: Vec<ast::Arg>,
    slash_with_default: (Vec<ast::Arg>, Vec<(ast::Arg, ast::Expr)>),
    param_no_default: Vec<ast::Arg>,
    param_with_default: Vec<(ast::Arg, ast::Expr)>,
    star_etc: Option<(
        Option<ast::Arg>,
        Vec<(ast::Arg, Option<ast::Expr>)>,
        Option<ast::Arg>,
    )>,
) -> ast::Arguments {
    let mut posonlyargs = slash_no_default;
    posonlyargs.extend(slash_with_default.0.iter().cloned());
    posonlyargs.extend(slash_with_default.1.iter().map(|x| x.0.clone()));

    let mut posargs = param_no_default;
    posargs.extend(param_with_default.iter().map(|x| x.0.clone()));

    let posdefaults: Vec<ast::Expr> = slash_with_default
        .1
        .iter()
        .map(|x| x.1.clone())
        .chain(param_with_default.iter().map(|x| x.1.clone()))
        .collect();

    // TODO: refactor remove option wrap for star_etc
    let (vararg, kwonly, kwarg) = star_etc.unwrap_or_default();
    let kwonlyargs: Vec<ast::Arg> = kwonly.iter().map(|x| x.0.clone()).collect();
    let kw_defaults: Vec<ast::Expr> = kwonly.iter().filter_map(|x| x.1.clone()).collect();

    ast::Arguments {
        posonlyargs,
        args: posargs,
        vararg: option_box(vararg),
        kwonlyargs,
        kw_defaults,
        kwarg: option_box(kwarg),
        defaults: posdefaults,
    }
}

fn make_empty_arguments() -> ast::Arguments {
    ast::Arguments {
        posonlyargs: Default::default(),
        args: Default::default(),
        vararg: Default::default(),
        kwonlyargs: Default::default(),
        kw_defaults: Default::default(),
        kwarg: Default::default(),
        defaults: Default::default(),
    }
}