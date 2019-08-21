/*
에러 전파하기
*/
use std::io;
use std::fs::File;
use std::io::Read;

pub fn result_spread_func() {
    let res = error_spread_func();
    println!("{:?}", res);
}

pub fn error_spread_func() -> Result<String, io::Error> {
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