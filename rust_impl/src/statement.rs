

pub trait Stmt {
    fn accept<T>(self,Visitor<T>) -> T
}
pub trait Visitor<T> {
    T visit_class_stmt(ClassStmt stmt);

}
pub struct ClassStmt {

}
impl Stmt for ClassStmt {
    fn accept<T>(self, vis Visitor<T>) -> T {
        vis.visit_block_stmt(self)
    }
}