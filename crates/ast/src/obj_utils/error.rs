pub enum ObjUtilsError {
    Error1,
}

pub struct OUError {
    kind: ObjUtilsError
}

#[allow(non_snake_case)]
pub fn Error(_s: &str) -> OUError {
    OUError {
        kind: ObjUtilsError::Error1
    }
}

pub enum ObjUtilsResult<T,E> {
    Ok(T),
    ObjUtilsErr(E)
}
