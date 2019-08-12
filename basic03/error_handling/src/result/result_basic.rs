use std::fs::File;
use std::io::ErrorKind;

pub fn result_basic_func() {
    let f = File::open("hello.txt");

    let f: File = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("file not found: {:?}", error);
        }
    };
    
    println!("{:?}", &f);

    file_open_or_create("hello2.txt");
    file_open("hello2.txt");
}

// 파일이 존재하면 open
// 파일이 존재하지 않으면 create
// 그 외 error (권한이 없더나 등)
fn file_open_or_create(name: &str) {
    let f = File::open(&name);

    let f: File = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create(&name) {
                Ok(file) => file,
                Err(error) => {
                    panic!("파일을 생성할 수 없습니다. {:?}", error);
                }
            }
        },
        Err(error) => {
            panic!("파일을 읽을 수 없습니다. {:?}", error);
        }
    };
}

fn file_open(name: &str) {
    let f = File::open(&name).expect("파일을 open할 수 없음");

    // 만약 &name이 없는 파일이라면 unwrap에러 panic!을 실행해줌
    let f = File::open(&name).unwrap();
    println!("{:?}", f);

}