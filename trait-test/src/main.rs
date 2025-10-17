mod serialization;

use serialization::SearchParams;

use crate::serialization::SearchHash;

fn main() {
    let s = "?a=1&b=2";
    let sp = SearchParams{ src: s };
    let result1 = sp.serialize::<&str>().unwrap_or_else(|e| e.text);
    let result2 = sp.serialize::<SearchHash>().unwrap();
    println!("result1: {}", result1);
    println!("result2: {}", result2);
    println!("a: {}", result2["a"]);
    println!("c: {}", result2["c"]);
}
