use std::str::FromStr;
use crate::days::Day;
use crate::util::number::parse_usize;

pub const DAY15: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    println!("Initialization sequence check result: {}", check_initialization_sequence(input));
}

fn puzzle2(input: &String) {
    println!("Initialization sequence check result: {}", run_initialization_sequence(input).unwrap());
}

fn run_hash(input: &str) -> usize {
    let mut hash = 0;

    for char in input.chars() {
        if !char.is_ascii() { panic!("Can only hash ascii chars") }
        hash += char as usize;
        hash *= 17;
        hash %= 256;
    }

    hash
}

fn check_initialization_sequence(input: &str) -> usize {
    input.split(",").map(|l| run_hash(l.trim())).sum()
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Operation {
    Remove,
    /** parameter is the focal strength of the lens **/
    Add(usize)
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Instruction {
    label: String,
    operation: Operation
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Lens {
    label: String,
    focal_strength: usize
}

fn run_initialization_sequence(input: &str) -> Result<usize, String> {
    // Each entry is a label and operation, two variants:
    // LAB=4 => Lens labelled 'LAB' with focus strength 4, needs to be inserted in the hash bucket determined by hashing the label
    // LAB- => Remove lens labelled 'LAB' from its hash bucket (if it's there)
    let mut hash_buckets: Vec<Vec<Lens>> = vec![];
    // Make sure we have enough buckets:
    for _ in 0..256 {
        hash_buckets.push(vec![]);
    }

    let instructions = input.split(",").map(|p| p.trim()).map(|p| p.parse::<Instruction>()).collect::<Result<Vec<_>, _>>()?;
    for instruction in instructions {
        let bucket_index = run_hash(&instruction.label);
        let vec = &mut hash_buckets[bucket_index];
        let existing_index = vec.iter().position(|lens| instruction.label == lens.label);

        match instruction.operation {
            Operation::Add(focal_strength) => {
                // Check if the lens already exists, if so, replace. Otherwise add to the end.
                match existing_index {
                    Some(index) => vec.get_mut(index).unwrap().focal_strength = focal_strength,
                    None => vec.push(Lens { label: instruction.label, focal_strength })
                }
            },
            Operation::Remove => {
                match existing_index {
                    Some(index) => { vec.remove(index); },
                    None => {} // No-op
                }
            }
        }
    }

    // The result is calculated by taking every lens and calculating it's value.
    // The value is: (bucket_index + 1) * (lens_index + 1) * (focal_strength)
    let mut result = 0;

    for i in 0..256 {
        let lenses = &hash_buckets[i];
        for li in 0..lenses.len() {
            result += (i+1) * (li+1) * lenses[li].focal_strength
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::days::day15::{run_hash, check_initialization_sequence, run_initialization_sequence};

    #[test]
    fn test_hash() {
        assert_eq!(run_hash("HASH"), 52);
    }

    #[test]
    fn test_initialization_sequence() {
        assert_eq!(check_initialization_sequence("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"), 1320);
    }

    #[test]
    fn test_run_initialization_sequence() {
        assert_eq!(run_initialization_sequence("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"), Ok(145));
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let operation_index = s.chars().position(|c| c == '=' || c == '-').ok_or(format!("Could not find a '=' or '-' in input '{}'", s))?;
        let label = s[0..operation_index].to_owned();
        let operation = match s.chars().nth(operation_index) {
            Some('=') => {
                let focal_strength = parse_usize(&s[operation_index+1..])?;
                Operation::Add(focal_strength)
            },
            Some('-') => Operation::Remove,
            _ => return Err("Could no longer find the operation char?!".to_string())
        };

        Ok(Self { label, operation })
    }
}