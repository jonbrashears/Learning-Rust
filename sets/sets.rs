use std::collections::HashSet;
use std::hash::Hash;

// Trait to convert to Hash Set
trait ToSet<T> {
    fn to_set(self) -> HashSet<T>;
}

// Implements ToSet trait for types that understand equality
// and implement interator
impl <T, I> ToSet<T> for I
where T: Eq + Hash, I: Iterator<Item = T> {

    fn to_set(self) -> HashSet<T> {
        self.collect()
    }

}

fn make_set(words: &str) -> HashSet<String> {
    words.split_whitespace().map(|s| s.to_string()).collect()
}

fn main() {
    let fruit = make_set("apple orange pear orange");
    let colors = make_set("brown purple orange yellow");

    let intersect = fruit.intersection(&colors).cloned().to_set();
    
    println!("{:?}", intersect);    // Prints "orange"
}
