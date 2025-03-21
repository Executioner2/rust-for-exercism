use time::{PrimitiveDateTime as DateTime};
use time::ext::NumericalDuration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.saturating_add(1_000_000_000.seconds())
}
