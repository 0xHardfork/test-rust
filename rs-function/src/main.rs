use std::any::type_name;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add(1, 2);
    println!("The sum of 1 and 2 is: {sum}");

}
