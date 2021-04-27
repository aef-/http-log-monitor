use super::{Display, SummaryStats};
use crate::alerts::{Alert, Status};
use chrono::prelude::{TimeZone, Utc};
use std::io;
use std::io::Error;

pub struct Cli {}
impl Display for Cli {
    fn summary_stats(&self, stats: SummaryStats) -> Result<(), Error> {
        summary_stats_output(&mut io::stdout(), stats)
    }
    fn alert(&self, alert: &Alert) -> std::result::Result<(), Error> {
        alert_output(&mut io::stdout(), alert)
    }
}

fn alert_output(stdout: &mut dyn io::Write, alert: &Alert) -> Result<(), Error> {
    match alert {
        Alert::HighTraffic(Status::Start(time)) => writeln!(
            stdout,
            "High traffic generated alert - hits = {}, triggered at {}",
            1, time
        ),
        Alert::HighTraffic(Status::End(time)) => writeln!(
            stdout,
            "Recovered from high traffic alert, triggered at {}",
            time
        ),
        _ => Ok(()),
    }
}
fn summary_stats_output(stdout: &mut dyn io::Write, stats: SummaryStats) -> Result<(), Error> {
    writeln!(
        stdout,
        "==== {} | {}s ====",
        Utc.timestamp(stats.from_date, 0),
        stats.to_date - stats.from_date
    )?;

    let mut http_errors: Vec<_> = stats.http_errors.iter().collect();
    http_errors.sort_by(|(_, count_a), (_, count_b)| count_b.partial_cmp(count_a).unwrap());
    for (error, &count) in http_errors {
        writeln!(
            stdout,
            "{} {} {:.0}%",
            error,
            count,
            (count as f32 / stats.total_hits as f32) * 100.0
        )?;
    }

    let mut section_hits: Vec<_> = stats.section_hits.iter().collect();
    section_hits.sort_by(|(_, count_a), (_, count_b)| count_b.partial_cmp(count_a).unwrap());
    for (http_method, &count) in section_hits.iter().take(3) {
        writeln!(
            stdout,
            "/{} {} {:.0}%",
            http_method,
            count,
            (count as f32 / stats.total_hits as f32) * 100.0
        )?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use processor::Record;
    use std::collections::{HashMap, VecDeque};

    #[test]
    fn it_should_print_high_alert() {
        let mut stdout: Vec<u8> = Vec::new();

        let _ = alert_output(&mut stdout, &Alert::HighTraffic(Status::Start(100)));
        println!("{:?}", stdout);
        assert_eq!(
            stdout,
            b"High traffic generated alert - hits = 1, triggered at 100\n"
        );

        stdout.clear();
        let _ = alert_output(&mut stdout, &Alert::HighTraffic(Status::End(5000)));
        println!("{:?}", stdout);
        assert_eq!(
            stdout,
            b"Recovered from high traffic alert, triggered at 5000\n"
        );
    }
    #[test]
    fn it_should_print_summary_stats() {
        let mut stdout: Vec<u8> = Vec::new();
        let mut records = VecDeque::new();
        let now = 1_500_000_000;
        let later = 1_600_000_000;

        let mut http_errors = HashMap::new();
        http_errors.insert(200, 2);
        http_errors.insert(400, 1);
        let mut section_hits = HashMap::new();
        section_hits.insert("/api".to_string(), 2);
        section_hits.insert("/".to_string(), 1);

        records.push_back(Record {
            remotehost: "0.0.0.1".to_string(),
            rfc931: "-".to_string(),
            authuser: "apache".to_string(),
            date: now,
            bytes: 9999,
            status: 200,
            request: "GET /api/user HTTP/1.0".to_string(),
        });
        records.push_back(Record {
            remotehost: "0.0.0.1".to_string(),
            rfc931: "-".to_string(),
            authuser: "apache".to_string(),
            date: now,
            bytes: 9999,
            status: 200,
            request: "GET /api/user HTTP/1.0".to_string(),
        });
        records.push_back(Record {
            remotehost: "0.0.0.1".to_string(),
            rfc931: "-".to_string(),
            authuser: "apache".to_string(),
            date: now,
            bytes: 9999,
            status: 400,
            request: "GET /api/user HTTP/1.0".to_string(),
        });
        let _results = summary_stats_output(
            &mut stdout,
            SummaryStats {
                from_date: now,
                to_date: later,
                total_hits: 3,
                section_hits,
                http_errors,
            },
        );
        assert_eq!(stdout, b"==== 2017-07-14 02:40:00 UTC | 100000000s ====\n200 2 67%\n400 1 33%\n//api 2 67%\n// 1 33%\n");
    }
}
