fn main() {
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();

    let mut stack_strings = input[0..9].to_owned();
    stack_strings.reverse();
    stack_strings = stack_strings[1..stack_strings.len()].to_owned();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        stacks.push(vec![]);
    }

    for row in stack_strings {
        let row: Vec<char> = row.chars().collect();
        let crates = row.chunks(4);

        for (stack, c) in crates.enumerate() {
            let letter = c[1];
            if letter != ' ' {
                stacks[stack].push(letter);
            }
        }
    }

    let instructions = input[10..input.len()].to_owned();

    for instruction in instructions {
        let nums: Vec<usize> = parse_instruction(instruction);
        let amount = nums[0];
        let from = nums[1] - 1;
        let to = nums[2] - 1;
        move_part_2(amount, from, to, &mut stacks);
    }

    let res: Vec<&char> = stacks.iter().map(|v| v.last().unwrap_or(&' ')).collect();
    for c in res {
        print!("{}", c.to_ascii_uppercase());
    }

    print!("\n");
}

fn print_stacks(stacks: &Vec<Vec<char>>) {
    for stack in stacks {
        for c in stack {
            print!("{}, ", c);
        }

        print!("\n");
    }

}

fn parse_instruction(instruction: &str) -> Vec<usize> {
    let words: Vec<&str> = instruction.split(" ").collect();
    let mut res: Vec<usize> = Vec::new();
    for word in words {
        match word.parse::<usize>() {
            Ok(num) => res.push(num),
            Err(_)  => continue,
        }
    }
    res
}

fn move_part_1(amount: usize, from: usize, to: usize, stacks: &mut Vec<Vec<char>>) {
    for _ in 0..amount {
        let c = stacks[from].pop();
        match c {
            Some(c) => stacks[to].push(c),
            None => break,
        }
    }
}

fn move_part_2(amount: usize, from: usize, to: usize, stacks: &mut Vec<Vec<char>>) {
    let mut intermediate: Vec<char> = Vec::new();

    for _ in 0..amount {
        let c = stacks[from].pop().unwrap();
        intermediate.push(c);
    }
    intermediate.reverse();
    stacks[to].append(&mut intermediate);
}
