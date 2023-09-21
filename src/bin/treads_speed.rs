use std::thread;
use std::time;
use std::time::Duration;
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

fn time_to_run<F, D>(func: F) -> Duration
where
    F: FnOnce() -> D,
{
    let start = Instant::now();
    let _result = func();
    let end = Instant::now();
    end.duration_since(start)
}

fn main() {
    const NUM_THREADS: usize = 10;

    let start = Instant::now();

    print!("First option");
    let opt1_elapsed = time_to_run(|| run_op1(NUM_THREADS));
    println!("first_option, elapsed time: {:.2?}", opt1_elapsed);

    println!("Second option");
    let opt2_elapsed = time_to_run(|| run_opt2(NUM_THREADS));
    println!("second_option, elapsed time: {:.2?}", opt2_elapsed);

    println!("Third option");
    let opt3_elapsed = time_to_run(|| run_opt3(NUM_THREADS));
    println!("third_option,  elapsed time: {:.2?}", opt3_elapsed);

    let elapsed = start.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);
}
