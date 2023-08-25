fn main() {
    let lines = include_str!("../input.txt").lines();

    let sum_one: u32 = lines.clone().map(get_prio).sum();

    println!("{}", sum_one);

    let sum_two = lines
        .collect::<Vec<_>>()
        .chunks(3)
        .map(get_badge)
        .sum::<u32>();

    println!("{}", sum_two);
}

fn get_badge(chunk: &[&str]) -> u32 {
    for c in chunk[0].chars() {
        if chunk[1].contains(c) && chunk[2].contains(c) {
            return to_prio(c);
        }
    }
    return 0;
}

fn get_prio(line: &str) -> u32 {
    let half = (line.len() / 2) as usize;
    let (first, second) = line.split_at(half);

    for a in first.chars() {
        if second.contains(a) {
            return to_prio(a);
        }
    }
    return 0;
}

fn to_prio(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => 0,
    }
}

#[test]
fn test_prio() {
    let mut prio = 1;
    for c in 'a'..='z' {
        assert_eq!(to_prio(c), prio);
        prio += 1;
    }

    for c in 'A'..='Z' {
        assert_eq!(to_prio(c), prio);
        prio += 1;
    }
}
