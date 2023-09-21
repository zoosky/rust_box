use std::thread;
use std::time;
use std::time::Instant;

fn do_work(i: usize) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let duration = time::Duration::from_millis(100);
        thread::sleep(duration);
        println!("thread {i} done");
    })
}

fn run_op1(num_threads: usize) {
    let mut handles = Vec::new();
    for i in 0..num_threads {
        let handle = do_work(i);
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Failed to join thread");
    }
}

fn run_opt2(num_threads: usize) {
    (0..num_threads).map(do_work).for_each(|handle| {
        handle.join().expect("Failed to join thread");
    });
}

fn run_opt3(num_threads: usize) {
    let handles: Vec<_> = (0..num_threads).map(do_work).collect();
    //  for_each would be much cleaner as a simple for loop
    handles.into_iter().for_each(move |handle| {
        handle.join().expect("Failed to join thread");
    });
}

fn main() {
    const NUM_THREADS: usize = 10;

    let start = Instant::now();

    print!("First option");
    let first_option = Instant::now();
    run_op1(NUM_THREADS);

    let opt1_elapsed = first_option.elapsed();
    println!("first_option, elapsed time: {:.2?}", opt1_elapsed);

    print!("Second option");
    let second_option = Instant::now(); // Create new Instant object for each segment

    run_opt2(NUM_THREADS);

    let opt2_elapsed = second_option.elapsed();
    println!("second_option, elapsed time: {:.2?}", opt2_elapsed);

    print!("Third option");
    let third_option = Instant::now();
    run_opt3(NUM_THREADS);
    let opt3_elapsed = third_option.elapsed();
    println!("third_option,  elapsed time: {:.2?}", opt3_elapsed);

    let elapsed = start.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);
}
