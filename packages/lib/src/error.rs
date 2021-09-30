pub use piteo_core::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn no(path: &'static str) -> Error {
    Error::msg(format!("Missing {}", path))
}