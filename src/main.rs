use encryptor;
use decryptor;

fn main() {
    let input: String = String::from("Hello 12039819878//(/(31ii0d9saiodjhuic8721y7m na 87ya87dn 2138721 y7op1ij34i12o7sy217");
    let enc = encryptor::encrypt(&input, 92871227);
    println!("Input: {}\n",input);
    println!("Input encoded: {}\n",&enc);
    println!("Decrypted value: {}",decryptor::decrypt(&enc, 92871227));
}
