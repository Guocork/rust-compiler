use std::collections::HashMap;

use crate::{ast::{BinaryOp, Expression, LiteralValue, Span, Statement}, visitor::{ExprVisitor, StmtVisitor}};


// 假设我们有以下代码：
// if (a > b) {
//     return a;
// } else {
//     return b;
// }
fn create_sample_ast() -> Statement {
    let span = Span {
        start: 0,
        end: 10,
        line: 1,
        column: 1,
    };

    // 创建条件表达式: a > b
    let condition = Box::new(
        Expression::Binary {
            left: Box::new(
                Expression::Variable {
                    name: "a".to_string(),
                    span: span.clone(),
                }
            ),
            operator: BinaryOp::GreaterThan,
            right: Box::new(
                Expression::Variable {
                    name: "b".to_string(),
                    span: span.clone(),
                }
            ),
            span: span.clone(),
        }
    );

    // then_branch 为什么不需要Block 我感觉是需要的
    let then_branch = Box::new(
        Statement::Block {
            statements: vec![
                Statement::Return {
                    value: Some(
                        Box::new(
                            Expression::Variable {
                                name: "a".to_string(),
                                span: span.clone(),
                            }
                        )
                    ),
                    span: span.clone(),
                }
            ],
            span: span.clone(),
        }
    );

    // else_branch
    let else_branch = Some(
        Box::new(
            Statement::Block {
                statements: vec![
                    Statement::Return {
                        value: Some(
                            Box::new(
                                Expression::Variable {
                                    name: "b".to_string(),
                                    span: span.clone(),
                                }
                            )
                        ),
                        span: span.clone(),
                    },
                ],
                span: span.clone(),
            }
        )
    );

    // 创建if语句
    Statement::If {
        condition,
        then_branch,
        else_branch,
        span,
    }
}


// 一个简单的Evaluator
// 这个Evaluator会遍历AST并计算表达式的值
struct Evaluator {
    variables: HashMap<String, LiteralValue>,
}

impl Evaluator {
    fn new() -> Self {
        Self { variables: HashMap::new() }
    }

    fn evaluate(&mut self, stmt: &Statement) -> Option<LiteralValue> {
        self.visit_stmt(stmt)
    }
}

impl StmtVisitor<Option<LiteralValue>> for Evaluator { // 访问者相当于把所有的ast转化成了想要的结果 T 这就是为什么要叫计算器
    fn visit_block(&mut self, statements: &[Statement], _span: &Span) -> Option<LiteralValue> {
        let mut result = None;
        for stmt in statements {
            result = self.visit_stmt(stmt);
            // 如果遇到return语句，直接返回结果
            if result.is_some() {
                return result;
            }
        }
        result
    }

    fn visit_if(&mut self, condition: &Expression, then_branch: &Statement, else_branch: Option<&Statement>, span: &Span) -> Option<LiteralValue> {
        let cond_value = 
    }
}

impl ExprVisitor<Option<LiteralValue>> for Evaluator {
    fn visit_binary(&mut self, left: &Expression, operator: &BinaryOp, right: &Expression, span: &Span) -> Option<LiteralValue> {
        let left_val = self.visit_expr(left);
        let right_val = self.visit_expr(right);
        match (left_val, right_val) {
            (Some(LiteralValue::Integer(l)), Some(LiteralValue::Integer(r))) => {
                match operator {
                    BinaryOp::Plus => Some(LiteralValue::Integer(l + r)),
                    BinaryOp::Minus => Some(LiteralValue::Integer(l - r)),
                    BinaryOp::Multiply => Some(LiteralValue::Integer(l * r)),
                    BinaryOp::Divide => Some(LiteralValue::Integer(l / r)),
                    BinaryOp::GreaterThan => Some(LiteralValue::Bool(l > r)),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}