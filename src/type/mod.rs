use crate::type_resolver::ExportsPackage;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub const BOOL_TYPE_NAME: &str = "Bool";
pub const INT_TYPE_NAME: &str = "Int";
pub const STRING_TYPE_NAME: &str = "String";

pub const STD_LIB_PKG_NAME: &str = "fe";
pub const STD_PRINT_FN_NAME: &str = "print";

#[derive(Debug, Clone)]
pub enum FeType {
    Package(Arc<Mutex<ExportsPackage>>),
    Callable(Callable),
    Struct(FeStruct),
    Instance(FeInstance),
    String(Option<StringDetails>),
    Bool(Option<bool>),
    Number(Option<NumberDetails>),
    Ref(FeRefOf),
    Owned(FeOwnedOf),
}

impl PartialEq for FeType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Package(pkg), Self::Package(other)) => {
                let pkg = { pkg.try_lock().unwrap().clone() };
                let other = other.try_lock().unwrap();
                return PartialEq::eq(&pkg, &other);
            }
            (Self::Callable(this), Self::Callable(other)) => return this == other,
            (Self::Struct(this), Self::Struct(other)) => return this == other,
            (Self::Instance(this), Self::Instance(other)) => return this == other,
            (Self::String(this), Self::String(other)) => return this == other,
            (Self::Bool(this), Self::Bool(other)) => return this == other,
            (Self::Number(this), Self::Number(other)) => return this == other,
            (Self::Ref(this), Self::Ref(other)) => return this == other,
            (Self::Owned(this), Self::Owned(other)) => return this == other,

            _ => return false,
        }
    }
}

impl FeType {
    pub fn instance(&self) -> Option<&FeInstance> {
        match self {
            Self::Instance(instance) => return Some(instance),
            Self::Owned(owned) => return owned.of.instance(),
            Self::Ref(r) => return r.of.instance(),
            _ => return None,
        }
    }

    pub fn actual_type(&self) -> &FeType {
        match &self {
            Self::Ref(t) => return &t.of,
            Self::Owned(t) => return &t.of,

            _ => return self,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Callable {
    pub special: Option<SpecialCallable>,
    pub name: Arc<str>,
    pub params: Vec<(Arc<str>, FeType)>,
    pub return_type: Option<Box<FeType>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecialCallable {
    Print,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FeStruct {
    pub special: Option<SpecialStruct>,
    pub name: Arc<str>,
    pub fields: Vec<FeStructField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FeStructField {
    pub is_pub: bool,
    pub name: Arc<str>,
    pub typ: FeType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecialStruct {}

#[derive(Debug, Clone, PartialEq)]
pub struct FeInstance {
    pub special: Option<SpecialInstance>,
    pub name: Arc<str>,
    pub fields: HashMap<Arc<str>, FeStructField>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecialInstance {}

#[derive(Debug, Clone, PartialEq)]
pub enum StringDetails {
    PlainLiteral,
    Format,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumberDetails {
    // TODO: number sizes, pos/neg, bounds, bignums, etc
    Integer(Option<i64>),
    Decimal(Option<f64>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FeRefOf {
    pub ref_type: FeRefType,
    pub of: Box<FeType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FeRefType {
    Const,
    Mut,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FeOwnedOf {
    pub owned_mut: FeOwnedMut,
    pub of: Box<FeType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FeOwnedMut {
    Const,
    Mut,
}
