use encryptor;
use decryptor;

fn main() {
    let input: String = String::from("Simple encryption program!");
    let seed: usize = 123456789;
    let enc = encryptor::encrypt(&input, seed);
    println!("Input encoded: {}\n",&enc);
    let decrypted_value_1 = decryptor::decrypt(&enc, seed);
    println!("Decrypted value: {}",decrypted_value_1);
}
