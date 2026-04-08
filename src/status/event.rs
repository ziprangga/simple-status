use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct Event {
    stage: Option<String>,
    current: Option<usize>,
    total: Option<usize>,
    message: Option<String>,
    path: Option<PathBuf>,
}

impl Event {
    pub fn builder() -> EventBuilder {
        EventBuilder::new()
    }

    pub fn stage(&self) -> Option<&str> {
        self.stage.as_deref()
    }

    pub fn current(&self) -> Option<usize> {
        self.current
    }

    pub fn total(&self) -> Option<usize> {
        self.total
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct EventBuilder {
    status_event: Event,
}

impl EventBuilder {
    pub fn new() -> Self {
        Self {
            status_event: Event::default(),
        }
    }

    pub fn stage(mut self, stage: impl Into<String>) -> Self {
        self.status_event.stage = Some(stage.into());
        self
    }

    pub fn current(mut self, current: usize) -> Self {
        self.status_event.current = Some(current);
        self
    }

    pub fn total(mut self, total: usize) -> Self {
        self.status_event.total = Some(total);
        self
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.status_event.message = Some(message.into());
        self
    }

    pub fn path(mut self, path: PathBuf) -> Self {
        self.status_event.path = Some(path);
        self
    }

    pub fn build(self) -> Event {
        self.status_event
    }
}
