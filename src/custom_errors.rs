use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("ON test une éreure")]    
    TestError,
}   