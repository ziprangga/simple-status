use crate::status_event::StatusEvent;

#[derive(Debug)]
pub struct StatusFormatConfig {
    pub stage: bool,
    pub current: bool,
    pub total: bool,
    pub message: bool,
    pub path: bool,
    pub separator: String,
}

impl StatusFormatConfig {
    pub fn stage(&mut self, write: bool) -> &mut Self {
        self.stage = write;
        self
    }
    pub fn current(&mut self, write: bool) -> &mut Self {
        self.current = write;
        self
    }
    pub fn total(&mut self, write: bool) -> &mut Self {
        self.total = write;
        self
    }
    pub fn message(&mut self, write: bool) -> &mut Self {
        self.message = write;
        self
    }
    pub fn path(&mut self, write: bool) -> &mut Self {
        self.path = write;
        self
    }
    pub fn separator(&mut self, sep: impl Into<String>) -> &mut Self {
        self.separator = sep.into();
        self
    }

    pub fn write(&self, status: &StatusEvent) -> String {
        let mut writer = StatusLayoutWriter {
            config: self,
            parts: Vec::new(),
        };
        writer.write(status);
        writer.parts.join(&self.separator)
    }
}

impl Default for StatusFormatConfig {
    fn default() -> Self {
        Self {
            stage: true,
            current: true,
            total: true,
            message: true,
            path: true,
            separator: " | ".to_string(),
        }
    }
}

struct StatusLayoutWriter<'a> {
    config: &'a StatusFormatConfig,
    parts: Vec<String>,
}

impl<'a> StatusLayoutWriter<'a> {
    fn write(&mut self, status: &StatusEvent) {
        if self.config.stage {
            if let Some(v) = &status.stage() {
                self.parts.push(v.to_string());
            }
        }
        if self.config.current {
            if let Some(v) = status.current() {
                self.parts.push(v.to_string());
            }
        }
        if self.config.total {
            if let Some(v) = status.total() {
                self.parts.push(v.to_string());
            }
        }
        if self.config.message {
            if let Some(v) = &status.message() {
                self.parts.push(v.to_string());
            }
        }
        if self.config.path {
            if let Some(v) = &status.path() {
                self.parts.push(v.display().to_string());
            }
        }
    }
}
