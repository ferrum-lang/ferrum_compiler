use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<T: ResolvedType = ()> {
    Expr(ExprStmt<T>),
}

#[derive(Debug, Clone)]
pub struct ExprStmt<T: ResolvedType = ()> {
    pub id: NodeId<Stmt>,
    pub expr: Arc<Mutex<Expr<T>>>,
}

impl<T: ResolvedType> PartialEq for ExprStmt<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.id != other.id {
            return false;
        }

        let expr = {
            let locked = self.expr.lock().unwrap();
            locked.clone()
        };

        let other = other.expr.lock().unwrap();

        if expr != *other {
            return false;
        }

        return true;
    }
}

// Visitor pattern
pub trait StmtVisitor<T: ResolvedType, R = ()> {
    fn visit_expr_stmt(&mut self, stmt: &mut ExprStmt<T>) -> R;
}

pub trait StmtAccept<T: ResolvedType, R, V: StmtVisitor<T, R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for Stmt<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return match self {
            Self::Expr(stmt) => stmt.accept(visitor),
        };
    }
}

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for ExprStmt<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_expr_stmt(self);
    }
}
