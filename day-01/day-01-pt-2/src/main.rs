use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];



    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut mod1  = 0;
    let mut mod2  = 0;
    let mut mod3  = 0;
    let mut increases = 0;
    let mut decreases = 0;
    let mut oldie = 0;
    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("wth");
        let reading =  line.parse().unwrap();
        if index < 3 {
            match index {
                0 => mod1 =reading,
                1 => mod2 =reading,
                2 => mod3 =reading,
                _=> return,
            }
            continue;
        }

        let modulus = index %3;
        match modulus{
            0 => mod1 =reading,
            1 => mod2 =reading,
            2 => mod3 =reading,
            _=> return,
        }

        let current =  mod1+mod2+mod3;

        if oldie < current && oldie != 0{
            increases += 1;
        }
        if oldie> current && oldie != 0{
            decreases += 1
        }


        
        oldie =  current;
    } 

    println!("increases {}, decreases {}", increases, decreases);
    return


}
