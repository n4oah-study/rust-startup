/*
에러 전파하기
*/
use std::io;
use std::fs::File;
use std::io::Read;

pub fn result_spread_func() {
    println!("========== error_spread_func ===========");
    let res = error_spread_func();
    println!("error_spread_func return: {:?}", res);

    println!("========== error_spread_short_func ===========");
    let res = error_spread_short_func();
    println!("error_spread_short_func return: {:?}", res);
}

fn error_spread_func() -> Result<String, io::Error> {
    let f = File::open("hello3.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error) // 에러시 error_spread_func을 리턴 함
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error)
    }
}

// ? 키워드로 Result가 Error일시 호출한쪽으로 에러를 전파함
fn error_spread_short_func() -> Result<String, io::Error> {
    let mut f = File::open("hello3.txt")?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    
    Ok(s)
}