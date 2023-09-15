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
