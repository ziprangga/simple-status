use crate::status_event::StatusEvent;

pub trait StatusFormatter {
    fn format(&self, event: &StatusEvent) -> String;
}

impl<F> StatusFormatter for F
where
    F: Fn(&StatusEvent) -> String,
{
    fn format(&self, event: &StatusEvent) -> String {
        (self)(event)
    }
}

pub struct StatusFormatConfig {
    pub parts: Vec<Box<dyn Fn(&StatusEvent) -> Option<String>>>,
    pub separator: Option<String>,
}

impl StatusFormatConfig {
    pub fn new() -> Self {
        Self {
            parts: Vec::new(),
            separator: None,
        }
    }

    pub fn stage<F: 'static + Fn(&str) -> String>(&mut self, fmt: F) -> &mut Self {
        self.parts
            .push(Box::new(move |s: &StatusEvent| s.stage().map(|v| fmt(v))));
        self
    }

    pub fn current<F: 'static + Fn(usize) -> String>(&mut self, fmt: F) -> &mut Self {
        self.parts
            .push(Box::new(move |s: &StatusEvent| s.current().map(|v| fmt(v))));
        self
    }

    pub fn total<F: 'static + Fn(usize) -> String>(&mut self, fmt: F) -> &mut Self {
        self.parts
            .push(Box::new(move |s: &StatusEvent| s.total().map(|v| fmt(v))));
        self
    }

    pub fn message<F: 'static + Fn(&str) -> String>(&mut self, fmt: F) -> &mut Self {
        self.parts
            .push(Box::new(move |s: &StatusEvent| s.message().map(|v| fmt(v))));
        self
    }

    pub fn path<F: 'static + Fn(&std::path::Path) -> String>(&mut self, fmt: F) -> &mut Self {
        self.parts
            .push(Box::new(move |s: &StatusEvent| s.path().map(|v| fmt(v))));
        self
    }

    pub fn separator(&mut self, sep: impl Into<String>) -> &mut Self {
        self.separator = Some(sep.into());
        self
    }

    pub fn write(&self, status: &StatusEvent) -> String {
        let parts: Vec<String> = self.parts.iter().filter_map(|f| f(status)).collect();
        match &self.separator {
            Some(sep) => parts.join(sep),
            None => parts.concat(),
        }
    }
}

impl Default for StatusFormatConfig {
    fn default() -> Self {
        let mut cfg = StatusFormatConfig::new();
        cfg.stage(|v| v.to_string());
        cfg.current(|v| v.to_string());
        cfg.total(|v| v.to_string());
        cfg.message(|v| v.to_string());
        cfg.path(|v| v.display().to_string());
        cfg.separator = Some(" | ".to_string());
        cfg
    }
}

impl StatusFormatter for StatusFormatConfig {
    fn format(&self, event: &StatusEvent) -> String {
        self.write(event)
    }
}
