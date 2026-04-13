mod event;

// use crate::format::{EventFormatConfig, EventFormatter};
pub use event::Event;

pub trait StatusFormatter {
    fn format(&self, status: &Status) -> String;
}

impl<F> StatusFormatter for F
where
    F: Fn(&Status) -> String,
{
    fn format(&self, status: &Status) -> String {
        (self)(status)
    }
}

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

    // pub fn format<F>(&self, f: F) -> String
    // where
    //     F: EventFormatter,
    // {
    //     f.format(&self.event)
    // }

    pub fn format_with<F>(&self, f: F) -> String
    where
        F: StatusFormatter,
    {
        f.format(self)
    }
}

// impl std::fmt::Display for Status {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let cfg = EventFormatConfig::default();
//         write!(f, "{}", self.format(cfg))
//     }
// }

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let e = &self.event();

        if let Some(stage) = &e.stage() {
            write!(f, "[{}] ", stage)?;
        }

        if let (Some(c), Some(t)) = (e.current(), e.total()) {
            write!(f, "{}/{} ", c, t)?;
        } else if let Some(c) = e.current() {
            write!(f, "{} ", c)?;
        } else if let Some(t) = e.total() {
            write!(f, "{} ", t)?;
        }

        if let Some(msg) = &e.message() {
            write!(f, "{}", msg)?;
        }

        if let Some(path) = &e.path() {
            write!(f, "{}", path.display())?;
        }

        Ok(())
    }
}
