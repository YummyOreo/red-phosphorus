use std::time::Duration;

pub fn to_ticks(time: Duration) -> u128 {
    let milis = time.as_millis();
    milis / 50
}
