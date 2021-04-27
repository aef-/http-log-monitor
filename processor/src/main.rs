//! An HTTP log monitoring tool.
//!
//! Provides a summary information and alerts of HTTP traffic.

#[macro_use]
extern crate assert_matches;
use clap::{AppSettings, Clap};
use csv::Reader;
use processor::Record;
use std::collections::VecDeque;
use std::error::Error;
use std::io;
use std::process;

pub mod alerts;
pub mod display;
pub mod stats;

use alerts::AlertHandler;
use display::get_display;

#[derive(Clap)]
#[clap(version = "1.0", author = "Adrian F. <aef@fastmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long, default_value = "cli")]
    display: String,
    #[clap(
        short,
        long,
        default_value = "10",
        about = "Summarize logs for every N seconds of log lines"
    )]
    summary_cadence: i64,
    #[clap(
        short,
        long,
        default_value = "10",
        about = "Avg. requests per second to trigger an alert."
    )]
    high_alert_seconds_per_request: i64,
}

fn process_logs(opts: Opts) -> Result<(), Box<dyn Error>> {
    let display = get_display(&opts.display);
    let high_alert_threshold = opts.high_alert_seconds_per_request;
    let summary_cadence: i64 = opts.summary_cadence;

    let mut interval_buffer: VecDeque<Record> = VecDeque::new();
    let mut alert_handler = AlertHandler::new(2 * 60, high_alert_threshold);

    let mut reader = Reader::from_reader(io::stdin());
    for result in reader.deserialize() {
        let record: Record = result?;
        if !interval_buffer.is_empty() {
            let start_time = interval_buffer[0].date;
            let curr_time = record.date;
            if curr_time > start_time && curr_time - start_time > summary_cadence {
                display.summary_stats(stats::build_summary_stats(&interval_buffer))?;
                interval_buffer.clear();
            }
        }
        if let Some(ref alert) = alert_handler.current_alert {
            display.alert(alert)?;
        }

        interval_buffer.push_back(record.to_owned());
        alert_handler.new_record(record.date);
    }
    Ok(())
}

fn main() {
    let opts: Opts = Opts::parse();

    if let Err(err) = process_logs(opts) {
        println!("Error processing logs: {}", err);
        process::exit(1);
    }
}
