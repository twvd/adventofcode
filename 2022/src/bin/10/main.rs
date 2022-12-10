use std::fs;

const CRT_W: usize = 40;
const CRT_H: usize = 6;
type Crt = [[bool; CRT_W]; CRT_H];

const RECORD_AT: [i32; 6] = [20, 60, 100, 140, 180, 220];

fn render_crt(crt: &Crt) {
    for ch in crt {
        println!("{}", ch.map(|b| if b { "#" } else { " " }).join(""));
    }
}

fn run(inp: &Vec<&str>) -> Result<(i32, Crt), &'static str> {
    let mut cycles: i32 = 0;
    let mut regx: i32 = 1;
    let mut strengths: i32 = 0;
    let mut crt: Crt = [[false; CRT_W]; CRT_H];

    let mut cycle = |x: i32| {
        let crt_y = cycles as usize / CRT_W % CRT_H;
        let crt_x = cycles as usize % CRT_W;
        if ((x - crt_x as i32) as i32).abs() <= 1 {
            crt[crt_y][crt_x] = true;
        }

        cycles += 1;
        if RECORD_AT.contains(&cycles) {
            strengths += cycles * x;
        }
    };

    for instr in inp {
        let instrs: Vec<&str> = instr.split(' ').collect();

        match instrs[0] {
            "noop" => cycle(regx),
            "addx" => {
                let val = instrs[1].parse::<i32>().unwrap();
                cycle(regx);
                cycle(regx);
                regx += val;
            }
            _ => return Err("Parse error"),
        }
    }
    Ok((strengths, crt))
}

fn main() {
    let inp = fs::read_to_string("inputs/10.txt").unwrap();
    let l = inp.lines().map(|s| s.trim()).collect::<Vec<&str>>();

    let (part1, part2) = run(&l).unwrap();

    println!("Answer #1: {}", part1);
    println!("Answer #2:");
    render_crt(&part2);
}
