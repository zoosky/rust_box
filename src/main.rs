fn main() {
    println!("Hello, Rust box!");

    for i in 0..14 {
        let result = mylib::fibonacci(i); // Replace 'mylib' with the actual name of your library
        println!("Fibonacci({}) = {}", i, result);
    }

    assert_eq!(mylib::fibonacci(8), 34);
}
