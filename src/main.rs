use std::process::ExitCode;
use std::env;
use glob::glob;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;
use std::io::Write;

fn main() -> ExitCode {
    let mut force: bool = false;
    let mut dir_path: Option<String> = None;
    for _arg in &env::args().collect::<Vec<String>>()[1..] {
        if _arg == "-f" {
            force = true;
        } else {
            dir_path = Some(_arg.to_string());
        }
    }
    if dir_path == None {
        return ExitCode::from(1);
    }

    if !force {
        print!("{}", dir_path.clone().unwrap() + ", Are you sure (y)? ");
        std::io::stdout().flush().unwrap(); // force flush

        let mut word = String::new();
        std::io::stdin().read_line(&mut word).unwrap();
        let answer = word.trim().to_string();
        if answer != "y" {
            return ExitCode::from(1);
        }
    }

    println!("Processing...");

    let mut path_list: Vec<PathBuf> = Vec::new();
    let absolute_dir_path: PathBuf = match fs::canonicalize(PathBuf::from(dir_path.unwrap())) {
        Ok(ap) => ap,
        Err(e) => {
            println!("Failed to read directory. Reason: {}", e.to_string());
            return ExitCode::from(1);
        }
    };

    let mut glob_pattern: PathBuf = absolute_dir_path.clone();
    glob_pattern.push("**/*");
    for entry in glob(&glob_pattern.to_string_lossy().into_owned()).unwrap() {
        let path: PathBuf = match entry {
            Ok(p) => p,
            Err(e) => {
                println!("Failed to read file or directory. Reason: {}", e.to_string());
                return ExitCode::from(1);
            }
        };
        path_list.insert(0, path);
    }

    for path in path_list {
        let filename: String = path.clone().file_name().unwrap()
            .to_string_lossy()
            .into_owned();

        let mut new_path: PathBuf = path.clone();
        new_path.set_file_name(PathBuf::from(filename.to_lowercase()));

        let mut tmp_path: PathBuf = absolute_dir_path.clone();
        tmp_path.push(".rtlc-".to_owned() + &Uuid::new_v4().to_string());

        match fs::rename(path.clone(), tmp_path.clone()) {
            Ok(_) => {},
            Err(e) => {
                println!(
                    "Could not rename `{}` to `{}`. Reason: {}",
                    path.display(),
                    tmp_path.clone().display(),
                    e.to_string(),
                );
                continue;
            },
        }
        match fs::rename(tmp_path.clone(), new_path.clone()) {
            Ok(_) => {},
            Err(e) => {
                println!(
                    "Could not rename `{}` to `{}`. Reason: {}",
                    tmp_path.display(),
                    new_path.display(),
                    e.to_string(),
                );
                continue;
            },
        }
    }

    return ExitCode::SUCCESS;
}
