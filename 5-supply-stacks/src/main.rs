use anyhow::anyhow;
use regex::Regex;
use std::collections::hash_map::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct Stacks(HashMap<u8, Vec<String>>);

impl Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, boxes) in self.0.iter() {
            f.write_str(format!("{}: {:?}\n", i, boxes).as_str())?;
        }

        Ok(())
    }
}

impl Display for Procedure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("move {} from {} to {}", self.amount, self.from, self.to).as_str())
    }
}

fn insert_row(stacks: &mut Stacks, line: String) -> anyhow::Result<()> {
    let r = Regex::new(r"\[([A-Z])\] |(    |   \n)|\[([A-Z])\]\n").unwrap();

    let matched: Vec<String> = r
        .captures_iter(&format!("{}\n", &line))
        .map(|m| {
            m.get(1)
                .or(m.get(2))
                .or(m.get(3))
                .unwrap()
                .as_str()
                .trim()
                .to_string()
        })
        .collect();

    for (i, b) in matched.into_iter().enumerate() {
        let i = i as u8;
        let stack = stacks.0.entry(i + 1).or_insert(Vec::new());
        if &b != "" {
            stack.push(b);
        }
    }

    Ok(())
}

#[derive(Debug)]
struct Procedure {
    amount: u8,
    from: u8,
    to: u8,
}

fn parse_procedure(procedure: String) -> anyhow::Result<Procedure> {
    let r = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let capture = r.captures(&procedure).ok_or(anyhow!("Invalid procedure"))?;
    let procedure = Procedure {
        amount: u8::from_str_radix(
            capture.get(1).ok_or(anyhow!("Invalid procedure"))?.as_str(),
            10,
        )?,
        from: u8::from_str_radix(
            capture.get(2).ok_or(anyhow!("Invalid procedure"))?.as_str(),
            10,
        )?,
        to: u8::from_str_radix(
            capture.get(3).ok_or(anyhow!("Invalid procedure"))?.as_str(),
            10,
        )?,
    };

    Ok(procedure)
}

fn handle_procedure(
    stacks: &mut Stacks,
    procedure: Procedure,
    keep_order: bool,
) -> anyhow::Result<()> {
    let from = stacks
        .0
        .get_mut(&procedure.from)
        .ok_or(anyhow!("From stack does not exist"))?;


    let amount = procedure.amount as usize;

    let boxes: Vec<String> = from.drain(0..amount).collect();

    drop(from);

    let to = stacks
        .0
        .get_mut(&procedure.to)
        .ok_or(anyhow!("To stack does not exist"))?;

    if keep_order {
        to.splice(0..0, boxes);
    } else {
        for b in boxes {
            to.insert(0, b);
        }
    }


    Ok(())
}

enum CrateMover {
    G9000,
    G9001
}

fn reorder_stacks(crate_mover: CrateMover) -> anyhow::Result<String> {
    let file = File::open("stacks.txt")?;
    let mut stacks = Stacks(HashMap::new());
    let mut lines = io::BufReader::new(file).lines();

    let lines: std::io::Lines<BufReader<File>> = loop {
        let line = if let Some(line) = lines.next() {
            line
        } else {
            break lines;
        }?;

        if &line == "" {
            break lines;
        }

        insert_row(&mut stacks, line)?;
    };


    for line in lines {
        let line = line?;

        if &line == "" {
            break;
        }

        let keep_order = match crate_mover {
            CrateMover::G9000 => false,
            CrateMover::G9001 => true
        };

        let procedure = parse_procedure(line)?;
        handle_procedure(&mut stacks, procedure, keep_order)?;
    }

    let mut top_boxes = String::new();

    let mut stacks_vec: Vec<(&u8, &Vec<String>)> = stacks.0.iter().collect();
    stacks_vec.sort_by(|a, b| a.cmp(b));

    for stack in stacks_vec {
        top_boxes.push_str(&stack.1[0]);
    }

    Ok(top_boxes)
}

fn main() -> anyhow::Result<()> {
    let g9000 = reorder_stacks(CrateMover::G9000);
    let g9001 = reorder_stacks(CrateMover::G9001);

    println!("CrateMover 9000: {}", g9000?);
    println!("CrateMover 9001: {}", g9001?);

    Ok(())
}
