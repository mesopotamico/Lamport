use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn prueba_hilo() {
    let contador = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for x in 0..11 {
        let contador_clon = Arc::clone(&contador);
        let handle = thread::spawn(move || {
            let mut num = contador_clon.lock().unwrap();
            *num += x; // Incrementa el contador
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // Espera a que cada hilo termine
    }

    println!("El valor final del contador es: {}", *contador.lock().unwrap());
}
