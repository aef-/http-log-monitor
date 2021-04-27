use chrono::prelude::Utc;
use std::collections::VecDeque;

type Date = i64;

#[derive(Debug)]
pub enum Status {
    Start(Date),
    InProgress,
    End(Date),
}
#[derive(Debug)]
pub enum Alert {
    HighTraffic(Status),
}

pub struct AlertHandler {
    buffer: VecDeque<Date>,
    pub current_alert: Option<Alert>,
    // TODO turn options into struct
    high_alert_threshold: i64,
    alert_ttl: i64, // in seconds
}

impl AlertHandler {
    pub fn new(alert_ttl: i64, high_alert_threshold: i64) -> AlertHandler {
        AlertHandler {
            buffer: VecDeque::new(),
            current_alert: None,
            high_alert_threshold,
            alert_ttl,
        }
    }

    pub fn new_record(&mut self, date: Date) {
        self.remove_records_by_ttl(self.alert_ttl);
        let alert_buffer = &self.buffer;
        let requests_per_second = alert_buffer.len() as f64 / (self.alert_ttl as f64);
        println!("{:?} {:?}", alert_buffer, requests_per_second);
        if requests_per_second >= self.high_alert_threshold as f64 {
            match self.current_alert {
                Some(Alert::HighTraffic(Status::Start(_))) => {
                    self.current_alert = Some(Alert::HighTraffic(Status::InProgress));
                }
                Some(_) => (),
                None => {
                    self.current_alert = Some(Alert::HighTraffic(Status::Start(date)));
                }
            }
        } else {
            match self.current_alert {
                Some(Alert::HighTraffic(Status::Start(_))) => {
                    self.current_alert = Some(Alert::HighTraffic(Status::End(date)));
                }
                Some(Alert::HighTraffic(Status::InProgress)) => {
                    self.current_alert = Some(Alert::HighTraffic(Status::End(date)));
                }
                Some(Alert::HighTraffic(Status::End(_))) => {
                    self.current_alert = None;
                }
                None => (),
            }
        };
        self.buffer.push_back(date);
    }
    fn remove_records_by_ttl(&mut self, ttl: i64) {
        let now = Utc::now().timestamp();
        let start = now - ttl;

        while let Some(date) = self.buffer.front() {
            if *date < start {
                self.buffer.pop_front();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_removes_expired_records() {
        let mut buf: VecDeque<Date> = VecDeque::new();
        let now = Utc::now().timestamp();
        buf.push_back(now - 10000);
        buf.push_back(now - 1000);
        buf.push_back(now - 500);
        buf.push_back(now - 100);
        buf.push_back(now - 50);
        buf.push_back(now);

        let mut handler = AlertHandler {
            buffer: buf,
            current_alert: None,
            high_alert_threshold: 2,
            alert_ttl: 500,
        };

        handler.remove_records_by_ttl(500);
        assert_eq!(handler.buffer.len(), 4);
    }

    #[test]
    fn it_does_not_trigger_alert() {
        let mut buf: VecDeque<Date> = VecDeque::new();
        let now = Utc::now().timestamp();
        buf.push_back(now);
        buf.push_back(now);
        buf.push_back(now);
        buf.push_back(now);
        buf.push_back(now);
        buf.push_back(now);
        buf.push_back(now);

        let mut handler = AlertHandler {
            buffer: buf,
            current_alert: None,
            high_alert_threshold: 10,
            alert_ttl: 1,
        };

        handler.new_record(now);
        assert_matches!(handler.current_alert, None);
    }

    #[test]
    fn it_triggers_alert_on_high_threshold() {
        let mut handler = AlertHandler::new(1, 2);
        let now = Utc::now().timestamp();
        handler.new_record(now);
        assert_matches!(handler.current_alert, None);
        handler.new_record(now);
        assert_matches!(handler.current_alert, None);
        handler.new_record(now);
        assert_matches!(
            handler.current_alert,
            Some(Alert::HighTraffic(Status::Start(_now)))
        );
        handler.new_record(now);
        assert_matches!(
            handler.current_alert,
            Some(Alert::HighTraffic(Status::InProgress))
        );
        // TODO test End state...
    }
}
