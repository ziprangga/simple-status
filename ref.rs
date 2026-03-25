// pub struct StatusFormatConfig {
//     pub parts: Vec<Box<dyn Fn(&StatusEvent) -> Option<String>>>,
//     pub separator: String,
// }

// impl StatusFormatConfig {
//     pub fn new() -> Self {
//         Self {
//             parts: Vec::new(),
//             separator: " | ".to_string(),
//         }
//     }

//     pub fn stage<F: 'static + Fn(&str) -> String>(&mut self, fmt: F) -> &mut Self {
//         self.parts
//             .push(Box::new(move |s: &StatusEvent| s.stage().map(|v| fmt(v))));
//         self
//     }

//     pub fn current<F: 'static + Fn(usize) -> String>(&mut self, fmt: F) -> &mut Self {
//         self.parts
//             .push(Box::new(move |s: &StatusEvent| s.current().map(|v| fmt(v))));
//         self
//     }

//     pub fn total<F: 'static + Fn(usize) -> String>(&mut self, fmt: F) -> &mut Self {
//         self.parts
//             .push(Box::new(move |s: &StatusEvent| s.total().map(|v| fmt(v))));
//         self
//     }

//     pub fn message<F: 'static + Fn(&str) -> String>(&mut self, fmt: F) -> &mut Self {
//         self.parts
//             .push(Box::new(move |s: &StatusEvent| s.message().map(|v| fmt(v))));
//         self
//     }

//     pub fn path<F: 'static + Fn(&std::path::Path) -> String>(&mut self, fmt: F) -> &mut Self {
//         self.parts
//             .push(Box::new(move |s: &StatusEvent| s.path().map(|v| fmt(v))));
//         self
//     }

//     pub fn separator(&mut self, sep: impl Into<String>) -> &mut Self {
//         self.separator = sep.into();
//         self
//     }

//     pub fn write(&self, status: &StatusEvent) -> String {
//         self.parts
//             .iter()
//             .filter_map(|f| f(status))
//             .collect::<Vec<_>>()
//             .join(&self.separator)
//     }
// }

// impl Default for StatusFormatConfig {
//     fn default() -> Self {
//         Self::new()
//             .stage(|v| v.to_string())
//             .current(|v| v.to_string())
//             .total(|v| v.to_string())
//             .message(|v| v.to_string())
//             .path(|v| v.display().to_string())
//     }
// }

// pub fn format<F>(&self, f: F) -> String
//     where
//         F: Fn(&StatusEvent) -> String,
//     {
//         f(&self.event)
//     }
