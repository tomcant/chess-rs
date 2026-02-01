use std::time::Duration;

pub enum TimeLimit {
    Dynamic { soft: Duration, hard: Duration },
    Fixed(Duration),
}

impl TimeLimit {
    pub fn fixed(duration: Duration) -> Self {
        Self::Fixed(duration)
    }

    //
    // Calculate time budget based on time left and increment per move.
    //
    //   - Compute a "reserve" (minimum time to always keep) as max(time left / 20, 50ms).
    //   - Subtract reserve from time left to get max time that can be used for this move.
    //   - Soft limit: min(time left / 30 + increment * 3/4, max time).
    //   - Hard limit: min(soft * 3, max time).
    //
    pub fn dynamic(time_left: Duration, time_inc: Option<Duration>) -> Self {
        if time_left.as_millis() == 0 {
            return Self::Fixed(time_left);
        }

        let reserve = (time_left / 20).max(Duration::from_millis(50));
        let max_time = time_left.saturating_sub(reserve);

        let soft = (time_left / 30 + time_inc.unwrap_or_default() * 3 / 4).min(max_time);
        let hard = (soft * 3).min(max_time);

        Self::Dynamic { soft, hard }
    }

    pub fn hard(&self) -> Duration {
        match self {
            Self::Dynamic { hard, .. } => *hard,
            Self::Fixed(duration) => *duration,
        }
    }
}
