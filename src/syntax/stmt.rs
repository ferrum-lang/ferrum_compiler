use super::*;

use crate::utils::{fe_from, fe_try_from, from, invert, try_from};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<T: ResolvedType = ()> {
    Expr(ExprStmt<T>),
}

impl<T: ResolvedType> From<Stmt<()>> for Stmt<Option<T>> {
    fn from(value: Stmt<()>) -> Self {
        match value {
            Stmt::Expr(stmt) => return Self::Expr(from(stmt)),
        }
    }
}

impl<T: ResolvedType> Resolvable for Stmt<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            Self::Expr(stmt) => return stmt.is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<Stmt<Option<T>>> for Stmt<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: Stmt<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            Stmt::Expr(stmt) => return Ok(Self::Expr(try_from(stmt)?)),
        }
    }
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

impl<T: ResolvedType> From<ExprStmt<()>> for ExprStmt<Option<T>> {
    fn from(value: ExprStmt<()>) -> Self {
        return Self {
            id: value.id,
            expr: fe_from(value.expr),
        };
    }
}

impl<T: ResolvedType> Resolvable for ExprStmt<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.expr.lock().unwrap().is_resolved();
    }
}

impl<T: ResolvedType> TryFrom<ExprStmt<Option<T>>> for ExprStmt<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: ExprStmt<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            expr: fe_try_from(value.expr)?,
        });
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
