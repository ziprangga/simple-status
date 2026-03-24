use std::path::PathBuf;
use std::sync::Arc;

pub trait StatusHandler: Send + Sync {
    fn handle_event(&self, event: StatusEvent);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Stage,
    Current,
    Total,
    Message,
    Path,
}

#[derive(Debug, Default, Clone)]
pub struct StatusEvent {
    pub stage: Option<String>,
    pub current: Option<usize>,
    pub total: Option<usize>,
    pub message: Option<String>,
    pub path: Option<PathBuf>,

    order: Vec<Field>,
    separator: String,
}

impl StatusEvent {
    pub fn new() -> Self {
        Self {
            separator: " ".to_string(),
            ..Default::default()
        }
    }

    pub fn with_stage(mut self, stage: impl Into<String>) -> Self {
        if self.stage.is_none() {
            self.order.push(Field::Stage);
        }
        self.stage = Some(stage.into());
        self
    }

    pub fn with_current(mut self, current: usize) -> Self {
        if self.current.is_none() {
            self.order.push(Field::Current);
        }
        self.current = Some(current);
        self
    }

    pub fn with_total(mut self, total: usize) -> Self {
        if self.total.is_none() {
            self.order.push(Field::Total);
        }
        self.total = Some(total);
        self
    }

    pub fn with_message(mut self, msg: impl Into<String>) -> Self {
        if self.message.is_none() {
            self.order.push(Field::Message);
        }
        self.message = Some(msg.into());
        self
    }

    pub fn with_path(mut self, path: PathBuf) -> Self {
        if self.path.is_none() {
            self.order.push(Field::Path);
        }
        self.path = Some(path);
        self
    }

    pub fn with_separator(mut self, sep: impl Into<String>) -> Self {
        self.separator = sep.into();
        self
    }
}

impl std::fmt::Display for StatusEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;

        for field in &self.order {
            let value: Option<String> = match field {
                Field::Stage => self.stage.clone(),
                Field::Message => self.message.clone(),
                Field::Current => self.current.map(|v| v.to_string()),
                Field::Total => self.total.map(|v| v.to_string()),
                Field::Path => self.path.as_ref().map(|p| p.display().to_string()),
            };

            if let Some(v) = value {
                if !first {
                    write!(f, "{}", self.separator)?;
                }
                write!(f, "{v}")?;
                first = false;
            }
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct StatusEmitter {
    handler: Arc<dyn StatusHandler>,
}

impl StatusEmitter {
    pub fn new(handler: Arc<dyn StatusHandler>) -> Self {
        Self { handler }
    }

    pub fn emit(&self, event: StatusEvent) {
        self.handler.handle_event(event);
    }
}
