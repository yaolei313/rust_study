pub fn study_condition_expression() {
    // if可以是表达式，所有分支必须返回相同类型
    let miles = 100;
    let car_desc = if miles > 0 { "new car" } else { "used car" };
    println!("{}", car_desc);
}

pub fn study_loop() {
    let mut count = 1;
    // loop
    let stop_count = loop {
        count += 1;
        if count == 100 {
            break count;
        }
    };
    println!("break count at {}", stop_count);

    // while condition {}
    while count < 200 {
        count += 10
    }
    println!("after while loop as {}", count);

    let birds = ["ostrich", "peacock", "stork"];
    // for item in xx {}
    for item in birds {
        print!("{}, \t", item);
    }
    println!("");
    for item in birds.iter() {
        print!("{}, \t", item);
    }
    println!("");
    for (index, value) in birds.iter().enumerate() {
        print!("({}, {}) \t", index, value);
    }
    println!("");
    for number in 0..5 {
        print!("{}, ", number)
    }
    println!("")
}
