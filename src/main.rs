use digitos::ClockDisplay;
use std::{thread, time::Duration};



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
        
        
        
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        let hora = reloj.funciona();
        draw_text(reloj.funciona(), 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
    
    
}