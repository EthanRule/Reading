fn main() {
    let fruit = vec!['p', 'e', 'a'];

    let buffer_overflow = fruit[4];
    assert_eq!(buffer_overflow, 'r');
}
