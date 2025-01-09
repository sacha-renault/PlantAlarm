use chrono::{Local, NaiveDateTime, ParseResult};

pub fn parse_date_str(date_str: String) -> ParseResult<NaiveDateTime> {
    NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S")
}

/// Parse an optional date string as NaiveDateTime, else Local::now()
pub fn parse_option_date_string(opt_date_str: Option<String>) -> ParseResult<NaiveDateTime> {
    // Parse the date string or use today's date if None
    match opt_date_str {
        Some(date_str) => parse_date_str(date_str),
        None => {
            // Use current local time
            ParseResult::Ok(Local::now().naive_local())
        }
    }
}

pub fn serialize_naive_datetime<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&date.format("%Y-%m-%d %H:%M:%S").to_string())
}
