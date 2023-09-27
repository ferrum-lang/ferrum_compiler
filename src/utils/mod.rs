mod string;
pub use string::*;

mod sync;
pub use sync::*;

mod run;
pub use run::*;

pub fn invert<T, E>(x: Option<Result<T, E>>) -> Result<Option<T>, E> {
    x.map_or(Ok(None), |v| v.map(Some))
}
