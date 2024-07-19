use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, Write},
};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build(BuildArgs),
    Check(CheckArgs),
}

#[derive(Parser)]
#[command(version, about = "Build GTA V Bahasa Indonesia to OIV (OpenIV)")]
struct BuildArgs {
    #[arg(short, long, default_value = "GTA-V-Bahasa-Indonesia-1.3.oiv")]
    dest_path: String,
}

#[derive(Parser)]
#[command(
    version,
    about = "Check for duplicate keys across multiple .oxt files in a specified folder"
)]
struct CheckArgs {
    #[arg(short, long, default_value = "update2")]
    path: String,
}

fn check_duplicate_key(list_file: Vec<String>) -> io::Result<()> {
    let mut duplicate_keys: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    let ignored_keys: Vec<&str> = vec!["Version 2 30", ""];

    for file_path in list_file {
        let file = File::open(file_path.clone())?;
        let reader = io::BufReader::new(file);
        let mut in_block = false;

        for (line_num, line) in reader.lines().enumerate() {
            let line = line?;
            let trimeed = line.trim();

            if trimeed == "{" {
                in_block = true;
            } else if trimeed == "}" {
                in_block = false;
            } else if in_block && trimeed.contains('=') {
                let key = trimeed.split('=').next().unwrap().trim().to_string();
                if !ignored_keys.contains(&key.as_str()) {
                    duplicate_keys
                        .entry(key)
                        .or_default()
                        .push((file_path.clone(), line_num + 1))
                }
            }
        }
    }

    for (key, occurrences) in duplicate_keys.iter() {
        if occurrences.len() > 1 {
            println!("Duplicate key: {}", key);
            for (file_path, line_num) in occurrences {
                println!("  {}:{}", file_path, line_num);
            }
            println!()
        }
    }

    Ok(())
}

fn build_to_oiv(dest_path: &str) -> io::Result<()> {
    let file = File::create(dest_path)?;
    let mut zip = zip::ZipWriter::new(file);

    let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let mut assembly = File::open("assembly.xml").expect("assembly.xml not found");
    let mut american_rpf = File::open("american_rel.rpf").expect("american_rel.rpf not found");
    let mut menyoo_translate =
        File::open("Menyoo/Indonesian.json").expect("Menyoo/Indonesian.json not found");

    zip.add_directory("", options)?;

    zip.start_file("assembly.xml", options)?;
    io::copy(&mut assembly, &mut zip)?;

    zip.add_directory("content/", options)?;

    zip.start_file("content/american_rel.rpf", options)?;
    io::copy(&mut american_rpf, &mut zip)?;
    zip.add_directory("content/menyoo/", options)?;
    zip.start_file("content/menyoo/Indonesian.json", options)?;
    io::copy(&mut menyoo_translate, &mut zip)?;

    zip.finish()?;

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build(args) => {
            if std::path::Path::new(&args.dest_path).exists() {
                print!(
                    "Destination file '{}' already exists. Overwrite? (y/N): ",
                    args.dest_path
                );
                if let Err(e) = io::stdout().flush() {
                    println!("Failed to flush stdout: {}", e);
                    std::process::exit(1);
                }

                let mut input = String::new();
                if let Err(e) = io::stdin().read_line(&mut input) {
                    println!("Failed to read line: {}", e);
                    std::process::exit(1);
                }
                let input = input.trim().to_lowercase();
                if input == "n" || input == "no" {
                    println!("Operation cancelled.");
                    std::process::exit(1);
                }
            }
            match build_to_oiv(&args.dest_path) {
                Ok(_) => println!("Build to {} success", args.dest_path),
                Err(e) => println!("Build to {} failed: {}", args.dest_path, e),
            };
        }
        Commands::Check(args) => {
            if !std::path::Path::new(&args.path).exists() {
                println!("Path {:?} does not exist", args.path);
                std::process::exit(1);
            }
            if let Err(e) = std::fs::metadata(&args.path) {
                println!("{:?} is not a directory ({})", args.path, e);
                std::process::exit(1);
            }
            let mut list_file = Vec::new();
            for entry in std::fs::read_dir(args.path.clone()).unwrap() {
                let entry = entry.unwrap_or_else(|e| {
                    println!("Failed to read directory entry: {}", e);
                    std::process::exit(1);
                });
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "oxt") {
                    list_file.push(path.to_string_lossy().into_owned());
                }
            }
            if list_file.is_empty() {
                println!("No list file found in {}", args.path);
                std::process::exit(1);
            }
            if let Err(e) = check_duplicate_key(list_file) {
                println!("Check duplicate key failed: {}", e);
                std::process::exit(1);
            }
        }
    }
}
