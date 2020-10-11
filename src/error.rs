pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum OperationError {
        #[error("General error")]
        Error,

        #[error("HTML parse error")]
        HtmlParseError,

        #[error("Invalid login or password")]
        Authentication,

        #[error(transparent)]
        IOError(#[from] std::io::Error)
    }
}