use std::any::type_name;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn printTest() {
let name = "Rustacean";
let age = 3;
let pi = 3.14159;

println!("Hello, {name}!");              // variable capture (Rust 2021+)
println!("Hello, {}!", name);            // positional placeholder
println!("{0} is {1}, {0} is cool", name, age); // indexed
println!("Pi to 2 places: {pi:.2}");     // formatting: 3.14
println!("Binary: {:b}", 42);            // 101010
println!("Hex: {:x}", 255);              // ff
println!("Debug: {:?}", (1, 2, 3));      // (1, 2, 3)
println!("Pretty: {:#?}", vec![1,2,3]);  // multi-line debug
}

fn main() {
    let sum = add(1, 2);
    println!("The sum of 1 and 2 is: {sum}");
    println!("The type of sum is: {}", type_of(&sum));

    println!("--------------------------printTest------------");
    printTest();

}
