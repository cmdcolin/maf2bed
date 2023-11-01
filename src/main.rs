use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Provide assembly name e.g. hg38 as argument")
    }
    let assembly_name = &args[1];
    let mut chrom = "".to_string(); //: Option<String> = None;
    let mut buffer = Vec::new();
    let mut start = 0;
    let mut end = 0;
    let mut id = 0;
    let mut score = 0.0;
    let lines = io::stdin().lines();
    for line in lines {
        let s = line.unwrap();
        if s.starts_with("s") {
            let r: Vec<&str> = s.split_ascii_whitespace().collect();
            let pref = format!("{}.", assembly_name);
            if r[1].starts_with(&pref) {
                // println!("{} yy {}", r[2], r[3]);
                let stripped = r[1].strip_prefix(&pref);
                chrom = stripped.unwrap().to_string();
                start = r[2].parse::<i32>().unwrap();
                let len = r[3].parse::<i32>().unwrap();
                end = start + len;
                buffer.push(r[1..].join(":"))
            } else {
                buffer.push(r[1..].join(":"))
            }
        } else if s.starts_with("a") {
            score = s.strip_prefix("a score=").unwrap().parse::<f32>().unwrap();
            if id > 0 {
                print!(
                    "{}\t{}\t{}\t{}_{}\t{}\t{}",
                    chrom,
                    start,
                    end,
                    assembly_name,
                    id,
                    score,
                    buffer.join(",")
                );
            }
            id = id + 1;
            buffer.clear();
        }
    }
    print!(
        "{}\t{}\t{}\t{}_{}\t{}\t{}",
        chrom,
        start,
        end,
        assembly_name,
        id,
        score,
        buffer.join(",")
    );
}
