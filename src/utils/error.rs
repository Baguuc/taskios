/// Return Ok value when the result is Ok, otherwise print the error and exit.
///
/// ### arguments:
/// + result: [Result]
///
pub fn error_if_necessary<T, E: ToString>(r: Result<T, E>) -> T {
    match r {
        Ok(ok) => ok,
        Err(err) => {
            clin::components::error("something went wrong", err.to_string());
            std::process::exit(1);
        }
    }
}