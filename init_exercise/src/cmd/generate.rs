/// This module contains source for the `generate` command.
use clap::ArgMatches;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
    process::Command,
};

static GITIGNORE_CONTENT: &'static str = "# Generated by Cargo
# will have compiled files and executables
/target/
**/*.rs.bk

# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
# More information here http://doc.crates.io/guide.html#cargotoml-vs-cargolock
Cargo.lock
";

static EXAMPLE_RS_CONTENT: &'static str = "//! Example implementation
//!
//! - Implement the solution to your exercise here.
//! - Put the stubs for any tested functions in `src/lib.rs`,
//!   whose variable names are `_` and
//!   whose contents are `unimplemented!()`.
//! - If your example implementation has dependencies, copy
//!   `Cargo.toml` into `Cargo-example.toml` and then make
//!   any modifications necessary to the latter so your example will run.
//! - Test your example by running `../../bin/test-exercise`
";

// Generate a new exercise with specified name and flags
fn generate_exercise(exercise_name: &str, run_configure: bool, use_maplit: bool) {
    let rev_parse_output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
        .expect("Failed to get the path to the track repo.");

    let track_root = String::from_utf8(rev_parse_output.stdout).unwrap();

    let exercise_path = Path::new(&track_root.trim())
        .join("exercises")
        .join(exercise_name);

    if exercise_path.exists() {
        panic!(
            "Exercise with the name {} already exists. Aborting",
            exercise_name
        );
    }

    println!(
        "Generating a new exercise at the following path: {}",
        exercise_path.to_str().unwrap()
    );

    let _cargo_new_output = Command::new("cargo")
        .arg("new")
        .arg("--lib")
        .arg(exercise_path.to_str().unwrap())
        .output()
        .expect("Failed to generate a new exercise via 'cargo new' command");

    ::std::fs::write(exercise_path.join(".gitignore"), GITIGNORE_CONTENT)
        .expect("Failed to create .gitignore file");

    if use_maplit {
        let mut cargo_toml_file = OpenOptions::new()
            .append(true)
            .open(exercise_path.join("Cargo.toml"))
            .unwrap();

        cargo_toml_file
            .write(b"maplit = \"1.0.1\"")
            .expect("Failed to add maplit dependency to the Cargo.toml");
    }

    ::std::fs::create_dir(exercise_path.join("tests"))
        .expect("Failed to create the tests directory");

    let mut test_file = File::create(
        exercise_path
            .join("tests")
            .join(format!("{}.rs", exercise_name)),
    ).expect("Failed to create test suite file");

    if use_maplit {
        test_file.write(b"#[macro_use]\nextern crate maplit;\n");
    }

    test_file
        .write(&format!("extern crate {};\n", exercise_name.replace("-", "_")).into_bytes())
        .unwrap();

    test_file
        .write(&format!("use {}::*;\n\n", exercise_name.replace("-", "_")).into_bytes())
        .unwrap();

    ::std::fs::write(exercise_path.join("example.rs"), EXAMPLE_RS_CONTENT)
        .expect("Failed to create example.rs file");
}

pub fn process_matches(matches: &ArgMatches) {
    let exercise_name = matches.value_of("exercise_name").unwrap();

    let run_configure = !matches.is_present("no_configure");

    let use_maplit = matches.is_present("use_maplit");

    generate_exercise(exercise_name, run_configure, use_maplit);
}
