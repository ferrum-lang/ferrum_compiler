use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RustIRDecl {
    Fn(RustIRFnDecl),
    Struct(RustIRStructDecl),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RustIRDeclMod {
    Pub,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRFnDecl {
    pub macros: Vec<RustIRMacro>,
    pub decl_mod: Option<RustIRDeclMod>,
    pub is_async: bool,
    pub generics: Option<RustIRFnGenerics>,
    pub name: Arc<str>,
    pub params: Vec<RustIRFnParam>,
    pub return_type: Option<RustIRStaticType>,
    pub body: RustIRBlockExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRFnGenerics {}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRFnParam {
    pub name: Arc<str>,
    pub static_type_ref: RustIRStaticType,
    pub trailing_comma: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRStructDecl {
    pub macros: Vec<RustIRMacro>,
    pub decl_mod: Option<RustIRDeclMod>,
    pub name: Arc<str>,
    pub generics: Option<RustIRStructGenerics>,
    pub fields: Vec<RustIRStructField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRStructGenerics {}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRStructField {
    pub field_mod: Option<RustIRStructFieldMod>,
    pub name: Arc<str>,
    pub static_type_ref: RustIRStaticType,
    pub trailing_comma: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RustIRStructFieldMod {
    Pub,
}

// Visitor pattern
pub trait RustIRDeclVisitor<R = ()> {
    fn visit_fn_decl(&mut self, decl: &mut RustIRFnDecl) -> R;
    fn visit_struct_decl(&mut self, decl: &mut RustIRStructDecl) -> R;
}

pub trait RustIRDeclAccept<R, V: RustIRDeclVisitor<R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<R, V: RustIRDeclVisitor<R>> RustIRDeclAccept<R, V> for RustIRDecl {
    fn accept(&mut self, visitor: &mut V) -> R {
        return match self {
            Self::Fn(decl) => decl.accept(visitor),
            Self::Struct(decl) => decl.accept(visitor),
        };
    }
}

impl<R, V: RustIRDeclVisitor<R>> RustIRDeclAccept<R, V> for RustIRFnDecl {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_fn_decl(self);
    }
}

impl<R, V: RustIRDeclVisitor<R>> RustIRDeclAccept<R, V> for RustIRStructDecl {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_struct_decl(self);
    }
}
