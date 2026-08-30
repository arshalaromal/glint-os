use x86_64::instructions::port::Port;

// The CMOS stores numbers in hex as if they were decimal. 
// eg: 0x12 means 12, not 18. This decodes it.
fn bcd_to_binary(bcd: u8) -> u8 {
    (bcd & 0x0F) + ((bcd / 16) * 10)
}

pub struct DateTime {
    pub year: u8,
    pub month: u8,
    pub day: u8,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

pub fn read_datetime() -> DateTime {
    let mut addr_port = Port::<u8>::new(0x70);
    let mut data_port = Port::<u8>::new(0x71);

    // Helper to read a specific CMOS register
    let mut read_reg = |reg: u8| -> u8 {
        unsafe {
            // Bitwise OR with 0x80 disables Non-Maskable Interrupts (NMI) 
            // so the CPU doesn't crash if interrupted mid-read.
            addr_port.write(reg | 0x80);
            data_port.read()
        }
    };

    // Standard CMOS Register Map
    let seconds = bcd_to_binary(read_reg(0x00));
    let minutes = bcd_to_binary(read_reg(0x02));
    let hours   = bcd_to_binary(read_reg(0x04));
    let day     = bcd_to_binary(read_reg(0x07));
    let month   = bcd_to_binary(read_reg(0x08));
    let year    = bcd_to_binary(read_reg(0x09));

    DateTime { year, month, day, hours, minutes, seconds }
}