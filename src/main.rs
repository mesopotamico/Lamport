use std::sync::{Arc, Mutex};
mod quick_sort;
mod rng_array;
mod thread_assign;

use thread_assign::thread_assign;
use quick_sort::quick_sort;
use rng_array::rng_array;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {


    let init2: usize = 0;
    let size: usize = 100000;
    let threads_number: usize = 5; 

    let disorder = rng_array(size);
    println!("{:?}", disorder);

    let mega_vec = Arc::new(Mutex::new(thread_assign(disorder, threads_number)));
    let mut handles = vec![];

    //quick_sort(&mut mega_vec[0],0,3);
    //println!("First sorted {:?}", mega_vec[0]);


    for id in 1..threads_number {
        let mega_vec_clone = Arc::clone(&mega_vec);
        let handle = thread::spawn(move || {
            let mut mega_vec_lock = mega_vec_clone.lock().unwrap();
            let end: usize = mega_vec_lock[id].len() - 1;
            quick_sort(&mut mega_vec_lock[id], 0, end );
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let data = mega_vec.lock().unwrap();

    for fila in &*data{
        println!("Ordered -> {:?}", fila);
    }

}

