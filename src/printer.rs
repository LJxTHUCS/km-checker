/// Print test info to the output.
pub trait Printer {
    /// Print an info string to the output.
    fn print(&mut self, s: &str);
}

/// Stdout printer.
pub struct StdoutPrinter;

impl Printer for StdoutPrinter {
    fn print(&mut self, s: &str) {
        println!("{}", s);
    }
}