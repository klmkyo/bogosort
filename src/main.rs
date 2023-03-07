use std::{env, sync::{Arc, Mutex}, time::Instant};

/// generates vector of n random numbers in range (min, max)
fn generate_vec(n: usize, min: i32, max: i32) -> Vec<i32> {
    let mut vec = Vec::with_capacity(n);
    for _ in 0..n {
        vec.push(rand::random::<i32>() % (max - min) + min);
    }
    vec
}

trait Shuffle {
    fn shuffle(&mut self);
}

// implement for all vector types
impl<T> Shuffle for Vec<T> {
    fn shuffle(&mut self) {
        fastrand::shuffle(self);
    }
}

// implementing bogosort with multithreading support
fn main() {
    let n = 13;

    let items = generate_vec(n, 0, 100);
    println!("Unsorted: {:?}", items);

    let mut sorted = items.clone();
    sorted.sort();

    let mut handles = Vec::new();

    let result = Arc::new(Mutex::new(None));

    let start_time = Instant::now();

    for _ in 0..8 {
        let items = items.clone();
        let sorted = sorted.clone();
        let result = result.clone();
        handles.push(std::thread::spawn(move || {
            let mut shuffled = items.clone();
            loop {
                shuffled.shuffle();
                if shuffled == sorted {
                    *result.lock().unwrap() = Some(shuffled);
                    break;
                }
            }
        }));
    }

    let new_result: Vec<i32>;

    // if one of the threads finds the sorted vector, we are done
    loop {
        if let Some(res) = result.lock().unwrap().clone() {
            new_result = res;
            break;
        }
        // sleep for 5ms
        std::thread::sleep(std::time::Duration::from_millis(5));
    }

    println!("Sorted: {:?}", new_result);
    println!("Time: {}s", start_time.elapsed().as_secs());
}
