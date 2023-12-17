use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum GoIRDecl {
    Fn(GoIRFnDecl),
    Struct(GoIRStructDecl),
}

#[derive(Debug, Clone, PartialEq)]
pub enum GoIRDeclMod {
    Pub,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRFnDecl {
    pub name: Arc<str>,
    pub params: Vec<GoIRFnParam>,
    pub return_type: Option<GoIRStaticType>,
    pub body: GoIRBlockExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRFnParam {
    pub name: Arc<str>,
    pub static_type_ref: GoIRStaticType,
    pub trailing_comma: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRStructDecl {
    pub name: Arc<str>,
    pub fields: Vec<GoIRStructField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRStructField {
    pub field_mod: Option<GoIRStructFieldMod>,
    pub name: Arc<str>,
    pub static_type_ref: GoIRStaticType,
    pub trailing_comma: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GoIRStructFieldMod {
    Pub,
}

// Visitor pattern
pub trait GoIRDeclVisitor<R = ()> {
    fn visit_fn_decl(&mut self, decl: &mut GoIRFnDecl) -> R;
    fn visit_struct_decl(&mut self, decl: &mut GoIRStructDecl) -> R;
}

pub trait GoIRDeclAccept<R, V: GoIRDeclVisitor<R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<R, V: GoIRDeclVisitor<R>> GoIRDeclAccept<R, V> for GoIRDecl {
    fn accept(&mut self, visitor: &mut V) -> R {
        return match self {
            Self::Fn(decl) => decl.accept(visitor),
            Self::Struct(decl) => decl.accept(visitor),
        };
    }
}

impl<R, V: GoIRDeclVisitor<R>> GoIRDeclAccept<R, V> for GoIRFnDecl {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_fn_decl(self);
    }
}

impl<R, V: GoIRDeclVisitor<R>> GoIRDeclAccept<R, V> for GoIRStructDecl {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_struct_decl(self);
    }
}
