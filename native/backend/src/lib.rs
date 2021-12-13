use std::env;
use std::path::{Path, PathBuf};
use std::*;
extern crate dirs;
use std::collections::HashMap;
use std::fs::*;

pub fn example_add(a: f32, b: f32) -> f32 {
 println!("[from native lib] example_add start");
 println!("[from native lib] a = {}, b = {}", a, b);
 let c = a + b;
 println!("[from native lib] c = {}", c);
 println!("[from native lib] example_add end");
 c
}

pub fn example_concat(a: &str, b: &str) -> String {
 println!("[from native lib] example_concat start");
 println!("[from native lib] a={}, b={}", a, b);
 let r = format!("{}{}", a, b);
 println!("[from native lib] r={}", r);
 println!("[from native lib] example_concat end");
 r
}

pub fn hello_from_rust() -> String {
 println!("[from native lib] hello_from_rust start");
 println!("[from native lib] hello_from_rust end");
 let phrase = "Saying hello from rust inside of node wrapped in electron!";
 phrase.to_string()
}

pub fn pwd() -> String {
 let path = env::current_dir();
 path.unwrap().to_str().unwrap().to_string()
}

pub fn help() -> String {
 String::from("The following commands are available to use\nls\npwd\ncd\nfind")
}
// for cd to $HOME directory
pub fn cd() -> String {
 let path = dirs::home_dir();
 match path {
  Some(p) => {
   let _new_dir = env::set_current_dir(p);
   let curr_dir = env::current_dir();
   match curr_dir {
    Ok(dir) => {
     dir.to_str().unwrap().to_string() // returns current home directory path
    }
    Err(_e) => String::from("Error!"), // should never occur
   }
  }
  None => String::from("Error! No $HOME directory found."),
 }
}

pub fn cd_with_args(dest: &str) -> String {
 let curr_dir = env::current_dir();
 match curr_dir {
  Ok(dir) => {
   let found_dirs: &HashMap<PathBuf, bool> = &get_dir_files(&dir);

   // for cd to parent directory
   if dest.trim() == ".." {
    let parent = dir.as_path().parent();
    let new_dir = env::set_current_dir(parent.unwrap());
    match new_dir {
     Ok(()) => {
      let curr_dir = env::current_dir();
      match curr_dir {
       Ok(dir) => {
        return dir.to_str().unwrap().to_string(); // returns current directory path
       }
       Err(_) => return String::from("Error! No parent directory found!"),
      }
     }
     Err(_) => {
      return String::from("Error!"); // Should never happen
     }
    }
   }

   // for cd to child directory
   if found_dirs.is_empty() {
    return String::from("No directory found!");
   }

   // Now we have a list of available directories, check if given destination matches any and cd if so
   for d in found_dirs {
    if *d.1 && d.0.to_str().unwrap() == dest {
     let new_dir = env::set_current_dir(d.0);
     match new_dir {
      Ok(_) => return d.0.to_str().unwrap().to_string(),
      Err(_) => return String::from("Error! Directory could not be moved to."),
     }
    } else if !(*d.1) && d.0.to_str().unwrap() == dest {
     return String::from("That is a file not a directory!");
    }
   }

   String::new()
  }
  Err(_e) => String::from("Error! Invalid \"cd\" call."),
 }
}

pub fn get_dir_files(dir: &Path) -> HashMap<PathBuf, bool> {
 let mut found_dirs: HashMap<PathBuf, bool> = HashMap::new(); // HashMap(K: Pathbuf, V: Bool) -- Value true if path is directory
 let dir_paths = fs::read_dir(dir.to_str().unwrap()).unwrap();

 for d in dir_paths {
  match d {
   Ok(p) => match p.metadata() {
    Ok(metadata) => {
     if metadata.is_dir() {
      found_dirs.insert(p.path(), true);
     } else {
      found_dirs.insert(p.path(), false);
     }
    }
    Err(e) => eprintln!("Error! {}", e),
   },
   Err(e) => eprintln!("Error! {}", e),
  }
 }

 found_dirs
}

pub fn mkdir(path: &str) -> String {
 let new_dir = create_dir(path);
 match new_dir {
  Ok(()) => {
   String::from(path) // Returns given path
  }
  Err(_) => {
   // Occurs when the parent of given path does not exist
   let _ = create_dir_all(path); // Creates new directory and all required parents for it
   String::from(path)
  }
 }
}

#[cfg(test)]
mod tests {
 use super::*;
 #[test]
 fn add() {
  let a = -1.23f32;
  let b = 4.56f32;
  let expected = a + b;
  let actual = example_add(a, b);
  assert_eq!(actual, expected);
 }

 #[test]
 fn concat() {
  let a = "su";
  let b = "shi";
  let expected = format!("{}{}", a, b);

  let actual = example_concat(a, b);

  assert_eq!(actual, expected);
 }

 #[test]
 fn hello() {
  let expected = "Saying hello from rust inside of node wrapped in electron!";
  let actual = hello_from_rust();

  assert_eq!(actual, expected);
 }
}
