fn main() {
    let stream: Vec<char> = include_str!("../input.txt").chars().collect();
    let size = 14;

    let (index, chars) = stream.windows(size)
                                        .enumerate()
                                        .find(|(index , chars)| four_unique(chars))
                                        .unwrap();
    println!("Solution: {}", index + size); 
}



fn four_unique(chars: &[char]) -> bool {
    let mut vec: Vec<&char> = Vec::with_capacity(4);
    for c in chars {
        if vec.contains(&c) {
            return false;
        }
        vec.push(c);
    }
    return true;
}