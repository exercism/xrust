/// This module contains source for the `generate` command.
use clap::ArgMatches;
use reqwest::{self, StatusCode};
use serde_json::Value as JsonValue;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
    process::Command,
};
use toml::Value as TomlValue;

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

// Try to get the canonical data for the exercise of the given name
fn get_canonical_data(exercise_name: &str) -> Option<JsonValue> {
    let url = format!("https://raw.githubusercontent.com/exercism/problem-specifications/master/exercises/{}/canonical-data.json", exercise_name);

    let mut response =
        reqwest::get(&url).expect("Failed to make HTTP request for the canonical data.");

    if response.status() != StatusCode::Ok {
        return None;
    } else {
        return Some(
            response
                .json()
                .expect("Failed to parse the JSON canonical-data response"),
        );
    }
}

// Generate .meta directory and it's contents without using the canonical data
fn generate_default_meta(exercise_name: &str, exercise_path: &Path) {
    ::std::fs::create_dir(exercise_path.join(".meta"))
        .expect("Failed to create the .meta directory");

    ::std::fs::write(
        exercise_path.join(".meta").join("description.md"),
        "Describe your exercise here.\n\nDon't forget that `README.md` is automatically generated; update this within `.meta/description.md`.",
    ).expect("Failed to create .meta/description.md file");

    ::std::fs::write(
        exercise_path.join(".meta").join("metadata.yml"),
        format!(
            "---\nblurb: \"{}\"\nsource: \"\"\nsource_url: \"\"",
            exercise_name
        ),
    ).expect("Failed to create .meta/metadata.yml file");

    let mut tests_file = OpenOptions::new()
        .append(true)
        .open(
            exercise_path
                .join("tests")
                .join(format!("{}.rs", exercise_name)),
        )
        .unwrap();

    tests_file.write(b"// Add your tests here").unwrap();
}

// Update Cargo.toml of the generated exercise according to the fetched canonical data
fn update_cargo_toml(exercise_name: &str, exercise_path: &Path, canonical_data: &JsonValue) {
    let cargo_toml_content = ::std::fs::read_to_string(exercise_path.join("Cargo.toml"))
        .expect("Error reading Cargo.toml");

    let mut cargo_toml: TomlValue = cargo_toml_content.parse().unwrap();

    {
        let mut package_table = (&mut cargo_toml["package"]).as_table_mut().unwrap();

        package_table.insert(
            "version".to_string(),
            TomlValue::String(canonical_data["version"].as_str().unwrap().to_string()),
        );

        package_table.insert(
            "name".to_string(),
            TomlValue::String(exercise_name.replace("-", "_")),
        );
    }

    ::std::fs::write(exercise_path.join("Cargo.toml"), cargo_toml.to_string())
        .expect("Failed to update Cargo.toml file");
}

// Generate test suite using the canonical data
fn generate_tests_from_canonical_data(
    exercise_name: &str,
    exercise_path: &Path,
    canonical_data: &JsonValue,
    use_maplit: bool,
) {
    update_cargo_toml(exercise_name, exercise_path, canonical_data);
}

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

    if let Some(canonical_data) = get_canonical_data(exercise_name) {
        println!("Generating tests from canonical data");

        generate_tests_from_canonical_data(
            &exercise_name,
            &exercise_path,
            &canonical_data,
            use_maplit,
        );
    } else {
        println!(
            "No canonical data for exercise '{}' found. Generating standard exercise template.",
            &exercise_name
        );

        generate_default_meta(&exercise_name, &exercise_path);
    }
}

pub fn process_matches(matches: &ArgMatches) {
    let exercise_name = matches.value_of("exercise_name").unwrap();

    let run_configure = !matches.is_present("no_configure");

    let use_maplit = matches.is_present("use_maplit");

    generate_exercise(exercise_name, run_configure, use_maplit);
}
