/// title: pointer address
/// tags: pointer address reference

fn pointer_address() {
    let name = "jotcode";
    let reference_value = &vec![1, 2, 3];

    println!(
        "pointer address for &str: {:p}\npointer address for vec pointer: {:p}",
        name, reference_value
    );
}
