mod decl;
pub use decl::*;

mod expr;
pub use expr::*;

mod node;
pub use node::*;

mod stmt;
pub use stmt::*;

mod r#use;
pub use r#use::*;

mod r#static;
pub use r#static::*;

use crate::r#type::FeType;
use crate::result::Result;
use crate::token;
use crate::utils::{fe_from, fe_try_from, from, invert, try_from};

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub trait SyntaxCompiler<IR> {
    fn compile_package(entry: Arc<Mutex<FeSyntaxPackage<FeType>>>) -> Result<IR>;
}

pub trait Resolvable {
    fn is_signature_resolved(&self) -> bool {
        return self.is_resolved();
    }

    fn is_resolved(&self) -> bool;
}

#[derive(Debug, Clone)]
pub enum FeSyntaxPackage<T: ResolvedType = ()> {
    File(FeSyntaxFile<T>),
    Dir(FeSyntaxDir<T>),
}

impl From<token::FeTokenPackage> for FeSyntaxPackage {
    fn from(value: token::FeTokenPackage) -> Self {
        match value {
            token::FeTokenPackage::File(file) => return FeSyntaxPackage::File(file.into()),
            token::FeTokenPackage::Dir(dir) => return FeSyntaxPackage::Dir(dir.into()),
        };
    }
}

impl<T: ResolvedType> From<FeSyntaxPackage<()>> for FeSyntaxPackage<Option<T>> {
    fn from(value: FeSyntaxPackage<()>) -> Self {
        match value {
            FeSyntaxPackage::File(file) => return Self::File(file.into()),
            FeSyntaxPackage::Dir(dir) => return Self::Dir(dir.into()),
        }
    }
}

impl<T: ResolvedType> Resolvable for FeSyntaxPackage<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            Self::File(file) => return file.is_resolved(),
            Self::Dir(dir) => return dir.is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<FeSyntaxPackage<Option<T>>> for FeSyntaxPackage<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: FeSyntaxPackage<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            FeSyntaxPackage::File(file) => {
                return Ok(FeSyntaxPackage::File(file.try_into()?));
            }
            FeSyntaxPackage::Dir(dir) => return Ok(FeSyntaxPackage::Dir(dir.try_into()?)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FeSyntaxFile<T: ResolvedType = ()> {
    pub name: SyntaxPackageName,
    pub path: PathBuf,
    pub syntax: Arc<Mutex<SyntaxTree<T>>>,
}

impl From<token::FeTokenFile> for FeSyntaxFile {
    fn from(value: token::FeTokenFile) -> Self {
        return Self {
            name: value.name.into(),
            path: value.path,
            syntax: Arc::new(Mutex::new(SyntaxTree {
                mods: vec![],
                uses: vec![],
                decls: vec![],
            })),
        };
    }
}

impl<T: ResolvedType> From<FeSyntaxFile<()>> for FeSyntaxFile<Option<T>> {
    fn from(value: FeSyntaxFile<()>) -> Self {
        return Self {
            name: value.name,
            path: value.path,
            syntax: fe_from(value.syntax),
        };
    }
}

impl<T: ResolvedType> Resolvable for FeSyntaxFile<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.syntax.lock().unwrap().is_resolved();
    }
}

impl<T: ResolvedType> TryFrom<FeSyntaxFile<Option<T>>> for FeSyntaxFile<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: FeSyntaxFile<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            name: value.name,
            path: value.path,
            syntax: fe_try_from(value.syntax)?,
        });
    }
}

#[derive(Debug, Clone)]
pub struct FeSyntaxDir<T: ResolvedType = ()> {
    pub name: SyntaxPackageName,
    pub path: PathBuf,
    pub entry_file: FeSyntaxFile<T>,
    pub local_packages: HashMap<SyntaxPackageName, Arc<Mutex<FeSyntaxPackage<T>>>>,
}

impl From<token::FeTokenDir> for FeSyntaxDir {
    fn from(value: token::FeTokenDir) -> Self {
        return Self {
            name: value.name.into(),
            path: value.path,
            entry_file: value.entry_file.into(),
            local_packages: value
                .local_packages
                .into_iter()
                .map(|(name, pkg)| (name.into(), fe_from(pkg)))
                .collect(),
        };
    }
}

impl<T: ResolvedType> From<FeSyntaxDir<()>> for FeSyntaxDir<Option<T>> {
    fn from(value: FeSyntaxDir<()>) -> Self {
        let local_packages = value
            .local_packages
            .into_iter()
            .map(|(name, pkg)| (name, fe_from(pkg)))
            .collect();

        return Self {
            name: value.name,
            path: value.path,
            entry_file: value.entry_file.into(),
            local_packages,
        };
    }
}

