fn main() {
    const MAX_POINTS: u32 = 100_000; // type annotation required

    println!("The value of MAX_POINTS is: {MAX_POINTS}");

    static LANGUAGE: &str = "Rust";
    println!("The value of LANGUAGE is: {LANGUAGE}");

    static mut COUNTER: u32 = 0;

    unsafe {
        COUNTER += 1;
        let val = std::ptr::addr_of!(COUNTER).read();
        println!("The value of COUNTER is: {val}");
    }

    static NAME: [u8; 4] = *b"Rust";
    println!("The value of NAME is: {NAME:?}");
}
