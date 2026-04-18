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

    
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    let y = { let x = 3; x + 1 };
    println!("The value of y is: {y}");



    let number = 7;

    if number < 5 {
        println!("less than 5");
    } else if number == 5 {
        println!("exactly 5");
    } else {
        println!("greater than 5");
    }

    println!("--------------------------loop------------");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The value of result is: {result}");

    println!("--------------------------loop label------------");
    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;

        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("Count: {count}");


    println!("--------------------------while------------");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    println!("--------------------------for------------");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    println!("--------------------------loop return------------");
    let mut counter = (1..=4).rev();
    // let mut counter=(1..=0).rev();
    let i: i32 = loop {
        if let Some(num) = counter.next() {
            if num == 3 { break num; }
        } else {
            break 0;
        }
    };
    println!("The value of i is: {i}");

    println!("--------------------------for------------");
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    // Range syntax
    for number in 1..4 {
        println!("{number}"); // 1, 2, 3 (exclusive upper bound)
    }

    for number in 1..=4 {
        println!("{number}"); // 1, 2, 3, 4 (inclusive upper bound)
    }

    // Reverse
    for number in (1..4).rev() {
        println!("{number}"); // 3, 2, 1
    }
    

    let guess: i32 = match "42".parse() {
        Ok(num) => num,
        Err(_) => {
            0 
        }
    };
    println!("The value of guess is: {guess}");

    
}
