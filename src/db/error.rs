use super::schema::users;
use diesel::deserialize::Queryable;
use diesel::QueryResult;

#[derive(Debug)]
pub enum Error {
    InsertNumError,
    DuplicateData(String),
    WapperError(String),
    NotFound,
    ForeignKeyViolation(String),
}

pub fn deal_insert_result(r: QueryResult<usize>) -> Result<(), Error> {
    match r {
        Ok(s) => {
            if s == 1 {
                Ok(())
            } else {
                Err(Error::InsertNumError)
            }
        }
        Err(e) => {
            if let diesel::result::Error::DatabaseError(UniqueViolation, _) = e {
                Err(Error::DuplicateData(e.to_string()))
            } else {
                Err(Error::WapperError(e.to_string()))
            }
        }
    }
}

pub fn deal_query_result<T>(r: QueryResult<T>) -> Result<T, Error> {
    match r {
        Ok(u) => Ok(u),
        Err(e) => {
            if let diesel::NotFound = e {
                Err(Error::NotFound)
            } else {
                Err(Error::WapperError(e.to_string()))
            }
        }
    }
}

pub fn deal_update_result(r: QueryResult<usize>) -> Result<(), Error> {
    match r {
        Ok(s) => {
            if s == 1 {
                Ok(())
            } else {
                Err(Error::NotFound)
            }
        }
        Err(e) => {
            if let diesel::NotFound = e {
                Err(Error::NotFound)
            } else {
                Err(Error::WapperError(e.to_string()))
            }
        }
    }
}
