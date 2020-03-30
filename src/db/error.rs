#[derive(Debug)]
pub enum Error {
    InsertNumError,
    DuplicateData(String),
    WapperError(String),
    NotFound,
    ForeignKeyViolation(String),
}
