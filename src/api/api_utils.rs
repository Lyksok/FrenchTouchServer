use chrono::Local;
use std::fmt::Debug;

pub fn print_log<T: Debug>(log_type: &str, entity_type: &str, entity: &T) {
    println!(
        "{} [{}] {}: {:?}",
        Local::now().format("%H:%M"),
        log_type,
        entity_type,
        entity
    );
}
