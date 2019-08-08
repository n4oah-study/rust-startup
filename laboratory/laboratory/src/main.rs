mod utils;

#[derive(Debug)]
enum Browser {
    Firefox(String),
    Chrome(String),
    Whale(String),
    IE11(String),
    Opera(String)
}

#[derive(Debug)]
enum Os {
    Windows,
    MacOs,
    Linux
}

#[derive(Debug)]
struct Computer {
    browser: Browser,
    archit: u8,
    Os: Os
}

impl Browser {
    fn test_method(self) -> String {
        match self {
            Browser::Chrome(browser) => browser.to_string(),
            Browser::Firefox(browser) => browser.to_string(),
            Browser::Whale(browser) => browser.to_string(),
            Browser::Opera(browser) => browser.to_string(),
            Browser::IE11(browser) => browser.to_string()
        }
    }
}

impl Computer {
    fn get_browser_str(&self) -> String {
        match &self.browser {
            Browser::Chrome(browser) => browser.to_string(),
            Browser::Firefox(browser) => browser.to_string(),
            Browser::Whale(browser) => browser.to_string(),
            Browser::Opera(browser) => browser.to_string(),
            Browser::IE11(browser) => browser.to_string()
        }
    }
}

fn main() {
    let computer: Computer = Computer {
        browser: Browser::Chrome(String::from("구글 크롬")),
        archit: 64,
        Os: Os::Windows
    };

    println!("{:?}", computer);

    /*
    if let Browser::Chrome(browser) = computer.browser {
        println!("{}", browser);
    }
    */

    let val: String = computer.get_browser_str();
    println!(": {}", val);
    let val: String = computer.get_browser_str();
    println!(": {}", val);

    let number: Option<u32> = Some(4444);
    let nn: u32 = number.unwrap();
    let nn: u32 = number.unwrap();

    let val: String = computer.browser.test_method();
    println!("{}", val);
    let val: String = computer.browser.test_method();
    println!("{}", val);
}