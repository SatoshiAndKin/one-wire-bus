use core::fmt::Debug;

pub type OneWireResult<T, E> = Result<T, OneWireError<E>>;

#[derive(Debug, Copy, Clone)]
pub enum OneWireError<E> {
    BusNotHigh,
    PinError(E),
    UnexpectedResponse,
    FamilyCodeMismatch,
    CrcMismatch,
    Timeout,
}
