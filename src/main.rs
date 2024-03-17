use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    #[arg(value_parser = parse_num)]
    nums: Vec<i64>,
}

fn parse_num(src: &str) -> Result<i64, std::num::ParseIntError> {
    let src = src.to_ascii_lowercase().replace('_', "");
    if let Some(hex) = src.strip_prefix("0x") {
        return i64::from_str_radix(hex, 16);
    }
    if let Some(bin) = src.strip_prefix("0b") {
        return i64::from_str_radix(bin, 2);
    }
    src.parse()
}

fn main() {
    let Args { nums } = Args::parse();

    let mut w1 = 0;
    let mut w2 = 0;
    let mut w3 = 0;
    for n in nums.iter() {
        let s = n.to_string();
        let l1 = s.len();
        let l2 = format!("{:b}", *n).len();
        let l3 = format!("{:x}", *n).len();
        if l1 > w1 {
            w1 = l1;
        }
        if l2 > w2 {
            w2 = l2;
        }
        if l3 > w3 {
            w3 = l3;
        }
    }

    let w1 = w1.max(4);

    match w2 % 4 {
        0 => {}
        n => w2 += 4 - n,
    }

    if w3 % 2 != 0 {
        w3 += 1
    }

    println!(
        "{0:>1$}    {2:>3$}    {4:>5$}",
        "#10",
        w1,
        "#2",
        w2 + 3,
        "#16",
        w3 + 3
    );
    for n in nums {
        println!("{0:>1$}    0b_{2:03$b}    0x_{4:05$x}", n, w1, n, w2, n, w3);
    }
}
