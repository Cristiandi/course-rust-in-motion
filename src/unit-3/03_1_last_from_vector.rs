fn main() {
    let nomempty_list = vec!['a', 'b', 'c'];
    println!("nomempty_list's last is: {:?}", nomempty_list.last());

    let empty_list: Vec<char> = vec![];
    println!("empty_list's last is: {:?}", empty_list.last());
}
