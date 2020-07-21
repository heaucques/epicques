use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, LineWriter};
use std::collections::HashMap;

//use crate::blocks; // this doesnt work for some reason and i have no clue why

#[derive(Hash, Eq, PartialEq, Debug)]

pub enum Position {
    Coordinate {
        x: i64,
        y: i64,
    }
}
pub struct Info {
    pub tiles: HashMap<Position, u64>,
    map: HashMap<String, u64>,
}

fn main() -> io::Result<()> {
    #![allow(dead_code)]

    // if let Ok(()) = make_file(String::from("foo")) {
    //     println!("epicmost");
    // } else {
    //     println!("bruh");
    // }

    // if let Ok(()) = read_file(String::from("foo")) {
    //     println!("epicques");
    // } else {
    //     println!(":(");
    // }

    let mut i = Info::new();
    i.load()?;
    i.print_tiles();
    Ok(())
}

impl Info {
    pub fn new() -> Self {
        Info {
            tiles: HashMap::new(),
            map: HashMap::new(),
        }
    }
    pub fn load(&mut self) -> io::Result<()> {
        self.initiate();
        self.initiate_map();
        
        let file_name = "foo";

        self.read_file(file_name)?;

        Ok(())
    }

    fn initiate(&mut self) {
        self.tiles = HashMap::new();
    }
    
    fn make_file<S: Into<String>>(&mut self, name : S) -> io::Result<()> {
        #![allow(dead_code)]

        let file_name = format!("{}.txt", name.into());
    
        let file = File::create(file_name)?;
        let mut file = LineWriter::new(file);
    
        let v = vec!["2"; 5];
        file.write_all(b"Hello World\n")?;
    
        for thing in v {
            file.write_all([String::from(thing), String::from("\n")].concat().as_bytes())?;
            file.flush()?;
        }
        Ok(())
    }
    
    fn read_file<S: Into<String>>(&mut self, name : S) -> io::Result<()> {
        let file_name = format!("{}.txt", name.into());

        let file = File::open(file_name)?;
        let file = BufReader::new(file);
    
        let mut y = 0;
        let mut x = 0;

        for line in file.lines() {
            let tile = line.unwrap();
            
            let words : Vec<&str> = tile.split_whitespace().collect();

            let mut set_pos = false;

            let mut x_increment = 0;
            x -= x_increment;
            for word in words {
                match word {
                    word if word.parse::<i64>().is_ok() => {
                        let pos = word.parse::<i64>().unwrap();
                        if set_pos {
                            // invert the inverted y axis
                            y = -pos;
                        } else {
                            x = pos;
                        }
                        set_pos = true;  
                    },
                    word if word.parse::<String>().is_ok() => {
                        // println!("{}", x);
                        x_increment += 1;
                        self.tiles.insert(Position::Coordinate {
                            x: x,
                            y: y,
                        }, self.map_tiles(word.to_string()));
                    },
                    &_ => (),
                }
                // does shift but when it shifts no tiles are built so ¯\_(ツ)_/¯
                x += 1;
            }
            // so that it doesnt shift
            if !set_pos {
                y += 1;    
            }

        }
        Ok(())
    }

    fn initiate_map(&mut self) {
        match self.map.insert(String::from("none"), 0) {
            _ => ()
        }
        match self.map.insert(String::from("block"), 1) {
            _ => ()
        }

    }
    pub fn map_tiles(&self, word : String) -> u64 {
        match self.map.get_key_value(&word).expect("key value pair not found in the word to integer mapper in files.rs") {
            (_string, i) => *i,
        }
    }

    pub fn print_tiles(&mut self) {
        for tile in &self.tiles {
            println!("{:?}", tile);
        }
    }
}
impl Position {
    pub fn get_xy(&self) -> (i64, i64) {
        #![allow(dead_code)]

        match self {
            Self::Coordinate {x, y} => {
                (*x, *y)
            }
        }
    }
    
    pub fn get_x(&self) -> i64 {
        match self {
            Self::Coordinate {x, y: _} => {
                *x
            }
        }
    }

    pub fn get_y(&self) -> i64 {
        match self {
            Self::Coordinate {x: _, y} => {
                *y
            }
        }
    }
}
