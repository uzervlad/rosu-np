use std::{collections::HashMap, time::{Duration, Instant}};

pub struct Ratelimiter {
  period: Duration,
  limits: HashMap<String, Instant>,
}

impl Ratelimiter {
  pub fn new(period_seconds: u64) -> Self {
    Self {
      period: Duration::from_secs(period_seconds),
      limits: HashMap::default(),
    }
  }

  pub fn trigger(&mut self, key: String) -> bool {
    match self.limits.get(&key) {
      Some(t) => {
        if t.elapsed() > self.period {
          self.limits.insert(key, Instant::now());
          true
        } else {
          false
        }
      },
      None => {
        self.limits.insert(key, Instant::now());
        true
      }
    }
  }
}
