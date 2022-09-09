/// title: match statement
/// tags: match, statement

fn match_statement() {
    let name: &str = "michael scott";

    match name {
        "michael scott" => {
            println!("1: yes");
        }
        "ridley scott" => println!("1: no"),
        _ => {
            println!("1: neither of these names");
        }
    }

    let result_2: &str = match name {
        "michael scott" => "2: yes",
        "ridley scott" => "2: no",
        _ => "2: neither of these names",
    };

    println!("{}", result_2);

    let sky = "blue";
    let temp = "hot";

    let result_3 = match (sky, temp) {
        ("blue", "hot") => "3: yes",
        _ => "3: something other than blue and hot",
    };

    println!("{}", result_3);

    let married = true;
    let children = 3;

    let result_4 = match (married, children) {
        (married, children) if married == true && children == 3 => "4: yes",
        _ => "4: no",
    };

    println!("{}", result_4);
}
