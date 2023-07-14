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
    pub name: Rc<str>,
    pub params: Vec<RustIRFnParam>,
    pub return_type: Option<RustIRStaticType>,
    pub body: RustIRCodeBlock,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRFnGenerics {}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRFnParam {}
