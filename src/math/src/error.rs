use thiserror::Error;

#[derive(Error,Debug)]
pub enum Vect3dError {
    #[error("Division by zero.")]
    DivideByZero,
}
