fn main() {
    let input = include_str!("../input.txt");

    let lines = input.lines();

    let scores = lines.clone().map(eval_round);

    let sum = scores.sum::<u32>();

    // Part 1
    println!("{}", sum);

    // Part 2
    let scores = lines.map(make_pairing);

    let sum = scores.sum::<u32>();
    println!("{}", sum);
}

fn make_pairing(hands: &str) -> u32 {
    let pair: Vec<&str> = hands.split(" ").collect();

    let elve = to_choice(pair[0]).unwrap();
    let human = make_result(elve, pair[1]);

    return eval_pairing(elve, human) + to_score(human);
}

fn make_result(elve: Choice, human: &str) -> Choice {
    match human {
        "X" => match elve {
            Choice::Rock => Choice::Scissor,
            Choice::Paper => Choice::Rock,
            Choice::Scissor => Choice::Paper,
        },
        "Y" => elve,
        "Z" => match elve {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissor,
            Choice::Scissor => Choice::Rock,
        },
        _ => elve,
    }
}

fn eval_round(hands: &str) -> u32 {
    let pair: Vec<&str> = hands.split(" ").collect();

    let elve = to_choice(pair[0]).unwrap();
    let human = to_choice(pair[1]).unwrap();

    return eval_pairing(elve, human) + to_score(human);
}

fn eval_pairing(elve: Choice, human: Choice) -> u32 {
    if elve == human {
        return 3;
    } else if human == Choice::Rock && elve == Choice::Scissor
        || human == Choice::Paper && elve == Choice::Rock
        || human == Choice::Scissor && elve == Choice::Paper
    {
        return 6;
    }
    return 0;
}

#[derive(PartialEq, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

fn to_choice(inp: &str) -> Option<Choice> {
    match inp {
        "A" | "X" => Some(Choice::Rock),
        "B" | "Y" => Some(Choice::Paper),
        "C" | "Z" => Some(Choice::Scissor),
        _ => None,
    }
}

fn to_score(c: Choice) -> u32 {
    match c {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissor => 3,
    }
}
