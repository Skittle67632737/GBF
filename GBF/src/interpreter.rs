use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 480;

pub struct Interpreter {
    byte_vector: Vec<u8>,
    current_x: usize,
    current_y: usize,
    commands: Vec<String>,
    palette: Vec<u32>,
    direction: Direction, 
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            byte_vector: vec![0; 32_000],
            current_x: 0,
            current_y: 0,
            commands: Vec::new(),
            palette: vec![0xFFFFFF, 0x000000],
            direction: Direction::Right,
        }
    }

    pub fn load_commands(&mut self, file_name: &str) -> io::Result<()> {
        let file = File::open(file_name)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line_content = line?.trim().to_string();
            if let Some(command) = line_content.split("//").next() {
                if !command.trim().is_empty() {
                    for token in command.split_whitespace() {
                        self.commands.push(token.to_string());
                    }
                }
            }
        }
        Ok(())
    }

    pub fn execute_commands(&mut self, buffer: &mut Vec<u32>) {
        let mut next_cell = 0;
    
        for command in &self.commands {
            match command.as_str() {
                "+" => {
                    self.byte_vector[next_cell] += 1;
                    println!("Added a byte to the current cell");
                }
                ">" => {
                    match self.direction {
                        Direction::Right => {
                            self.current_x += 1;
                            if self.current_x >= WIDTH {
                                self.current_x = 0;
                                self.current_y += 1;
                            }
                        }
                        Direction::Left => {
                            if self.current_x > 0 {
                                self.current_x -= 1;
                            } else if self.current_y > 0 {
                                self.current_y -= 1;
                                self.current_x = WIDTH - 1;
                            } else {
                                println!("The pointer is already at the beginning");
                            }
                        }
                        Direction::Up => {
                            if self.current_y > 0 {
                                self.current_y -= 1;
                            }
                        }
                        Direction::Down => {
                            self.current_y += 1;
                            if self.current_y >= HEIGHT {
                                self.current_y = 0;
                            }
                        }
                    }
                }
                "<" => {
                    match self.direction {
                        Direction::Left => {
                            if self.current_x > 0 {
                                self.current_x -= 1;
                            } else if self.current_y > 0 {
                                self.current_y -= 1;
                                self.current_x = WIDTH - 1;
                            } else {
                                println!("The pointer is already at the beginning");
                            }
                        }
                        Direction::Right => {
                            self.current_x += 1;
                            if self.current_x >= WIDTH {
                                self.current_x = 0;
                                self.current_y += 1;
                            }
                        }
                        Direction::Up => {
                            if self.current_y > 0 {
                                self.current_y -= 1;
                            }
                        }
                        Direction::Down => {
                            self.current_y += 1;
                            if self.current_y >= HEIGHT {
                                self.current_y = 0;
                            }
                        }
                    }
                }

                "<<" => {
                    println!("Cell value: {}", self.byte_vector[next_cell]);
                }
                "l" => self.direction = Direction::Left,
                "r" => self.direction = Direction::Right,
                "u" => self.direction = Direction::Up,
                "d" => self.direction = Direction::Down,
                "-" => {
                    self.byte_vector[next_cell] -= 1;
                    println!("Decreased byte in the current cell");
                }
                "c" => {
                    self.byte_vector[next_cell] = 0;
                    println!("Current cell (cell: {}) is zeroed", next_cell)
                }
                _ => {
                    println!("Unknown command: {}", command);
                }
            }
            let byte = self.byte_vector[next_cell];
            let color = self.get_cell_color(byte);
    
            let pixel_index = self.current_y * WIDTH + self.current_x;
            if pixel_index < WIDTH * HEIGHT {
                buffer[pixel_index] = color;
            }
    
            next_cell += 1;
            if next_cell >= self.byte_vector.len() {
                break; 
            }
        }
    }

    pub fn get_byte_vector(&self) -> &Vec<u8> {
        &self.byte_vector
    }

    pub fn get_cell_color(&self, cell_value: u8) -> u32 {
        if cell_value == 1 {
            0xFFFFFF
        } else {
            0x000000
        }
    }
}
