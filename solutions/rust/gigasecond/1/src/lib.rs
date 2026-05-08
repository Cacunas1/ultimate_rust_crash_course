use time::PrimitiveDateTime as DateTime;
use time;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let delta: i64 = 1_000 * 1_000 *1_000;
    start + time::Duration::seconds(delta)
}
