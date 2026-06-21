/// Options as to why the server was ended
#[derive(Debug)]
pub enum ExitReason {
    UserEnd,
    FailedStartup,
    Unrecoverable,
}

impl PartialEq for ExitReason {
    fn eq(&self, other: &Self) -> bool {
        if self == other {
            return true;
        } else {
            return false;
        }
    }
}
