use std::collections::HashSet;
use std::hash::Hash;

// Trait to convert to Hash Set
trait ToSet<T> {
    fn to_set(self) -> HashSet<T>;
}

// Implements ToSet trait for types that understand equality
// and implement interator. Lifetime for cloned()
impl <'a, T, I> ToSet<T> for I
where T: 'a + Eq + Hash + Clone, I: Iterator<Item = &'a T> {

    fn to_set(self) -> HashSet<T> {
        self.cloned().collect()          // cloned() to insure we don't run into lifetime issues
    }

}

fn make_set(words: &str) -> HashSet<String> {
    words.split_whitespace().map(|s| s.to_string()).collect()
}

fn main() {
    let fruit = make_set("apple orange pear orange");
    let colors = make_set("brown purple orange yellow");

    let intersect = fruit.intersection(&colors).to_set();
    
    println!("{:?}", intersect);    // Prints "orange"
}
