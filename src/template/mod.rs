
/// Creates the constant `DAY` and sets up the input and runner for each part.
#[macro_export]
macro_rules! solution {
    ($day:expr) => {
        use std::fs::File;
        use std::io::{self, Read};

        fn main() {
            for i in 1..4 {
                process(01,i);
            }
        }

        fn process(day:i8, part:i8){
            if let Ok(x) = read_file(day, part) {
                print!("{}",x);
                let res = match part {
                    1 => part_one(x),
                    2 => part_two(x),
                    3 => part_three(x),
                    _ => unreachable!()
                };
                println!("Part {}: {}",part, res);
            }
        }

        fn read_file(day: i8, part: i8) -> Result<String, io::Error> {
            let mut file = File::open(format!("data/inputs/{:02}-{}.txt", day, part))?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(contents)
        }
    };
}