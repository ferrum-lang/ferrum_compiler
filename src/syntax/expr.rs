use super::*;

use crate::result::Result;
use crate::token::Token;
use crate::utils::{fe_from, fe_try_from, from, try_from};

#[derive(Debug, Clone)]
pub enum Expr<T: ResolvedType = ()> {
    BoolLiteral(Arc<Mutex<BoolLiteralExpr<T>>>),
    NumberLiteral(Arc<Mutex<NumberLiteralExpr<T>>>),
    PlainStringLiteral(Arc<Mutex<PlainStringLiteralExpr<T>>>),
    FmtStringLiteral(Arc<Mutex<FmtStringLiteralExpr<T>>>),
    Ident(Arc<Mutex<IdentExpr<T>>>),
    Call(Arc<Mutex<CallExpr<T>>>),
    Unary(Arc<Mutex<UnaryExpr<T>>>),
    Binary(Arc<Mutex<BinaryExpr<T>>>),
    StaticRef(Arc<Mutex<StaticRefExpr<T>>>),
    Construct(Arc<Mutex<ConstructExpr<T>>>),
    Get(Arc<Mutex<GetExpr<T>>>),
    If(Arc<Mutex<IfExpr<T>>>),
    Loop(Arc<Mutex<LoopExpr<T>>>),
    While(Arc<Mutex<WhileExpr<T>>>),
}

