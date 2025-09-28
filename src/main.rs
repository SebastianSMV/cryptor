use encryptor;
use decryptor;

fn main() {
    let input: String = String::from("Simple encryption program!");
    let enc = encryptor::encrypt(&input, 92871227);
    println!("Input: {}\n",input);
    println!("Input encoded: {}\n",&enc);
    println!("Decrypted value: {}",decryptor::decrypt(&enc, 92871227));
}
