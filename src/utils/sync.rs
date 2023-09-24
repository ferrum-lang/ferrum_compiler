use std::sync::{Arc, Mutex};

use crate::result::Result;

use super::invert;

pub trait FeFrom<T>: Sized {
    fn from(value: T) -> Self;
}

pub fn fe_from<T, R: FeFrom<T>>(value: T) -> R {
    return FeFrom::from(value);
}

pub fn from<T, R: From<T>>(value: T) -> R {
    return From::from(value);
}

pub trait FeTryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub fn fe_try_from<T, E, R: FeTryFrom<T, Error = E>>(value: T) -> Result<R, E> {
    return FeTryFrom::try_from(value);
}

pub fn try_from<T, E, R: TryFrom<T, Error = E>>(value: T) -> Result<R, E> {
    return TryFrom::try_from(value);
}

impl<T: Clone, R: Sized + From<T>> FeFrom<Arc<Mutex<T>>> for Arc<Mutex<R>> {
    fn from(value: Arc<Mutex<T>>) -> Self {
        return Arc::new(Mutex::new(From::from(value.lock().unwrap().clone())));
    }
}

impl<T: Clone, E, R: Sized + TryFrom<T, Error = E>> FeTryFrom<Arc<Mutex<T>>> for Arc<Mutex<R>> {
    type Error = E;

    fn try_from(value: Arc<Mutex<T>>) -> Result<Self, E> {
        return Ok(Arc::new(Mutex::new(TryFrom::try_from(
            value.lock().unwrap().clone(),
        )?)));
    }
}

impl<T: Clone, R: Sized + From<T>> FeFrom<Vec<T>> for Vec<R> {
    fn from(value: Vec<T>) -> Self {
        return value.into_iter().map(From::from).collect();
    }
}

impl<T: Clone, E, R: Sized + TryFrom<T, Error = E>> FeTryFrom<Vec<T>> for Vec<R> {
    type Error = E;

    fn try_from(value: Vec<T>) -> Result<Self, E> {
        return value
            .into_iter()
            .map(TryFrom::try_from)
            .collect::<Result<Self, E>>();
    }
}

impl<T: Clone, R: Sized + From<T>> FeFrom<Option<T>> for Option<R> {
    fn from(value: Option<T>) -> Self {
        return value.map(From::from);
    }
}

impl<T: Clone, E, R: Sized + TryFrom<T, Error = E>> FeTryFrom<Option<T>> for Option<R> {
    type Error = E;

    fn try_from(value: Option<T>) -> Result<Self, E> {
        return invert(value.map(TryFrom::try_from));
    }
}