impl<T: ResolvedType> PartialEq for Expr<T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::BoolLiteral(d) => {
                let Self::BoolLiteral(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::NumberLiteral(d) => {
                let Self::NumberLiteral(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::PlainStringLiteral(d) => {
                let Self::PlainStringLiteral(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::FmtStringLiteral(d) => {
                let Self::FmtStringLiteral(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::Ident(d) => {
                let Self::Ident(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::Call(d) => {
                let Self::Call(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::Unary(d) => {
                let Self::Unary(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::Binary(d) => {
                let Self::Binary(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::StaticRef(d) => {
                let Self::StaticRef(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::Construct(d) => {
                let Self::Construct(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::Get(d) => {
                let Self::Get(other) = other else {
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
            Self::While(d) => {
                let Self::While(other) = other else {
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
        }
    }
}

impl<T: ResolvedType> Expr<T> {
    pub fn resolved_type(&self) -> Option<T> {
        match self {
            Self::BoolLiteral(v) => return Some(v.try_lock().unwrap().resolved_type.clone()),
            Self::NumberLiteral(v) => return Some(v.try_lock().unwrap().resolved_type.clone()),
            Self::PlainStringLiteral(v) => {
                return Some(v.try_lock().unwrap().resolved_type.clone())
            }
            Self::FmtStringLiteral(v) => return Some(v.try_lock().unwrap().resolved_type.clone()),
            Self::Ident(v) => return Some(v.try_lock().unwrap().resolved_type.clone()),
            Self::Call(v) => return v.try_lock().unwrap().resolved_type.clone(),
            Self::Unary(v) => return Some(v.try_lock().unwrap().resolved_type.clone()),
            Self::Binary(v) => return Some(v.try_lock().unwrap().resolved_type.clone()),
            Self::StaticRef(v) => return Some(v.try_lock().unwrap().resolved_type.clone()),
            Self::Construct(v) => return Some(v.try_lock().unwrap().resolved_type.clone()),
            Self::Get(v) => return Some(v.try_lock().unwrap().resolved_type.clone()),
            Self::If(v) => return v.try_lock().unwrap().resolved_type.clone(),
            Self::Loop(v) => return v.try_lock().unwrap().resolved_type.clone(),
            Self::While(v) => return v.try_lock().unwrap().resolved_type.clone(),
        }
    }
}

impl<T: ResolvedType> Node<Expr> for Expr<T> {
    fn node_id(&self) -> NodeId<Expr> {
        match self {
            Self::BoolLiteral(expr) => return expr.try_lock().unwrap().node_id(),
            Self::NumberLiteral(expr) => return expr.try_lock().unwrap().node_id(),
            Self::PlainStringLiteral(expr) => return expr.try_lock().unwrap().node_id(),
            Self::FmtStringLiteral(expr) => return expr.try_lock().unwrap().node_id(),
            Self::Ident(expr) => return expr.try_lock().unwrap().node_id(),
            Self::Call(expr) => return expr.try_lock().unwrap().node_id(),
            Self::Unary(expr) => return expr.try_lock().unwrap().node_id(),
            Self::Binary(expr) => return expr.try_lock().unwrap().node_id(),
            Self::StaticRef(expr) => return expr.try_lock().unwrap().node_id(),
            Self::Construct(expr) => return expr.try_lock().unwrap().node_id(),
            Self::Get(expr) => return expr.try_lock().unwrap().node_id(),
            Self::If(expr) => return expr.try_lock().unwrap().node_id(),
            Self::Loop(expr) => return expr.try_lock().unwrap().node_id(),
            Self::While(expr) => return expr.try_lock().unwrap().node_id(),
        }
    }
}

impl<T: ResolvedType> From<Expr<()>> for Expr<Option<T>> {
    fn from(value: Expr<()>) -> Self {
        match value {
            Expr::BoolLiteral(expr) => return Self::BoolLiteral(fe_from(expr)),
            Expr::NumberLiteral(expr) => return Self::NumberLiteral(fe_from(expr)),
            Expr::PlainStringLiteral(expr) => return Self::PlainStringLiteral(fe_from(expr)),
            Expr::FmtStringLiteral(expr) => return Self::FmtStringLiteral(fe_from(expr)),
            Expr::Ident(expr) => return Self::Ident(fe_from(expr)),
            Expr::Call(expr) => return Self::Call(fe_from(expr)),
            Expr::Unary(expr) => return Self::Unary(fe_from(expr)),
            Expr::Binary(expr) => return Self::Binary(fe_from(expr)),
            Expr::StaticRef(expr) => return Self::StaticRef(fe_from(expr)),
            Expr::Construct(expr) => return Self::Construct(fe_from(expr)),
            Expr::Get(expr) => return Self::Get(fe_from(expr)),
            Expr::If(expr) => return Self::If(fe_from(expr)),
            Expr::Loop(expr) => return Self::Loop(fe_from(expr)),
            Expr::While(expr) => return Self::While(fe_from(expr)),
        }
    }
}

impl<T: ResolvedType> Resolvable for Expr<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            Expr::BoolLiteral(expr) => return expr.try_lock().unwrap().is_resolved(),
            Expr::NumberLiteral(expr) => return expr.try_lock().unwrap().is_resolved(),
            Expr::PlainStringLiteral(expr) => return expr.try_lock().unwrap().is_resolved(),
            Expr::FmtStringLiteral(expr) => return expr.try_lock().unwrap().is_resolved(),
            Expr::Ident(expr) => return expr.try_lock().unwrap().is_resolved(),
            Expr::Call(expr) => return expr.try_lock().unwrap().is_resolved(),
            Expr::Unary(expr) => return expr.try_lock().unwrap().is_resolved(),
            Expr::Binary(expr) => return expr.try_lock().unwrap().is_resolved(),
            Expr::StaticRef(expr) => return expr.try_lock().unwrap().is_resolved(),
            Expr::Construct(expr) => return expr.try_lock().unwrap().is_resolved(),
            Expr::Get(expr) => return expr.try_lock().unwrap().is_resolved(),
            Expr::If(expr) => return expr.try_lock().unwrap().is_resolved(),
            Expr::Loop(expr) => return expr.try_lock().unwrap().is_resolved(),
            Expr::While(expr) => return expr.try_lock().unwrap().is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<Expr<Option<T>>> for Expr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: Expr<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            Expr::BoolLiteral(expr) => return Ok(Self::BoolLiteral(fe_try_from(expr)?)),
            Expr::NumberLiteral(expr) => return Ok(Self::NumberLiteral(fe_try_from(expr)?)),
            Expr::PlainStringLiteral(expr) => {
                return Ok(Self::PlainStringLiteral(fe_try_from(expr)?))
            }
            Expr::FmtStringLiteral(expr) => return Ok(Self::FmtStringLiteral(fe_try_from(expr)?)),
            Expr::Ident(expr) => return Ok(Self::Ident(fe_try_from(expr)?)),
            Expr::Call(expr) => return Ok(Self::Call(fe_try_from(expr)?)),
            Expr::Unary(expr) => return Ok(Self::Unary(fe_try_from(expr)?)),
            Expr::Binary(expr) => return Ok(Self::Binary(fe_try_from(expr)?)),
            Expr::StaticRef(expr) => return Ok(Self::StaticRef(fe_try_from(expr)?)),
            Expr::Construct(expr) => return Ok(Self::Construct(fe_try_from(expr)?)),
            Expr::Get(expr) => return Ok(Self::Get(fe_try_from(expr)?)),
            Expr::If(expr) => return Ok(Self::If(fe_try_from(expr)?)),
            Expr::Loop(expr) => return Ok(Self::Loop(fe_try_from(expr)?)),
            Expr::While(expr) => return Ok(Self::While(fe_try_from(expr)?)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NestedExpr<T: ResolvedType = ()>(pub Arc<Mutex<Expr<T>>>);
impl<T: ResolvedType> PartialEq for NestedExpr<T> {
    fn eq(&self, other: &Self) -> bool {
        let cloned = {
            let locked = self.0.try_lock().unwrap();
            locked.clone()
        };

        let other = other.0.try_lock().unwrap();

        return cloned == *other;
    }
}

impl<T: ResolvedType> From<NestedExpr<()>> for NestedExpr<Option<T>> {
    fn from(value: NestedExpr<()>) -> Self {
        return Self(fe_from(value.0));
    }
}

impl<T: ResolvedType> Resolvable for NestedExpr<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.0.try_lock().unwrap().is_resolved();
    }
}

impl<T: ResolvedType> TryFrom<NestedExpr<Option<T>>> for NestedExpr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: NestedExpr<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self(fe_try_from(value.0)?));
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoolLiteralExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub literal: Arc<Token>,
    pub resolved_type: T,
}

impl<T: ResolvedType> Node<Expr> for BoolLiteralExpr<T> {
    fn node_id(&self) -> NodeId<Expr> {
        return self.id;
    }
}

impl<T: ResolvedType> From<BoolLiteralExpr<()>> for BoolLiteralExpr<Option<T>> {
    fn from(value: BoolLiteralExpr<()>) -> Self {
        return Self {
            id: value.id,
            literal: value.literal,
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for BoolLiteralExpr<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<BoolLiteralExpr<Option<T>>> for BoolLiteralExpr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: BoolLiteralExpr<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            literal: value.literal,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteralExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub literal: Arc<Token>,
    pub details: NumberLiteralDetails,
    pub resolved_type: T,
}

impl<T: ResolvedType> Node<Expr> for NumberLiteralExpr<T> {
    fn node_id(&self) -> NodeId<Expr> {
        return self.id;
    }
}

impl<T: ResolvedType> From<NumberLiteralExpr<()>> for NumberLiteralExpr<Option<T>> {
    fn from(value: NumberLiteralExpr<()>) -> Self {
        return Self {
            id: value.id,
            literal: value.literal,
            details: value.details,
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for NumberLiteralExpr<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<NumberLiteralExpr<Option<T>>> for NumberLiteralExpr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: NumberLiteralExpr<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            literal: value.literal,
            details: value.details,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumberLiteralDetails {
    // TODO: bignums
    Integer(u64),
    Decimal(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlainStringLiteralExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub literal: Arc<Token>,
    pub resolved_type: T,
}

impl<T: ResolvedType> Node<Expr> for PlainStringLiteralExpr<T> {
    fn node_id(&self) -> NodeId<Expr> {
        return self.id;
    }
}

impl<T: ResolvedType> From<PlainStringLiteralExpr<()>> for PlainStringLiteralExpr<Option<T>> {
    fn from(value: PlainStringLiteralExpr<()>) -> Self {
        return Self {
            id: value.id,
            literal: value.literal,
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for PlainStringLiteralExpr<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<PlainStringLiteralExpr<Option<T>>> for PlainStringLiteralExpr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: PlainStringLiteralExpr<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            literal: value.literal,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FmtStringLiteralExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub first: Arc<Token>,
    pub rest: Vec<FmtStringPart<T>>,
    pub resolved_type: T,
}

impl<T: ResolvedType> Node<Expr> for FmtStringLiteralExpr<T> {
    fn node_id(&self) -> NodeId<Expr> {
        return self.id;
    }
}

impl<T: ResolvedType> From<FmtStringLiteralExpr<()>> for FmtStringLiteralExpr<Option<T>> {
    fn from(value: FmtStringLiteralExpr<()>) -> Self {
        return Self {
            id: value.id,
            first: value.first,
            rest: value.rest.into_iter().map(from).collect(),
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for FmtStringLiteralExpr<Option<T>> {
    fn is_resolved(&self) -> bool {
        for part in &self.rest {
            if !part.expr.is_resolved() {
                return false;
            }
        }

        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<FmtStringLiteralExpr<Option<T>>> for FmtStringLiteralExpr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: FmtStringLiteralExpr<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            first: value.first,
            rest: value
                .rest
                .into_iter()
                .map(|part| {
                    Ok(FmtStringPart {
                        expr: try_from(part.expr)?,
                        string: part.string,
                    })
                })
                .collect::<Result<Vec<FmtStringPart<T>>, Self::Error>>()?,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FmtStringPart<T: ResolvedType = ()> {
    pub expr: NestedExpr<T>,
    pub string: Arc<str>,
}

impl<T: ResolvedType> From<FmtStringPart<()>> for FmtStringPart<Option<T>> {
    fn from(value: FmtStringPart<()>) -> Self {
        return Self {
            expr: from(value.expr),
            string: value.string,
        };
    }
}

impl<T: ResolvedType> Resolvable for FmtStringPart<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.expr.is_resolved();
    }
}

impl<T: ResolvedType> TryFrom<FmtStringPart<Option<T>>> for FmtStringPart<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: FmtStringPart<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            expr: try_from(value.expr)?,
            string: value.string,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IdentExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub ident: Arc<Token>,
    pub resolved_type: T,
}

impl<T: ResolvedType> Node<Expr> for IdentExpr<T> {
    fn node_id(&self) -> NodeId<Expr> {
        return self.id;
    }
}

impl<T: ResolvedType> From<IdentExpr<()>> for IdentExpr<Option<T>> {
    fn from(value: IdentExpr<()>) -> Self {
        return Self {
            id: value.id,
            ident: value.ident,
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for IdentExpr<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<IdentExpr<Option<T>>> for IdentExpr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: IdentExpr<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            ident: value.ident,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub callee: NestedExpr<T>,
    pub open_paren_token: Arc<Token>,
    pub pre_comma_token: Option<Arc<Token>>,
    pub args: Vec<CallArg<T>>,
    pub close_paren_token: Arc<Token>,
    pub resolved_type: Option<T>,
}

impl<T: ResolvedType> Node<Expr> for CallExpr<T> {
    fn node_id(&self) -> NodeId<Expr> {
        return self.id;
    }
}

impl<T: ResolvedType> From<CallExpr<()>> for CallExpr<Option<T>> {
    fn from(value: CallExpr<()>) -> Self {
        return Self {
            id: value.id,
            callee: from(value.callee),
            open_paren_token: value.open_paren_token,
            pre_comma_token: value.pre_comma_token,
            args: value.args.into_iter().map(from).collect(),
            close_paren_token: value.close_paren_token,
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for CallExpr<Option<T>> {
    fn is_resolved(&self) -> bool {
        if let Some(resolved_type) = &self.resolved_type {
            if resolved_type.is_none() {
                dbg!("false");
                return false;
            }
        }

        if !self.callee.is_resolved() {
            dbg!("false");
            return false;
        }

        for arg in &self.args {
            if !arg.is_resolved() {
                dbg!("false");
                return false;
            }
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<CallExpr<Option<T>>> for CallExpr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: CallExpr<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            callee: try_from(value.callee)?,
            open_paren_token: value.open_paren_token,
            pre_comma_token: value.pre_comma_token,
            args: value
                .args
                .into_iter()
                .map(try_from)
                .collect::<Result<Vec<CallArg<T>>, Self::Error>>()?,
            close_paren_token: value.close_paren_token,
            resolved_type: if let Some(resolved_type) = value.resolved_type {
                Some(resolved_type.ok_or(FinalizeResolveTypeError {
                    file: file!(),
                    line: line!(),
                })?)
            } else {
                None
            },
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArg<T: ResolvedType = ()> {
    pub param_name: Option<CallArgParamName>,
    pub value: NestedExpr<T>,
    pub post_comma_token: Option<Arc<Token>>,
    pub resolved_type: T,
}

impl<T: ResolvedType> From<CallArg<()>> for CallArg<Option<T>> {
    fn from(value: CallArg<()>) -> Self {
        return Self {
            param_name: value.param_name,
            value: from(value.value),
            post_comma_token: value.post_comma_token,
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for CallArg<Option<T>> {
    fn is_resolved(&self) -> bool {
        if self.resolved_type.is_none() {
            dbg!("false");
            return false;
        }

        if !self.value.is_resolved() {
            dbg!("false");
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<CallArg<Option<T>>> for CallArg<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: CallArg<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            param_name: value.param_name,
            value: try_from(value.value)?,
            post_comma_token: value.post_comma_token,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArgParamName {
    pub name: Token,
    pub eq_token: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub op: UnaryOp,
    pub value: NestedExpr<T>,
    pub resolved_type: T,
}

impl<T: ResolvedType> Node<Expr> for UnaryExpr<T> {
    fn node_id(&self) -> NodeId<Expr> {
        return self.id;
    }
}

impl<T: ResolvedType> From<UnaryExpr<()>> for UnaryExpr<Option<T>> {
    fn from(value: UnaryExpr<()>) -> Self {
        return Self {
            id: value.id,
            op: value.op,
            value: from(value.value),
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for UnaryExpr<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.value.is_resolved() {
            return false;
        }

        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<UnaryExpr<Option<T>>> for UnaryExpr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: UnaryExpr<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            op: value.op,
            value: try_from(value.value)?,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Ref(RefType),
    Not(Arc<Token>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub lhs: NestedExpr<T>,
    pub op: BinaryOp,
    pub rhs: NestedExpr<T>,
    pub resolved_type: T,
}

impl<T: ResolvedType> Node<Expr> for BinaryExpr<T> {
    fn node_id(&self) -> NodeId<Expr> {
        return self.id;
    }
}

impl<T: ResolvedType> From<BinaryExpr<()>> for BinaryExpr<Option<T>> {
    fn from(value: BinaryExpr<()>) -> Self {
        return Self {
            id: value.id,
            lhs: from(value.lhs),
            op: value.op,
            rhs: from(value.rhs),
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for BinaryExpr<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.lhs.is_resolved() {
            return false;
        }

        if !self.rhs.is_resolved() {
            return false;
        }

        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<BinaryExpr<Option<T>>> for BinaryExpr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: BinaryExpr<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            lhs: try_from(value.lhs)?,
            op: value.op,
            rhs: try_from(value.rhs)?,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add(Arc<Token>),
    Less(Arc<Token>),
    LessEq(Arc<Token>),
    Greater(Arc<Token>),
    GreaterEq(Arc<Token>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StaticRefExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub static_path: StaticPath<T>,
    pub resolved_type: T,
}

impl<T: ResolvedType> Node<Expr> for StaticRefExpr<T> {
    fn node_id(&self) -> NodeId<Expr> {
        return self.id;
    }
}

impl<T: ResolvedType> From<StaticRefExpr<()>> for StaticRefExpr<Option<T>> {
    fn from(value: StaticRefExpr<()>) -> Self {
        return Self {
            id: value.id,
            static_path: from(value.static_path),
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for StaticRefExpr<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.static_path.is_resolved() {
            return false;
        }

        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<StaticRefExpr<Option<T>>> for StaticRefExpr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: StaticRefExpr<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            static_path: try_from(value.static_path)?,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone)]
pub enum ConstructTarget<T: ResolvedType = ()> {
    Ident(Arc<Mutex<IdentExpr<T>>>),
    StaticPath(StaticPath<T>),
}

impl<T: ResolvedType> PartialEq for ConstructTarget<T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Ident(d) => {
                let Self::Ident(other) = other else {
                    return false;
                };
                let cloned = { d.try_lock().unwrap().clone() };
                return PartialEq::eq(&cloned, &other.try_lock().unwrap());
            }
            Self::StaticPath(t) => {
                let Self::StaticPath(other) = other else {
                    return false;
                };
                return PartialEq::eq(t, other);
            }
        }
    }
}

impl<T: ResolvedType> From<ConstructTarget<()>> for ConstructTarget<Option<T>> {
    fn from(value: ConstructTarget<()>) -> Self {
        match value {
            ConstructTarget::Ident(target) => return Self::Ident(fe_from(target)),
            ConstructTarget::StaticPath(target) => return Self::StaticPath(from(target)),
        }
    }
}

impl<T: ResolvedType> Resolvable for ConstructTarget<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            ConstructTarget::Ident(target) => return target.try_lock().unwrap().is_resolved(),
            ConstructTarget::StaticPath(target) => return target.is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<ConstructTarget<Option<T>>> for ConstructTarget<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: ConstructTarget<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            ConstructTarget::Ident(target) => return Ok(Self::Ident(fe_try_from(target)?)),
            ConstructTarget::StaticPath(target) => return Ok(Self::StaticPath(try_from(target)?)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstructExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub target: ConstructTarget<T>,
    pub open_squirly_brace: Arc<Token>,
    pub args: Vec<ConstructArg<T>>,
    pub close_squirly_brace: Arc<Token>,
    pub resolved_type: T,
}

impl<T: ResolvedType> Node<Expr> for ConstructExpr<T> {
    fn node_id(&self) -> NodeId<Expr> {
        return self.id;
    }
}

impl<T: ResolvedType> From<ConstructExpr<()>> for ConstructExpr<Option<T>> {
    fn from(value: ConstructExpr<()>) -> Self {
        return Self {
            id: value.id,
            target: from(value.target),
            open_squirly_brace: value.open_squirly_brace,
            args: value.args.into_iter().map(from).collect(),
            close_squirly_brace: value.close_squirly_brace,
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for ConstructExpr<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.target.is_resolved() {
            dbg!("false");
            return false;
        }

        for arg in &self.args {
            if !arg.is_resolved() {
                dbg!("false");
                return false;
            }
        }

        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<ConstructExpr<Option<T>>> for ConstructExpr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: ConstructExpr<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            target: try_from(value.target)?,
            open_squirly_brace: value.open_squirly_brace,
            args: value
                .args
                .into_iter()
                .map(try_from)
                .collect::<Result<Vec<ConstructArg<T>>, Self::Error>>()?,
            close_squirly_brace: value.close_squirly_brace,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstructArg<T: ResolvedType = ()> {
    Field(ConstructField<T>),
}

impl<T: ResolvedType> From<ConstructArg<()>> for ConstructArg<Option<T>> {
    fn from(value: ConstructArg<()>) -> Self {
        match value {
            ConstructArg::Field(arg) => return Self::Field(from(arg)),
        }
    }
}

impl<T: ResolvedType> Resolvable for ConstructArg<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            ConstructArg::Field(arg) => return arg.is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<ConstructArg<Option<T>>> for ConstructArg<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: ConstructArg<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            ConstructArg::Field(arg) => return Ok(Self::Field(try_from(arg)?)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstructField<T: ResolvedType = ()> {
    pub name: Arc<Token>,
    pub colon_token: Arc<Token>,
    pub value: NestedExpr<T>,
    pub comma_token: Option<Arc<Token>>,
}

impl<T: ResolvedType> From<ConstructField<()>> for ConstructField<Option<T>> {
    fn from(value: ConstructField<()>) -> Self {
        return Self {
            name: value.name,
            colon_token: value.colon_token,
            value: from(value.value),
            comma_token: value.comma_token,
        };
    }
}

impl<T: ResolvedType> Resolvable for ConstructField<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.value.0.try_lock().unwrap().is_resolved() {
            dbg!("false");
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<ConstructField<Option<T>>> for ConstructField<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: ConstructField<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            name: value.name,
            colon_token: value.colon_token,
            value: try_from(value.value)?,
            comma_token: value.comma_token,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub target: NestedExpr<T>,
    pub dot_token: Arc<Token>,
    pub name: Arc<Token>,
    pub resolved_type: T,
}

impl<T: ResolvedType> Node<Expr> for GetExpr<T> {
    fn node_id(&self) -> NodeId<Expr> {
        return self.id;
    }
}

impl<T: ResolvedType> From<GetExpr<()>> for GetExpr<Option<T>> {
    fn from(value: GetExpr<()>) -> Self {
        return Self {
            id: value.id,
            target: from(value.target),
            dot_token: value.dot_token,
            name: value.name,
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for GetExpr<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.target.is_resolved() {
            dbg!("false");
            return false;
        }

        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<GetExpr<Option<T>>> for GetExpr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: GetExpr<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            target: try_from(value.target)?,
            dot_token: value.dot_token,
            name: value.name,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub if_token: Arc<Token>,
    pub label: Option<Arc<Token>>,
    pub condition: NestedExpr<T>,
    pub then: IfExprThen<T>,
    pub else_ifs: Vec<IfExprElseIf<T>>,
    pub else_: Option<IfExprElse<T>>,
    pub semicolon_token: Option<Arc<Token>>,
    pub resolved_terminal: Option<bool>,
    pub resolved_type: Option<T>,
}

impl<T: ResolvedType> Node<Expr> for IfExpr<T> {
    fn node_id(&self) -> NodeId<Expr> {
        return self.id;
    }
}

impl<T: ResolvedType> IsTerminal<T> for IfExpr<T> {
    fn is_terminal(&mut self) -> bool {
        if let Some(resolved) = &self.resolved_terminal {
            return *resolved;
        }

        let mut is_terminal = false;

        /* TODO: account for then stmts:

        const val = if some_condition()
            if other_condition()
                then "foo"
            ;

            return
        ;

        ^ That if is NOT terminal because the `then` allows returning from the if
        */

        self.resolved_terminal = Some(is_terminal);

        return is_terminal;
    }
}

impl<T: ResolvedType> From<IfExpr<()>> for IfExpr<Option<T>> {
    fn from(value: IfExpr<()>) -> Self {
        return Self {
            id: value.id,
            if_token: value.if_token,
            label: value.label,
            condition: from(value.condition),
            then: from(value.then),
            else_ifs: fe_from(value.else_ifs),
            else_: fe_from(value.else_),
            semicolon_token: value.semicolon_token,
            resolved_terminal: value.resolved_terminal,
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for IfExpr<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.condition.is_resolved() {
            dbg!("false");
            return false;
        }

        if !self.then.is_resolved() {
            dbg!("false", &self);
            return false;
        }

        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<IfExpr<Option<T>>> for IfExpr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: IfExpr<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            if_token: value.if_token,
            label: value.label,
            condition: try_from(value.condition)?,
            then: try_from(value.then)?,
            else_ifs: fe_try_from(value.else_ifs)?,
            else_: fe_try_from(value.else_)?,
            semicolon_token: value.semicolon_token,
            resolved_terminal: value.resolved_terminal,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfExprThen<T: ResolvedType = ()> {
    Ternary(IfExprThenTernary<T>),
    Block(IfExprThenBlock<T>),
}

impl<T: ResolvedType> From<IfExprThen<()>> for IfExprThen<Option<T>> {
    fn from(value: IfExprThen<()>) -> Self {
        match value {
            IfExprThen::Ternary(value) => Self::Ternary(from(value)),
            IfExprThen::Block(value) => Self::Block(from(value)),
        }
    }
}

impl<T: ResolvedType> Resolvable for IfExprThen<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            IfExprThen::Ternary(value) => return value.is_resolved(),
            IfExprThen::Block(value) => return value.is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<IfExprThen<Option<T>>> for IfExprThen<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: IfExprThen<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            IfExprThen::Ternary(value) => return Ok(Self::Ternary(try_from(value)?)),
            IfExprThen::Block(value) => return Ok(Self::Block(try_from(value)?)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExprThenTernary<T: ResolvedType = ()> {
    pub then_token: Arc<Token>,
    pub then_expr: NestedExpr<T>,
}

impl<T: ResolvedType> From<IfExprThenTernary<()>> for IfExprThenTernary<Option<T>> {
    fn from(value: IfExprThenTernary<()>) -> Self {
        return Self {
            then_token: value.then_token,
            then_expr: from(value.then_expr),
        };
    }
}

impl<T: ResolvedType> Resolvable for IfExprThenTernary<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.then_expr.is_resolved() {
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<IfExprThenTernary<Option<T>>> for IfExprThenTernary<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: IfExprThenTernary<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            then_token: value.then_token,
            then_expr: try_from(value.then_expr)?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExprThenBlock<T: ResolvedType = ()> {
    pub block: CodeBlock<T, ()>,
}

impl<T: ResolvedType> From<IfExprThenBlock<()>> for IfExprThenBlock<Option<T>> {
    fn from(value: IfExprThenBlock<()>) -> Self {
        return Self {
            block: from(value.block),
        };
    }
}

impl<T: ResolvedType> Resolvable for IfExprThenBlock<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.block.is_resolved() {
            dbg!("false");
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<IfExprThenBlock<Option<T>>> for IfExprThenBlock<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: IfExprThenBlock<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            block: try_from(value.block)?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfExprElseIf<T: ResolvedType = ()> {
    Ternary(IfExprElseIfTernary<T>),
    Block(IfExprElseIfBlock<T>),
}

impl<T: ResolvedType> From<IfExprElseIf<()>> for IfExprElseIf<Option<T>> {
    fn from(value: IfExprElseIf<()>) -> Self {
        match value {
            IfExprElseIf::Ternary(value) => Self::Ternary(from(value)),
            IfExprElseIf::Block(value) => Self::Block(from(value)),
        }
    }
}

impl<T: ResolvedType> Resolvable for IfExprElseIf<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            IfExprElseIf::Ternary(value) => return value.is_resolved(),
            IfExprElseIf::Block(value) => return value.is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<IfExprElseIf<Option<T>>> for IfExprElseIf<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: IfExprElseIf<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            IfExprElseIf::Ternary(value) => return Ok(Self::Ternary(try_from(value)?)),
            IfExprElseIf::Block(value) => return Ok(Self::Block(try_from(value)?)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExprElseIfTernary<T: ResolvedType> {
    pub else_token: Arc<Token>,
    pub if_token: Arc<Token>,
    pub condition: NestedExpr<T>,
    pub then_token: Arc<Token>,
    pub expr: NestedExpr<T>,
}

impl<T: ResolvedType> From<IfExprElseIfTernary<()>> for IfExprElseIfTernary<Option<T>> {
    fn from(value: IfExprElseIfTernary<()>) -> Self {
        return Self {
            else_token: value.else_token,
            if_token: value.if_token,
            condition: from(value.condition),
            then_token: value.then_token,
            expr: from(value.expr),
        };
    }
}

impl<T: ResolvedType> Resolvable for IfExprElseIfTernary<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.condition.is_resolved() {
            dbg!("false");
            return false;
        }

        if !self.expr.is_resolved() {
            dbg!("false");
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<IfExprElseIfTernary<Option<T>>> for IfExprElseIfTernary<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: IfExprElseIfTernary<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            else_token: value.else_token,
            if_token: value.if_token,
            condition: try_from(value.condition)?,
            then_token: value.then_token,
            expr: try_from(value.expr)?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExprElseIfBlock<T: ResolvedType> {
    pub else_token: Arc<Token>,
    pub if_token: Arc<Token>,
    pub label: Option<Arc<Token>>,
    pub condition: NestedExpr<T>,
    pub block: CodeBlock<T, ()>,
}

impl<T: ResolvedType> From<IfExprElseIfBlock<()>> for IfExprElseIfBlock<Option<T>> {
    fn from(value: IfExprElseIfBlock<()>) -> Self {
        return Self {
            else_token: value.else_token,
            if_token: value.if_token,
            label: value.label,
            condition: from(value.condition),
            block: from(value.block),
        };
    }
}

impl<T: ResolvedType> Resolvable for IfExprElseIfBlock<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.condition.is_resolved() {
            dbg!("false");
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<IfExprElseIfBlock<Option<T>>> for IfExprElseIfBlock<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: IfExprElseIfBlock<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            else_token: value.else_token,
            if_token: value.if_token,
            label: value.label,
            condition: try_from(value.condition)?,
            block: try_from(value.block)?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfExprElse<T: ResolvedType = ()> {
    Ternary(IfExprElseTernary<T>),
    Block(IfExprElseBlock<T>),
}

impl<T: ResolvedType> From<IfExprElse<()>> for IfExprElse<Option<T>> {
    fn from(value: IfExprElse<()>) -> Self {
        match value {
            IfExprElse::Ternary(value) => Self::Ternary(from(value)),
            IfExprElse::Block(value) => Self::Block(from(value)),
        }
    }
}

impl<T: ResolvedType> Resolvable for IfExprElse<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            IfExprElse::Ternary(value) => return value.is_resolved(),
            IfExprElse::Block(value) => return value.is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<IfExprElse<Option<T>>> for IfExprElse<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: IfExprElse<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            IfExprElse::Ternary(value) => return Ok(Self::Ternary(try_from(value)?)),
            IfExprElse::Block(value) => return Ok(Self::Block(try_from(value)?)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExprElseTernary<T: ResolvedType> {
    pub else_token: Arc<Token>,
    pub else_expr: NestedExpr<T>,
}

impl<T: ResolvedType> From<IfExprElseTernary<()>> for IfExprElseTernary<Option<T>> {
    fn from(value: IfExprElseTernary<()>) -> Self {
        return Self {
            else_token: value.else_token,
            else_expr: from(value.else_expr),
        };
    }
}

impl<T: ResolvedType> Resolvable for IfExprElseTernary<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.else_expr.is_resolved() {
            dbg!("false");
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<IfExprElseTernary<Option<T>>> for IfExprElseTernary<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: IfExprElseTernary<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            else_token: value.else_token,
            else_expr: try_from(value.else_expr)?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExprElseBlock<T: ResolvedType> {
    pub else_token: Arc<Token>,
    pub label: Option<Arc<Token>>,
    pub block: CodeBlock<T, ()>,
}

impl<T: ResolvedType> From<IfExprElseBlock<()>> for IfExprElseBlock<Option<T>> {
    fn from(value: IfExprElseBlock<()>) -> Self {
        return Self {
            else_token: value.else_token,
            label: value.label,
            block: from(value.block),
        };
    }
}

impl<T: ResolvedType> Resolvable for IfExprElseBlock<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.block.is_resolved() {
            dbg!("false");
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<IfExprElseBlock<Option<T>>> for IfExprElseBlock<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: IfExprElseBlock<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            else_token: value.else_token,
            label: value.label,
            block: try_from(value.block)?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoopExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub loop_token: Arc<Token>,
    pub label: Option<Arc<Token>>,
    pub block: CodeBlock<T>,
    pub resolved_terminal: Option<bool>,
    pub resolved_type: Option<T>,
}

impl<T: ResolvedType> Node<Expr> for LoopExpr<T> {
    fn node_id(&self) -> NodeId<Expr> {
        return self.id;
    }
}

impl<T: ResolvedType> IsTerminal<T> for LoopExpr<T> {
    fn is_terminal(&mut self) -> bool {
        if let Some(resolved) = &self.resolved_terminal {
            return *resolved;
        }

        let mut is_terminal = false;

        // TODO: same issues as loop expr

        self.resolved_terminal = Some(is_terminal);

        return is_terminal;
    }
}

impl<T: ResolvedType> From<LoopExpr<()>> for LoopExpr<Option<T>> {
    fn from(value: LoopExpr<()>) -> Self {
        return Self {
            id: value.id,
            loop_token: value.loop_token,
            label: value.label,
            block: from(value.block),
            resolved_terminal: value.resolved_terminal,
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for LoopExpr<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.block.is_resolved() {
            dbg!("false");
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<LoopExpr<Option<T>>> for LoopExpr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: LoopExpr<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            loop_token: value.loop_token,
            label: value.label,
            block: try_from(value.block)?,
            resolved_terminal: value.resolved_terminal,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub while_token: Arc<Token>,
    pub label: Option<Arc<Token>>,
    pub condition: NestedExpr<T>,
    pub block: CodeBlock<T, ()>,
    pub then: Option<WhileExprThen<T>>,
    pub else_: Option<WhileExprElse<T>>,
    pub semicolon_token: Option<Arc<Token>>,
    pub resolved_terminal: Option<bool>,
    pub resolved_type: Option<T>,
}

impl<T: ResolvedType> Node<Expr> for WhileExpr<T> {
    fn node_id(&self) -> NodeId<Expr> {
        return self.id;
    }
}

impl<T: ResolvedType> IsTerminal<T> for WhileExpr<T> {
    fn is_terminal(&mut self) -> bool {
        if let Some(resolved) = &self.resolved_terminal {
            return *resolved;
        }

        let mut is_terminal = false;

        // TODO: same issues as while expr

        self.resolved_terminal = Some(is_terminal);

        return is_terminal;
    }
}

impl<T: ResolvedType> From<WhileExpr<()>> for WhileExpr<Option<T>> {
    fn from(value: WhileExpr<()>) -> Self {
        return Self {
            id: value.id,
            while_token: value.while_token,
            label: value.label,
            condition: from(value.condition),
            block: from(value.block),
            then: fe_from(value.then),
            else_: fe_from(value.else_),
            semicolon_token: value.semicolon_token,
            resolved_terminal: value.resolved_terminal,
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for WhileExpr<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.condition.is_resolved() {
            dbg!("false");
            return false;
        }

        if !self.block.is_resolved() {
            dbg!("false");
            return false;
        }

        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<WhileExpr<Option<T>>> for WhileExpr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: WhileExpr<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            while_token: value.while_token,
            label: value.label,
            condition: try_from(value.condition)?,
            block: try_from(value.block)?,
            then: fe_try_from(value.then)?,
            else_: fe_try_from(value.else_)?,
            semicolon_token: value.semicolon_token,
            resolved_terminal: value.resolved_terminal,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WhileExprThen<T: ResolvedType = ()> {
    Ternary(WhileExprThenTernary<T>),
    Block(WhileExprThenBlock<T>),
}

impl<T: ResolvedType> From<WhileExprThen<()>> for WhileExprThen<Option<T>> {
    fn from(value: WhileExprThen<()>) -> Self {
        match value {
            WhileExprThen::Ternary(value) => Self::Ternary(from(value)),
            WhileExprThen::Block(value) => Self::Block(from(value)),
        }
    }
}

impl<T: ResolvedType> Resolvable for WhileExprThen<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            WhileExprThen::Ternary(value) => return value.is_resolved(),
            WhileExprThen::Block(value) => return value.is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<WhileExprThen<Option<T>>> for WhileExprThen<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: WhileExprThen<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            WhileExprThen::Ternary(value) => return Ok(Self::Ternary(try_from(value)?)),
            WhileExprThen::Block(value) => return Ok(Self::Block(try_from(value)?)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileExprThenTernary<T: ResolvedType = ()> {
    pub then_token: Arc<Token>,
    pub then_expr: NestedExpr<T>,
}

impl<T: ResolvedType> From<WhileExprThenTernary<()>> for WhileExprThenTernary<Option<T>> {
    fn from(value: WhileExprThenTernary<()>) -> Self {
        return Self {
            then_token: value.then_token,
            then_expr: from(value.then_expr),
        };
    }
}

impl<T: ResolvedType> Resolvable for WhileExprThenTernary<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.then_expr.is_resolved() {
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<WhileExprThenTernary<Option<T>>> for WhileExprThenTernary<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: WhileExprThenTernary<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            then_token: value.then_token,
            then_expr: try_from(value.then_expr)?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileExprThenBlock<T: ResolvedType = ()> {
    pub then_token: Arc<Token>,
    pub block: CodeBlock<T, ()>,
}

impl<T: ResolvedType> From<WhileExprThenBlock<()>> for WhileExprThenBlock<Option<T>> {
    fn from(value: WhileExprThenBlock<()>) -> Self {
        return Self {
            then_token: value.then_token,
            block: from(value.block),
        };
    }
}

impl<T: ResolvedType> Resolvable for WhileExprThenBlock<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.block.is_resolved() {
            dbg!("false");
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<WhileExprThenBlock<Option<T>>> for WhileExprThenBlock<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: WhileExprThenBlock<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            then_token: value.then_token,
            block: try_from(value.block)?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WhileExprElse<T: ResolvedType = ()> {
    Ternary(WhileExprElseTernary<T>),
    Block(WhileExprElseBlock<T>),
}

impl<T: ResolvedType> From<WhileExprElse<()>> for WhileExprElse<Option<T>> {
    fn from(value: WhileExprElse<()>) -> Self {
        match value {
            WhileExprElse::Ternary(value) => Self::Ternary(from(value)),
            WhileExprElse::Block(value) => Self::Block(from(value)),
        }
    }
}

impl<T: ResolvedType> Resolvable for WhileExprElse<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            WhileExprElse::Ternary(value) => return value.is_resolved(),
            WhileExprElse::Block(value) => return value.is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<WhileExprElse<Option<T>>> for WhileExprElse<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: WhileExprElse<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            WhileExprElse::Ternary(value) => return Ok(Self::Ternary(try_from(value)?)),
            WhileExprElse::Block(value) => return Ok(Self::Block(try_from(value)?)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileExprElseTernary<T: ResolvedType> {
    pub else_token: Arc<Token>,
    pub else_expr: NestedExpr<T>,
}

impl<T: ResolvedType> From<WhileExprElseTernary<()>> for WhileExprElseTernary<Option<T>> {
    fn from(value: WhileExprElseTernary<()>) -> Self {
        return Self {
            else_token: value.else_token,
            else_expr: from(value.else_expr),
        };
    }
}

impl<T: ResolvedType> Resolvable for WhileExprElseTernary<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.else_expr.is_resolved() {
            dbg!("false");
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<WhileExprElseTernary<Option<T>>> for WhileExprElseTernary<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: WhileExprElseTernary<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            else_token: value.else_token,
            else_expr: try_from(value.else_expr)?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileExprElseBlock<T: ResolvedType> {
    pub else_token: Arc<Token>,
    pub block: CodeBlock<T, ()>,
}

impl<T: ResolvedType> From<WhileExprElseBlock<()>> for WhileExprElseBlock<Option<T>> {
    fn from(value: WhileExprElseBlock<()>) -> Self {
        return Self {
            else_token: value.else_token,
            block: from(value.block),
        };
    }
}

impl<T: ResolvedType> Resolvable for WhileExprElseBlock<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.block.is_resolved() {
            dbg!("false");
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<WhileExprElseBlock<Option<T>>> for WhileExprElseBlock<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: WhileExprElseBlock<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            else_token: value.else_token,
            block: try_from(value.block)?,
        });
    }
}

// Visitor pattern
pub trait ExprVisitor<T: ResolvedType, R = ()> {
    fn visit_bool_literal_expr(&mut self, expr: Arc<Mutex<BoolLiteralExpr<T>>>) -> R;
    fn visit_number_literal_expr(&mut self, expr: Arc<Mutex<NumberLiteralExpr<T>>>) -> R;
    fn visit_plain_string_literal_expr(&mut self, expr: Arc<Mutex<PlainStringLiteralExpr<T>>>)
        -> R;
    fn visit_fmt_string_literal_expr(&mut self, expr: Arc<Mutex<FmtStringLiteralExpr<T>>>) -> R;
    fn visit_ident_expr(&mut self, expr: Arc<Mutex<IdentExpr<T>>>) -> R;
    fn visit_call_expr(&mut self, expr: Arc<Mutex<CallExpr<T>>>) -> R;
    fn visit_unary_expr(&mut self, expr: Arc<Mutex<UnaryExpr<T>>>) -> R;
    fn visit_binary_expr(&mut self, expr: Arc<Mutex<BinaryExpr<T>>>) -> R;
    fn visit_static_ref_expr(&mut self, expr: Arc<Mutex<StaticRefExpr<T>>>) -> R;
    fn visit_construct_expr(&mut self, expr: Arc<Mutex<ConstructExpr<T>>>) -> R;
    fn visit_get_expr(&mut self, expr: Arc<Mutex<GetExpr<T>>>) -> R;
    fn visit_if_expr(&mut self, expr: Arc<Mutex<IfExpr<T>>>) -> R;
    fn visit_loop_expr(&mut self, expr: Arc<Mutex<LoopExpr<T>>>) -> R;
    fn visit_while_expr(&mut self, expr: Arc<Mutex<WhileExpr<T>>>) -> R;
}

pub trait ExprAccept<T: ResolvedType, R, V: ExprVisitor<T, R>> {
    fn accept(&self, visitor: &mut V) -> R;
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for Expr<T> {
    fn accept(&self, visitor: &mut V) -> R {
        return match self {
            Self::BoolLiteral(expr) => expr.accept(visitor),
            Self::NumberLiteral(expr) => expr.accept(visitor),
            Self::PlainStringLiteral(expr) => expr.accept(visitor),
            Self::FmtStringLiteral(expr) => expr.accept(visitor),
            Self::Ident(expr) => expr.accept(visitor),
            Self::Call(expr) => expr.accept(visitor),
            Self::Unary(expr) => expr.accept(visitor),
            Self::Binary(expr) => expr.accept(visitor),
            Self::StaticRef(expr) => expr.accept(visitor),
            Self::Construct(expr) => expr.accept(visitor),
            Self::Get(expr) => expr.accept(visitor),
            Self::If(expr) => expr.accept(visitor),
            Self::Loop(expr) => expr.accept(visitor),
            Self::While(expr) => expr.accept(visitor),
        };
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V>
    for Arc<Mutex<BoolLiteralExpr<T>>>
{
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_bool_literal_expr(self.clone());
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V>
    for Arc<Mutex<NumberLiteralExpr<T>>>
{
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_number_literal_expr(self.clone());
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V>
    for Arc<Mutex<PlainStringLiteralExpr<T>>>
{
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_plain_string_literal_expr(self.clone());
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V>
    for Arc<Mutex<FmtStringLiteralExpr<T>>>
{
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_fmt_string_literal_expr(self.clone());
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for Arc<Mutex<IdentExpr<T>>> {
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_ident_expr(self.clone());
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for Arc<Mutex<CallExpr<T>>> {
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_call_expr(self.clone());
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for Arc<Mutex<UnaryExpr<T>>> {
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_unary_expr(self.clone());
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for Arc<Mutex<BinaryExpr<T>>> {
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_binary_expr(self.clone());
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V>
    for Arc<Mutex<StaticRefExpr<T>>>
{
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_static_ref_expr(self.clone());
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V>
    for Arc<Mutex<ConstructExpr<T>>>
{
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_construct_expr(self.clone());
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for Arc<Mutex<GetExpr<T>>> {
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_get_expr(self.clone());
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for Arc<Mutex<IfExpr<T>>> {
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_if_expr(self.clone());
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for Arc<Mutex<LoopExpr<T>>> {
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_loop_expr(self.clone());
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for Arc<Mutex<WhileExpr<T>>> {
    fn accept(&self, visitor: &mut V) -> R {
        return visitor.visit_while_expr(self.clone());
    }
}
