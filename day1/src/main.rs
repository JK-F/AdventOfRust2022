
fn main() {
    let input = include_str!("../input.txt");
    let clusters = input.split("\n\n")
        .map(|cluster| cluster.lines().map(|line| line.parse::<u32>().unwrap()));
    
    let sums = clusters.map(|cluster| cluster.sum::<u32>());
    
    let res_one = sums.clone().max().unwrap();

    // Part 1:
    println!("{}", res_one);

    let mut sorted = sums.collect::<Vec<u32>>();
    sorted.sort();
    sorted.reverse();

    let res_two = sorted[0..=2].iter().sum::<u32>();

    // Part 2:
    println!("{}", res_two);
    
}
