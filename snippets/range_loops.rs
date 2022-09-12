/// title: range loops
/// tags: range, loop, loops

fn range_loops() {

    for i in 0..5 {
        println!("current iteration 1 is: {}", i);
    }

    for i in 0..=5 {
        println!("current iternation 2 is: {}", i);
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("{:?}", result);
}