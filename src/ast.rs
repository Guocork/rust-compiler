#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum Statement {
    Block {                // { ... }
        statements: Vec<Statement>,
        span: Span,
    },
    If {                   // if (condition) { ... } else { ... }
        condition: Box<Expression>,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
        span: Span,
    },
    While {                // while (condition) { ... }
        condition: Box<Expression>,
        body: Box<Statement>,
        span: Span,
    },
    For {                  // for (init; condition; increment) { ... }
        init: Option<Box<Statement>>,
        condition: Box<Expression>,
        increment: Option<Box<Statement>>,
        body: Box<Statement>,
        span: Span,
    },
    Return {               // return value;
        value: Option<Box<Expression>>,
        span: Span,
    },
    Expression {           // expression;
        expression: Box<Expression>,
        span: Span,
    },
    VariableDeclaration {  // let x: i64 = 5;
        name: String,
        type_ann: Option<Type>,
        initializer: Option<Box<Expression>>,
        span: Span,
    },
    // FunctionDeclaration {  // fun foo(x, y) { ... }
    //     name: String,
    //     parameters: Vec<String>,
    //     body: Box<Statement>,
    //     span: Span,
    // },
    // FunctionCall {         // foo(x, y);
    //     callee: Box<Expression>,
    //     arguments: Vec<Expression>,
    //     span: Span,
    // },
    // ClassDeclaration {     // class Foo { ... }
    //     name: String,
    //     methods: Vec<FunctionDeclaration>,
    //     span: Span,
    // },
    // MethodDeclaration {    // fun methodName() { ... }
    //     name: String,
    //     parameters: Vec<String>,
    //     body: Box<Statement>,
    //     span: Span,
    // },
    // ClassMethodCall {      // instance.methodName(args);
    //     instance: Box<Expression>,
    //     method: String,
    //     arguments: Vec<Expression>,
    //     span: Span,
    // },
    // ClassFieldAccess {     // instance.fieldName
    //     instance: Box<Expression>,
    //     field: String,
    //     span: Span,
    // },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Binary {                    // 5 + 3
        left: Box<Expression>,
        operator: BinaryOp,
        right: Box<Expression>,
        span: Span,
    },
    Unary {                     // -5
        operator: PrefixOp,
        operand: Box<Expression>,
        span: Span,
    },
    Literal {                   // 5, "hello", true
        value: LiteralValue,
        span: Span,
    },
    Variable {                 // x
        name: String,
        span: Span,
    },
    Call {                     // foo(x, y)
        callee: Box<Expression>,
        arguments: Vec<Expression>,
        span: Span,
    },
    Assign {                   // x = 5
        target: Box<Expression>,
        value: Box<Expression>,
        span: Span,
    },
}

#[derive(Debug, Clone)]
pub enum PrefixOp {
    Not,               // !
    Neg,               // -
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Plus,              // +
    Minus,             // -
    Multiply,          // *
    Divide,            // /
    Modulo,            // %
    Equal,             // ==
    NotEqual,          // !=
    LessThan,          // <
    GreaterThan,       // >
    LessEqual,         // <=
    GreaterEqual,      // >=
    And,               // &&
    Or,                // ||
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Integer(i64),
    String(String),
    Char(char),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Void,
    Array(Box<Type>),
    Object(String),
    Function(Vec<Type>, Box<Type>), // (param_types) -> return_type
}