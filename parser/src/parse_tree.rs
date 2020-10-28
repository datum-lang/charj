use crate::location::{Loc, Location};

#[derive(Debug, PartialEq)]
pub struct SourceUnit(pub Vec<SourceUnitPart>);

#[derive(Debug, PartialEq)]
pub enum SourceUnitPart {
    ImportDirective(Import),
    MultipleImportDirective(Vec<Import>),
    PackageDirective(Package),
    StructFuncDef(Box<StructFuncDef>),
    FuncDef(Box<FuncDef>),
    StructDef(Box<StructDef>),
}

pub type Suite = Vec<Statement>;

#[derive(Debug, PartialEq)]
pub struct FuncDef {
    pub loc: Loc,
    pub name: Identifier,
    pub params: Vec<(Loc, Option<Parameter>)>,
    pub body: Suite,
}

#[derive(Debug, PartialEq)]
pub struct StructFuncDef {
    pub loc: Loc,
    pub name: Identifier,
    pub struct_name: Identifier,
    pub params: Vec<(Loc, Option<Parameter>)>,
    pub body: Suite,
}

#[derive(Debug, PartialEq, Default)]
pub struct Parameters {
    pub args: Vec<Parameter>,
}

/// A single formal parameter to a function.
#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub loc: Loc,
    pub ty: Expression,
    pub name: Option<Identifier>,
}

/// An expression at a given location in the sourcecode.
pub type Expression = Located<ExpressionType>;

/// A certain type of expression.
#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    /// A `list` literal value.
    List { elements: Vec<Expression> },
    /// An identifier, designating a certain variable or type.
    Identifier { name: Identifier },

    /// Attribute access in the form of `value.name`.
    Attribute {
        value: Box<Expression>,
        name: Identifier,
    },
    /// A call expression.
    Call {
        function: Box<Expression>,
        args: Vec<Argument>,
        // keywords: Vec<Keyword>,
    },

    /// A chained comparison. Note that in python you can use
    /// `1 < a < 10` for example.
    Compare {
        vals: Vec<Expression>,
        ops: Vec<Comparison>,
    },
}
//
// #[derive(Debug, PartialEq)]
// pub struct ArgumentList {
//     pub args: Vec<Argument>,
// }

#[derive(Debug, PartialEq)]
pub struct Argument {
    pub location: Location,
    pub expr: Expression,
}

#[derive(Debug, PartialEq)]
pub struct Keyword {
    pub name: Option<String>,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct Located<T> {
    pub location: Location,
    pub node: T,
}

pub type Statement = Located<StatementType>;

#[derive(Debug, PartialEq)]
pub enum StatementType {
    Break,
    Continue,
    If,
    While,
    For,
    Loop,

    Return { value: Option<Expression> },
    List { elements: Vec<Expression> },
}

#[derive(Debug, PartialEq)]
pub struct StructDef {
    pub loc: Loc,
    pub name: Identifier,
    pub fields: Vec<VariableDeclaration>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {}

#[derive(Debug, PartialEq)]
pub enum Package {
    Plain(Identifier),
}

#[derive(Debug, PartialEq)]
pub enum Import {
    Standard(Identifier),
    Remote, // for such github.com/phodal/coca
    GlobalSymbol(StringLiteral, Identifier),
    Rename(StringLiteral, Vec<(Identifier, Option<Identifier>)>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub loc: Loc,
    pub string: String,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Identifier {
    pub loc: Loc,
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DocComment {
    pub offset: usize,
    pub tag: String,
    pub value: String,
}

/// A comparison operation.
#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    In,
    NotIn,
    Is,
    IsNot,
}