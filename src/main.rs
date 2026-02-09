use reqwest;
use chrono::Local;
use std::io::Write;
use std::thread;
use std::time::{Duration, Instant};
use colored::*;

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}

fn monitoring() {
    loop {
        clear_terminal();
        let urls = vec![
            "https://google.com",
            "https://crm.zoho.com",
            "https://coda.io",
            "https://datawarehouse.dbd.go.th/",
        ];

        println!("Rust Network Monitor with latency\n");

        for url in urls {
            let now = Local::now().format("%H:%M:%S"); // Formatted for cleaner logs
            let start = Instant::now();

            match reqwest::blocking::get(url) {
                Ok(resp) => {
                    let duration = start.elapsed();
                    
                    // Logic: Check from LONGEST to SHORTEST duration
                    let status = if duration >= Duration::from_millis(5000) {
                        "Down".truecolor(220, 20, 60).bold() // Crimson
                    } else if duration >= Duration::from_millis(3000) {
                        "Very Slow".red()
                    } else if duration >= Duration::from_millis(1000) {
                        "Slow".truecolor(255, 165, 0) // Orange
                    } else if duration >= Duration::from_millis(500) {
                        "Good".yellow()
                    } else {
                        "Excellent".green()
                    };

                    let response: ColoredString = if resp.status() == reqwest::StatusCode::OK {
                        resp.status().to_string().cyan().bold()
                    } else {
                        resp.status().to_string().red().bold()
                    };

                    let message = format!(
                        "Site: {} -> Status Code: {}, Result: {}, latency: {:.2?}",
                        url, response.to_string(), status.to_string(), duration
                    );
                    write_log(&message);
                    println!("{}", message);
                }
                Err(err) => {
                    write_log(&format!("{} -> ERROR: {}", url, err));
                    println!("[{}] {} -> {}", now, url, "ERROR".red().bold());
                    println!("      └─ {}", err.to_string().truecolor(169, 169, 169)); // Dark Gray for error details
                }
            }
        }
        thread::sleep(Duration::from_secs(10));
        write_log("\n");
    }
}

fn write_log(message: &str) {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("network_monitor.log")
        .unwrap();
        
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    file.write_all(format!("[{}] {}\n", timestamp, message).as_bytes()).unwrap();
    file.sync_all().unwrap();
}
fn main() {
    monitoring();
}