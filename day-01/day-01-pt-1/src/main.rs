use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];



    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut oldie  = -1;
    let mut increases = 0;
    let mut decreases = 0;
    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("wth");
        let current =  line.parse().unwrap();
        if index == 0 {
            oldie =  current;
            continue;
        }
        if oldie < current{
            increases += 1;
        }
        if oldie> current {
            decreases += 1
        }
        
        oldie =  current;
    } 

    println!("increases {}, decreases {}", increases, decreases);
    return


}
