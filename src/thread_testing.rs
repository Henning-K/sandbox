extern crate num_cpus;

use std::thread;

pub fn thread_test() -> Result<(), &str> {
    let handle = thread::spawn(|| {
        // println!("Single thread spawned.");
    });

    handle.join().map_err(|err| "Error joining thread.").and_then(|_| Ok(()))
}

pub fn n_threads(n: usize) -> Result<(), &str> {
    let mut handles = Vec::with_capacity(n);
    for _ in 0..n {
        let handle = thread::spawn(move || {
            // println!("Thread {} of {} spawned.", i, n - 1);
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join().and_then(|_| Ok(()));
    }
    Ok(())
}

pub fn cpu_count_threads() -> Result<(), &str> {
    let cpu_count = num_cpus::get();
    println!("CPU count: {}", cpu_count);
    let mut handles = Vec::with_capacity(cpu_count);
    for _ in 0..num_cpus::get() {
        let handle = thread::spawn(|| {

        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join().and_then(|_| Ok(()));
    }
    Ok(())
}

use std::sync::{Arc, Mutex};

pub fn mutating_threads() -> Result<(), &str> {
    const N: usize = 1000;
    let mut handles = Vec::with_capacity(N);
    let data = Arc::new(Mutex::new((0..N).collect::<Vec<usize>>()));

    for i in 0..N {
        let data = data.clone();
        let handle = thread::spawn(move || {
            let mut data = data.lock().expect("Error locking Mutex.");
            data[i] += i;
        });
        handles.push(handle);
    }

    for h in handles {
        if let Err(_) = h.join() {
            return Err("Joining child thread returned an error.");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn single_thread_empty_return() {
        assert_eq!(thread_test(), Ok(()));
    }
    #[test]
    fn n_threads_test() {
        assert_eq!(n_threads(1usize), Ok(()));
        assert_eq!(n_threads(2usize), Ok(()));
        assert_eq!(n_threads(4usize), Ok(()));
        assert_eq!(n_threads(8usize), Ok(()));
        assert_eq!(n_threads(16usize), Ok(()));
        assert_eq!(n_threads(32usize), Ok(()));
        assert_eq!(n_threads(64usize), Ok(()));
        assert_eq!(n_threads(128usize), Ok(()));
    }
    #[test]
    fn cpu_count_threads_test() {
        assert_eq!(cpu_count_threads(), Ok(()));
    }
    #[test]
    fn mutating_threads_test() {
        assert_eq!(mutating_threads(), Ok(()));
    }
}
