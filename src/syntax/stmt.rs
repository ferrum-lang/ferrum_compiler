use super::*;

use crate::token::Token;
use crate::utils::{fe_from, fe_try_from, from, invert, try_from};

#[derive(Debug, Clone)]
pub enum Stmt<T: ResolvedType = ()> {
    Expr(Arc<Mutex<ExprStmt<T>>>),
    VarDecl(Arc<Mutex<VarDeclStmt<T>>>),
    Assign(Arc<Mutex<AssignStmt<T>>>),
    Return(Arc<Mutex<ReturnStmt<T>>>),
    If(Arc<Mutex<IfStmt<T>>>),
    Loop(Arc<Mutex<LoopStmt<T>>>),
    While(Arc<Mutex<WhileStmt<T>>>),
    Break(Arc<Mutex<BreakStmt<T>>>),
    Then(Arc<Mutex<ThenStmt<T>>>),
}

impl<T: ResolvedType> PartialEq for Stmt<T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Expr(d) => {
                let Self::Expr(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::VarDecl(d) => {
                let Self::VarDecl(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::Assign(d) => {
                let Self::Assign(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::Return(d) => {
                let Self::Return(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::If(d) => {
                let Self::If(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::Loop(d) => {
                let Self::Loop(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::While(d) => {
                let Self::While(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::Break(d) => {
                let Self::Break(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::Then(d) => {
                let Self::Then(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
        }
    }
}

impl<T: ResolvedType> Node<Stmt> for Stmt<T> {
    fn node_id(&self) -> NodeId<Stmt> {
        match self {
            Self::Expr(stmt) => return stmt.try_lock().unwrap().node_id(),
            Self::VarDecl(stmt) => return stmt.try_lock().unwrap().node_id(),
            Self::Assign(stmt) => return stmt.try_lock().unwrap().node_id(),
            Self::Return(stmt) => return stmt.try_lock().unwrap().node_id(),
            Self::If(stmt) => return stmt.try_lock().unwrap().node_id(),
            Self::Loop(stmt) => return stmt.try_lock().unwrap().node_id(),
            Self::While(stmt) => return stmt.try_lock().unwrap().node_id(),
            Self::Break(stmt) => return stmt.try_lock().unwrap().node_id(),
            Self::Then(stmt) => return stmt.try_lock().unwrap().node_id(),
        }
    }
}

impl<T: ResolvedType> IsTerminal<T> for Stmt<T> {
    fn is_terminal(&mut self) -> bool {
        match self {
            Self::Expr(stmt) => return stmt.try_lock().unwrap().is_terminal(),
            Self::VarDecl(stmt) => return stmt.try_lock().unwrap().is_terminal(),
            Self::Assign(stmt) => return stmt.try_lock().unwrap().is_terminal(),
            Self::Return(stmt) => return stmt.try_lock().unwrap().is_terminal(),
            Self::If(stmt) => return stmt.try_lock().unwrap().is_terminal(),
            Self::Loop(stmt) => return stmt.try_lock().unwrap().is_terminal(),
            Self::While(stmt) => return stmt.try_lock().unwrap().is_terminal(),
            Self::Break(stmt) => return stmt.try_lock().unwrap().is_terminal(),
            Self::Then(stmt) => return stmt.try_lock().unwrap().is_terminal(),
        }
    }
}

impl<T: ResolvedType> From<Stmt<()>> for Stmt<Option<T>> {
    fn from(value: Stmt<()>) -> Self {
        match value {
            Stmt::Expr(stmt) => return Self::Expr(fe_from(stmt)),
            Stmt::VarDecl(stmt) => return Self::VarDecl(fe_from(stmt)),
            Stmt::Assign(stmt) => return Self::Assign(fe_from(stmt)),
            Stmt::Return(stmt) => return Self::Return(fe_from(stmt)),
            Stmt::If(stmt) => return Self::If(fe_from(stmt)),
            Stmt::Loop(stmt) => return Self::Loop(fe_from(stmt)),
            Stmt::While(stmt) => return Self::While(fe_from(stmt)),
            Stmt::Break(stmt) => return Self::Break(fe_from(stmt)),
            Stmt::Then(stmt) => return Self::Then(fe_from(stmt)),
        }
    }
}

impl<T: ResolvedType> Resolvable for Stmt<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            Self::Expr(stmt) => return stmt.try_lock().unwrap().is_resolved(),
            Self::VarDecl(stmt) => return stmt.try_lock().unwrap().is_resolved(),
            Self::Assign(stmt) => return stmt.try_lock().unwrap().is_resolved(),
            Self::Return(stmt) => return stmt.try_lock().unwrap().is_resolved(),
            Self::If(stmt) => return stmt.try_lock().unwrap().is_resolved(),
            Self::Loop(stmt) => return stmt.try_lock().unwrap().is_resolved(),
            Self::While(stmt) => return stmt.try_lock().unwrap().is_resolved(),
            Self::Break(stmt) => return stmt.try_lock().unwrap().is_resolved(),
            Self::Then(stmt) => return stmt.try_lock().unwrap().is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<Stmt<Option<T>>> for Stmt<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: Stmt<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            Stmt::Expr(stmt) => return Ok(Self::Expr(fe_try_from(stmt)?)),
            Stmt::VarDecl(stmt) => return Ok(Self::VarDecl(fe_try_from(stmt)?)),
            Stmt::Assign(stmt) => return Ok(Self::Assign(fe_try_from(stmt)?)),
            Stmt::Return(stmt) => return Ok(Self::Return(fe_try_from(stmt)?)),
            Stmt::If(stmt) => return Ok(Self::If(fe_try_from(stmt)?)),
            Stmt::Loop(stmt) => return Ok(Self::Loop(fe_try_from(stmt)?)),
            Stmt::While(stmt) => return Ok(Self::While(fe_try_from(stmt)?)),
            Stmt::Break(stmt) => return Ok(Self::Break(fe_try_from(stmt)?)),
            Stmt::Then(stmt) => return Ok(Self::Then(fe_try_from(stmt)?)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExprStmt<T: ResolvedType = ()> {
    pub id: NodeId<Stmt>,
    pub expr: Arc<Mutex<Expr<T>>>,
}

impl<T: ResolvedType> Node<Stmt> for ExprStmt<T> {
    fn node_id(&self) -> NodeId<Stmt> {
        return self.id;
    }
}

impl<T: ResolvedType> IsTerminal<T> for ExprStmt<T> {}

impl<T: ResolvedType> PartialEq for ExprStmt<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.id != other.id {
            return false;
        }

        let expr = {
            let locked = self.expr.try_lock().unwrap();
            locked.clone()
        };

        let other = other.expr.try_lock().unwrap();

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
        return self.expr.try_lock().unwrap().is_resolved();
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
    fn node_id(&self) -> NodeId<Stmt> {
        return self.id;
    }
}

impl<T: ResolvedType> IsTerminal<T> for VarDeclStmt<T> {}

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

#[derive(Debug, Clone)]
pub enum VarDeclTarget<T: ResolvedType = ()> {
    Ident(Arc<Mutex<IdentExpr<T>>>),
}

impl<T: ResolvedType> PartialEq for VarDeclTarget<T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Ident(d) => {
                let Self::Ident(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
        }
    }
}

impl<T: ResolvedType> From<VarDeclTarget<()>> for VarDeclTarget<Option<T>> {
    fn from(value: VarDeclTarget<()>) -> Self {
        match value {
            VarDeclTarget::Ident(target) => return Self::Ident(fe_from(target)),
        }
    }
}

impl<T: ResolvedType> Resolvable for VarDeclTarget<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            Self::Ident(target) => return target.try_lock().unwrap().is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<VarDeclTarget<Option<T>>> for VarDeclTarget<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: VarDeclTarget<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            VarDeclTarget::Ident(target) => return Ok(Self::Ident(fe_try_from(target)?)),
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

impl<T: ResolvedType> IsTerminal<T> for AssignStmt<T> {}

impl<T: ResolvedType> Node<Stmt> for AssignStmt<T> {
    fn node_id(&self) -> NodeId<Stmt> {
        return self.id;
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
    PlusEq(Arc<Token>),
}

#[derive(Debug, Clone)]
pub enum ReturnHandler {
    Fn(Arc<Mutex<FnDecl<Option<FeType>>>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt<T: ResolvedType = ()> {
    pub id: NodeId<Stmt>,
    pub return_token: Arc<Token>,
    pub value: Option<NestedExpr<T>>,
}

impl<T: ResolvedType> Node<Stmt> for ReturnStmt<T> {
    fn node_id(&self) -> NodeId<Stmt> {
        return self.id;
    }
}

impl<T: ResolvedType> IsTerminal<T> for ReturnStmt<T> {
    fn is_terminal(&mut self) -> bool {
        return true;
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
    pub then: CodeBlock<T, ()>,
    pub else_ifs: Vec<IfStmtElseIf<T>>,
    pub else_: Option<IfStmtElse<T>>,
    pub semicolon_token: Arc<Token>,
    pub resolved_terminal: Option<bool>,
}

impl<T: ResolvedType> Node<Stmt> for IfStmt<T> {
    fn node_id(&self) -> NodeId<Stmt> {
        return self.id;
    }
}

impl<T: ResolvedType> IsTerminal<T> for IfStmt<T> {
    fn is_terminal(&mut self) -> bool {
        if let Some(resolved) = &self.resolved_terminal {
            return *resolved;
        }

        // Remember, 'then' is illegal in IfStmt, so any terminals in self are not related to this

        let mut is_terminal = true;

        if let Some(stmt) = self.then.stmts.last() {
            if !stmt.try_lock().unwrap().is_terminal() {
                is_terminal = false;
            }
        }

        for else_if in &self.else_ifs {
            if let Some(stmt) = else_if.then.stmts.last() {
                if !stmt.try_lock().unwrap().is_terminal() {
                    is_terminal = false;
                }
            } else {
                is_terminal = false;
            }
        }

        if let Some(else_) = &self.else_ {
            if let Some(stmt) = else_.then.stmts.last() {
                if !stmt.try_lock().unwrap().is_terminal() {
                    is_terminal = false;
                }
            } else {
                is_terminal = false;
            }
        } else {
            is_terminal = false;
        }

        self.resolved_terminal = Some(is_terminal);

        return is_terminal;
    }
}

impl<T: ResolvedType> From<IfStmt<()>> for IfStmt<Option<T>> {
    fn from(value: IfStmt<()>) -> Self {
        return Self {
            id: value.id,
            if_token: value.if_token,
            condition: from(value.condition),
            then: from(value.then),
            else_ifs: fe_from(value.else_ifs),
            else_: fe_from(value.else_),
            semicolon_token: value.semicolon_token,
            resolved_terminal: value.resolved_terminal,
        };
    }
}

impl<T: ResolvedType> Resolvable for IfStmt<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.condition.is_resolved() {
            dbg!("false");
            return false;
        }

        if !self.then.is_resolved() {
            dbg!("false");
            return false;
        }

        for else_if in &self.else_ifs {
            if !else_if.is_resolved() {
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
            then: try_from(value.then)?,
            else_ifs: fe_try_from(value.else_ifs)?,
            else_: fe_try_from(value.else_)?,
            semicolon_token: value.semicolon_token,
            resolved_terminal: value.resolved_terminal,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmtElseIf<T: ResolvedType = ()> {
    pub else_token: Arc<Token>,
    pub if_token: Arc<Token>,
    pub condition: NestedExpr<T>,
    pub then: CodeBlock<T, ()>,
}

impl<T: ResolvedType> From<IfStmtElseIf<()>> for IfStmtElseIf<Option<T>> {
    fn from(value: IfStmtElseIf<()>) -> Self {
        return Self {
            else_token: value.else_token,
            if_token: value.if_token,
            condition: from(value.condition),
            then: from(value.then),
        };
    }
}

impl<T: ResolvedType> Resolvable for IfStmtElseIf<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.condition.is_resolved() {
            dbg!("false");
            return false;
        }

        if !self.then.is_resolved() {
            dbg!("false");
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<IfStmtElseIf<Option<T>>> for IfStmtElseIf<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: IfStmtElseIf<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            else_token: value.else_token,
            if_token: value.if_token,
            condition: try_from(value.condition)?,
            then: try_from(value.then)?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmtElse<T: ResolvedType = ()> {
    pub else_token: Arc<Token>,
    pub then: CodeBlock<T, ()>,
}

impl<T: ResolvedType> From<IfStmtElse<()>> for IfStmtElse<Option<T>> {
    fn from(value: IfStmtElse<()>) -> Self {
        return Self {
            else_token: value.else_token,
            then: from(value.then),
        };
    }
}

impl<T: ResolvedType> Resolvable for IfStmtElse<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.then.is_resolved() {
            dbg!("false");
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<IfStmtElse<Option<T>>> for IfStmtElse<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: IfStmtElse<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            else_token: value.else_token,
            then: try_from(value.then)?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoopStmt<T: ResolvedType = ()> {
    pub id: NodeId<Stmt>,
    pub loop_token: Arc<Token>,
    pub label: Option<Arc<Token>>,
    pub block: CodeBlock<T>,
    pub resolved_terminal: Option<bool>,
}

impl<T: ResolvedType> Node<Stmt> for LoopStmt<T> {
    fn node_id(&self) -> NodeId<Stmt> {
        return self.id;
    }
}

impl<T: ResolvedType> IsTerminal<T> for LoopStmt<T> {
    fn is_terminal(&mut self) -> bool {
        if let Some(resolved) = &self.resolved_terminal {
            return *resolved;
        }

        let mut is_terminal = false;

        // TODO: check for infinite loop?
        // TODO: check for loop that is terminal without just breaking out of the loop?

        /* TODO: also account for weird situations like so:
        loop
            if some_condition()
                return
            ;

            ...
        ;

        ^ That loop is terminal because the only way out is returning
        */

        self.resolved_terminal = Some(is_terminal);

        return is_terminal;
    }
}

impl<T: ResolvedType> From<LoopStmt<()>> for LoopStmt<Option<T>> {
    fn from(value: LoopStmt<()>) -> Self {
        return Self {
            id: value.id,
            loop_token: value.loop_token,
            label: value.label,
            block: from(value.block),
            resolved_terminal: value.resolved_terminal,
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
            label: value.label,
            block: try_from(value.block)?,
            resolved_terminal: value.resolved_terminal,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStmt<T: ResolvedType = ()> {
    pub id: NodeId<Stmt>,
    pub while_token: Arc<Token>,
    pub label: Option<Arc<Token>>,
    pub condition: NestedExpr<T>,
    pub block: CodeBlock<T, ()>,
    pub else_: Option<WhileStmtElse<T>>,
    pub semicolon_token: Arc<Token>,
    pub resolved_terminal: Option<bool>,
}

impl<T: ResolvedType> Node<Stmt> for WhileStmt<T> {
    fn node_id(&self) -> NodeId<Stmt> {
        return self.id;
    }
}

impl<T: ResolvedType> IsTerminal<T> for WhileStmt<T> {
    fn is_terminal(&mut self) -> bool {
        if let Some(resolved) = &self.resolved_terminal {
            return *resolved;
        }

        let is_terminal = false;

        // TODO: check for infinite while loop?
        // TODO: check for while loop that is terminal without just breaking out of the while?

        // TODO: does else-case influence terminality? ie if block AND else-case are terminal

        self.resolved_terminal = Some(is_terminal);

        return is_terminal;
    }
}

impl<T: ResolvedType> From<WhileStmt<()>> for WhileStmt<Option<T>> {
    fn from(value: WhileStmt<()>) -> Self {
        return Self {
            id: value.id,
            while_token: value.while_token,
            label: value.label,
            condition: from(value.condition),
            block: from(value.block),
            else_: fe_from(value.else_),
            semicolon_token: value.semicolon_token,
            resolved_terminal: value.resolved_terminal,
        };
    }
}

impl<T: ResolvedType> Resolvable for WhileStmt<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.condition.is_resolved() {
            dbg!("false");
            return false;
        }

        if !self.block.is_resolved() {
            dbg!("false");
            return false;
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

impl<T: ResolvedType> TryFrom<WhileStmt<Option<T>>> for WhileStmt<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: WhileStmt<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            while_token: value.while_token,
            label: value.label,
            condition: try_from(value.condition)?,
            block: try_from(value.block)?,
            else_: fe_try_from(value.else_)?,
            semicolon_token: value.semicolon_token,
            resolved_terminal: value.resolved_terminal,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStmtElse<T: ResolvedType> {
    pub else_token: Arc<Token>,
    pub block: CodeBlock<T, ()>,
}

impl<T: ResolvedType> From<WhileStmtElse<()>> for WhileStmtElse<Option<T>> {
    fn from(value: WhileStmtElse<()>) -> Self {
        return Self {
            else_token: value.else_token,
            block: from(value.block),
        };
    }
}

impl<T: ResolvedType> Resolvable for WhileStmtElse<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.block.is_resolved() {
            dbg!("false");
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<WhileStmtElse<Option<T>>> for WhileStmtElse<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: WhileStmtElse<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            else_token: value.else_token,
            block: try_from(value.block)?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BreakStmt<T: ResolvedType = ()> {
    pub id: NodeId<Stmt>,
    pub break_token: Arc<Token>,
    pub label: Option<Arc<Token>>,
    pub value: Option<NestedExpr<T>>,
    pub resolved_type: Option<T>,
    pub handler: Option<BreakHandler>,
}

impl<T: ResolvedType> Node<Stmt> for BreakStmt<T> {
    fn node_id(&self) -> NodeId<Stmt> {
        return self.id;
    }
}

impl<T: ResolvedType> IsTerminal<T> for BreakStmt<T> {
    fn is_terminal(&mut self) -> bool {
        return true;
    }
}

impl<T: ResolvedType> From<BreakStmt<()>> for BreakStmt<Option<T>> {
    fn from(value: BreakStmt<()>) -> Self {
        return Self {
            id: value.id,
            break_token: value.break_token,
            label: value.label,
            value: value.value.map(from),
            resolved_type: value.resolved_type.map(|_| None),
            handler: value.handler,
        };
    }
}

impl<T: ResolvedType> Resolvable for BreakStmt<Option<T>> {
    fn is_resolved(&self) -> bool {
        if let Some(value) = &self.value {
            if !value.is_resolved() {
                dbg!("false");
                return false;
            }
        }

        if let Some(res) = &self.resolved_type {
            if !res.is_some() {
                dbg!("false");
                return false;
            }
        }

        if !self.handler.is_some() {
            dbg!("false");
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<BreakStmt<Option<T>>> for BreakStmt<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: BreakStmt<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            break_token: value.break_token,
            label: value.label,
            value: invert(value.value.map(try_from))?,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
            handler: value.handler,
        });
    }
}

#[derive(Debug, Clone)]
pub enum BreakHandler {
    LoopStmt(Arc<Mutex<LoopStmt<Option<FeType>>>>),
    LoopExpr(Arc<Mutex<LoopExpr<Option<FeType>>>>),
    WhileStmt(Arc<Mutex<WhileStmt<Option<FeType>>>>),
    WhileExpr(Arc<Mutex<WhileExpr<Option<FeType>>>>),
}

impl PartialEq for BreakHandler {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::LoopStmt(v) => {
                let cloned = { v.try_lock().unwrap().clone() };
                let Self::LoopStmt(other) = other else {
                    return false;
                };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::LoopExpr(v) => {
                let cloned = { v.try_lock().unwrap().clone() };
                let Self::LoopExpr(other) = other else {
                    return false;
                };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::WhileStmt(v) => {
                let cloned = { v.try_lock().unwrap().clone() };
                let Self::WhileStmt(other) = other else {
                    return false;
                };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::WhileExpr(v) => {
                let cloned = { v.try_lock().unwrap().clone() };
                let Self::WhileExpr(other) = other else {
                    return false;
                };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThenStmt<T: ResolvedType = ()> {
    pub id: NodeId<Stmt>,
    pub then_token: Arc<Token>,
    pub label: Option<Arc<Token>>,
    pub value: NestedExpr<T>,
    pub resolved_type: T,
    pub handler: Option<ThenHandler>,
}

impl<T: ResolvedType> Node<Stmt> for ThenStmt<T> {
    fn node_id(&self) -> NodeId<Stmt> {
        return self.id;
    }
}

impl<T: ResolvedType> IsTerminal<T> for ThenStmt<T> {
    fn is_terminal(&mut self) -> bool {
        return true;
    }
}

impl<T: ResolvedType> From<ThenStmt<()>> for ThenStmt<Option<T>> {
    fn from(value: ThenStmt<()>) -> Self {
        return Self {
            id: value.id,
            then_token: value.then_token,
            label: value.label,
            value: from(value.value),
            resolved_type: None,
            handler: value.handler,
        };
    }
}

impl<T: ResolvedType> Resolvable for ThenStmt<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.value.is_resolved() {
            dbg!("false");
            return false;
        }

        if !self.resolved_type.is_some() {
            dbg!("false");
            return false;
        }

        if !self.handler.is_some() {
            dbg!("false");
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<ThenStmt<Option<T>>> for ThenStmt<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: ThenStmt<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            then_token: value.then_token,
            label: value.label,
            value: try_from(value.value)?,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
            handler: value.handler,
        });
    }
}

#[derive(Debug, Clone)]
pub enum ThenHandler {
    IfExpr(IfBlock, Arc<Mutex<IfExpr<Option<FeType>>>>),
    IfStmt(IfBlock, Arc<Mutex<IfStmt<Option<FeType>>>>),
}

impl PartialEq for ThenHandler {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::IfExpr(v1, v2) => {
                let cloned = { v2.try_lock().unwrap().clone() };
                let Self::IfExpr(other1, other2) = other else {
                    return false;
                };
                return v1 == other1 && PartialEq::eq(&cloned, &other2.try_lock().unwrap());
            }
            Self::IfStmt(v1, v2) => {
                let cloned = { v2.try_lock().unwrap().clone() };
                let Self::IfStmt(other1, other2) = other else {
                    return false;
                };
                return v1 == other1 && PartialEq::eq(&cloned, &other2.try_lock().unwrap());
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfBlock {
    Then,
    ElseIf(usize),
    Else,
}

// Visitor pattern
pub trait StmtVisitor<T: ResolvedType, R = ()> {
    fn visit_expr_stmt(&mut self, stmt: Arc<Mutex<ExprStmt<T>>>) -> R;
    fn visit_var_decl_stmt(&mut self, stmt: Arc<Mutex<VarDeclStmt<T>>>) -> R;
    fn visit_assign_stmt(&mut self, stmt: Arc<Mutex<AssignStmt<T>>>) -> R;
    fn visit_return_stmt(&mut self, stmt: Arc<Mutex<ReturnStmt<T>>>) -> R;
    fn visit_if_stmt(&mut self, stmt: Arc<Mutex<IfStmt<T>>>) -> R;
    fn visit_loop_stmt(&mut self, stmt: Arc<Mutex<LoopStmt<T>>>) -> R;
    fn visit_while_stmt(&mut self, stmt: Arc<Mutex<WhileStmt<T>>>) -> R;
    fn visit_break_stmt(&mut self, stmt: Arc<Mutex<BreakStmt<T>>>) -> R;
    fn visit_then_stmt(&mut self, stmt: Arc<Mutex<ThenStmt<T>>>) -> R;
}

pub trait StmtAccept<T: ResolvedType, R, V: StmtVisitor<T, R>> {
    fn accept(&self, visitor: &mut V) -> R;
}

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for Stmt<T> {
    fn accept(&self, visitor: &mut V) -> R {
        return match self {
            Self::Expr(stmt) => stmt.accept(visitor),
            Self::VarDecl(stmt) => stmt.accept(visitor),
            Self::Assign(stmt) => stmt.accept(visitor),
            Self::Return(stmt) => stmt.accept(visitor),
            Self::If(stmt) => stmt.accept(visitor),
            Self::Loop(stmt) => stmt.accept(visitor),
            Self::While(stmt) => stmt.accept(visitor),
            Self::Break(stmt) => stmt.accept(visitor),
            Self::Then(stmt) => stmt.accept(visitor),
        };
    }
}

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for Arc<Mutex<ExprStmt<T>>> {
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_expr_stmt(self.clone());
    }
}

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for Arc<Mutex<VarDeclStmt<T>>> {
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_var_decl_stmt(self.clone());
    }
}

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for Arc<Mutex<AssignStmt<T>>> {
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_assign_stmt(self.clone());
    }
}

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for Arc<Mutex<ReturnStmt<T>>> {
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_return_stmt(self.clone());
    }
}

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for Arc<Mutex<IfStmt<T>>> {
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_if_stmt(self.clone());
    }
}

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for Arc<Mutex<LoopStmt<T>>> {
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_loop_stmt(self.clone());
    }
}

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for Arc<Mutex<WhileStmt<T>>> {
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_while_stmt(self.clone());
    }
}

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for Arc<Mutex<BreakStmt<T>>> {
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_break_stmt(self.clone());
    }
}

impl<T: ResolvedType, R, V: StmtVisitor<T, R>> StmtAccept<T, R, V> for Arc<Mutex<ThenStmt<T>>> {
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_then_stmt(self.clone());
    }
}
