use std::{sync::{Arc, Mutex}, time::Instant};
use clap::{Parser};

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
    #[inline(always)]
    fn shuffle(&mut self) {
        fastrand::shuffle(self);
    }
}

// takes a vector, returns a sorted vector and the time it took to sort it
fn bogosort_multithreaded(items: Vec<i32>) -> (Vec<i32>, u128) {

    // first sort the vector so that we can compare it the bogosorted vector
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
            let mut shuffled = items;
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

    (new_result, start_time.elapsed().as_micros())
}

fn bogosort_singlethreaded(mut items: Vec<i32>) -> (Vec<i32>, u128) {
    let start_time = Instant::now();
    let mut sorted = items.clone();
    sorted.sort();
    loop {
        items.shuffle();
        if items == sorted {
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