/// title: basic loops
/// tags: loops, looping

fn basic_loops() {
    for i in 1..11 {
        print!("{},", i);
    }

    println!("");
    let mut counter = 1;

    loop {
        if counter <= 10 {
            print!("{},", counter);
            counter += 1;
        } else {
            break;
        }
    }
}
