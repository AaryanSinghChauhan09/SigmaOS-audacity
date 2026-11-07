//! Options model.

#[derive(Debug, Clone, PartialEq)]
pub enum Mode { Default, Create(String, u32), Track(usize, String), Record(usize, f64), List }

#[derive(Debug, Clone)]
pub struct Options { pub mode: Mode }
impl Default for Options { fn default() -> Self { Self { mode: Mode::Default } } }
#[cfg(test)] mod tests { use super::*; #[test] fn o() { assert!(matches!(Options::default().mode, Mode::Default)); } }
