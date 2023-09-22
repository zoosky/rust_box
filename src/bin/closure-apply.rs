fn apply<F, D, R>(f: F, d: D) -> R
where
    F: Fn(D) -> R,
{
    f(d)
}

fn main() {
    let add_one = |x: i32| x + 1;
    let result = apply(add_one, 5);
    assert_eq!(result, 6);

    let to_uppercase = |s: String| s.to_uppercase();
    let result = apply(to_uppercase, "hello".to_string());
    assert_eq!(result, "HELLO");
}
