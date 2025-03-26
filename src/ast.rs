pub enum Statement {
    Let(String, Expression), 
    Return(Expression),
    Expression(Expression),
    Block(Vec<Statement>),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    While(Expression, Box<Statement>),
}

pub enum Expression {
    Identifier(String),
    IntegerLiteral(i64),
    StringLiteral(String),
    CharLiteral(char),
    Prefix(PrefixOp, Box<Expression>),
    Infix(Box<Expression>, InfixOp, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Index(Box<Expression>, Box<Expression>),
}

pub enum PrefixOp {
    Not,
    Neg,
}

pub enum InfixOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    And,
    Or,
}