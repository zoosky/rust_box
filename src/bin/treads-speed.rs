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

fn main() {
    let mut handles = Vec::new();
    let start = Instant::now();

    print!("First option");
    let first_option = Instant::now();

    for i in 0..10 {
        let handle = do_work(i);
        handles.push(handle);
    }

    for handle in handles {
        handle.join();
    }
    let opt1_elapsed = first_option.elapsed();
    println!("first_option, elapsed time: {:.2?}", opt1_elapsed);

    print!("Second option");
    let second_option = Instant::now(); // Create new Instant object for each segment

    (0..10).map(do_work).for_each(|handle| {
        handle.join();
    });
    let opt2_elapsed = second_option.elapsed();
    println!("second_option, elapsed time: {:.2?}", opt2_elapsed);


    print!("Third option");
    let third_option = Instant::now();
    let handles: Vec<_> = (0..10).map(do_work).collect();
    //  for_each would be much cleaner as a simple for loop
    handles.into_iter().for_each(move |handle| {
        handle.join();
    });
    let opt3_elapsed = third_option.elapsed();
    println!("third_option,  elapsed time: {:.2?}", opt3_elapsed);

    let elapsed = start.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);
}
