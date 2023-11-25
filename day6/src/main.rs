fn main() {
    let stream: Vec<char> = include_str!("../input.txt").chars().collect();
    let size = 14;

    let (index, chars) = stream.windows(size)
                                        .enumerate()
                                        .find(|(index , chars)| all_unique(chars, size))
                                        .unwrap();
    println!("Solution: {}", index + size); 
}



fn all_unique(chars: &[char], size: usize) -> bool {
    let mut vec: Vec<&char> = Vec::with_capacity(size);
    for c in chars {
        if vec.contains(&c) {
            return false;
        }
        vec.push(c);
    }
    return true;
}