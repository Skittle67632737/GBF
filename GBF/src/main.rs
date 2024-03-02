use std::fs::File;
use std::io::{self, BufRead, BufReader};
use minifb::{Key, Window, WindowOptions};

mod interpreter;
use interpreter::Interpreter;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window =
        Window::new("GBF", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let file_name = "write_your_file_lol.gbf";
    let mut interpreter = Interpreter::new();
    if let Err(e) = interpreter.load_commands(file_name) {
        eprintln!("Error loading commands: {}", e);
        return;
    }

    let mut commands_executed = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if !commands_executed {
            interpreter.execute_commands(&mut buffer);
            commands_executed = true;
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap_or_else(|e| {
                println!("{}", e);
            });

        commands_executed = true;
    }
}
