fn main() {
    let mut letters = vec!['a', 'b', 'c', 'd'];

    for letter in letters {
        println!("{}", letter);
        letters.push(letter.clone());
    }
}
