fn fizz_buzz(count: i32) -> String {
    if count % 15 == 0 {
        "FizzBuzz".to_string()
    } else if count % 3 == 0 {
        "Buzz".to_string()
    } else if count % 5 == 0 {
        "Fizz".to_string()
    }
    else {
        count.to_string()
    }
}

fn main() {
    for i in 1..101 {
        println!("{}", fizz_buzz(i));
    }
}
