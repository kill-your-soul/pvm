use dirs::home_dir;
use serde::Serialize;
use std::collections::BTreeMap;
use std::env;
use std::fs;

#[derive(Debug, Default, Serialize)]
struct PythonPaths<'a> {
    python: BTreeMap<&'a str, PythonPath<'a>>,
}

#[derive(Serialize, Debug)]
struct PythonPath<'a> {
    #[serde(rename = "path")]
    path: Vec<&'a str>,
}

pub fn find_python_windows() {
    // println!("Windows")
    // find variable in path
    // if not found, return error
    // if found, return path
    let path = env::var("PATH").unwrap();
    // println!("PATH: {}", path);

    // split path into vector
    let paths: Vec<&str> = path.split(";").collect();
    // println!("PATHS: {:?}", paths);

    // find all Python in vector and add to another variable
    // create user_python_paths vector
    let mut user_python_paths: Vec<&str> = Vec::new();
    for path in paths.iter() {
        if path.contains("Python") {
            user_python_paths.push(path);
        }
    }
    // println!("USER PYTHON PATHS: {:?}", user_python_paths);

    // let python = paths.iter().find(|&x| x.contains("Python"));
    // println!("PYTHON: {:?}", python);

    // if python is found, return path
    // if python is not found, return error
    // match python {
    //     Some(python) => println!("Python found at: {}", python),
    //     None => eprintln!("Python not found"),
    // }

    // get system path
    let system_path = env::var("SystemRoot").unwrap();
    let system_paths: Vec<&str> = system_path.split(";").collect();
    // println!("SYSTEM PYTHON: {:?}", system_python);
    let mut system_python_paths: Vec<&str> = Vec::new();
    for path in system_paths.iter() {
        if path.contains("Python") {
            system_python_paths.push(path);
        }
    }
    // create toml file and write python ant system_python to it
    let mut map = PythonPaths::default();
    map.python.insert(
        "user",
        PythonPath {
            path: user_python_paths,
        },
    );

    map.python.insert(
        "system",
        PythonPath {
            path: system_python_paths,
        },
    );

    let toml_string = toml::to_string(&map).unwrap();
    let home = home_dir().unwrap();
    fs::write(home.join(".pvm/python.toml"), toml_string).unwrap();
}
