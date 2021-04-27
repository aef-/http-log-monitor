use super::alerts::Alert;
use super::stats::SummaryStats;
use std::io::Error;
mod cli;

pub trait Display {
    fn summary_stats(&self, stats: SummaryStats) -> Result<(), Error>;
    fn alert(&self, alert: &Alert) -> Result<(), Error>;
}

pub fn get_display(name: &str) -> Box<dyn Display> {
    if name == "cli" {
        Box::new(cli::Cli {})
    } else {
        panic!("'{}' is not a valid display", name);
    }
}

#[cfg(test)]
mod tests {
    use super::get_display;

    #[test]
    fn it_should_return_cli_display() {
        get_display("cli");
    }

    #[test]
    #[should_panic(expected = "'fake-display' is not a valid display")]
    fn it_should_not_return_an_unsupported_display() {
        get_display("fake-display");
    }
}
