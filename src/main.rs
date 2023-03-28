use std::{sync::{Arc, Mutex, atomic::{self, AtomicBool}}, time::Instant};
use clap::{Parser};
use itertools::Itertools;

/// generates vector of n random numbers in range (min, max)
fn generate_vec(n: usize, min: i32, max: i32) -> Vec<i32> {
    let mut vec = Vec::with_capacity(n);
    vec.resize_with(n, || fastrand::i32(min..max));
    vec
}

trait Shuffle {
    fn shuffle(&mut self);
}

// implement for all vector types
impl<T> Shuffle for Vec<T> {
    #[inline(always)]
    fn shuffle(&mut self) {
        fastrand::shuffle(self);
    }
}

trait Sorted {
    fn is_sorted(&self) -> bool;
}

// implement for all vector types
impl<T: Ord> Sorted for Vec<T> {
    #[inline(always)]
    fn is_sorted(&self) -> bool {
        self.iter().tuple_windows().all(|(a, b)| a <= b)
    }
}

// takes a vector, returns a sorted vector and the time it took to sort it
fn bogosort_multithreaded(items: Vec<i32>) -> (Vec<i32>, u128) {

    // first sort the vector so that we can compare it the bogosorted vector
    let mut sorted = items.clone();
    sorted.sort();

    let mut handles = Vec::new();

    let found = Arc::new(AtomicBool::new(false));

    let result = Arc::new(Mutex::new(None));

    let start_time = Instant::now();

    for _ in 0..8 {
        let items = items.clone();
        let found = found.clone();
        let result = result.clone();
        // let rx = rx.clone();
        handles.push(std::thread::spawn(move || {
            let mut shuffled = items;
            loop {
                // to my testing, putting it in a for loop does not make it faster
                // but it does make the threads live longer than they have to,
                // thus making the program slower
                
                // for _ in 0..10000 {
                shuffled.shuffle();
                if shuffled.is_sorted() {
                    *result.lock().unwrap() = Some(shuffled);
                    found.store(true, atomic::Ordering::Relaxed);
                    return;
                }
                // }
                // if a sorted vector has already been found, we can stop
                if found.load(atomic::Ordering::Relaxed) {
                    return;
                }
            }
        }));
    }

    // if one of the threads finds the sorted vector, we are done
    // loop {
    //     if let Some(res) = result.lock().unwrap().clone() {
    //         new_result = res;
    //         break;
    //     }
    //     // sleep for 5ms
    //     std::thread::sleep(std::time::Duration::from_millis(5));
    // }
    
    // join all threads
    for handle in handles {
        handle.join().unwrap();
    }

    let result = result.lock().unwrap().clone().unwrap();

    (result, start_time.elapsed().as_micros())
}

fn bogosort_singlethreaded(mut items: Vec<i32>) -> (Vec<i32>, u128) {
    let start_time = Instant::now();
    loop {
        items.shuffle();
        if items.is_sorted() {
            break;
        }
    }
    (items, start_time.elapsed().as_micros())
}

#[derive(Parser)]
struct Args {
    /// number of elements to sort
    n: usize,

    /// run singlethreaded
    #[clap(short, long)]
    singlethreaded: bool,

    /// print the time it took to sort in microseconds
    #[clap(short, long)]
    time: bool,
}

fn main() {
    let args = Args::parse();

    let vec = generate_vec(args.n, 0, 1000);

    let (_sorted, time_micros) = if args.singlethreaded {
        bogosort_singlethreaded(vec)
    } else {
        bogosort_multithreaded(vec)
    };


    if args.time {
        println!("{}", time_micros);
    }
}

// code for finding the fastest seed
// // multithreaded version
// // find the fastest seed
// let fastest_seed: Arc<Mutex<u64>> = Arc::new(Mutex::new(u64::MAX));
// let fastest_time: Arc<Mutex<u128>> = Arc::new(Mutex::new(u128::MAX));

// let mut handles = Vec::new();

// let seed_num = 1000;

// // pool of not yet used seeds
// let seeds = Arc::new(Mutex::new(Vec::with_capacity(seed_num)));
// {
//     let mut seeds = seeds.lock().unwrap();
//     for seed in 0..seed_num {
//         seeds.push(seed as u64);
//     }
// }

// for i in 0..4 {
//     let seeds = seeds.clone();
//     let fastest_seed = fastest_seed.clone();
//     let fastest_time = fastest_time.clone();
//     handles.push(std::thread::spawn(move || {
//         loop {
//             let seed = {
//                 let mut seeds = seeds.lock().unwrap();
//                 if seeds.len() == 0 {
//                     return;
//                 }
//                 seeds.pop().unwrap()
//             };

//             fastrand::seed(seed);
//             let vec = generate_vec(n, 0, 1000);
//             let (_, time_micros) = bogosort_singlethreaded(vec);
//             if time_micros < *fastest_time.lock().unwrap() {
//                 *fastest_time.lock().unwrap() = time_micros;
//                 *fastest_seed.lock().unwrap() = seed;

//                 println!("\tFound faster seed!");
//                 println!("\tseed: {}", seed);
//                 println!("time: {}", time_micros);
//             }

//             print!("(i: {}, s: {}, t: {}) ", i, seed, time_micros);
//             let _ = std::io::stdout().flush();
//         }
//     }));
// }

// // join all threads
// for handle in handles {
//     handle.join().unwrap();
// }