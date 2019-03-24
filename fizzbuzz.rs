fn main() {
    for count in 0..101 {
        if count % 3 == 0 && count % 5 == 0 {
            println!("{} fizzbuzz", count)
        } else if count % 5 == 0 {
            println!("{} buzz", count)
        } else if count % 3 == 0{
            println!("{} fizz", count)
        } else {
            println!("{}", count)
        }
    }
}