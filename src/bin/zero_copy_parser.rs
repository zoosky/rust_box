pub struct ParsedData<'a> {
    pub header: u8,
    pub payload: &'a str,
}

impl ParsedData<'_> {
    pub fn parse(data: &[u8]) -> ParsedData {
        let header = data[0];

        let payload = std::str::from_utf8(&data[1..data.len()]).unwrap();

        ParsedData { header, payload }
    }
}

pub fn get_data() -> Vec<u8> {
    const DATA: [u8; 5] = [255, 't' as u8, 'e' as u8, 's' as u8, 't' as u8];
    DATA.to_vec() // Return dynamically allocated array (Vector)
}

fn main() {
    // Simulate getting data from somewhere else (Ex: Socket) (Rust allows us to return a object)
    let buffer = get_data();

    // Parse buffer into ParsedData struct
    let parsed_data = ParsedData::parse(&buffer);

    // Print payload content
    println!("{}", parsed_data.payload);
}

#[cfg(test)]
mod benchmarks {
    //#![feature(test)] // Enable the `test` feature

    //extern crate test; // Import the test crate

    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    // Import the code you want to benchmark
    use crate::ParsedData;

    fn parse_benchmark(c: &mut Criterion) {
        // Prepare your input data (e.g., a buffer)
        let data = crate::get_data();

        c.bench_function("parse", |b| {
            // Define the benchmarking code here
            b.iter(|| {
                let parsed_data = ParsedData::parse(black_box(&data)); // Use black_box to prevent optimization
                                                                       // You can add assertions here to validate the results if needed
            });
        });
    }

    criterion_group!(benches, parse_benchmark); // Group the benchmarks
    criterion_main!(benches); // Run the benchmarks
}
