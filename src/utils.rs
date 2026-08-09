//! Useful functions used in this crate.

use std::io::{self, Write};

/// Converts an [i8] array into a String.
pub fn i8_array_to_string(buf: &[i8]) -> String {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    let bytes: Vec<u8> = buf[..len].iter().map(|&b| b as u8).collect();
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Converts a String into an [i8] array, for using as input in FFI functions.
pub fn string_to_i8_array<const N: usize>(s: &String) -> [i8; N] {
    let mut arr: [i8; N] = [0; N];

    let bytes = s.as_bytes();
    let len = bytes.len().min(N.saturating_sub(1));

    for i in 0..len {
        arr[i] = bytes[i] as i8;
    }

    arr
}

/// Prints a message to stdout, and block execution until the user presses
/// "ENTER". This function is meant to be used during tests.
pub fn press_button_to_continue(message: &str) {
    println!("{}", message);
    print!("Press ENTER to continue...");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}
