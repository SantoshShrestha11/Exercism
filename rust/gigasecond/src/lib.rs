use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.replace_second(1000000000);
    todo!("What time is a gigasecond later than {start}");
}
