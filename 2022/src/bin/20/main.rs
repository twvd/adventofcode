use std::fs;

const KEY: i128 = 811589153;

fn mix_nums(nums: &Vec<i128>, rounds: usize) -> Vec<i128> {
    let mut idxnums = nums
        .clone()
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, i128)>>();

    for _ in 0..rounds {
        for (idx, &num) in nums.iter().enumerate() {
            let curidx = idxnums.iter().position(|(i, _)| idx == *i).unwrap();
            let n = nums.len() - 1;
            let newidx: usize = (curidx as i128 + (num)).rem_euclid(n as i128) as usize;
            idxnums.remove(curidx);
            idxnums.insert(newidx, (idx, num));
        }
    }
    idxnums.iter().map(|(_, n)| *n).collect::<Vec<i128>>()
}

fn grove_coords(nums: &Vec<i128>) -> i128 {
    let zeroidx = nums.iter().position(|n| *n == 0).unwrap();
    nums[(zeroidx + 1000).rem_euclid(nums.len())]
        + nums[(zeroidx + 2000).rem_euclid(nums.len())]
        + nums[(zeroidx + 3000).rem_euclid(nums.len())]
}

fn main() {
    let inp = fs::read_to_string("inputs/20.txt").unwrap();
    let nums = inp
        .lines()
        .map(|n| n.trim().parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    println!("Answer #1: {}", grove_coords(&mix_nums(&nums, 1)));
    println!(
        "Answer #2: {}",
        grove_coords(&mix_nums(
            &nums.into_iter().map(|m| m * KEY).collect::<Vec<i128>>(),
            10
        ))
    );
}
