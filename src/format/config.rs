use crate::status::Event;

pub trait EventFormatter {
    fn format(&self, event: &Event) -> String;
}

impl<F> EventFormatter for F
where
    F: Fn(&Event) -> String,
{
    fn format(&self, event: &Event) -> String {
        (self)(event)
    }
}

pub struct EventFormatConfig {
    pub field: Vec<Box<dyn Fn(&Event) -> Option<String>>>,
    pub separator: Option<String>,
}

impl EventFormatConfig {
    pub fn new() -> Self {
        Self {
            field: Vec::new(),
            separator: None,
        }
    }

    fn parts<F>(&mut self, f: F)
    where
        F: 'static + Fn(&Event) -> Option<String>,
    {
        self.field.push(Box::new(f));
    }

    pub fn stage<F: 'static + Fn(&str) -> String>(&mut self, fmt: F) -> &mut Self {
        self.parts(move |s: &Event| s.stage().map(|v| fmt(v)));
        self
    }

    pub fn current<F: 'static + Fn(usize) -> String>(&mut self, fmt: F) -> &mut Self {
        self.parts(move |s: &Event| s.current().map(|v| fmt(v)));
        self
    }

    pub fn total<F: 'static + Fn(usize) -> String>(&mut self, fmt: F) -> &mut Self {
        self.parts(move |s: &Event| s.total().map(|v| fmt(v)));
        self
    }

    pub fn message<F: 'static + Fn(&str) -> String>(&mut self, fmt: F) -> &mut Self {
        self.parts(move |s: &Event| s.message().map(|v| fmt(v)));
        self
    }

    pub fn path<F: 'static + Fn(&std::path::Path) -> String>(&mut self, fmt: F) -> &mut Self {
        self.parts(move |s: &Event| s.path().map(|v| fmt(v)));
        self
    }

    pub fn separator(&mut self, sep: impl Into<String>) -> &mut Self {
        self.separator = Some(sep.into());
        self
    }

    pub fn write(&self, event: &Event) -> String {
        let field: Vec<String> = self.field.iter().filter_map(|f| f(event)).collect();
        match &self.separator {
            Some(sep) => field.join(sep),
            None => field.concat(),
        }
    }
}

impl Default for EventFormatConfig {
    fn default() -> Self {
        let mut cfg = EventFormatConfig::new();
        cfg.stage(|v| v.to_string());
        cfg.current(|v| v.to_string());
        cfg.total(|v| v.to_string());
        cfg.message(|v| v.to_string());
        cfg.path(|v| v.display().to_string());
        cfg.separator = Some(" | ".to_string());
        cfg
    }
}

impl EventFormatter for EventFormatConfig {
    fn format(&self, event: &Event) -> String {
        self.write(event)
    }
}
