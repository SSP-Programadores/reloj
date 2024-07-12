use digitos::ClockDisplay;
use std::{thread::{self, Thread}, time::Duration};



/*fn main() {
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
*/

use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut reloj = ClockDisplay::new(); 
    
    loop {       
        
        clear_background(WHITE);
        reloj.funciona();
        let hora = reloj.to_string();
        draw_text(&hora, 200.0, 200.0, 100.0, RED);
        thread::sleep(Duration::from_millis(1000));
        next_frame().await
    }
    
    
}