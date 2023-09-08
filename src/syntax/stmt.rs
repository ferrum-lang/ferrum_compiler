use super::*;

use crate::token::Token;
use crate::utils::{fe_from, fe_try_from, from, invert, try_from};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<T: ResolvedType = ()> {
    Expr(ExprStmt<T>),
    VarDecl(VarDeclStmt<T>),
    Assign(AssignStmt<T>),
    Return(ReturnStmt<T>),
    If(IfStmt<T>),
    Loop(LoopStmt<T>),
}

impl<T: ResolvedType> Node<Stmt> for Stmt<T> {
    fn node_id(&self) -> &NodeId<Stmt> {
        match self {
            Self::Expr(stmt) => return stmt.node_id(),
            Self::VarDecl(stmt) => return stmt.node_id(),
            Self::Assign(stmt) => return stmt.node_id(),
            Self::Return(stmt) => return stmt.node_id(),
            Self::If(stmt) => return stmt.node_id(),
            Self::Loop(stmt) => return stmt.node_id(),
        }
    }
}

impl<T: ResolvedType> From<Stmt<()>> for Stmt<Option<T>> {
    fn from(value: Stmt<()>) -> Self {
        match value {
            Stmt::Expr(stmt) => return Self::Expr(from(stmt)),
            Stmt::VarDecl(stmt) => return Self::VarDecl(from(stmt)),
            Stmt::Assign(stmt) => return Self::Assign(from(stmt)),
            Stmt::Return(stmt) => return Self::Return(from(stmt)),
            Stmt::If(stmt) => return Self::If(from(stmt)),
            Stmt::Loop(stmt) => return Self::Loop(from(stmt)),
        }
    }
}

impl<T: ResolvedType> Resolvable for Stmt<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            Self::Expr(stmt) => return stmt.is_resolved(),
            Self::VarDecl(stmt) => return stmt.is_resolved(),
            Self::Assign(stmt) => return stmt.is_resolved(),
            Self::Return(stmt) => return stmt.is_resolved(),
            Self::If(stmt) => return stmt.is_resolved(),
            Self::Loop(stmt) => return stmt.is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<Stmt<Option<T>>> for Stmt<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: Stmt<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            Stmt::Expr(stmt) => return Ok(Self::Expr(try_from(stmt)?)),
            Stmt::VarDecl(stmt) => return Ok(Self::VarDecl(try_from(stmt)?)),
            Stmt::Assign(stmt) => return Ok(Self::Assign(try_from(stmt)?)),
            Stmt::Return(stmt) => return Ok(Self::Return(try_from(stmt)?)),
            Stmt::If(stmt) => return Ok(Self::If(try_from(stmt)?)),
            Stmt::Loop(stmt) => return Ok(Self::Loop(try_from(stmt)?)),
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

#[derive(Debug, Clone, PartialEq)]
pub struct AssignStmt<T: ResolvedType = ()> {
    pub id: NodeId<Stmt>,
    pub target: NestedExpr<T>,
    pub op: AssignOp,
    pub value: NestedExpr<T>,
}

impl<T: ResolvedType> Node<Stmt> for AssignStmt<T> {
    fn node_id(&self) -> &NodeId<Stmt> {
        return &self.id;
    }
}

impl<T: ResolvedType> From<AssignStmt<()>> for AssignStmt<Option<T>> {
    fn from(value: AssignStmt<()>) -> Self {
        return Self {
            id: value.id,
            target: from(value.target),
            op: value.op,
            value: from(value.value),
        };
    }
}

