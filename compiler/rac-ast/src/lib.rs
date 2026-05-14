type Name = String;

pub struct Module<N> {
    name: N,
    defs: Vec<Definition<N>>,
    expr: Option<Expr<N>>
}

pub enum Definition<N> {
    AbstractDef(N),
    CaseClassDef(N, Vec<Box<Type<N>>>),
    FunDef(N, Vec<(N, Type<N>)>, Type<N>, Expr<N>),
}

pub enum Expr<N> {
    // Variables
    Variable(N),

    // Literals
    IntLiteral(i32),
    BoolLiteral(bool),
    StringLiteral(String),
    UnitLiteral,
    
    // Binary operators
    Plus(Box<Expr<N>>, Box<Expr<N>>),
    Minus(Box<Expr<N>>, Box<Expr<N>>),
    Times(Box<Expr<N>>, Box<Expr<N>>),
    Div(Box<Expr<N>>, Box<Expr<N>>),
    Mod(Box<Expr<N>>, Box<Expr<N>>),
    LessThan(Box<Expr<N>>, Box<Expr<N>>),
    LessEquals(Box<Expr<N>>, Box<Expr<N>>),
    And(Box<Expr<N>>, Box<Expr<N>>),
    Or(Box<Expr<N>>, Box<Expr<N>>),
    Equals(Box<Expr<N>>, Box<Expr<N>>),
    Concat(Box<Expr<N>>, Box<Expr<N>>),

    // Unary operators
    Not(Box<Expr<N>>),
    Neg(Box<Expr<N>>),

    // Function/constructor call
    Call(N, Vec<Box<Expr<N>>>),

    // Control flow
    Sequence(Box<Expr<N>>, Box<Expr<N>>),
    Let(N, Type<N>, Box<Expr<N>>, Box<Expr<N>>),
    Ite(Box<Expr<N>>, Box<Expr<N>>, Box<Expr<N>>),

    // Pattern matching
    Match(Box<Expr<N>>, Vec<(Box<Expr<N>>, Box<Expr<N>>)>),

    // Errors
    Error(Box<Expr<N>>),
}

// Do we even need a special pattern enum? we can just say `_` is an
// expression, and treat patterns as expressions...

// pub enum Pattern<N> {
//     Wildcard,
//     IdPattern(N),
//     
// }

pub enum Type<N> {
    // Primitive types
    IntType,
    BoolType,
    StringType,
    UnitType,
    // User-defined types
    ClassType(N),
}
