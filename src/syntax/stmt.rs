use super::*;

use crate::token::Token;
use crate::utils::{fe_from, fe_try_from, from, invert, try_from};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<T: ResolvedType = ()> {
    Expr(ExprStmt<T>),
    VarDecl(VarDeclStmt<T>),
}

impl<T: ResolvedType> Node<Stmt> for Stmt<T> {
    fn node_id(&self) -> &NodeId<Stmt> {
        match self {
            Self::Expr(stmt) => return stmt.node_id(),
            Self::VarDecl(stmt) => return stmt.node_id(),
        }
    }
}

impl<T: ResolvedType> From<Stmt<()>> for Stmt<Option<T>> {
    fn from(value: Stmt<()>) -> Self {
        match value {
            Stmt::Expr(stmt) => return Self::Expr(from(stmt)),
            Stmt::VarDecl(stmt) => return Self::VarDecl(from(stmt)),
        }
    }
}

impl<T: ResolvedType> Resolvable for Stmt<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            Self::Expr(stmt) => return stmt.is_resolved(),
            Self::VarDecl(stmt) => return stmt.is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<Stmt<Option<T>>> for Stmt<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: Stmt<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            Stmt::Expr(stmt) => return Ok(Self::Expr(try_from(stmt)?)),
            Stmt::VarDecl(stmt) => return Ok(Self::VarDecl(try_from(stmt)?)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExprStmt<T: ResolvedType = ()> {
    pub id: NodeId<Stmt>,
    pub expr: Arc<Mutex<Expr<T>>>,
}

impl<T: ResolvedType> Node<Stmt> for ExprStmt<T> {
    fn node_id(&self) -> &NodeId<Stmt> {
        return &self.id;
    }
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

#[derive(Debug, Clone, PartialEq)]
pub struct VarDeclStmt<T: ResolvedType = ()> {
    pub id: NodeId<Stmt>,
    pub var_mut: VarDeclMut,
    pub target: VarDeclTarget<T>,
    pub explicit_type: Option<VarDeclExplicitType<T>>,
    pub value: Option<VarDeclValue<T>>,
}

impl<T: ResolvedType> Node<Stmt> for VarDeclStmt<T> {
    fn node_id(&self) -> &NodeId<Stmt> {
        return &self.id;
    }
}

impl<T: ResolvedType> From<VarDeclStmt<()>> for VarDeclStmt<Option<T>> {
    fn from(value: VarDeclStmt<()>) -> Self {
        return Self {
            id: value.id,
            var_mut: value.var_mut,
            target: from(value.target),
            explicit_type: value.explicit_type.map(from),
            value: value.value.map(from),
        };
    }
}

impl<T: ResolvedType> Resolvable for VarDeclStmt<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.target.is_resolved() {
            return false;
        }

        if let Some(et) = &self.explicit_type {
            if !et.is_resolved() {
                return false;
            }
        }

        if let Some(v) = &self.value {
            if !v.is_resolved() {
                return false;
            }
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<VarDeclStmt<Option<T>>> for VarDeclStmt<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: VarDeclStmt<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            var_mut: value.var_mut,
            target: try_from(value.target)?,
            explicit_type: invert(value.explicit_type.map(try_from))?,
            value: invert(value.value.map(try_from))?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarDeclMut {
    Const(Arc<Token>),
    Mut(Arc<Token>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarDeclTarget<T: ResolvedType = ()> {
    Ident(IdentExpr<T>),
}

impl<T: ResolvedType> From<VarDeclTarget<()>> for VarDeclTarget<Option<T>> {
    fn from(value: VarDeclTarget<()>) -> Self {
        match value {
            VarDeclTarget::Ident(target) => return Self::Ident(from(target)),
        }
    }
}

impl<T: ResolvedType> Resolvable for VarDeclTarget<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            Self::Ident(target) => return target.is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<VarDeclTarget<Option<T>>> for VarDeclTarget<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: VarDeclTarget<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            VarDeclTarget::Ident(target) => return Ok(Self::Ident(try_from(target)?)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDeclExplicitType<T: ResolvedType = ()> {
    pub colon_token: Arc<Token>,

    // pub static_ref: StaticType<T>,
    pub tmp: T,
}

impl<T: ResolvedType> From<VarDeclExplicitType<()>> for VarDeclExplicitType<Option<T>> {
    fn from(value: VarDeclExplicitType<()>) -> Self {
        return Self {
            colon_token: value.colon_token,

            tmp: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for VarDeclExplicitType<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.tmp.is_some();
    }
}

impl<T: ResolvedType> TryFrom<VarDeclExplicitType<Option<T>>> for VarDeclExplicitType<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: VarDeclExplicitType<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            colon_token: value.colon_token,

            tmp: value.tmp.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDeclValue<T: ResolvedType = ()> {
    pub eq_token: Arc<Token>,
    pub value: NestedExpr<T>,
}

impl<T: ResolvedType> From<VarDeclValue<()>> for VarDeclValue<Option<T>> {
    fn from(value: VarDeclValue<()>) -> Self {
        return Self {
            eq_token: value.eq_token,
            value: from(value.value),
        };
    }
}

impl<T: ResolvedType> Resolvable for VarDeclValue<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.value.is_resolved();
    }
}

impl<T: ResolvedType> TryFrom<VarDeclValue<Option<T>>> for VarDeclValue<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: VarDeclValue<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            eq_token: value.eq_token,
            value: try_from(value.value)?,
        });
    }
}

// Visitor pattern
pub trait StmtVisitor<T: ResolvedType, R = ()> {
    fn visit_expr_stmt(&mut self, stmt: &mut ExprStmt<T>) -> R;
    fn visit_var_decl_stmt(&mut self, stmt: &mut VarDeclStmt<T>) -> R;
}

pub trait StmtAccept<T: ResolvedType, R, V: StmtVisitor<T, R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for Stmt<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return match self {
            Self::Expr(stmt) => stmt.accept(visitor),
            Self::VarDecl(stmt) => stmt.accept(visitor),
        };
    }
}

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for ExprStmt<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_expr_stmt(self);
    }
}

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for VarDeclStmt<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_var_decl_stmt(self);
    }
}
