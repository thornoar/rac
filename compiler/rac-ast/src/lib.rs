type Name = String;

pub enum AST<N> {
    // Variables
    Variable(N),

    // Literals
    IntLiteral(i32),
    BoolLiteral(bool),
    StringLiteral(String),
    UnitLiteral,
    
    // Operators
    Plus(Box<AST<N>>, Box<AST<N>>)
}
