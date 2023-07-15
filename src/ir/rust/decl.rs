use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RustIRDecl {
    Fn(RustIRFnDecl),
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
    pub body: RustIRCodeBlock,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRFnGenerics {}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRFnParam {}

// Visitor pattern
pub trait RustIRDeclVisitor<R = ()> {
    fn visit_fn_decl(&mut self, decl: &mut RustIRFnDecl) -> R;
}

pub trait RustIRDeclAccept<R, V: RustIRDeclVisitor<R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<R, V: RustIRDeclVisitor<R>> RustIRDeclAccept<R, V> for RustIRDecl {
    fn accept(&mut self, visitor: &mut V) -> R {
        return match self {
            Self::Fn(decl) => decl.accept(visitor),
        };
    }
}

impl<R, V: RustIRDeclVisitor<R>> RustIRDeclAccept<R, V> for RustIRFnDecl {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_fn_decl(self);
    }
}
