use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut doi;
    if args.len() > 1 {
        doi = args[1].clone();
    } else {
        eprintln!("DOI could not be parsed");
        return;
    }

    doi = doi.replace("/", "-");
    doi.push_str(".pdf");

    let mut target_file = match env::var("PO_STORAGE_DIR") {
        Ok(value) => PathBuf::from(value),
        Err(_) => env::current_dir().expect("Should be able to get current directory"),
    };

    target_file.push(&doi);

    Command::new("firefox")
        .arg(doi)
        .spawn()
        .expect("Failed to start firefox");
}
