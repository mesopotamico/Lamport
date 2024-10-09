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

// use std::io::{self, BufRead,BufReader, Write};
// use std::net::{TcpListener, TcpStream};

enum socket {
    source,
    prio,
}

fn main() {

    // let mut stream = TcpStream::connect("127.0.0.1:80")?;
    // println!("Conectado al servidor en 127.0.0.1:80");



    let init2: usize = 0;
    let size: usize = 10;
    let threads_number: usize = 5; 


    let mut disorder = rng_array(size);
    println!("Disorder threads {:?}", disorder);

    let mega_vec = Arc::new(Mutex::new(thread_assign(disorder, threads_number)));
    let mut handles = vec![];

    //println!("First sorted {:?}", mega_vec[0]);


    for id in 0..threads_number {
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
        println!("Ordered threads -> {:?}", fila);
    }

}

fn thread_distribution (bvec: Vec<Vec<i32>>,  )

//Pass function as parameter to assign threas to a divide an conquer process
