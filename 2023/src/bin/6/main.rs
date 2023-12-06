const INPUT: [(u64, u64); 4] = [(40, 233), (82, 1011), (84, 1110), (92, 1487)];

fn main() {
    let solve = |t, record| {
        (0..t)
            .map(|tpress| tpress * (t - tpress))
            .filter(|&dist| dist > record)
            .count()
    };

    println!(
        "Answer #1: {}",
        INPUT
            .into_iter()
            .fold(1, |a, (t, record)| a * solve(t, record))
    );

    println!("Answer #2: {}", solve(40828492, 233101111101487));
}
