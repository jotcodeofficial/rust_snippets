/// title: reveal type
/// tags: reveal, type, console

fn reveal_type() {
    let tuple = (50, "tuple", 25.49);
    /*
       you can enter a non-existent function to force the rust compiler
       to reveal the type in the console '({integer}, &str, {float})':
    */
    tuple.fake_function();
}
