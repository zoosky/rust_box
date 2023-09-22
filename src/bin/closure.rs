fn main() {
    let data = 42;
    // Define the types D and R
    type D = i32;
    type R = i32;

    // Define a closure that takes a function F and data D as parameters
    let my_closure = |func: fn(D) -> R, data: D| -> R { func(data) };

    // Define a function that we can pass to the closure
    fn my_function(x: i32) -> i32 {
        x * 2
    }

    // Call the closure with the function and data
    let result = my_closure(my_function, data);

    println!("Result: {}", result);
}
