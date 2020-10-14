pub mod types {
    use crate::error::error::OperationError;

    pub type OperationResult<R> = Result<R, OperationError>;
    pub type OptionResult<R> = Result<Option<R>, OperationError>;
}
