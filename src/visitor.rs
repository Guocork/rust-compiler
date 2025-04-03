use crate::ast::{BinaryOp, Expression, LiteralValue, PrefixOp, Span, Statement, Type};

pub trait StmtVisitor<T> {
    fn visit_block(&mut self, statements: &[Statement], span: &Span) -> T;
    fn visit_if(&mut self, condition: &Expression, then_branch: &Statement, else_branch: Option<&Statement>, span: &Span) -> T;
    fn visit_while(&mut self, condition: &Expression, body: &Statement, span: &Span) -> T;
    fn visit_for(&mut self, init: Option<&Statement>, condition: &Expression, increment: Option<&Statement>, body: &Statement, span: &Span) -> T;
    fn visit_return(&mut self, value: Option<&Expression>, span: &Span) -> T;
    fn visit_expression(&mut self, expression: &Expression, span: &Span) -> T;
    fn visit_variable_declaration(&mut self, name: &str, type_ann: &Option<Type>, initializer: Option<&Expression>, span: &Span) -> T;

    // 遍历路由 完美解耦
    fn visit_stmt(&mut self, stmt: &Statement) -> T {
        match stmt {
            Statement::Block { statements, span } => self.visit_block(statements, span),
            Statement::If { condition, then_branch, else_branch, span } => self.visit_if(condition, then_branch, else_branch.as_deref(), span),
            Statement::While { condition, body, span } => self.visit_while(condition, body, span),
            Statement::For { init, condition, increment, body, span } => self.visit_for(init.as_deref(), condition, increment.as_deref(), body, span),
            Statement::Return { value, span } => self.visit_return(value.as_deref(), span),
            Statement::Expression { expression, span } => self.visit_expression(expression, span),
            Statement::VariableDeclaration { name, type_ann, initializer, span } => self.visit_variable_declaration(name, type_ann, initializer.as_deref(), span),
        }
    }
}

pub trait ExprVisitor<T> {
    fn visit_binary(&mut self, left: &Expression, operator: &BinaryOp, right: &Expression, span: &Span) -> T;
    fn visit_unary(&mut self, operator: &PrefixOp, argument: &Expression, span: &Span) -> T;
    fn visit_literal(&mut self, value: &LiteralValue, span: &Span) -> T;
    fn visit_assign(&mut self, target: &Expression, value: &Expression, span: &Span) -> T;
    fn visit_call(&mut self, callee: &Expression, arguments: &[Expression], span: &Span) -> T;
    fn visit_variable(&mut self, name: &str, span: &Span) -> T;


    fn visit_expr(&mut self, expr: &Expression) -> T {
        match expr {
            Expression::Binary { left, operator, right, span } => self.visit_binary(left, operator, right, span),
            Expression::Unary { operator, operand, span } => self.visit_unary(operator, operand, span),
            Expression::Literal { value, span } => self.visit_literal(value, span),
            Expression::Assign { target, value, span } => self.visit_assign(target, value, span),
            Expression::Call { callee, arguments, span } => self.visit_call(callee, arguments, span),
            Expression::Variable { name, span } => self.visit_variable(name, span)
        }
    }
    
}