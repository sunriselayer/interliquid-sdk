use std::ops::{Add, Sub};
use std::time::Duration;

use borsh_derive::{BorshDeserialize, BorshSerialize};

/// Unix seconds
#[derive(Clone, Copy, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Timestamp(u64);

impl Timestamp {
    pub fn new(unix_seconds: u64) -> Self {
        Self(unix_seconds)
    }

    pub fn as_secs(&self) -> u64 {
        self.0
    }
}

impl Add<Duration> for Timestamp {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        Self(self.0 + duration.as_secs())
    }
}

impl Add<Duration> for &Timestamp {
    type Output = Timestamp;

    fn add(self, duration: Duration) -> Self::Output {
        Timestamp(self.0 + duration.as_secs())
    }
}

impl Sub<Duration> for Timestamp {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        Self(self.0.saturating_sub(duration.as_secs()))
    }
}

impl Sub<Duration> for &Timestamp {
    type Output = Timestamp;

    fn sub(self, duration: Duration) -> Self::Output {
        Timestamp(self.0.saturating_sub(duration.as_secs()))
    }
}

impl Sub<Timestamp> for Timestamp {
    type Output = Duration;

    fn sub(self, other: Timestamp) -> Self::Output {
        Duration::from_secs(self.0.saturating_sub(other.0))
    }
}

impl Sub<&Timestamp> for Timestamp {
    type Output = Duration;

    fn sub(self, other: &Timestamp) -> Self::Output {
        Duration::from_secs(self.0.saturating_sub(other.0))
    }
}

impl Sub<Timestamp> for &Timestamp {
    type Output = Duration;

    fn sub(self, other: Timestamp) -> Self::Output {
        Duration::from_secs(self.0.saturating_sub(other.0))
    }
}

impl Sub<&Timestamp> for &Timestamp {
    type Output = Duration;

    fn sub(self, other: &Timestamp) -> Self::Output {
        Duration::from_secs(self.0.saturating_sub(other.0))
    }
}
