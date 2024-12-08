use std::sync::{Arc, Mutex};

fn main() {
    let x = Arc::new(Mutex::new(5));
    
    let y = x.clone();
    let z = x.clone();
    
    let thread1 = std::thread::spawn(move || {
        *y.lock().unwrap() = 10;
    });
    
    let thread2 = std::thread::spawn(move || {
        *z.lock().unwrap() = 15; 
    });
    
    thread1.join().unwrap();
    thread2.join().unwrap();
    
    println!("x = {}", *x.lock().unwrap());
} 