fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add(1, 2);
    println!("The sum of 1 and 2 is: {sum}");

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

}
