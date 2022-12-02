use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    if let Ok(lines) = read_lines("input.txt") {
        let mut depths = vec![];
        for line in lines {
            let depth = line.unwrap();
            depths.push(depth.parse::<i32>().unwrap());
        }

        let mut previous = 0;
        let mut increases = 0;
        for (i, depth) in depths.iter().enumerate() {
            if i == 0 {
                previous = *depth;
                continue;
            }
            if *depth > previous {
                increases += 1;
            }
            previous = *depth;
        }
        println!("Part 1: There were {} increases in depth.", increases);

        let mut rolling = vec![];
        let endpoint = depths.len() - 1;
        for i in 1..endpoint {
            rolling.push(depths[i] + depths[i-1] + depths[i+1]);
        }

        let mut previous = 0;
        let mut increases = 0;
        for (i, depth) in rolling.iter().enumerate() {
            if i == 0 {
                previous = *depth;
                continue;
            }
            if *depth > previous {
                increases += 1;
            }
            previous = *depth;
        }
        println!("Part 2: There were {} increases in rolling-sum depth.", increases);

    }
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
