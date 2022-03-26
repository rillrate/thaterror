use thiserror::Error;

/// If option or parameter is required.
#[derive(Error, Debug)]
#[error("value is required")]
pub struct Required;
