use std::{env, io};
fn main() {
    let args: Vec<String> = env::args().collect();
    let asm = &args[1];
    let mut chr = "".to_string();
    let mut b = Vec::new();
    let mut start = 0;
    let mut end = 0;
    let mut id = 0;
    let mut score = 0.0;
    let check_asm_str = format!("{}.", asm);
    for line in io::stdin().lines() {
        let s = line.unwrap();
        if s.starts_with("s") {
            let r: Vec<&str> = s.split_ascii_whitespace().collect();
            if let Some(stripped) = r[1].strip_prefix(&check_asm_str) {
                chr = stripped.to_string();
                start = r[2].parse::<i32>().unwrap();
                let len = r[3].parse::<i32>().unwrap();
                end = start + len;
            }
            b.push(r[1..].join(":"))
        } else if let Some(s) = s.strip_prefix("a score=") {
            score = s.parse::<f32>().unwrap();
            if id > 0 {
                println!(
                    "{}\t{}\t{}\t{}_{}\t{}\t{}",
                    chr,
                    start,
                    end,
                    asm,
                    id,
                    score,
                    b.join(",")
                );
            }
            id = id + 1;
            b.clear();
        }
    }
    println!(
        "{}\t{}\t{}\t{}_{}\t{}\t{}",
        chr,
        start,
        end,
        asm,
        id,
        score,
        b.join(",")
    );
}
