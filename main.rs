fn main() {
    let mut x = 45;
    println!("x is {}", x);
    x = 90;
    println!("x is changed to {}", x);

    if x < 30 {
        println!("x is less than 30");
    } else {
        println!("x is greater than or equal to 30");
    }
}
