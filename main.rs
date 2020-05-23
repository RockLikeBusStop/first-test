fn main() {
    // let mut n = 1;

    // loop {
    //     n += 1;
    //
    //     if n == 5 {
    //         continue;
    //     }
    //
    //     if n > 10 {
    //         break;
    //     }
    //
    //     println!("n is {}", n);
    // }

    // while n <= 50 {
    //
    //     if n % 5 == 0 {
    //         println!("n is {}", n);
    //     }
    //     n+=1;
    // }

    // let nums = 30..51;
    // for num in nums {
    //     println!("num is {}", nums);
    // }

    let animals = vec!["Rabbit", "Dog", "Cat"];
    for (index, animal)  in animals.iter().enumerate() {
        println!("{}:{}",index,animal);
    }

    println!("animal 1 is {}", animals[0]);
}
