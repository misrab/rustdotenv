
//func Read(filenames ...string) (envMap map[string]string, err error)
//func Load(filenames ...string) (err error)

use anyhow::{Error, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::hash::Hash;



pub fn read(filenames: Vec<&str>) -> Result<HashMap<String, String>> {
    let mut values = HashMap::new();
    
    for filename in filenames {
        println!("{:}", filename);
        
        let lines = read_lines(filename)?;
        for line in lines {
            if let Ok(kv) = line {
                println!("{:}", kv);
                if let Some((key, value)) = kv.split_once("=") {
                    let key = key.replace(&[' ', '"'][..], "");
                    let value = value.replace(&[' ', '"'][..], "");

                    values.insert(key, value);
                };
            }
        }
    
    }


    Ok(values)
} 

// courtesy: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html 
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// inspired by https://stackoverflow.com/questions/58615910/checking-two-hashmaps-for-identical-keyset-in-rust
fn hashmap_equal<T:std::cmp::Eq + std::hash::Hash,U:std::cmp::Eq>(map1:&HashMap<T,U>, map2:&HashMap<T,U>) -> bool {
  // Make sure that map1.keys() ⊆ map2.keys()
  for (key, value1) in map1 {
    match map2.get(key) {
      None => return false,
      Some(value2) => {
          if value1 != value2 {
              return false;
          } 
      }
    }
  }
  // If map1.keys() ⊆ map2.keys() and their sizes equal, then the sets are equal
  map1.len() == map2.len()
}

#[cfg(test)]
mod tests {
    use super::*;
   #[test]
   fn test_read() {
    let filenames = vec!["src/test_data/test1.env"];
    let res = read(filenames);

    assert_eq!(res.is_err(), false, "read should not return an error on test files: {:?}", res.err());

    println!("Result is {:?}", res);
  
    let result = res.unwrap();
    let mut expected: HashMap<String, String> = HashMap::new();
    expected.insert("ZOO".to_string(), "3".to_string());
    expected.insert("FOO".to_string(), "TWO".to_string());
    expected.insert("MOO".to_string(), "MOO1".to_string());
    
    assert_eq!(hashmap_equal(&expected, &result), true);
    }
   
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
