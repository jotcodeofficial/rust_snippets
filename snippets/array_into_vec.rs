/// title: array into vec
/// tags: vec, into, array

fn array_into_vec() {
    let names: [&str; 3] = ["peter", "adam", "sally"];
    // you can use <_> if you want rust to determine the type
    let mut vec_names: Vec<&str> = names.into();
    vec_names.push("john");
    println!("{:#?}", vec_names);
}
