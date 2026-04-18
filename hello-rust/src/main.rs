fn main() {
    println!("--------------------------integer------------");
    let i1:i16=130;
    println!("The value of i1 is: {i1}");
    let i2:i8=127;
    println!("The value of i2 is: {i2}");
    let i3:i8=-128;
    println!("The value of i3 is: {i3}");

    println!("--------------------------------------");
    let i4:i8=i2.wrapping_add(1);
    println!("The value of i4 is: {i4}");
    let i5:i8=i2.saturating_add(1);
    println!("The value of i5 is: {i5}");
    let i6=i2.checked_add(1);
    println!("The value of i6 is: {i6:?}");

    println!("--------------------------char------------");
    let c = 'z';
    let heart = '❤';
    let kanji = '漢';
    println!("{} {} {}", c, heart, kanji);


    println!("--------------------------tuple------------");
    let tup: (i32, f64, u8) = (500, 6.4, 1);

// Destructuring
    let (x, y, z) = tup;
    println!("y is {y}"); // 6.4

    // Index access (0-based)
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    println!("five_hundred is {five_hundred}");
    println!("six_point_four is {six_point_four}");



    println!("--------------------------array------------");
    let a = [1, 2, 3, 4, 5];        // [i32; 5]
    let b = [3; 5];                   // [3, 3, 3, 3, 3] — 5 elements, all 3
    let first = a[0];   
    println!("a is {:?}", a);
    println!("b is {:?}", b);
    println!("first is {first}");

    
    
}
