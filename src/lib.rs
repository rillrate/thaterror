use thiserror::Error;

pub trait OptionExt<T> {
    fn required(self) -> Result<T, Required>;
}

impl<T> OptionExt<T> for Option<T> {
    fn required(self) -> Result<T, Required> {
        self.ok_or(Required)
    }
}

/// If option or parameter is required.
#[derive(Error, Debug)]
#[error("value is required")]
pub struct Required;
