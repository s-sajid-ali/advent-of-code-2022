use fs_err as fs;
use std::collections::HashMap;

mod parse;
mod types;
use crate::types::types::Line;

pub fn run(filename: String) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;
    let component_lines = contents.lines().collect::<Vec<_>>();

    // map of files from location string to size
    let mut files_map: HashMap<String, usize> = HashMap::new();
    let mut basename: Vec<String> = Vec::new();

    for line in component_lines {
        if line.is_empty() {
            break;
        } else {
            match parse::parse_line(line) {
                Some(res) => match res {
                    Line::LsCommand => {
                        //println!("ls command at: {}", basename.join("/"));
                    }
                    Line::CdCommand { location } => {
                        //println!("cd command with dst {}", location);
                        if location == ".." {
                            basename.pop();
                        } else if location == "/" {
                            /* do nothing for this case, the join method will add a / */
                        } else {
                            basename.push(location.to_string());
                        }
                        //println!("basename is : {}", basename.join("/"));
                    }
                    Line::DirOutput { .. } => {
                        //println!("dir with name {}", name);
                    }
                    Line::FileOutput { size, name } => {
                        //println!("file with name {} and size {} at location {} ", name, size, basename.join("/"));
                        basename.push(name.to_string());
                        files_map.insert(basename.join("/"), size);
                        basename.pop();
                    }
                },
                None => {
                    println!("failed to parse");
                }
            }
        }
    }

    let total_filesize = files_map.values().sum::<usize>();
    println!("total size of files is : {}", total_filesize);

    /* using the hash map of file-locations, file-sizes, create
     * a hashmap of directory-locations and directory-sizes */
    let mut dirs_map: HashMap<String, usize> = HashMap::new();
    for (key, value) in files_map.iter() {
        let filesize = value;
        let mut filename: String = key.to_string();
        let offset = filename.rfind('/').unwrap_or(0);

        let dirname: String = if offset == 0 {
            "/".to_string()
        } else {
            filename.drain(..offset).collect()
        };

        dirs_map
            .entry(dirname)
            .and_modify(|e| *e = *e + *filesize)
            .or_insert(*filesize);
    }

    /* hash map was probably a bad idea ...
     * we need to make a copy and add the missing
     * intermediate directories! */
    let mut dirs_map_concatenated = dirs_map.clone();
    for (key1, _) in dirs_map.iter() {
        let dirname: Vec<_> = key1.split('/').collect();
        for i in 0..dirname.len() {
            dirs_map_concatenated
                .entry(dirname[0..i].to_vec().join("/"))
                .or_insert(0);
        }
    }

    /* concatenate sub dir sizes into parent dirs! */
    for (key1, value1) in dirs_map_concatenated.iter_mut() {
        for (key2, value2) in dirs_map.iter() {
            if key2.contains(key1) & (key1 != key2) {
                *value1 = *value1 + *value2;
            }
        }
    }

    /* we already have the result for entry /, sum of sizes of all files. */
    _ = dirs_map_concatenated.insert("/".to_string(), total_filesize);
    _ = dirs_map_concatenated.remove_entry("");

    println!(
        "total size of dirs is : {}",
        dirs_map_concatenated
            .values()
            .filter(|&&size| size <= 100000)
            .sum::<usize>()
    );

    let curr_empty_size: usize = 70000000 - total_filesize;
    let req_empty_size: usize = 30000000;

    let req_size: usize = if curr_empty_size > req_empty_size {
        0
    } else {
        req_empty_size - curr_empty_size
    };

    if req_size != 0 {
        println!(
            "smallest dir that can be deleted has size is : {}",
            dirs_map_concatenated
                .values()
                .filter(|&&size| size >= req_size)
                .min()
                .expect("error when finding smallest viable dir to delete!")
        );
    } else {
        println!("required space is already available!");
    }

    //dbg!(dirs_map_concatenated);

    Ok(())
}
