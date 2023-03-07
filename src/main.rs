use std::{env, sync::{Arc, Mutex}, time::Instant};

/// generates vector of n random numbers in range (min, max)
fn generate_vec(n: usize, min: i32, max: i32) -> Vec<i32> {
    let mut vec = Vec::with_capacity(n);
    for _ in 0..n {
        vec.push(rand::random::<i32>() % (max - min) + min);
    }
    vec
}

// implementing bogosort with multithreading support
fn main() {
    let n = 11;

    let items = generate_vec(n, 0, 100);
    println!("Unsorted: {:?}", items);

    let mut sorted = items.clone();
    sorted.sort();

    let mut handles = Vec::new();

    let found = Arc::new(Mutex::new(false));

    let start_time = Instant::now();

    for _ in 0..8 {
        let items = items.clone();
        let found = found.clone();
        let sorted = sorted.clone();
        handles.push(std::thread::spawn(move || {
            let mut shuffled = items.clone();
            loop {
                fastrand::shuffle(&mut shuffled);
                if shuffled == sorted {
                    let mut found = found.lock().unwrap();
                    *found = true;
                    return Some(shuffled);
                } else if *found.lock().unwrap() {
                    return None;
                }
            }
        }));
    }

    // get the first found sorted vector
    let sorted = handles.into_iter().find_map(|h| h.join().unwrap());

    println!("Sorted: {:?}", sorted);
    println!("Time: {}s", start_time.elapsed().as_secs());
}
