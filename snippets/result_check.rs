/// title: result check
/// tags: result, check

fn checker() -> Result<i32, String>{

    let check = 1;
    if check == 1 {
        return Ok(1);
    } else {
        return Err("This is error".to_string());
    }   
}

fn result_check() {
    let result = checker();
    if result.is_err() {
        println!("{:?}", result);
    }
    if result.is_ok() {
        println!("{:?}", result.unwrap());
    }
}