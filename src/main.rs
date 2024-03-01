use rs_linked_list::list_unsafe::LinkedList;

fn main() {
    let _a = 5;
    let mut list = LinkedList::new();
    list.push_left(3.0);
    let el = list.pop_left().unwrap();
    println!("Hello, world! {}", el);
}
