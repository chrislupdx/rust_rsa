fn main() {
    let args: Vec<String> = env::args().collect();
    let inputstring = &args[1];
    let inputas128 = parsenum(inputstring);

    //takes in an input string
    use toy_rsa_lib::*; //this is probably going to have to go in a bunch of places

    let u128_val: u128 = 123456; //we are trying to encode this
    pub const EXP: u64 = 65_537;

    println!("Hello, world!");

    //1.
    //2.
    //3.
    //4.
    //5.
}

///outputs a key
fn keygen() -> u64 {
        
        //produce two very large primes
}


mod tests {
    #[test]
    fn test_keygen() {
        use crate::keygen;
        //this is a sanity check to ensure that geys are generated in the right fashion
    }

}
