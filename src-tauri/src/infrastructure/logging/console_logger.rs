use crate::shared::logger::Logger;

/// Console logger — writes log output to stderr via `eprintln!`.
///
/// This is the **adapter** in ports-and-adapters terminology.
/// Replace with a `TracingLogger` or `LogLogger` later by implementing
/// the `Logger` trait against the desired backend.
#[derive(Clone)]
pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn error(&self, message: &str) {
        eprintln!("[ERROR] {}", message);
    }

    fn warn(&self, message: &str) {
        eprintln!("[WARN] {}", message);
    }

    fn info(&self, message: &str) {
        eprintln!("[INFO] {}", message);
    }
}
