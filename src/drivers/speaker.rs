use x86_64::instructions::port::Port;
use crate::arch::time::sleep_ms;

pub struct Speaker {
    cmd_port: Port<u8>,
    data_port: Port<u8>,
    ctrl_port: Port<u8>,
}

impl Speaker {
    pub fn new() -> Self {
        Self {
            cmd_port: Port::new(0x43), // PIT Command Port
            data_port: Port::new(0x42), // PIT Channel 2 Data Port
            ctrl_port: Port::new(0x61), // Motherboard System Control
        }
    }

    /// Starts playing a specific frequency (in Hertz)
    pub fn play_sound(&mut self, frequency: u32) {
        if frequency == 0 { return; }

        // The PIT's base clock speed is 1,193,180 Hz
        let divisor = 1193180 / frequency;

        unsafe {
            // Tell the PIT to set Channel 2 (the speaker channel)
            self.cmd_port.write(0xB6);
            
            // Send the divisor byte by byte (Low byte, then High byte)
            self.data_port.write((divisor & 0xFF) as u8);
            self.data_port.write((divisor >> 8) as u8);

            // Turn on the speaker by setting the bottom two bits of Port 0x61
            let tmp = self.ctrl_port.read();
            if tmp != (tmp | 3) {
                self.ctrl_port.write(tmp | 3);
            }
        }
    }

    /// Stops the sound immediately
    pub fn stop(&mut self) {
        unsafe {
            // Turn off the speaker by clearing the bottom two bits
            let tmp = self.ctrl_port.read();
            self.ctrl_port.write(tmp & 0xFC);
        }
    }

    /// Plays a frequency for a set duration, then stops
    pub fn beep(&mut self, frequency: u32, duration_ms: usize) {
        self.play_sound(frequency);
        sleep_ms(duration_ms);
        self.stop();
        // A 30ms pause between notes so identical notes don't come together
        sleep_ms(30); 
    }
}