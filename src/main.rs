use std::fs::File;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main()  {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("wordlist.txt") {
        let mut words = HashMap::new();
        // Consumes the iterator, returns an (Optional) String
//        let mut passes : Vec<String>=Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                //println!("{}", ip);
                //
                let vec: Vec<&str> = ip.split("\t").collect();
                //let vec = split.collect::<Vec<&str>>();

//                passes.push(vec[1]);
                words.insert(
                   vec[0].parse::<u32>().unwrap(),
                   String::from(vec[1]) 
                    );
                //println!("{} : {}",vec[0].parse::<u32>().unwrap(),vec[1]);
                //for (key,value) in &words {
                //    println!("{} : {}",key,value);
                //}

                

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
           // println!("{}:{}",n);
           // match words.get(&x){
           //     Some(p) => println!("{}",p),
           //     None => println!("Error about to come!"),
           // }
            let mut v: Vec<char>=words.get(&x).unwrap().to_string().chars().collect();
            v[0]=v[0].to_uppercase().nth(0).unwrap();
            let string:String=v.into_iter().collect();
            result = [result,string].join("");
        }
        //println!("{}",x);
        //println!("{}",words.get(&x).unwrap())
        println!("{}",result);
        
    }
    
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

