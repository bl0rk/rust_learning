#![allow(dead_code)]
use std::fs; 
use std::env; 
use std::path::Path; 

// Note: This is one of the most earliest projects here and i'm aware that this isn't actually any real filetype changing and converting 
// and only renaming files. But the goal of this was mostly to change .webp files to .png or .jpg so i wouldn't have to deal with a browser 
// opening when just wanting to look at a image. 

pub fn change_filetype_run() { 
    let mut args = env::args(); //Get args. 
    args.next(); // Skip unneeded argument. 
    let args: Vec<String> = args.collect(); 

    match args.get(0) { 
        Some(arg) => { 
            if arg == "help" { 
                println!("[folder_path] .[filetype_from] .[filetype_to]"); 
                return; 
            } 
        }, 
        None => { 
            println!("Arguments required. Use the help argument to see all arguments!"); 
            return; 
        } 
    } 

    let mut config = match CFTConfig::build(args) { 
        Ok(conf) => conf, 
        Err(e) => { 
            println!("Error encountered: {e}"); 
            return; 
        } 
    }; 

    change_files(&mut config); 
} 

fn change_files(config: &mut CFTConfig) { 
    let path = Path::new(&config.path); 
    let mut changed_files = 0; 

    if !path.is_dir() { 
        return; 
    } 
    
    for entry in fs::read_dir(path).unwrap() { 
        let entry = entry.unwrap(); 
        let mut entry_path = entry.path().to_owned(); 

        if !entry_path.is_file() { 
            continue; 
        } 

        if !(format!(".{}", entry_path.extension().unwrap().to_str().unwrap()) == config.from) { 
            continue; 
        } 

        entry_path.set_file_name(format!("{}{}", entry_path.file_stem().unwrap().to_str().unwrap(), config.to)); 
        fs::rename(entry.path(), entry_path).unwrap(); 
        changed_files += 1; 
    } 

    println!("Changed <{}> files from <{}> to <{}>", changed_files, config.from, config.to); 
} 

// CFTConfig = ChangeFileTypeConfig 
struct CFTConfig { 
    path: String, 
    from: String, 
    to: String 
} 

impl CFTConfig { 
    fn build(mut args: Vec<String>) -> Result<CFTConfig, String> { 
        if args.len() != 3 { 
            return Err(String::from("3 Arguments required.")); 
        } 
        // Always remove from index 0 because everytime index 0 is removed all other values are indexed one lower. 
        Ok(CFTConfig { path: args.remove(0), from: args.remove(0), to: args.remove(0) }) 
    } 
} 