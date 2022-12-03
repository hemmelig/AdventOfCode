use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    if let Ok(lines) = read_lines("input.txt") {
        let mut cals = vec![];

        let mut counter = 0;
        for line in lines {
            let item = line.unwrap();

            match item.as_str() {
                "" => {
                    cals.push(counter);
                    counter = 0;
                },
                _ => {
                    counter += item.parse::<i32>().unwrap();
                },
            }
        }
        cals.sort();
        cals.reverse();

        // Puzzle 1
        println!(
            "Part 1: The maximum number of calories carried is {}.", cals[0]
        );

        // Puzzle 2
        let mut tmp = 0;
        for i in 0..3 {
            tmp += cals[i];
        }
        println!("Part 2: The top 3 elves are carrying {} calories.", tmp);
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
