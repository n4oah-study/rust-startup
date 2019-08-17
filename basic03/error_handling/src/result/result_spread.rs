/*
에러 전파하기
*/
pub fn result_spread_func() {
    
}

pub fn error_spread_func() -> Result<String, io::Error> {
    let f = File::open("hello3.txt")

    let f = match f {
        Ok(file) => file,
        Err(error) => Err(e)
    };
}