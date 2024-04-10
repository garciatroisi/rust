fn main() {
    mutability();
    constants();
    shadowing();
    arrays();
    tuples();
    print_labeled_measurement(5, 'h');
    expressions();
    stat_fn();
    loops();
    loop_labels();
    while_loop();
    reverse();
}

fn reverse() {
    for number in (1..6).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn while_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    //error prone
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    for element in a {
        println!("the value is: {element}");
    }
}
fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
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
    println!("End count = {count}");
}

fn loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The loop result is {result}");
}

fn mutability() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn constants() {
    const TWO: u32 = 1 + 1;
    println!("The TWO const value: {TWO}");
}

fn shadowing() {
    let x = 5;
    println!("The first value of x is: {x}");
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

fn arrays() {
    let a = [1, 2, 3, 4, 5];
    let index = 0;
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}

fn tuples() {
    let t = ([1; 2], [3; 4]);
    let (a, _) = t;
    println!("The value of sum tuple arrays {}", a[0] + t.1[0]);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expressions() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn f(x: i32) -> i32 {
    x + 1
}
fn stat_fn() {
    println!(
        "The value returned is: {}",
        f({
            let y = 1;
            y + 1
        })
    );
}
