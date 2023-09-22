fn main() {
    let data = 42;

    trait MyFunction<D, R> {
        fn call(&self, data: D) -> R;
    }

    // Implement the trait for a specific function
    struct MyFunctionImpl;
    impl MyFunction<i32, i32> for MyFunctionImpl {
        fn call(&self, data: i32) -> i32 {
            data * 2
        }
    }

    // Create an instance of the trait object
    let my_function: Box<dyn MyFunction<i32, i32>> = Box::new(MyFunctionImpl);

    // Define a closure that takes a trait object
    let my_closure = |func: &dyn MyFunction<i32, i32>, data: i32| -> i32 { func.call(data) };

    // Call the closure with the trait object and data
    let result = my_closure(&*my_function, data);

    println!("Result: {}", result);
}
