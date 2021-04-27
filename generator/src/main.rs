use chrono::prelude::Utc;
use processor::Record;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::error::Error;
use std::io;
use std::{thread, time};

fn main() -> Result<(), Box<dyn Error>> {
    let hosts = ["10.0.0.1", "10.0.0.2", "10.0.0.3", "10.0.0.4", "10.0.0.5"];
    let http_methods = ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD"];
    let sections = ["/api", "/report", "/user", "/", "/123-abc"];
    let status = [200, 204, 301, 400, 401, 403, 404, 500];

    let mut rng = thread_rng();

    let mut wtr = csv::WriterBuilder::new().from_writer(io::stdout());

    for n in 1..100_000_000 {
        let section = sections.choose(&mut rng).unwrap().to_string();
        let http_method = http_methods.choose(&mut rng).unwrap().to_string();
        let record = Record {
            remotehost: hosts.choose(&mut rng).unwrap().to_string(),
            rfc931: "-".to_owned(),
            authuser: "apache".to_owned(),
            date: Utc::now().timestamp(),
            bytes: rng.gen_range(0..99999),
            status: *status.choose(&mut rng).unwrap(),
            request: format!("{} {} HTTP/1.0", http_method, section),
        };
        wtr.serialize(record)?;

        if n % 10 == 0 {
            wtr.flush()?;
        }

        let duration = time::Duration::from_millis(rng.gen_range(0..300));
        thread::sleep(duration);
    }

    Ok(())
}
