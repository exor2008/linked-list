use linked_list::list_unsafe::LinkedList;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut list = LinkedList::new();
    for _ in 0..1000 {
        for _ in 0..1000 {
            list.push_left(3.0);
        }
        for _ in 0..1000 {
            let _el = list.pop_left().unwrap();
        }
    }
    println!("Done! {}", now.elapsed().as_micros());
}
