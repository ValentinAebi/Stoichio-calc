
#[macro_export]
macro_rules! return_on_error {
    ($e: expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Err(e)
        }
    };
}
