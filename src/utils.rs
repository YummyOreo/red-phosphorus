use std::time::Duration;

/// Assumes 20mspt
pub fn to_ticks(time: Duration) -> u128 {
    let milis = time.as_millis();
    milis / 50
}

/// Assumes 20mspt
pub fn from_ticks(ticks: u64) -> Duration {
    let milis = ticks * 50;
    Duration::from_millis(milis)
}
