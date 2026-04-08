mod event;

pub use event::Event;

use crate::format::{EventFormatConfig, EventFormatter};

#[derive(Debug, Default, Clone)]
pub struct Status {
    event: Event,
}

impl Status {
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    pub fn reset_event(&mut self) {
        self.event = Event::default();
    }

    pub fn event(&self) -> &Event {
        &self.event
    }

    pub fn event_mut(&mut self) -> &mut Event {
        &mut self.event
    }

    pub fn set_event(&mut self, event: Event) {
        self.event = event;
    }

    pub fn format<F>(&self, f: F) -> String
    where
        F: EventFormatter,
    {
        f.format(&self.event)
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cfg = EventFormatConfig::default();
        write!(f, "{}", self.format(cfg))
    }
}
