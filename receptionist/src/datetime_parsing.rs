use chrono::{DateTime, Local};

pub fn parse_datetime_from_default_fmt(datetime: DateTime<Local>) -> String {
    datetime.format("%H:%M %d-%m-%Y").to_string()
}
