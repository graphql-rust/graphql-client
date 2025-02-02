pub struct Error {
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    message: Option<String>,
    location: &'static std::panic::Location<'static>,
}

impl Error {
    #[track_caller]
    pub fn message(msg: String) -> Self {
        Error {
            source: None,
            message: Some(msg),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn source_with_message(
        source: impl std::error::Error + Send + Sync + 'static,
        message: String,
    ) -> Self {
        let mut err = Error::message(message);
        err.source = Some(Box::new(source));
        err
    }
}

// This is the impl that shows up when the error bubbles up to `main()`.
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.message, &self.source) {
            (Some(msg), Some(source)) => {
                write!(f, "{msg}\nCause: {source}\nLocation: {}", self.location)
            }
            (Some(msg), None) => write!(f, "{msg}\nLocation: {}", self.location),
            (None, Some(source)) => write!(f, "{source}\nLocation: {}", self.location),
            (None, None) => write!(f, "\nLocation: {}", self.location),
        }
    }
}

impl<T> From<T> for Error
where
    T: std::error::Error + Send + Sync + 'static,
{
    #[track_caller]
    fn from(err: T) -> Self {
        Error {
            message: None,
            source: Some(Box::new(err)),
            location: std::panic::Location::caller(),
        }
    }
}
