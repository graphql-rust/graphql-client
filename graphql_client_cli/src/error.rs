use std::fmt::{Debug, Display};

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
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(msg) = &self.message {
            f.write_str(msg)?;
            f.write_str("\n")?;
        }

        if self.source.is_some() && self.message.is_some() {
            f.write_str("Cause: ")?;
        }

        if let Some(source) = self.source.as_ref() {
            Display::fmt(source, f)?;
        }

        f.write_str("\nLocation: ")?;
        Display::fmt(self.location, f)?;

        Ok(())
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