impl<T: ResolvedType> Resolvable for AssignStmt<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.target.is_resolved() {
            return false;
        }

        if !self.value.is_resolved() {
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<AssignStmt<Option<T>>> for AssignStmt<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: AssignStmt<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            target: try_from(value.target)?,
            op: value.op,
            value: try_from(value.value)?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignOp {
    Eq(Arc<Token>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt<T: ResolvedType = ()> {
    pub id: NodeId<Stmt>,
    pub return_token: Arc<Token>,
    pub value: Option<NestedExpr<T>>,
}

impl<T: ResolvedType> Node<Stmt> for ReturnStmt<T> {
    fn node_id(&self) -> &NodeId<Stmt> {
        return &self.id;
    }
}

impl<T: ResolvedType> From<ReturnStmt<()>> for ReturnStmt<Option<T>> {
    fn from(value: ReturnStmt<()>) -> Self {
        return Self {
            id: value.id,
            return_token: value.return_token,
            value: value.value.map(from),
        };
    }
}

impl<T: ResolvedType> Resolvable for ReturnStmt<Option<T>> {
    fn is_resolved(&self) -> bool {
        if let Some(value) = &self.value {
            if !value.is_resolved() {
                return false;
            }
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<ReturnStmt<Option<T>>> for ReturnStmt<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: ReturnStmt<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            return_token: value.return_token,
            value: invert(value.value.map(try_from))?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt<T: ResolvedType = ()> {
    pub id: NodeId<Stmt>,
    pub if_token: Arc<Token>,
    pub condition: NestedExpr<T>,
    pub then_block: CodeBlock<T, ()>,
    pub else_ifs: Vec<ElseIfBranch<T>>,
    pub else_: Option<ElseBranch<T>>,
    pub semicolon_token: Arc<Token>,
}

impl<T: ResolvedType> Node<Stmt> for IfStmt<T> {
    fn node_id(&self) -> &NodeId<Stmt> {
        return &self.id;
    }
}

impl<T: ResolvedType> From<IfStmt<()>> for IfStmt<Option<T>> {
    fn from(value: IfStmt<()>) -> Self {
        return Self {
            id: value.id,
            if_token: value.if_token,
            condition: from(value.condition),
            then_block: from(value.then_block),
            else_ifs: value.else_ifs.into_iter().map(from).collect(),
            else_: value.else_.map(from),
            semicolon_token: value.semicolon_token,
        };
    }
}

impl<T: ResolvedType> Resolvable for IfStmt<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.condition.is_resolved() {
            dbg!("false");
            return false;
        }

        if !self.then_block.is_resolved() {
            dbg!("false");
            return false;
        }

        for elseif in &self.else_ifs {
            if !elseif.is_resolved() {
                dbg!("false");
                return false;
            }
        }

        if let Some(else_) = &self.else_ {
            if !else_.is_resolved() {
                dbg!("false");
                return false;
            }
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<IfStmt<Option<T>>> for IfStmt<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: IfStmt<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            if_token: value.if_token,
            condition: try_from(value.condition)?,
            then_block: try_from(value.then_block)?,
            else_ifs: value
                .else_ifs
                .into_iter()
                .map(try_from)
                .collect::<Result<Vec<_>, Self::Error>>()?,
            else_: invert(value.else_.map(try_from))?,
            semicolon_token: value.semicolon_token,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElseIfBranch<T: ResolvedType = ()> {
    pub else_token: Arc<Token>,
    pub if_token: Arc<Token>,
    pub condition: NestedExpr<T>,
    pub then_block: CodeBlock<T, ()>,
}

impl<T: ResolvedType> From<ElseIfBranch<()>> for ElseIfBranch<Option<T>> {
    fn from(value: ElseIfBranch<()>) -> Self {
        return Self {
            else_token: value.else_token,
            if_token: value.if_token,
            condition: from(value.condition),
            then_block: from(value.then_block),
        };
    }
}

impl<T: ResolvedType> Resolvable for ElseIfBranch<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.condition.is_resolved() {
            return false;
        }

        if !self.then_block.is_resolved() {
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<ElseIfBranch<Option<T>>> for ElseIfBranch<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: ElseIfBranch<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            else_token: value.else_token,
            if_token: value.if_token,
            condition: try_from(value.condition)?,
            then_block: try_from(value.then_block)?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElseBranch<T: ResolvedType = ()> {
    pub else_token: Arc<Token>,
    pub then_block: CodeBlock<T, ()>,
}

impl<T: ResolvedType> From<ElseBranch<()>> for ElseBranch<Option<T>> {
    fn from(value: ElseBranch<()>) -> Self {
        return Self {
            else_token: value.else_token,
            then_block: from(value.then_block),
        };
    }
}

impl<T: ResolvedType> Resolvable for ElseBranch<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.then_block.is_resolved() {
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<ElseBranch<Option<T>>> for ElseBranch<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: ElseBranch<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            else_token: value.else_token,
            then_block: try_from(value.then_block)?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoopStmt<T: ResolvedType = ()> {
    pub id: NodeId<Stmt>,
    pub loop_token: Arc<Token>,
    pub block: CodeBlock<T>,
}

impl<T: ResolvedType> Node<Stmt> for LoopStmt<T> {
    fn node_id(&self) -> &NodeId<Stmt> {
        return &self.id;
    }
}

impl<T: ResolvedType> From<LoopStmt<()>> for LoopStmt<Option<T>> {
    fn from(value: LoopStmt<()>) -> Self {
        return Self {
            id: value.id,
            loop_token: value.loop_token,
            block: from(value.block),
        };
    }
}

impl<T: ResolvedType> Resolvable for LoopStmt<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.block.is_resolved() {
            dbg!("false");
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<LoopStmt<Option<T>>> for LoopStmt<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: LoopStmt<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            loop_token: value.loop_token,
            block: try_from(value.block)?,
        });
    }
}

// Visitor pattern
pub trait StmtVisitor<T: ResolvedType, R = ()> {
    fn visit_expr_stmt(&mut self, stmt: &mut ExprStmt<T>) -> R;
    fn visit_var_decl_stmt(&mut self, stmt: &mut VarDeclStmt<T>) -> R;
    fn visit_assign_stmt(&mut self, stmt: &mut AssignStmt<T>) -> R;
    fn visit_return_stmt(&mut self, stmt: &mut ReturnStmt<T>) -> R;
    fn visit_if_stmt(&mut self, stmt: &mut IfStmt<T>) -> R;
    fn visit_loop_stmt(&mut self, stmt: &mut LoopStmt<T>) -> R;
}

pub trait StmtAccept<T: ResolvedType, R, V: StmtVisitor<T, R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for Stmt<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return match self {
            Self::Expr(stmt) => stmt.accept(visitor),
            Self::VarDecl(stmt) => stmt.accept(visitor),
            Self::Assign(stmt) => stmt.accept(visitor),
            Self::Return(stmt) => stmt.accept(visitor),
            Self::If(stmt) => stmt.accept(visitor),
            Self::Loop(stmt) => stmt.accept(visitor),
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

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for AssignStmt<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_assign_stmt(self);
    }
}

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for ReturnStmt<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_return_stmt(self);
    }
}

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for IfStmt<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_if_stmt(self);
    }
}

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for LoopStmt<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_loop_stmt(self);
    }
}
