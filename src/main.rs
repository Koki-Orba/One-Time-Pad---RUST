use ferris_says::say;
use std::io::{stdout,  BufWriter};
use std::str;

struct Cipher {
    key:  &'static str,
    message: &'static str,
    cyphertext: &'static str
}

trait OTP {
    fn new(message: &'static str, key: &'static str, cyphertext: &'static str) -> Self;

    // Instance method
    fn convert_binary(&self, string: &'static str) -> &[u8];
    fn xor(&self, encripted : bool) -> String;
    fn save_cyphertext(&mut self, cyphertext:String);

    fn ecrypt(&self) -> String{
        return self.xor(true);
    }

    fn decrypt(&mut self, cyphertext: String) -> String{
        self.save_cyphertext(cyphertext);
        return self.xor(false);
    }

}

impl OTP for Cipher {
    fn new(message: &'static str, key: &'static str, cyphertext: &'static str) -> Cipher {
        Cipher { message: message, key: key, cyphertext: ""}
    }

    fn save_cyphertext(&mut self, s: String) {
        self.cyphertext = "test";
    }

    fn convert_binary(&self, string: &'static str) -> &[u8] {
        let bytes = string.as_bytes();
        return bytes;
    }

    fn xor(&self, encripted : bool) -> String {
        let mut xor: Vec<u8> = Vec::new();
        let mut text_bytes: &[u8];

        if (encripted){
            text_bytes = self.convert_binary(self.message);
        }else{
            text_bytes = self.convert_binary(self.cyphertext);
        }

        let key_bytes = self.convert_binary(self.key);

        for n in 0.. text_bytes.len(){
            xor.push((text_bytes[n] ^ key_bytes[n]));
        }

        return  String::from_utf8(xor).unwrap();
    }
}

fn main() {
    let message = "Holamundo";
    let key = "123456789";

    let mut opt_algorithm: Cipher = OTP::new(message, key, "");

    let cyphertext = opt_algorithm.ecrypt();
    println!("Encrypted Message: {:?}", cyphertext);

    let message_decrypted = opt_algorithm.decrypt(cyphertext);
    //println!("Dencrypted Cyphertext: {:?}", cyphertext);
}