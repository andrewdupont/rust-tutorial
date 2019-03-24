// fn main() {
//     for count in 0..101 {
//         let result = if count % 15 == 0 {
//             "fizzbuzz"
//         } else if count % 3 == 0 {
//             "fizz"
//         } else if count % 5 == 0{
//             "buzz"
//         } else {
//             count.to_string()
//         };
//     println!("{}", result);
//     }
// }

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

