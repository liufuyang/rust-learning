#![feature(dbg_macro)]
use std::fmt::Debug;

// Trait can associate functions
trait CookGene {
    fn cook(&self) -> String;
}
/**
 * Trait can also associate types
 *
 * trait Iterator {
 *     type Item;
 *     fn next(&mut self) -> Option<Self::Item>;
 * }
 *
 * impl Iterator for FourIntegers {
 *      type Item = i32;
 *      fn next(&mut self) -> Option<i32> {...}
 * }
 */

// must not have trait Debug
struct A {}

#[derive(Debug)]
struct ADebug(u32);

impl CookGene for A {
    fn cook(&self) -> String {
        format!("I am A, I make 1 dishes for you")
    }
}

impl CookGene for i32 {
    fn cook(&self) -> String {
        format!("I am integer, I make {} dishes for you", self)
    }
}

impl CookGene for String {
    fn cook(&self) -> Self {
        format!("I am a String {}, I can make you nothing", &self)
    }
}

// Trait bound, using generics in impls
// impl<T: Debug> CookGene for Vec<T>
impl<T> CookGene for Vec<T>
where
    T: Debug,
{
    fn cook(&self) -> String {
        let mut s: String = String::from("");
        for t in self {
            s.push_str(&format!(
                "I am guy in a Vec<T: Debug>, I can make dish number {:?}\n", // using debug
                t
            ));
        }
        s
    }
}
// Here we can define CookGene for Vec<A>, but now A must not have trait Debug, otherwise conflict with above define
impl CookGene for Vec<A> {
    fn cook(&self) -> String {
        let mut s: String = String::from("");
        for _t in self {
            s.push_str(&format!("I am a guy in Vec<A>, I can make a dish\n"));
        }
        s
    }
}

// Trait bound
// fn make_food_party<T: CookGene>(cooks: &[T])
fn make_food_party<T>(cooks: &[T])
where
    T: CookGene,
{
    println!("Making food party now: ");
    for cook in cooks {
        println!("{}", cook.cook());
    }
}

// Trait Object
fn make_food_party_dyn(cooks: &[Box<dyn CookGene>]) {
    println!("Making food party dyn now: ");
    for cook in cooks {
        println!("{}", cook.cook());
    }
}

fn main() {
    println!("{}", 33.cook());
    println!("{}", <i32 as CookGene>::cook(&44)); // unambiguous function call syntax, meaning:
                                                  // the cook method here is from CookGene trait implemented on type i32
    println!("{}", "whatever".to_owned().cook());

    let bad_cooks = vec!["Stupid1".to_owned(), "Stupid2".to_owned()];
    let good_cooks = vec![1, 2];

    make_food_party(&bad_cooks);
    make_food_party(&good_cooks);

    let v = vec![A {}, A {}];
    println!("\n--> {}", v.cook());
    make_food_party(&v);

    let v = vec![ADebug(2), ADebug(44)];
    dbg!(&v); // trying some new feature here
    println!("\n--> {}", v.cook());

    let some_cooks: Vec<Box<CookGene>> = vec![Box::new(A {}), Box::new(22)];
    make_food_party_dyn(&some_cooks);
}
