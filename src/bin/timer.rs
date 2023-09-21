use std::time::{Duration, Instant};
use thousands::{digits, Separable, SeparatorPolicy};

fn timer<F, R>(func: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = func();
    let end = Instant::now();
    let elapsed_time = end.duration_since(start);
    (result, elapsed_time)
}

fn how_long() -> u128 {
    let mut sum: u128 = 0;
    for i in 1..=100_000_000 {
        sum += i;
    }
    sum
}

fn main() {
    let (sum, elapsed_time) = timer(|| {
        let sum = how_long();
        sum
    });

    println!("Sum: {}", format_number(sum));
    println!("Elapsed time: {:.2?}", format_duration(elapsed_time));
}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    format!(
        "{}.{:03} seconds, or {} nanoseconds.",
        secs,
        millis,
        duration.as_nanos()
    )
}

fn format_number(num: u128) -> String {
    let separator_policy = SeparatorPolicy {
        separator: "'",
        groups: &[3],
        digits: digits::ASCII_DECIMAL,
    };
    num.separate_by_policy(separator_policy)
}
