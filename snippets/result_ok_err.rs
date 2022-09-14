/// title: result ok err
/// tags: result, ok, err


fn check_result(name: String) -> Result<u32, ()>{
    if name == "bob".to_string() {
        return Ok(1);
    } else {
        return Err(());
    }
}

fn main() {
    let result = check_result("bob".to_string());
    if result.is_ok() {
        println!("The result is ok: {:?}", result.unwrap());
    }
    if result.is_err() {
        println!("The result is an err");
    }
}