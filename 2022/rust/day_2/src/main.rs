use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    if let Ok(lines) = read_lines("input.txt") {
        let mut score_pt1 = 0;
        let mut score_pt2 = 0;
        for line in lines {
            let item = line.unwrap();
            let game = item.as_str();
            
            match game {
                "A X" => score_pt1 += 4,
                "A Y" => score_pt1 += 8,
                "A Z" => score_pt1 += 3,
                "B X" => score_pt1 += 1,
                "B Y" => score_pt1 += 5,
                "B Z" => score_pt1 += 9,
                "C X" => score_pt1 += 7,
                "C Y" => score_pt1 += 2,
                "C Z" => score_pt1 += 6,
                _ => continue,
            }

            match game {
                "A X" => score_pt2 += 3,
                "A Y" => score_pt2 += 4,
                "A Z" => score_pt2 += 8,
                "B X" => score_pt2 += 1,
                "B Y" => score_pt2 += 5,
                "B Z" => score_pt2 += 9,
                "C X" => score_pt2 += 2,
                "C Y" => score_pt2 += 6,
                "C Z" => score_pt2 += 7,
                _ => continue,
            }
        }
        println!("Part 1: Expected score of {}!", score_pt1);
        println!("Part 2: Expected score of {}!", score_pt2);
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
