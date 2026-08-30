use crate::drivers::vga::Writer;
use core::fmt::Write;
use pc_keyboard::{layouts, DecodedKey, HandleControl, PS2Keyboard, ScancodeSet1};

pub struct Shell {
    keyboard: PS2Keyboard<layouts::Us104Key, ScancodeSet1>,
    buffer: [u8; 128], // Stores up to 128 typed characters
    cursor_pos: usize, // How many characters are in the buffer
}

impl Shell {
    pub fn new() -> Self {
        Self {
            keyboard: PS2Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                HandleControl::Ignore,
            ),
            buffer: [0; 128],
            cursor_pos: 0,
        }
    }

    pub fn handle_scancode(&mut self, scancode: u8, display: &mut Writer) {
    if let Ok(Some(key_event)) = self.keyboard.add_byte(scancode) {
        if let Some(decoded_key) = self.keyboard.process_keyevent(key_event) {
            match decoded_key {
                DecodedKey::Unicode(character) => {
                    
                    if character == '\x08' {
                        //BACKSPACE
                        if self.cursor_pos > 0 {
                            self.cursor_pos -= 1;
                            self.buffer[self.cursor_pos] = 0; // Erase from memory
                            display.backspace();              // Erase from screen
                        }
                    } 
else if character == '\n' {
                        //ENTER KEY
                        let _ = write!(display, "\n");
                        
                        // Convert the raw byte buffer into a Rust string slice
                        let input_slice = &self.buffer[0..self.cursor_pos];
                        if let Ok(input_str) = core::str::from_utf8(input_slice) {
                            let input_trimmed = input_str.trim();
                            
                            // Split into exactly two parts:command and everything els
                            let mut parts = input_trimmed.splitn(2, ' ');
                            let cmd = parts.next().unwrap_or("");
                            let args = parts.next().unwrap_or("");

                            // Route the command to the correct file
                            match cmd {
                                "clear" => crate::commands::clear::execute(display),
                                "echo" => crate::commands::echo::execute(display, args),
                                "time" => crate::commands::time::execute(display),
                                "sysinfo" => crate::commands::sysinfo::execute(display),
                                "play" => crate::commands::play::execute(display, args),
                                "" => {} // User just pressed enter, do nothing
                                _ => {
                                    display.color = [255, 100, 100]; // Red error text
                                    let _ = write!(display, "Unknown command: {}\n", cmd);
                                }
                            }
                        }

                        // Reset the buffer for the next command
                        self.cursor_pos = 0;
                        self.buffer = [0; 128];
                        
                        // Print a new prompt line
                        display.color = [0, 255, 0];
                        let _ = write!(display, "> ");
                        display.color = [255, 255, 255]; // Reset to white for typing
                    }
                    else if self.cursor_pos < 128 {
                        // --- NORMAL TYPING ---
                        self.buffer[self.cursor_pos] = character as u8;
                        self.cursor_pos += 1;
                        
                        display.color = [255, 255, 255];
                        let _ = write!(display, "{}", character);
                    }
                }
                DecodedKey::RawKey(_) => {}
            }
        }
    }
}
}