use std::{env, process};
use std::thread;
use std::sync::{mpsc, Mutex, Arc};
use std::time::Instant;

const END_OF_STREAM: usize = 0;

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print_usage_and_error();
    }

    let threads: &i32 = &args[1].parse::<i32>().expect("Must be a number");
    let limit = &args[2].parse::<usize>().expect("Must be a number");

    let (tx, rx) = mpsc::channel::<usize>();
    let receiver = Arc::new(Mutex::new(rx));
    let primes: Arc<Mutex<Vec<usize>>>  = Arc::new(Mutex::new(vec![]));

    // Creates threads
    let mut handles = vec![];
    for _ in 0..*threads {
        let receiver = Arc::clone(&receiver);
        let primes = Arc::clone(&primes);
        let handle = thread::spawn(move || loop {
            let n = receiver.lock().unwrap().recv().unwrap();
            if n == END_OF_STREAM {
                break;
            }
            if is_prime(n) {
                primes.lock().unwrap().push(n);
            }
        });
        handles.push(handle);
    }
    for i in 2..=*limit {
        tx.send(i).unwrap();
    }
    for _ in 0..*threads {
        tx.send(END_OF_STREAM).unwrap();
    }
    for handle in handles {
        handle.join().unwrap();
    }
    // println!("Primes: {:?}", primes.lock().unwrap());

    let end_time = Instant::now();
    let elapsed = end_time - start_time;
    println!("Elapsed time: {:?}", elapsed);
}

fn print_usage_and_error() {
    print!("Usage: cargo run [number of threads] [limit]");
    process::exit(1);
}

fn is_prime(n: usize) -> bool{
    let mut i = 2;
    let limit = (n as f64).sqrt() as usize;
    while i <= limit {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}