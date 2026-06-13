/// Logger port — abstraction over logging.
///
/// This is the **port** in ports-and-adapters terminology.
/// The infrastructure layer provides concrete implementations
/// (e.g., `ConsoleLogger`, and later `TracingLogger` or `LogLogger`).
pub trait Logger: Send + Sync {
    fn error(&self, message: &str);
    fn warn(&self, message: &str);
    fn info(&self, message: &str);
}
