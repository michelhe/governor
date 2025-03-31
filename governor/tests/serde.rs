// #![cfg(feature = "serde")]

use std::time::Duration;

use governor::{
    clock::{Clock, FakeRelativeClock},
    state::InMemoryState,
    DefaultDirectRateLimiter, Quota, RateLimiter,
};
use nonzero_ext::nonzero;

#[test]
fn fake_clock_direct() {
    let clock = FakeRelativeClock::default();
    let lb = RateLimiter::direct_with_clock(Quota::per_second(nonzero!(2u32)), clock.clone());
    let ms = Duration::from_millis(1);

    // use up our burst capacity (2 in the first second):
    assert_eq!(Ok(()), lb.check(), "Now: {:?}", clock.now());
    clock.advance(ms);

    assert_eq!(Ok(()), lb.check(), "Now: {:?}", clock.now());
    clock.advance(ms);

    // Next should be not OK.
    assert!(lb.check().is_err());

    let json = serde_json::to_string(&lb).unwrap();
    // Deserialize - we need to specify the full type here
    let deserialized: RateLimiter<
        governor::state::NotKeyed,
        InMemoryState,
        FakeRelativeClock,
        governor::middleware::NoOpMiddleware<governor::nanos::Nanos>,
    > = serde_json::from_str(&json).unwrap();

    // Should be not OK.
    assert!(deserialized.check().is_err());

    // Advance the clock so we get enough tokens
    clock.advance(2000 * ms);
    let json = serde_json::to_string(&lb).unwrap();
    let deserialized: RateLimiter<
        governor::state::NotKeyed,
        InMemoryState,
        FakeRelativeClock,
        governor::middleware::NoOpMiddleware<governor::nanos::Nanos>,
    > = serde_json::from_str(&json).unwrap();

    // Should be OK.
    assert!(deserialized.check().is_err());
}

#[test]
fn default_clock_direct() {
    let lb: DefaultDirectRateLimiter =
        DefaultDirectRateLimiter::direct(Quota::per_second(nonzero!(2u32)));
    // Consume the burst capacity.
    assert_eq!(Ok(()), lb.check());
    assert_eq!(Ok(()), lb.check());

    // Note - with a real clock we might actually flake here if our process is not getting scheduled
    // for a while.

    let json = serde_json::to_string(&lb).unwrap();

    eprintln!("Serialized: {}", json);
    // Deserialize - we need to specify the full type here
    let deserialized: DefaultDirectRateLimiter = serde_json::from_str(&json).unwrap();

    // Should be not OK.
    assert!(deserialized.check().is_err());
}

#[test]
fn quanta_clock_direct() {
    let quanta_clock = governor::clock::QuantaClock::default();
    let lb = RateLimiter::direct_with_clock(Quota::per_second(nonzero!(2u32)), quanta_clock);
    // Consume the burst capacity.
    assert_eq!(Ok(()), lb.check());
    assert_eq!(Ok(()), lb.check());

    // Note - with a real clock we might actually flake here if our process is not getting scheduled
    // for a while.

    let json = serde_json::to_string(&lb).unwrap();

    eprintln!("Serialized: {}", json);
    // Deserialize - we need to specify the full type here
    let deserialized: DefaultDirectRateLimiter = serde_json::from_str(&json).unwrap();

    // Should be not OK.
    assert!(deserialized.check().is_err());
}

#[test]
fn fake_clock_keyed_dashmap() {
    let clock = FakeRelativeClock::default();
    let lb = RateLimiter::dashmap_with_clock(Quota::per_second(nonzero!(2u32)), clock.clone());
    let ms = Duration::from_millis(1);

    // use up our burst capacity (2 in the first second):
    assert_eq!(Ok(()), lb.check_key(&1), "Now: {:?}", clock.now());
    clock.advance(ms);
    assert_eq!(Ok(()), lb.check_key(&1), "Now: {:?}", clock.now());
    clock.advance(ms);

    let json = serde_json::to_string(&lb).unwrap();
    // Deserialize - we need to specify the full type here
    let deserialized: RateLimiter<
        i32,
        dashmap::DashMap<i32, InMemoryState>,
        FakeRelativeClock,
        governor::middleware::NoOpMiddleware<governor::nanos::Nanos>,
    > = serde_json::from_str(&json).unwrap();

    // Should be not OK.
    assert!(deserialized.check_key(&1).is_err());
}

#[test]
fn fake_clock_keyed_hashmap() {
    let clock = FakeRelativeClock::default();
    let lb = RateLimiter::hashmap_with_clock(Quota::per_second(nonzero!(2u32)), clock.clone());
    let ms = Duration::from_millis(1);

    // use up our burst capacity (2 in the first second):
    assert_eq!(Ok(()), lb.check_key(&1), "Now: {:?}", clock.now());
    clock.advance(ms);
    assert_eq!(Ok(()), lb.check_key(&1), "Now: {:?}", clock.now());
    clock.advance(ms);

    let json = serde_json::to_string(&lb).unwrap();
    // Deserialize - we need to specify the full type here
    let deserialized: RateLimiter<
        i32,
        dashmap::DashMap<i32, InMemoryState>,
        FakeRelativeClock,
        governor::middleware::NoOpMiddleware<governor::nanos::Nanos>,
    > = serde_json::from_str(&json).unwrap();

    // Should be not OK.
    assert!(deserialized.check_key(&1).is_err());
}
