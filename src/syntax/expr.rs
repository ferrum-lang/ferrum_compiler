use super::*;

use crate::result::Result;
use crate::token::Token;
use crate::utils::{fe_from, fe_try_from, from, invert, try_from};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<T: ResolvedType = ()> {
    BoolLiteral(BoolLiteralExpr<T>),
    NumberLiteral(NumberLiteralExpr<T>),
    PlainStringLiteral(PlainStringLiteralExpr<T>),
    FmtStringLiteral(FmtStringLiteralExpr<T>),
    Ident(IdentExpr<T>),
    Call(CallExpr<T>),
    Unary(UnaryExpr<T>),
    Binary(BinaryExpr<T>),
    StaticRef(StaticRefExpr<T>),
    Construct(ConstructExpr<T>),
    Get(GetExpr<T>),
    If(IfExpr<T>),
    Loop(LoopExpr<T>),
    While(WhileExpr<T>),
}

impl<T: ResolvedType> Expr<T> {
    pub fn resolved_type(&self) -> Option<&T> {
        match self {
            Self::BoolLiteral(v) => return Some(&v.resolved_type),
            Self::NumberLiteral(v) => return Some(&v.resolved_type),
            Self::PlainStringLiteral(v) => return Some(&v.resolved_type),
            Self::FmtStringLiteral(v) => return Some(&v.resolved_type),
            Self::Ident(v) => return Some(&v.resolved_type),
            Self::Call(v) => return v.resolved_type.as_ref(),
            Self::Unary(v) => return Some(&v.resolved_type),
            Self::Binary(v) => return Some(&v.resolved_type),
            Self::StaticRef(v) => return Some(&v.resolved_type),
            Self::Construct(v) => return Some(&v.resolved_type),
            Self::Get(v) => return Some(&v.resolved_type),
            Self::If(v) => return v.resolved_type.as_ref(),
            Self::Loop(v) => return v.resolved_type.as_ref(),
            Self::While(v) => return v.resolved_type.as_ref(),
        }
    }
}

impl<T: ResolvedType> Node<Expr> for Expr<T> {
    fn node_id(&self) -> &NodeId<Expr> {
        match self {
            Self::BoolLiteral(expr) => return expr.node_id(),
            Self::NumberLiteral(expr) => return expr.node_id(),
            Self::PlainStringLiteral(expr) => return expr.node_id(),
            Self::FmtStringLiteral(expr) => return expr.node_id(),
            Self::Ident(expr) => return expr.node_id(),
            Self::Call(expr) => return expr.node_id(),
            Self::Unary(expr) => return expr.node_id(),
            Self::Binary(expr) => return expr.node_id(),
            Self::StaticRef(expr) => return expr.node_id(),
            Self::Construct(expr) => return expr.node_id(),
            Self::Get(expr) => return expr.node_id(),
            Self::If(expr) => return expr.node_id(),
            Self::Loop(expr) => return expr.node_id(),
            Self::While(expr) => return expr.node_id(),
        }
    }
}

impl<T: ResolvedType> From<Expr<()>> for Expr<Option<T>> {
    fn from(value: Expr<()>) -> Self {
        match value {
            Expr::BoolLiteral(expr) => return Self::BoolLiteral(from(expr)),
            Expr::NumberLiteral(expr) => return Self::NumberLiteral(from(expr)),
            Expr::PlainStringLiteral(expr) => return Self::PlainStringLiteral(from(expr)),
            Expr::FmtStringLiteral(expr) => return Self::FmtStringLiteral(from(expr)),
            Expr::Ident(expr) => return Self::Ident(from(expr)),
            Expr::Call(expr) => return Self::Call(from(expr)),
            Expr::Unary(expr) => return Self::Unary(from(expr)),
            Expr::Binary(expr) => return Self::Binary(from(expr)),
            Expr::StaticRef(expr) => return Self::StaticRef(from(expr)),
            Expr::Construct(expr) => return Self::Construct(from(expr)),
            Expr::Get(expr) => return Self::Get(from(expr)),
            Expr::If(expr) => return Self::If(from(expr)),
            Expr::Loop(expr) => return Self::Loop(from(expr)),
            Expr::While(expr) => return Self::While(from(expr)),
        }
    }
}

