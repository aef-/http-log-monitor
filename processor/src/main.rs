//! An HTTP log monitoring tool.
//!
//! Provides a summary information and alerts of HTTP traffic.

#[macro_use] extern crate assert_matches;
use chrono::prelude::Utc;
use clap::{AppSettings, Clap};
use csv::Reader;
use std::collections::VecDeque;
use std::error::Error;
use std::io;
use std::process;
use processor::{Record, Date};

pub mod display;
pub mod stats;

use display::{get_display, Alert, Status};

const ALERT_THRESHOLD: i64 = 2 * 60; // two minutes

#[derive(Clap)]
#[clap(version = "1.0", author = "Adrian F. <aef@fastmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long, default_value = "cli")]
    display: String,
    #[clap(short, long, default_value = "10", about = "Summarize logs for every N seconds of log lines")]
    summary_cadence: i64,
    #[clap(short, long, default_value = "10", about = "Avg. requests per second to trigger an alert.")]
    high_alert_seconds_per_request: i64,
}

fn remove_records_by_ttl(buffer: &mut VecDeque<Date>, ttl: i64) {
    let now = Utc::now().timestamp();
    let start = now - ttl;

    while let Some(date) = buffer.front() {
        if *date < start {
            buffer.pop_front();
        } else {
            break;
        }
    }
}

fn process_logs(opts: Opts) -> Result<(), Box<dyn Error>> {
    let display = get_display(&opts.display);
    let high_alert_threshold = opts.high_alert_seconds_per_request;
    let summary_cadence: i64 = opts.summary_cadence;

    let mut interval_buffer: VecDeque<Record> = VecDeque::new();
    let mut alert_buffer: VecDeque<Date> = VecDeque::new();
    let mut current_alert: Option<Alert> = None;

    let mut reader = Reader::from_reader(io::stdin());
    for result in reader.deserialize() {
        let record: Record = result?;
        if interval_buffer.len() > 0 {
            let start_time = interval_buffer[0].date;
            let curr_time = record.date;
            if curr_time > start_time && curr_time - start_time > summary_cadence {
                display.summary_stats(stats::build_summary_stats(&interval_buffer))?;
                interval_buffer.clear();
            }
        }
        remove_records_by_ttl(&mut alert_buffer, ALERT_THRESHOLD);

        let requests_per_second = alert_buffer.len() as f64 / (ALERT_THRESHOLD as f64);
        if requests_per_second > high_alert_threshold as f64 {
            match current_alert {
                Some(Alert::HighTraffic(Status::Start(_))) => {
                    current_alert = Some(Alert::HighTraffic(Status::InProgress));
                },
                Some(_) => (),
                None => {
                    current_alert = Some(Alert::HighTraffic(Status::Start(record.date)));
                }
            }
        } else {
            match current_alert {
                Some(Alert::HighTraffic(Status::Start(_))) => {
                    current_alert = Some(Alert::HighTraffic(Status::End(record.date)));
                },
                Some(Alert::HighTraffic(Status::InProgress)) => {
                    current_alert = Some(Alert::HighTraffic(Status::End(record.date)));
                },
                Some(Alert::HighTraffic(Status::End(_))) => {
                    current_alert = None;
                }
                None => ()
            }
        }

        if let Some(ref alert) = current_alert {
            display.alert(alert);
        }

        interval_buffer.push_back(record.to_owned());
        alert_buffer.push_back(record.date);
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_removes_expired_records() {
        //let mut buf: VecDecque<Date> = VecDeque::new();
        /*
        buf.push_back(3);
        buf.push_back(4);
        buf.push_back(5);

        remove_records_by_ttl(buffer: &mut VecDeque<Date>, ttl: i64)
        assert_eq!(2 + 2, 4);
        */
    }
}