impl<T: ResolvedType> Resolvable for FeSyntaxDir<Option<T>> {
    fn is_resolved(&self) -> bool {
        if !self.entry_file.is_resolved() {
            dbg!("false");
            return false;
        }

        for pkg in self.local_packages.values() {
            if !pkg.lock().unwrap().is_resolved() {
                dbg!("false");
                return false;
            }
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<FeSyntaxDir<Option<T>>> for FeSyntaxDir<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: FeSyntaxDir<Option<T>>) -> Result<Self, Self::Error> {
        let local_packages = value
            .local_packages
            .into_iter()
            .map(|(name, pkg)| Ok((name, fe_try_from(pkg)?)))
            .collect::<Result<
                HashMap<SyntaxPackageName, Arc<Mutex<FeSyntaxPackage<T>>>>,
                FinalizeResolveTypeError,
            >>()?;

        return Ok(FeSyntaxDir {
            name: value.name,
            path: value.path,
            entry_file: value.entry_file.try_into()?,
            local_packages,
        });
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SyntaxPackageName(pub Arc<str>);

impl From<token::TokenPackageName> for SyntaxPackageName {
    fn from(value: token::TokenPackageName) -> Self {
        return Self(value.0);
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxTree<T: ResolvedType = ()> {
    pub mods: Vec<Mod>,
    pub uses: Vec<Arc<Mutex<Use<T>>>>,
    pub decls: Vec<Arc<Mutex<Decl<T>>>>,
}

impl<T: ResolvedType> From<SyntaxTree<()>> for SyntaxTree<Option<T>> {
    fn from(value: SyntaxTree<()>) -> Self {
        return Self {
            mods: value.mods,
            uses: value.uses.into_iter().map(|u| fe_from(u)).collect(),
            decls: value.decls.into_iter().map(|d| fe_from(d)).collect(),
        };
    }
}

impl<T: ResolvedType> Resolvable for SyntaxTree<Option<T>> {
    fn is_resolved(&self) -> bool {
        for u in &self.uses {
            if !u.lock().unwrap().is_resolved() {
                dbg!("false");
                return false;
            }
        }

        for d in &self.decls {
            if !d.lock().unwrap().is_resolved() {
                dbg!("false");
                return false;
            }
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<SyntaxTree<Option<T>>> for SyntaxTree<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: SyntaxTree<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            mods: value.mods,
            uses: value
                .uses
                .into_iter()
                .map(|u| fe_try_from(u))
                .collect::<Result<Vec<Arc<Mutex<Use<T>>>>, Self::Error>>()?,
            decls: value
                .decls
                .into_iter()
                .map(|d| fe_try_from(d))
                .collect::<Result<Vec<Arc<Mutex<Decl<T>>>>, Self::Error>>()?,
        });
    }
}

#[derive(Debug, Clone)]
pub struct Mod(pub Arc<str>);

#[derive(Debug, Clone)]
pub struct FinalizeResolveTypeError {
    pub file: &'static str,
    pub line: u32,
}

impl std::fmt::Display for FinalizeResolveTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{self:?}");
    }
}
impl std::error::Error for FinalizeResolveTypeError {}

pub trait ResolvedType: std::fmt::Debug + Clone + PartialEq {}
impl ResolvedType for () {}
impl ResolvedType for FeType {}
impl<T: ResolvedType> ResolvedType for Option<T> {}

#[derive(Debug, Clone, PartialEq)]
pub enum BaseTerminationType<T: ResolvedType> {
    Break(Option<NestedExpr<T>>),
    Then(NestedExpr<T>),
    Return,
    InfiniteLoop,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TerminationType<T: ResolvedType> {
    Contains(Vec<BaseTerminationType<T>>),
    Base(BaseTerminationType<T>),
}

impl BaseTerminationType<Option<FeType>> {
    pub fn resolved_type(&self) -> Option<FeType> {
        match self {
            Self::Then(expr) | Self::Break(Some(expr)) => {
                return expr.0.lock().unwrap().resolved_type().cloned().flatten()
            }
            Self::Break(None) | Self::Return | Self::InfiniteLoop => return None,
        }
    }
}

impl TerminationType<Option<FeType>> {
    pub fn resolved_type(&self) -> Vec<FeType> {
        match self {
            Self::Contains(vals) => {
                return vals
                    .iter()
                    .map(|val| val.resolved_type())
                    .filter_map(|v| v)
                    .collect();
            }
            Self::Base(b) => {
                let mut vals = vec![];

                if let Some(v) = b.resolved_type() {
                    vals.push(v);
                }

                return vals;
            }
        }
    }
}

pub trait IsTerminal<T: ResolvedType> {
    fn is_terminal(&mut self) -> Option<TerminationType<T>> {
        return None;
    }
}

impl<T: ResolvedType> From<BaseTerminationType<()>> for BaseTerminationType<Option<T>> {
    fn from(value: BaseTerminationType<()>) -> Self {
        match value {
            BaseTerminationType::Break(e) => return Self::Break(e.map(from)),
            BaseTerminationType::Then(e) => return Self::Then(from(e)),
            BaseTerminationType::Return => return Self::Return,
            BaseTerminationType::InfiniteLoop => return Self::InfiniteLoop,
        }
    }
}

impl<T: ResolvedType> TryFrom<BaseTerminationType<Option<T>>> for BaseTerminationType<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: BaseTerminationType<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            BaseTerminationType::Break(e) => return Ok(Self::Break(invert(e.map(try_from))?)),
            BaseTerminationType::Then(e) => return Ok(Self::Then(try_from(e)?)),
            BaseTerminationType::Return => return Ok(Self::Return),
            BaseTerminationType::InfiniteLoop => return Ok(Self::InfiniteLoop),
        }
    }
}

impl<T: ResolvedType> From<TerminationType<()>> for TerminationType<Option<T>> {
    fn from(value: TerminationType<()>) -> Self {
        match value {
            TerminationType::Contains(vals) => {
                return Self::Contains(vals.into_iter().map(from).collect())
            }
            TerminationType::Base(b) => return Self::Base(from(b)),
        }
    }
}

impl<T: ResolvedType> TryFrom<TerminationType<Option<T>>> for TerminationType<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: TerminationType<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            TerminationType::Contains(vals) => {
                return Ok(Self::Contains(
                    vals.into_iter()
                        .map(|val| try_from(val))
                        .collect::<Result<Vec<_>, Self::Error>>()?,
                ))
            }
            TerminationType::Base(b) => return Ok(Self::Base(try_from(b)?)),
        }
    }
}
