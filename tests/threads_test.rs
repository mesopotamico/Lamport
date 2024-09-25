use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::thread;

#[test]
fn prueba_hilo() {
    // Creamos un canal (tx = transmisor, rx = receptor)
    let (tx, rx) = mpsc::channel();

    // Creamos múltiples hilos productores (usuarios)
    for id in 1..6 {
        let tx_clone = tx.clone(); // Clonamos el transmisor para cada hilo
        thread::spawn(move || {
            // Cada hilo genera varias actividades
            for actividad in 1..4 {
                let mensaje = format!("Usuario {} ha realizado la actividad {}", id, actividad);
                tx_clone.send(mensaje).unwrap();
                thread::sleep(Duration::from_millis(500)); // Simulamos tiempo de procesamiento
            }
        });
    }

    // Hilo consumidor: recibirá e imprimirá todas las actividades
    thread::spawn(move || {
        for recibido in rx {
            println!("Actividad registrada: {}", recibido);
        }
    });

    // Damos tiempo suficiente para que todos los hilos terminen antes de que el programa finalice
    thread::sleep(Duration::from_secs(5));
}
