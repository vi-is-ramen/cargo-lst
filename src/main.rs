use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

use ignore::WalkBuilder;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut output_to_stderr = false;
    let mut output_file: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-2" => output_to_stderr = true,
            "-o" => {
                if i + 1 < args.len() {
                    output_file = Some(args[i + 1].clone());
                    i += 1; // consume the argument
                } else {
                    eprintln!("Error: -o requires a file argument");
                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!("Warning: unknown argument '{}'", args[i]);
            }
        }
        i += 1;
    }

    // Determine output destination: file > stderr > stdout
    let mut output: Box<dyn Write> = if let Some(ref path) = output_file {
        if std::fs::exists(path).or::<()>(Ok(false)).unwrap() {
            if std::fs::remove_file(path).is_err() {
                eprintln!("warning: can't remove previous output file");
            }
        }
        Box::new(BufWriter::new(File::create(path)?))
    } else if output_to_stderr {
        Box::new(BufWriter::new(io::stderr()))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    // Build the walker with .gitignore support and custom .lstignore
    let root = Path::new(".");
    let mut builder = WalkBuilder::new(root);
    builder.git_ignore(true); // enabled by default, but explicit
    builder.hidden(true); // collect hiddens too!

    // Add .lstignore if it exists in the current directory
    let lstignore_path = Path::new(".lstignore");
    if lstignore_path.exists() {
        builder.add_ignore(lstignore_path);
    }

    // Collect all entries (path, is_dir)
    let mut entries: Vec<(PathBuf, bool)> = Vec::new();
    for result in builder.build() {
        match result {
            Ok(entry) => {
                let path = entry.path().to_path_buf();
                let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                entries.push((path, is_dir));
            }
            Err(e) => eprintln!("Warning: could not read entry: {}", e),
        }
    }

    // Sort lexicographically by path
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    // ----- 1. Directory structure -----
    writeln!(output, "# Directory Structure\n")?;

    for (path, is_dir) in &entries {
        // depth is the number of components (relative to root ".")
        let depth = path.components().count() - 1;
        let indent = "  ".repeat(depth);
        let name = path.file_name().unwrap_or(OsStr::new("")).to_string_lossy();
        let suffix = if *is_dir { "/" } else { "" };
        writeln!(output, "{}- {}{}", indent, name, suffix)?;
    }

    // ----- 2. File contents for UTF-8 text files -----
    writeln!(output, "\n# File Contents\n")?;

    for (path, is_dir) in &entries {
        if *is_dir {
            continue;
        }

        // Try to read as UTF-8 text
        match std::fs::read_to_string(path) {
            Ok(content) => {
                let is_md_like = content.contains("```");
                let is_meta_md = content.contains("````");
                let path_str = path.to_string_lossy();
                writeln!(output, "## `{}`\n", path_str)?;
                write!(
                    output,
                    "{}",
                    if is_meta_md {
                        "`````"
                    } else if is_md_like {
                        "````"
                    } else {
                        "```"
                    }
                )?;
                if path.extension().is_some() {
                    let _ = writeln!(output, "{}", path.extension().unwrap().to_str().unwrap());
                }
                write!(output, "{}", content)?;
                writeln!(
                    output,
                    "{}",
                    if is_meta_md {
                        "\n`````\n"
                    } else if is_md_like {
                        "\n````\n"
                    } else {
                        "\n```\n"
                    }
                )?;
            }
            Err(_) => {
                // Not a valid UTF-8 file, skip silently
            }
        }
    }

    // Ensure all writes are flushed
    output.flush()?;
    Ok(())
}