impl<T: ResolvedType> Resolvable for Expr<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            Expr::BoolLiteral(expr) => return expr.is_resolved(),
            Expr::NumberLiteral(expr) => return expr.is_resolved(),
            Expr::PlainStringLiteral(expr) => return expr.is_resolved(),
            Expr::FmtStringLiteral(expr) => return expr.is_resolved(),
            Expr::Ident(expr) => return expr.is_resolved(),
            Expr::Call(expr) => return expr.is_resolved(),
            Expr::Unary(expr) => return expr.is_resolved(),
            Expr::Binary(expr) => return expr.is_resolved(),
            Expr::StaticRef(expr) => return expr.is_resolved(),
            Expr::Construct(expr) => return expr.is_resolved(),
            Expr::Get(expr) => return expr.is_resolved(),
            Expr::If(expr) => return expr.is_resolved(),
            Expr::Loop(expr) => return expr.is_resolved(),
            Expr::While(expr) => return expr.is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<Expr<Option<T>>> for Expr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: Expr<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            Expr::BoolLiteral(expr) => return Ok(Self::BoolLiteral(try_from(expr)?)),
            Expr::NumberLiteral(expr) => return Ok(Self::NumberLiteral(try_from(expr)?)),
            Expr::PlainStringLiteral(expr) => return Ok(Self::PlainStringLiteral(try_from(expr)?)),
            Expr::FmtStringLiteral(expr) => return Ok(Self::FmtStringLiteral(try_from(expr)?)),
            Expr::Ident(expr) => return Ok(Self::Ident(try_from(expr)?)),
            Expr::Call(expr) => return Ok(Self::Call(try_from(expr)?)),
            Expr::Unary(expr) => return Ok(Self::Unary(try_from(expr)?)),
            Expr::Binary(expr) => return Ok(Self::Binary(try_from(expr)?)),
            Expr::StaticRef(expr) => return Ok(Self::StaticRef(try_from(expr)?)),
            Expr::Construct(expr) => return Ok(Self::Construct(try_from(expr)?)),
            Expr::Get(expr) => return Ok(Self::Get(try_from(expr)?)),
            Expr::If(expr) => return Ok(Self::If(try_from(expr)?)),
            Expr::Loop(expr) => return Ok(Self::Loop(try_from(expr)?)),
            Expr::While(expr) => return Ok(Self::While(try_from(expr)?)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NestedExpr<T: ResolvedType = ()>(pub Arc<Mutex<Expr<T>>>);
impl<T: ResolvedType> PartialEq for NestedExpr<T> {
    fn eq(&self, other: &Self) -> bool {
        let cloned = {
            let locked = self.0.lock().unwrap();
            locked.clone()
        };

        let other = other.0.lock().unwrap();

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
        return self.0.lock().unwrap().is_resolved();
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
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
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
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
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
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
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
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
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
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
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
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
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
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
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
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
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
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
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

#[derive(Debug, Clone, PartialEq)]
pub enum ConstructTarget<T: ResolvedType = ()> {
    Ident(IdentExpr<T>),
    StaticPath(StaticPath<T>),
}

impl<T: ResolvedType> From<ConstructTarget<()>> for ConstructTarget<Option<T>> {
    fn from(value: ConstructTarget<()>) -> Self {
        match value {
            ConstructTarget::Ident(target) => return Self::Ident(from(target)),
            ConstructTarget::StaticPath(target) => return Self::StaticPath(from(target)),
        }
    }
}

impl<T: ResolvedType> Resolvable for ConstructTarget<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            ConstructTarget::Ident(target) => return target.is_resolved(),
            ConstructTarget::StaticPath(target) => return target.is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<ConstructTarget<Option<T>>> for ConstructTarget<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: ConstructTarget<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            ConstructTarget::Ident(target) => return Ok(Self::Ident(try_from(target)?)),
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
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
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
        if !self.value.0.lock().unwrap().is_resolved() {
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
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
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
    pub condition: NestedExpr<T>,
    pub then: IfThen<T>,
    pub resolved_terminal: Option<Option<TerminationType<T>>>,
    pub resolved_type: Option<T>,
}

impl<T: ResolvedType> Node<Expr> for IfExpr<T> {
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
    }
}

impl<T: ResolvedType> IsTerminal<T> for IfExpr<T> {
    fn is_terminal(&mut self) -> Option<TerminationType<T>> {
        if let Some(resolved) = &self.resolved_terminal {
            return resolved.clone();
        }

        match &mut self.then {
            IfThen::Ternary(then) => todo!(),

            IfThen::Classic(then) => {
                let mut contains = Vec::new();

                let mut then_term = None;
                for stmt in &then.then_block.stmts {
                    if let Some(term) = stmt.lock().unwrap().is_terminal() {
                        match term.clone() {
                            TerminationType::Contains(terms) => contains.extend(terms),
                            TerminationType::Base(other) => {
                                then_term = Some(other);
                                break;
                            }
                        }
                    }
                }

                for elseif in &then.else_ifs {
                    let mut elseif_term = None;

                    for stmt in &elseif.then_block.stmts {
                        if let Some(term) = stmt.lock().unwrap().is_terminal() {
                            match term.clone() {
                                TerminationType::Contains(terms) => contains.extend(terms),
                                TerminationType::Base(other) => {
                                    elseif_term = Some(other);
                                    break;
                                }
                            }
                        }
                    }

                    match (&then_term, &elseif_term) {
                        (None, None) => {}

                        (None, Some(term)) => {
                            contains.push(term.clone());
                        }

                        (Some(term), None) => {
                            contains.push(term.clone());
                            then_term = None;
                        }

                        (Some(BaseTerminationType::Then(res)), Some(_any))
                        | (Some(_any), Some(BaseTerminationType::Then(res))) => {
                            then_term = Some(BaseTerminationType::Then(res.clone()));
                        }

                        (Some(BaseTerminationType::Break(res)), Some(_any))
                        | (Some(_any), Some(BaseTerminationType::Break(res))) => {
                            then_term = Some(BaseTerminationType::Break(res.clone()));
                        }

                        (Some(BaseTerminationType::Return), Some(_any))
                        | (Some(_any), Some(BaseTerminationType::Return)) => {
                            then_term = Some(BaseTerminationType::Return);
                        }

                        (
                            Some(BaseTerminationType::InfiniteLoop),
                            Some(BaseTerminationType::InfiniteLoop),
                        ) => {}
                    }
                }

                if let Some(else_) = &then.else_ {
                    let mut else_term = None;

                    for stmt in &else_.then_block.stmts {
                        if let Some(term) = stmt.lock().unwrap().is_terminal() {
                            match term.clone() {
                                TerminationType::Contains(terms) => contains.extend(terms),
                                TerminationType::Base(other) => {
                                    else_term = Some(other);
                                    break;
                                }
                            }
                        }
                    }

                    match (&then_term, &else_term) {
                        (None, None) => {}

                        (None, Some(term)) => {
                            contains.push(term.clone());
                        }

                        (Some(term), None) => {
                            contains.push(term.clone());
                            then_term = None;
                        }

                        (Some(BaseTerminationType::Then(res)), Some(_any))
                        | (Some(_any), Some(BaseTerminationType::Then(res))) => {
                            then_term = Some(BaseTerminationType::Then(res.clone()));
                        }

                        (Some(BaseTerminationType::Break(res)), Some(_any))
                        | (Some(_any), Some(BaseTerminationType::Break(res))) => {
                            then_term = Some(BaseTerminationType::Break(res.clone()));
                        }

                        (Some(BaseTerminationType::Return), Some(_any))
                        | (Some(_any), Some(BaseTerminationType::Return)) => {
                            then_term = Some(BaseTerminationType::Return);
                        }

                        (
                            Some(BaseTerminationType::InfiniteLoop),
                            Some(BaseTerminationType::InfiniteLoop),
                        ) => {}
                    }
                }

                if contains.is_empty() {
                    if then.else_ifs.is_empty() && then.else_.is_none() {
                        if let Some(term) = &then_term {
                            contains.push(term.clone());
                            then_term = None;
                            self.resolved_terminal =
                                Some(Some(TerminationType::Contains(contains)));
                        }
                    } else {
                        self.resolved_terminal = Some(then_term.map(TerminationType::Base));
                    }
                } else {
                    self.resolved_terminal = Some(Some(TerminationType::Contains(contains)));
                }
            }
        }

        return self.resolved_terminal.as_ref().unwrap().clone();
    }
}

impl<T: ResolvedType> From<IfExpr<()>> for IfExpr<Option<T>> {
    fn from(value: IfExpr<()>) -> Self {
        return Self {
            id: value.id,
            if_token: value.if_token,
            condition: from(value.condition),
            then: from(value.then),
            resolved_terminal: value.resolved_terminal.map(|v| v.map(from)),
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
        let resolved_terminal = if let Some(term) = value.resolved_terminal {
            Some(if let Some(term) = term {
                Some(try_from(term)?)
            } else {
                None
            })
        } else {
            None
        };

        return Ok(Self {
            id: value.id,
            if_token: value.if_token,
            condition: try_from(value.condition)?,
            then: try_from(value.then)?,
            resolved_terminal,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfThen<T: ResolvedType = ()> {
    Ternary(IfThenTernary<T>),
    Classic(IfThenClassic<T>),
}

impl<T: ResolvedType> From<IfThen<()>> for IfThen<Option<T>> {
    fn from(value: IfThen<()>) -> Self {
        match value {
            IfThen::Ternary(value) => Self::Ternary(from(value)),
            IfThen::Classic(value) => Self::Classic(from(value)),
        }
    }
}

impl<T: ResolvedType> Resolvable for IfThen<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            IfThen::Ternary(value) => return value.is_resolved(),
            IfThen::Classic(value) => return value.is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<IfThen<Option<T>>> for IfThen<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: IfThen<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            IfThen::Ternary(value) => return Ok(Self::Ternary(try_from(value)?)),
            IfThen::Classic(value) => return Ok(Self::Classic(try_from(value)?)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfThenTernary<T: ResolvedType = ()> {
    pub then_token: Arc<Token>,
    pub then_expr: NestedExpr<T>,
    pub else_: Option<TernaryElse<T>>,
}

impl<T: ResolvedType> From<IfThenTernary<()>> for IfThenTernary<Option<T>> {
    fn from(value: IfThenTernary<()>) -> Self {
        return Self {
            then_token: value.then_token,
            then_expr: from(value.then_expr),
            else_: value.else_.map(from),
        };
    }
}

impl<T: ResolvedType> Resolvable for IfThenTernary<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.then_expr.is_resolved() {
            return false;
        }

        if let Some(else_) = &self.else_ {
            if !else_.is_resolved() {
                return false;
            }
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<IfThenTernary<Option<T>>> for IfThenTernary<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: IfThenTernary<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            then_token: value.then_token,
            then_expr: try_from(value.then_expr)?,
            else_: invert(value.else_.map(try_from))?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TernaryElse<T: ResolvedType = ()> {
    pub else_token: Arc<Token>,
    pub else_expr: NestedExpr<T>,
}

impl<T: ResolvedType> From<TernaryElse<()>> for TernaryElse<Option<T>> {
    fn from(value: TernaryElse<()>) -> Self {
        return Self {
            else_token: value.else_token,
            else_expr: from(value.else_expr),
        };
    }
}

impl<T: ResolvedType> Resolvable for TernaryElse<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.else_expr.is_resolved() {
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<TernaryElse<Option<T>>> for TernaryElse<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: TernaryElse<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            else_token: value.else_token,
            else_expr: try_from(value.else_expr)?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfThenClassic<T: ResolvedType = ()> {
    pub then_block: CodeBlock<T, ()>,
    pub else_ifs: Vec<ElseIfBranch<T>>,
    pub else_: Option<ElseBranch<T>>,
    pub semicolon_token: Arc<Token>,
}

impl<T: ResolvedType> From<IfThenClassic<()>> for IfThenClassic<Option<T>> {
    fn from(value: IfThenClassic<()>) -> Self {
        return Self {
            then_block: from(value.then_block),
            else_ifs: value.else_ifs.into_iter().map(from).collect(),
            else_: value.else_.map(from),
            semicolon_token: value.semicolon_token,
        };
    }
}

impl<T: ResolvedType> Resolvable for IfThenClassic<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.then_block.is_resolved() {
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

impl<T: ResolvedType> TryFrom<IfThenClassic<Option<T>>> for IfThenClassic<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: IfThenClassic<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
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
pub struct LoopExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub loop_token: Arc<Token>,
    pub block: CodeBlock<T>,
    pub resolved_terminal: Option<Option<TerminationType<T>>>,
    pub resolved_type: Option<T>,
}

impl<T: ResolvedType> Node<Expr> for LoopExpr<T> {
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
    }
}

impl<T: ResolvedType> IsTerminal<T> for LoopExpr<T> {
    fn is_terminal(&mut self) -> Option<TerminationType<T>> {
        if let Some(term) = &self.resolved_terminal {
            return term.clone();
        }

        let mut contains = Vec::new();
        let mut term = None;
        for stmt in &self.block.stmts {
            match stmt.lock().unwrap().is_terminal() {
                None => {}

                Some(TerminationType::Base(other)) => {
                    term = Some(other);
                    break;
                }

                Some(TerminationType::Contains(terms)) => {
                    contains.extend(terms);
                }
            }
        }

        if !contains.is_empty() {
            if contains.len() == 1 && contains.contains(&BaseTerminationType::Return) {
                self.resolved_terminal =
                    Some(Some(TerminationType::Base(BaseTerminationType::Return)));
            } else {
                self.resolved_terminal = Some(Some(TerminationType::Contains(contains)));
            }
        } else {
            match term {
                Some(BaseTerminationType::Break(res)) => {
                    contains.push(BaseTerminationType::Break(res));
                    self.resolved_terminal = Some(Some(TerminationType::Contains(contains)));
                }

                Some(term) => {
                    self.resolved_terminal = Some(Some(TerminationType::Base(term)));
                }

                None => {
                    self.resolved_terminal = Some(Some(TerminationType::Base(
                        BaseTerminationType::InfiniteLoop,
                    )));
                }
            }
        }

        return self.resolved_terminal.as_ref().unwrap().clone();
    }
}

impl<T: ResolvedType> From<LoopExpr<()>> for LoopExpr<Option<T>> {
    fn from(value: LoopExpr<()>) -> Self {
        let resolved_terminal = if let Some(term) = value.resolved_terminal {
            Some(if let Some(term) = term {
                Some(from(term))
            } else {
                None
            })
        } else {
            None
        };

        return Self {
            id: value.id,
            loop_token: value.loop_token,
            block: from(value.block),
            resolved_terminal,
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
        let resolved_terminal = if let Some(term) = value.resolved_terminal {
            Some(if let Some(term) = term {
                Some(try_from(term)?)
            } else {
                None
            })
        } else {
            None
        };

        return Ok(Self {
            id: value.id,
            loop_token: value.loop_token,
            block: try_from(value.block)?,
            resolved_terminal,
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
    pub condition: NestedExpr<T>,
    pub block: CodeBlock<T>,
    pub resolved_terminal: Option<Option<TerminationType<T>>>,
    pub resolved_type: Option<T>,
}

impl<T: ResolvedType> Node<Expr> for WhileExpr<T> {
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
    }
}

impl<T: ResolvedType> IsTerminal<T> for WhileExpr<T> {
    fn is_terminal(&mut self) -> Option<TerminationType<T>> {
        if let Some(term) = &self.resolved_terminal {
            return term.clone();
        }

        let mut contains = Vec::new();
        let mut term = None;
        for stmt in &self.block.stmts {
            match stmt.lock().unwrap().is_terminal() {
                None => {}

                Some(TerminationType::Base(other)) => {
                    term = Some(other);
                    break;
                }

                Some(TerminationType::Contains(terms)) => {
                    contains.extend(terms);
                }
            }
        }

        if !contains.is_empty() {
            if contains.len() == 1 && contains.contains(&BaseTerminationType::Return) {
                self.resolved_terminal =
                    Some(Some(TerminationType::Base(BaseTerminationType::Return)));
            } else {
                self.resolved_terminal = Some(Some(TerminationType::Contains(contains)));
            }
        } else {
            match term {
                Some(BaseTerminationType::Break(res)) => {
                    contains.push(BaseTerminationType::Break(res));
                    self.resolved_terminal = Some(Some(TerminationType::Contains(contains)));
                }

                Some(term) => {
                    self.resolved_terminal = Some(Some(TerminationType::Base(term)));
                }

                None => {
                    self.resolved_terminal = Some(None);
                }
            }
        }

        return self.resolved_terminal.as_ref().unwrap().clone();
    }
}

impl<T: ResolvedType> From<WhileExpr<()>> for WhileExpr<Option<T>> {
    fn from(value: WhileExpr<()>) -> Self {
        let resolved_terminal = if let Some(term) = value.resolved_terminal {
            Some(if let Some(term) = term {
                Some(from(term))
            } else {
                None
            })
        } else {
            None
        };

        return Self {
            id: value.id,
            while_token: value.while_token,
            condition: from(value.condition),
            block: from(value.block),
            resolved_terminal,
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
        let resolved_terminal = if let Some(term) = value.resolved_terminal {
            Some(if let Some(term) = term {
                Some(try_from(term)?)
            } else {
                None
            })
        } else {
            None
        };

        return Ok(Self {
            id: value.id,
            while_token: value.while_token,
            condition: try_from(value.condition)?,
            block: try_from(value.block)?,
            resolved_terminal,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

// Visitor pattern
pub trait ExprVisitor<T: ResolvedType, R = ()> {
    fn visit_bool_literal_expr(&mut self, expr: &mut BoolLiteralExpr<T>) -> R;
    fn visit_number_literal_expr(&mut self, expr: &mut NumberLiteralExpr<T>) -> R;
    fn visit_plain_string_literal_expr(&mut self, expr: &mut PlainStringLiteralExpr<T>) -> R;
    fn visit_fmt_string_literal_expr(&mut self, expr: &mut FmtStringLiteralExpr<T>) -> R;
    fn visit_ident_expr(&mut self, expr: &mut IdentExpr<T>) -> R;
    fn visit_call_expr(&mut self, expr: &mut CallExpr<T>) -> R;
    fn visit_unary_expr(&mut self, expr: &mut UnaryExpr<T>) -> R;
    fn visit_binary_expr(&mut self, expr: &mut BinaryExpr<T>) -> R;
    fn visit_static_ref_expr(&mut self, expr: &mut StaticRefExpr<T>) -> R;
    fn visit_construct_expr(&mut self, expr: &mut ConstructExpr<T>) -> R;
    fn visit_get_expr(&mut self, expr: &mut GetExpr<T>) -> R;
    fn visit_if_expr(&mut self, expr: &mut IfExpr<T>) -> R;
    fn visit_loop_expr(&mut self, expr: &mut LoopExpr<T>) -> R;
    fn visit_while_expr(&mut self, expr: &mut WhileExpr<T>) -> R;
}

pub trait ExprAccept<T: ResolvedType, R, V: ExprVisitor<T, R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for Expr<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
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

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for BoolLiteralExpr<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_bool_literal_expr(self);
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for NumberLiteralExpr<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_number_literal_expr(self);
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for PlainStringLiteralExpr<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_plain_string_literal_expr(self);
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for FmtStringLiteralExpr<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_fmt_string_literal_expr(self);
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for IdentExpr<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_ident_expr(self);
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for CallExpr<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_call_expr(self);
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for UnaryExpr<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_unary_expr(self);
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for BinaryExpr<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_binary_expr(self);
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for StaticRefExpr<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_static_ref_expr(self);
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for ConstructExpr<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_construct_expr(self);
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for GetExpr<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_get_expr(self);
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for IfExpr<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_if_expr(self);
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for LoopExpr<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_loop_expr(self);
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for WhileExpr<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_while_expr(self);
    }
}
