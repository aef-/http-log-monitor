use crate::Record;
use std::collections::{HashMap, VecDeque};

const HTTP_ERROR_START: u16 = 400;

#[derive(Debug)]
pub struct SummaryStats {
    pub http_errors: HashMap<u16, usize>,
    pub section_hits: HashMap<String, usize>,
    pub total_hits: usize,
    pub from_date: i64,
    pub to_date: i64,
}

pub fn get_request_method_and_path_info(request_str: &String) -> (String, String, String) {
    let request: Vec<&str> = request_str.split_whitespace().collect();
    let http_method = request[0];
    let path: Vec<&str> = request[1].split('/').collect();
    return (
        http_method.to_string(),
        request[1].to_string(),
        path[1].to_string(),
    );
}

pub fn build_summary_stats(records: &VecDeque<Record>) -> SummaryStats {
    let mut section_hits: HashMap<String, usize> = HashMap::new();
    let mut http_errors: HashMap<u16, usize> = HashMap::new();

    for record in records {
        let (_http_method, _path, path_section) = get_request_method_and_path_info(&record.request);
        let hit = section_hits.entry(path_section).or_insert(0);
        *hit += 1;
        if record.status >= HTTP_ERROR_START {
            let error = http_errors.entry(record.status).or_insert(0);
            *error += 1;
        }
    }

    SummaryStats {
        section_hits,
        http_errors,
        total_hits: records.len(),
        from_date: records.front().unwrap().date,
        to_date: records.back().unwrap().date,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_request_method_and_path_info() {
        let results =
            get_request_method_and_path_info(&"DELETE /12345678/1/2 HTTP/1.0".to_string());
        let expected = (
            "DELETE".to_string(),
            "/12345678/1/2".to_string(),
            "12345678".to_string(),
        );
        assert_eq!(results.0, expected.0);
        assert_eq!(results.1, expected.1);
        assert_eq!(results.2, expected.2);

        let results = get_request_method_and_path_info(&"GET / HTTP/1.0".to_string());
        let expected = ("GET".to_string(), "/".to_string(), "".to_string());
        assert_eq!(results.0, expected.0);
        assert_eq!(results.1, expected.1);
        assert_eq!(results.2, expected.2);
    }

    #[test]
    fn it_builds_summary_stats() {
        let mut records = VecDeque::new();
        let now = 1_500_000_000;
        let _later = 1_600_000_000;

        let mut http_errors = HashMap::new();
        http_errors.insert(200, 2);
        http_errors.insert(400, 1);
        http_errors.insert(500, 1);
        let mut section_hits = HashMap::new();
        section_hits.insert("/api".to_string(), 3);
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
        records.push_back(Record {
            remotehost: "0.0.0.1".to_string(),
            rfc931: "-".to_string(),
            authuser: "apache".to_string(),
            date: now,
            bytes: 9999,
            status: 500,
            request: "GET / HTTP/1.0".to_string(),
        });
        assert_matches!(
            build_summary_stats(&records),
            SummaryStats {
                from_date: _now,
                to_date: _later,
                total_hits: 4,
                section_hits: _section_hits,
                http_errors: _http_errors,
            }
        );
    }
}
