use std;

use anyhow;

pub type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;
