use std::fs;

const SNAFUDIGIT: [char; 5] = ['=', '-', '0', '1', '2'];

fn snafu_to_int(snafu: &str) -> i128 {
    snafu
        .chars()
        .rev()
        .map(|c| SNAFUDIGIT.iter().position(|&d| c == d).unwrap() as i128 - 2)
        .enumerate()
        .fold(0i128, |a, (i, n)| a + (n * 5i128.pow(i as u32)))
}

fn int_to_snafu(tbc: i128) -> String {
    let mut i = tbc;
    let mut ret = String::new();
    while i > 0 {
        ret.insert(0, SNAFUDIGIT[((i + 2) % 5) as usize]);
        i = (i + 2) / 5;
    }
    ret
}

fn part1(inp: &str) -> String {
    int_to_snafu(inp.trim().lines().map(|l| snafu_to_int(l.trim())).sum())
}

fn main() {
    let inp = fs::read_to_string("inputs/25.txt").unwrap();

    assert_eq!(1, snafu_to_int("1"));
    assert_eq!(2, snafu_to_int("2"));
    assert_eq!(3, snafu_to_int("1="));
    assert_eq!(4, snafu_to_int("1-"));
    assert_eq!(5, snafu_to_int("10"));
    assert_eq!(6, snafu_to_int("11"));
    assert_eq!(7, snafu_to_int("12"));
    assert_eq!(8, snafu_to_int("2="));
    assert_eq!(9, snafu_to_int("2-"));
    assert_eq!(10, snafu_to_int("20"));
    assert_eq!(15, snafu_to_int("1=0"));
    assert_eq!(20, snafu_to_int("1-0"));
    assert_eq!(2022, snafu_to_int("1=11-2"));
    assert_eq!(12345, snafu_to_int("1-0---0"));
    assert_eq!(314159265, snafu_to_int("1121-1110-1=0"));

    assert_eq!(int_to_snafu(1), "1");
    assert_eq!(int_to_snafu(2), "2");
    assert_eq!(int_to_snafu(3), "1=");
    assert_eq!(int_to_snafu(4), "1-");
    assert_eq!(int_to_snafu(5), "10");
    assert_eq!(int_to_snafu(6), "11");
    assert_eq!(int_to_snafu(7), "12");
    assert_eq!(int_to_snafu(8), "2=");
    assert_eq!(int_to_snafu(9), "2-");
    assert_eq!(int_to_snafu(10), "20");
    assert_eq!(int_to_snafu(15), "1=0");
    assert_eq!(int_to_snafu(20), "1-0");
    assert_eq!(int_to_snafu(2022), "1=11-2");
    assert_eq!(int_to_snafu(12345), "1-0---0");
    assert_eq!(int_to_snafu(314159265), "1121-1110-1=0");

    println!("Answer #1: {}", part1(&inp));
}
