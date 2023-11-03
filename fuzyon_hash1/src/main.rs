extern crate ring;
use ring::digest;
use std::io;


fn main() {
    println!("En az 5 karakterlik bir metin girin:");
    let mut input_text = String::new();

    io::stdin().read_line(&mut input_text).expect("Okuma hatası");

    let trimmed_input = input_text.trim();
    if trimmed_input.len() < 5 {
        println!("Girdi en az 5 karakter içermelidir.");
        return;
    }

    let hash_value = sha256_hash(trimmed_input.as_bytes());

    println!("Girilen Metin: {}", trimmed_input);
    println!("SHA-256 Hash Değeri: {:x?}", hash_value);
}

fn sha256_hash(data: &[u8]) -> [u8; 32] {
    let algorithm = &digest::SHA256;
    let digest = digest::digest(algorithm, data);
    let mut result = [0u8; 32];
    result.copy_from_slice(digest.as_ref());
    result
}
