fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is {x}");
    x = 10;
    println!("Passing x to function after reassignment");
    example_function(x);
    print_labeled_measurement(x, 'C');
    x = five();
    println!("Five function: {x} ");
    x = plus_one(5);
    println!("{x}");
    
    //control flow
    control_flow_example(5);

    //loop example
    loop_example(x);

    //iterate array
    iter_array(x);
}

fn example_function(x: i32) {
    println!("The value of x is: {x}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn control_flow_example(x: i32) {
    let number = x;
    if number < 5 {
        println!("Number is less than five");
    } else if number == 5 {
        println!("Number is equal to five");
    } else {
        println!("Number is greater than five");
    }
}

fn loop_example(x: i32) {
    let mut counter = x;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 12;
        }
    };
    println!("The result is: {result}");
}

fn iter_array(x: i32) {
    let arr = [x+1,x+3,x+4,x+x];

    for element in arr {
        println!("Value is: {element}");
    }
}