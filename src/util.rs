
#[macro_export]
macro_rules! return_on_error {
    ($e: expr) => {
        let res: Result<_, _> = $e;
        match res {
            Ok(_) => {}
            Err(e) => return Err(e)
        }
    };
}
