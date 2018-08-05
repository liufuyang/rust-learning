use std::fmt;

#[derive(Debug, Clone)]
enum Language {
    English,
    Chinese,
    Unknown,
}

#[derive(Debug, Clone)]
struct Greeter {
    lang: Language,
}

impl Greeter {
    fn new() -> Greeter {
        Greeter {
            lang: Language::English,
        }
    }

    // use clone() to allow passing a borrowed self when calling greeter.with_language(self ...)
    fn with_language(mut self, lang: Language) -> Greeter {
        // let mut s = self.clone();
        // s.lang = lang;
        // s
        self.lang = lang;
        self
    }
}

impl fmt::Display for Greeter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let greeting = match self.lang {
            Language::English => "Hello",
            Language::Chinese => "你好",
            _ => "Alien saying: Hello",
        };
        // println!("{} Rust", greeting)
        write!(f, "{} Rust", greeting)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn greet_works() {
        let mut greeter = Greeter::new().with_language(Language::English);
        println!("{}", greeter);
        assert_eq!(format!("{}", greeter), "Hello Rust");

        println!("{}", greeter);
        greeter = greeter.with_language(Language::Chinese);
        println!("{}", greeter);
        // println!("{:?}", greeter);
    }
}