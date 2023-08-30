use thiserror::Error;


#[derive(Error, Debug)]
pub enum CprjError
{
    #[error("Placeholder Error: {0}")]
    Placeholder(String),

    #[error("No template named {0}")]
    NoTemplate(String),

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    InvalidPath(String),
}
