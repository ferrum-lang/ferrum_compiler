use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(ExprStmt),
}

#[derive(Debug, Clone)]
pub struct ExprStmt {
    pub id: NodeId<Stmt>,
    pub expr: Arc<Mutex<Expr>>,
}

impl PartialEq for ExprStmt {
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
pub trait StmtVisitor<R = ()> {
    fn visit_expr_stmt(&mut self, stmt: &mut ExprStmt) -> R;
}

pub trait StmtAccept<R, V: StmtVisitor<R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<R, V: StmtVisitor<R>> StmtAccept<R, V> for Stmt {
    fn accept(&mut self, visitor: &mut V) -> R {
        return match self {
            Self::Expr(stmt) => stmt.accept(visitor),
        };
    }
}

impl<R, V: StmtVisitor<R>> StmtAccept<R, V> for ExprStmt {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_expr_stmt(self);
    }
}
