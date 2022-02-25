pub trait Stmt {
    fn accept<T>(&mut self, vis: &mut Box<dyn Visitor<T>>) -> T;
}
pub struct ClassStmt {}
pub trait Visitor<T> {
    fn visit_class_stmt(&mut self, stmt: &ClassStmt) -> T;
    fn visit_expr_stmt(&mut self, stmt: &ExprStmt) -> T;
}

impl Stmt for ClassStmt {
    fn accept<T>(&mut self, vis: &mut Box<dyn Visitor<T>>) -> T {
        vis.visit_class_stmt(self)
    }
}

pub struct ExprStmt {}
impl Stmt for ExprStmt {
    fn accept<T>(&mut self, vis: &mut Box<dyn Visitor<T>>) -> T {
        vis.visit_expr_stmt(self)
    }
}
