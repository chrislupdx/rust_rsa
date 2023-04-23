use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let inputstring = &args[1];
    let inputasu32 = parsenum(inputstring); //this is the ''message''

    //takes in an input string
    //use toy_rsa_lib::*; //this is probably going to have to go in a bunch of places
    let keytuple: (u32, u32) = keygen();
    let ciphertext = encrypt(keytuple.0.into(), inputasu32);
    println!("{}",ciphertext);
}

/// Parse the given string as a `u64`.
fn parsenum(s: &str) -> u32 {
    s.parse().unwrap_or_else(|_| error())
}

/// Print a usage error message and exit.
fn error() -> ! {
    eprintln!("toy_rsa: error");
    #[cfg(not(test))]
    std::process::exit(1);
    #[cfg(test)]
    panic!("error");
}

///outputs a key
fn keygen() -> (u32, u32) {
    use toy_rsa_lib::*; //this is probably going to have to go in a bunch of places
    let key1: u32 = rsa_prime();
    let key2: u32 = rsa_prime();
    //oh are key1 and key2 being returned as u64
    return (key1, key2);
}

//attempts to convert u32 -> u64
fn u32tou64(val: u32) -> u64 {
   // val.parse().unwrap_or_else(|_| error())
   val as u64
}

/// Encrypt the plaintext 'msg' using the rsa
/// public 'key' and return ciphertext
fn encrypt(key: u64, msg: u32) -> u64 {
    use toy_rsa_lib::*; //this is probably going to have to go in a bunch of places
    pub const EXP: u64 = 65_537;
    let u64m = u32tou64(msg);
    let res: u64 = modexp(u64m, EXP, key);
    res
}

mod tests {
    #[test]
    fn test_keygen() {
        //use crate::keygen;
        //this is a sanity check to ensure that the keys are generated in the right fashion
        //returns the expected type
    }
    
    #[test]
    fn test_encrypt() {
        //use crate::encrypt;
        //this is a sanity check to ensure that encryption is done in the right fashion
    }
}
