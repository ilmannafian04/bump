use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    NotFound,
}
