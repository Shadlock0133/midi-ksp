use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

use midi::midi::{AxiomAirController, Channel, MidiMessageIn};
use minifb::{Key, Window, WindowOptions};

static STOP: AtomicBool = AtomicBool::new(false);

fn draw(buffer: &mut [u32], size: (usize, usize), bg: u32) {
    for y in 0..size.1 {
        for x in 0..size.0 {
            let pixel = &mut buffer[y * size.0 + x];
            *pixel = bg;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ctrlc::set_handler(|| STOP.store(true, Ordering::SeqCst)).unwrap();

    let (conn, r) = AxiomAirController::new()?;
    println!("Connection open, reading input ...");

    let mut timer = Instant::now();
    let mut window = Window::new(
        "test",
        480,
        320,
        WindowOptions {
            ..Default::default()
        },
    )?;
    let mut color = 0xffffff;
    let mut reset_timer = 0;
    let mut buffer = vec![color; 480 * 320];
    let frame = Duration::from_secs(1) / 60;

    while !STOP.load(Ordering::SeqCst) && window.is_open() && !window.is_key_down(Key::Escape) {
        while let Ok(e) = r.try_recv() {
            match e {
                (2, MidiMessageIn::Note(Channel::Ch16, 81, b)) if b != 0 => {
                    reset_timer = 30;
                }
                (2, MidiMessageIn::ControlChange(Channel::Ch16, 33, b)) => {
                    let b = b << 1;
                    color = u32::from_le_bytes([b, b, b, 0]);
                }
                _ => (),
            }
        }
        if timer.elapsed() > frame {
            if reset_timer > 0 {
                reset_timer -= 1;
                draw(&mut buffer, (480, 320), color);
            } else {
                buffer = vec![0; 320 * 480];
            }
            timer = Instant::now();
            window.update_with_buffer(&buffer, 480, 320)?;
        }
    }

    conn.close()?;
    Ok(())
}
