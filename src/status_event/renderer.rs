use crate::status_event::StatusEvent;
use crate::status_event::StatusEventRenderer;

pub struct DefaultRenderer;

impl StatusEventRenderer for DefaultRenderer {
    type Output = String;

    fn render(&self, se: &StatusEvent) -> String {
        let mut out = String::new();

        if let Some(msg) = se.message() {
            out.push('[');
            out.push_str(msg);
            out.push_str("] ");
        }

        if let Some(action) = se.event().action() {
            out.push('[');
            out.push_str(action);
            out.push_str("] ");
        }

        match (se.event().current(), se.event().total()) {
            (Some(current), Some(total)) => {
                out.push_str(&format!("[{current}/{total}] "));
            }
            (Some(current), None) => {
                out.push_str(&format!("[{current}] "));
            }
            (None, Some(total)) => {
                out.push_str(&format!("[/{total}] "));
            }
            (None, None) => {}
        }

        if let Some(path) = se.path() {
            out.push_str(" | path=");
            out.push_str(&path.display().to_string());
        }

        out
    }
}

impl std::fmt::Display for StatusEvent {
    // Note:
    // Formatting order is fixed:
    //
    //     [action] current/total message path
    //
    // Missing fields are skipped completely.
    //
    // Keep this implementation allocation-free. Avoid building intermediate
    // strings unless necessary.
    // fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //     let e = &self.event();
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = DefaultRenderer.render(self);
        f.write_str(&text)
    }
}
