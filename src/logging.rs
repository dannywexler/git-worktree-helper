use std::fmt::Debug;
use std::process;

pub fn log_fatal(msg: &str) -> ! {
    let message = msg.strip_prefix("Error").unwrap_or(msg).trim();
    eprintln!("ERROR {}", message);
    process::exit(1);
}

pub trait LogFatalOption<T> {
    fn fatal(self, msg: &str) -> T;
}

impl<T> LogFatalOption<T> for Option<T> {
    #[track_caller]
    fn fatal(self, msg: &str) -> T {
        match self {
            Some(item) => item,
            None => {
                log_fatal(&msg);
            }
        }
    }
}

pub trait LogFatalResult<T, GenericError: Debug> {
    fn fatal(self, msg: &str) -> T;

    fn fatal_err<GenericErrorFormatter: FnOnce(&GenericError) -> String>(
        self,
        make_msg: GenericErrorFormatter,
    ) -> T;
}

impl<T, GenericError: Debug> LogFatalResult<T, GenericError> for Result<T, GenericError> {
    #[track_caller]
    fn fatal(self, msg: &str) -> T {
        match self {
            Ok(item) => item,
            Err(err) => {
                let msg = format!("{}. Got error: {:?}", msg, err);
                log_fatal(&msg);
            }
        }
    }

    #[track_caller]
    fn fatal_err<GenericErrorFormatter: FnOnce(&GenericError) -> String>(
        self,
        make_msg: GenericErrorFormatter,
    ) -> T {
        match self {
            Ok(item) => item,
            Err(err) => {
                log_fatal(&make_msg(&err));
            }
        }
    }
}
