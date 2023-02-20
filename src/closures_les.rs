use std::thread;

pub fn closures_mn() -> u32 {
    let mut a = 55;
    let f = |x: u32| { x + a };
    thread::spawn(move || println!("{}",1 + a)).join().unwrap();
    let fx = f(2);
    fx
}