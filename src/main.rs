use digitos::ClockDisplay;
use std::{thread, time::Duration};



fn main() {
    let mut reloj = ClockDisplay::new(); 
    let hilo = thread::spawn(move || {
        loop {
            reloj.funciona();
            println!("{}", reloj);
            thread::sleep(Duration::from_secs(1));
        }
    });
    let _ = hilo.join();
}


