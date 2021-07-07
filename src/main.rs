use std::fs::File;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
fn main()  {
    if let Ok(lines) = read_lines("wordlist.txt") {
        let mut words = HashMap::new();
        for line in lines {
            if let Ok(ip) = line {
                let vec: Vec<&str> = ip.split("\t").collect();
                words.insert(
                   vec[0].parse::<u32>().unwrap(),
                   String::from(vec[1])
                    );
            }
        }
        let numbers: Vec<u32>;
        let mut rng = rand::thread_rng();
        let cast = Uniform::from(1..7);
        let mut result:String = "".to_string();
        for _ in 1..=6 {
            let mut x:u32=0;
            for _ in 1..=5 {
               x = 10*x + cast.sample(&mut rng);
            }
            let mut v: Vec<char>=words.get(&x).unwrap().to_string().chars().collect();
            v[0]=v[0].to_uppercase().nth(0).unwrap();
            let string:String=v.into_iter().collect();
            result = [result,string].join("");
        }
        println!("{}",result);
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
