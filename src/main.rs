use std::{thread, time::Duration};

fn main() {
    let mut v= 9;
    v+=1;
    let handle = thread::spawn(move||{
        for i in 1..10 {
            println!("Spawned1: {}", i*v);
            thread::sleep(Duration::from_millis(1000));
        }
        return 1; 
    });

    for i in 1..5 {
        println!("Main: {}", i*v);
        thread::sleep(Duration::from_millis(1));
    }

    let data = handle.join().unwrap();
    println!("{}", data)
}
