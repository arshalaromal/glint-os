use crate::drivers::vga::Writer;
use core::fmt::Write;

// A simple lookup function converting musical notes to frequencies (Hertz)
fn get_frequency(note: &str) -> u32 {
    match note {
        "C4" => 261, "CS4" => 277, "D4" => 293, "DS4" => 311, "E4" => 329, "F4" => 349, "FS4" => 370,
        "G4" => 392, "GS4" => 415, "A4" => 440, "AS4" => 466, "B4" => 493, 
        "C5" => 523, "CS5" => 554, "D5" => 587, "DS5" => 622, "E5" => 659, "F5" => 698, "FS5" => 740,
        "G5" => 783, "GS5" => 831, "A5" => 880, "AS5" => 932, "B5" => 987,
        _ => 0, // Rest/Silence
    }
}

pub fn execute(display: &mut Writer, args: &str) {
    if args.is_empty() {
        display.color = [255, 100, 100];
        let _ = writeln!(display, "Usage: play <notes> OR play <template>");
        let _ = writeln!(display, "Templates: mario, potter, shire");
        return;
    }

    // Intercept the argument and swap it for a hardcoded string if it matches
    let tune = match args {
        "mario" => "E5-150 E5-150 _-150 E5-150 _-150 C5-150 E5-300 _-150 G5-300 _-300 G4-300",
        "potter" => "B4-300 E5-450 G5-150 FS5-300 E5-600 B5-300 A5-900 FS5-600", 
        "shire" => "D4-300 E4-300 FS4-600 A4-300 FS4-300 E4-900",
        _ => args, // If not a template, play the raw notes the user typed
    };

    let mut speaker = crate::drivers::speaker::Speaker::new();
    display.color = [0, 255, 255];
    let _ = writeln!(display, "Playing tune...");


    for part in tune.split(' ') {
        let mut note_parts = part.split('-');
        let note = note_parts.next().unwrap_or("");
        let duration_str = note_parts.next().unwrap_or("200");
        let duration = duration_str.parse::<usize>().unwrap_or(200);

        if note == "_" {
            crate::arch::time::sleep_ms(duration);
        } else {
            let freq = get_frequency(note);
            if freq > 0 {
                speaker.beep(freq, duration);
            }
        }
    }
}