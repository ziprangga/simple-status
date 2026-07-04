// This module is for internal macro use only.
// Do not use these items directly in your code.
#[doc(hidden)]
pub mod __private {
    use crate::Event;
    use crate::Status;
    use std::path::PathBuf;

    /// Constructs a `Status` object from optional fields passed by macros.
    /// This function handles the boilerplate of updating the `Event::builder()`.
    pub fn build_status(
        stage: Option<String>,
        current: Option<usize>,
        total: Option<usize>,
        message: Option<String>,
        path: Option<PathBuf>,
    ) -> Status {
        let mut builder = Event::builder();

        if let Some(stage) = stage {
            builder = builder.stage(stage);
        }
        if let Some(current) = current {
            builder = builder.current(current);
        }
        if let Some(total) = total {
            builder = builder.total(total);
        }
        if let Some(message) = message {
            builder = builder.message(message);
        }
        if let Some(path) = path {
            builder = builder.path(path);
        }

        Status::new(builder.build())
    }
}
