use std::time::Duration;

//
// Calculate how much time to allocate for the next move based on the time left
// and the increment per move.
//
//   - Compute a "reserve" (minimum time to always keep) as max(time left / 20, 50ms).
//   - Subtract reserve from time left to get max time that can be used for this move.
//   - Allocate time as (time left / 30 + increment / 2), but capped at the max time.
//
//   +-----------------+-----------+-----------+----------+-----------+-----------+
//   | Scenario        | time left | increment | reserve  | max time  | allocated |
//   +-----------------+-----------+-----------+----------+-----------+-----------+
//   | Rapid           | 600_000ms | 10_000ms  | 30_000ms | 570_000ms | 25_000ms  |
//   | Blitz           | 180_000ms | 2_000ms   | 9_000ms  | 171_000ms | 7_000ms   |
//   | Bullet          | 60_000ms  | 0ms       | 3_000ms  | 57_000ms  | 2_000ms   |
//   | Time scramble   | 5_000ms   | 500ms     | 250ms    | 4_750ms   | 416ms     |
//   | Long inc. (cap) | 90_000ms  | 200_000ms | 4_500ms  | 85_500ms  | 85_500ms  |
//   +-----------------+-----------+-----------+----------+-----------+-----------+
//
pub fn calculate_allocated_time(time_left: Duration, time_inc: Option<Duration>) -> Option<Duration> {
    if time_left.as_millis() == 0 {
        return None;
    }

    let reserve = (time_left / 20).max(Duration::from_millis(50));
    let max_time = time_left.saturating_sub(reserve);

    let allocated = (time_left / 30 + time_inc.unwrap_or_default() / 2).min(max_time);
    Some(Duration::from_millis(allocated.as_millis() as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rapid_time_allocation() {
        let time_left = Duration::from_millis(600_000);
        let increment = Some(Duration::from_millis(10_000));

        let allocated = calculate_allocated_time(time_left, increment);

        assert_eq!(allocated, Some(Duration::from_millis(25_000)));
    }

    #[test]
    fn blitz_time_allocation() {
        let time_left = Duration::from_millis(180_000);
        let increment = Some(Duration::from_millis(2_000));

        let allocated = calculate_allocated_time(time_left, increment);

        assert_eq!(allocated, Some(Duration::from_millis(7_000)));
    }

    #[test]
    fn bullet_time_allocation() {
        let time_left = Duration::from_millis(60_000);
        let increment = None;

        let allocated = calculate_allocated_time(time_left, increment);

        assert_eq!(allocated, Some(Duration::from_millis(2_000)));
    }

    #[test]
    fn time_scramble_allocation() {
        let time_left = Duration::from_millis(5_000);
        let increment = Some(Duration::from_millis(500));

        let allocated = calculate_allocated_time(time_left, increment);

        assert_eq!(allocated, Some(Duration::from_millis(416)));
    }

    #[test]
    fn long_increment_capped_allocation() {
        let time_left = Duration::from_millis(90_000);
        let increment = Some(Duration::from_millis(200_000));

        let allocated = calculate_allocated_time(time_left, increment);

        assert_eq!(allocated, Some(Duration::from_millis(85_500)));
    }

    #[test]
    fn zero_time_left() {
        let time_left = Duration::from_millis(0);
        let increment = Some(Duration::from_millis(1_000));

        let allocated = calculate_allocated_time(time_left, increment);

        assert_eq!(allocated, None);
    }
}
