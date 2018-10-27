/// This module contains source for the `generate` command.
use exercise::Result;
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

static GITIGNORE_CONTENT: &'static str = "# Generated by exercism rust track exercise tool
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

// Generate .meta directory and it's contents without using the canonical data
fn generate_default_meta(exercise_name: &str, exercise_path: &Path) -> Result<()> {
    fs::create_dir(exercise_path.join(".meta"))?;

    fs::write(
        exercise_path.join(".meta").join("description.md"),
        "Describe your exercise here.\n\nDon't forget that `README.md` is automatically generated; update this within `.meta/description.md`.\n",
    )?;

    fs::write(
        exercise_path.join(".meta").join("metadata.yml"),
        format!(
            "---\nblurb: \"{}\"\nsource: \"\"\nsource_url: \"\"",
            exercise_name
        ),
    )?;

    let mut tests_file = OpenOptions::new().append(true).open(
        exercise_path
            .join("tests")
            .join(format!("{}.rs", exercise_name)),
    )?;

    tests_file.write_all(b"// Add your tests here")?;

    Ok(())
}

// Generate test suite using the canonical data
fn generate_tests_from_canonical_data(
    exercise_name: &str,
    exercise_path: &Path,
    canonical_data: &JsonValue,
    use_maplit: bool,
) -> Result<()> {
    exercise::update_cargo_toml_version(exercise_name, canonical_data)?;

    let tests_path = exercise_path
        .join("tests")
        .join(format!("{}.rs", exercise_name));

    let tests_content = exercise::get_tests_content(exercise_name)?;

    let updated_tests_content = format!(
        "//! Tests for {exercise_name} \n\
        //! \n\
        //! Generated by [utility][utility] using [canonical data][canonical_data]\n\
        //! \n\
        //! [utility]: https://github.com/exercism/rust/tree/master/util/exercise\n\
        //! [canonical_data]: https://raw.githubusercontent.com/exercism/problem-specifications/master/exercises/{exercise_name}/canonical-data.json\n\
        \n\
        {} \n\
        ",
        tests_content,
        exercise_name=exercise_name,
    );

    fs::write(&tests_path, updated_tests_content)?;

    let mut property_functions: HashMap<&str, String> = HashMap::new();

    let mut test_functions: Vec<String> = Vec::new();

    let cases = canonical_data
        .get("cases")
        .ok_or(format_err!("cases list not present in canonical data"))?;

    for case in cases
        .as_array()
        .ok_or(format_err!("case list inexpressable as array"))?
        .iter()
    {
        if let Some(sub_cases) = case.get("cases") {
            for sub_case in sub_cases
                .as_array()
                .ok_or(format_err!("subcase list inexpressable as array"))?
                .iter()
            {
                if let Some(property) = sub_case.get("property") {
                    let property = property
                        .as_str()
                        .ok_or(format_err!("property inexpressable as str"))?;

                    if !property_functions.contains_key(property) {
                        property_functions
                            .insert(property, exercise::generate_property_body(property));
                    }
                }

                test_functions.push(exercise::generate_test_function(&sub_case, use_maplit)?);
            }
        } else {
            if let Some(property) = case.get("property") {
                let property = property
                    .as_str()
                    .ok_or(format_err!("property inexpressable as str"))?;

                if !property_functions.contains_key(property) {
                    property_functions.insert(property, exercise::generate_property_body(property));
                }
            }

            test_functions.push(exercise::generate_test_function(&case, use_maplit)?);
        }
    }

    if !test_functions.is_empty() {
        let first_test_function = test_functions.remove(0).replace("#[ignore]\n", "");

        test_functions.insert(0, first_test_function);
    }

    let mut tests_file = OpenOptions::new().append(true).open(&tests_path)?;

    for (_, property_body) in &property_functions {
        tests_file.write_all(property_body.as_bytes())?;
    }

    tests_file.write_all(test_functions.join("\n\n").as_bytes())?;

    exercise::rustfmt(&tests_path)?;

    Ok(())
}

// Run bin/configlet generate command to generate README for the exercise
fn generate_readme(exercise_name: &str) -> Result<()> {
    println!(
        "Generating README for {} via 'bin/configlet generate'",
        exercise_name
    );

    let problem_specifications_path = Path::new(&*exercise::TRACK_ROOT)
        .join("..")
        .join("problem-specifications");

    if !problem_specifications_path.exists() {
        let problem_specifications_url = "https://github.com/exercism/problem-specifications.git";
        println!(
            "problem-specifications repository not found. Cloning the repository from {}",
            problem_specifications_url
        );

        Command::new("git")
            .current_dir(&*exercise::TRACK_ROOT)
            .stdout(Stdio::inherit())
            .arg("clone")
            .arg(problem_specifications_url)
            .arg(&problem_specifications_path)
            .output()?;
    }

    exercise::run_configlet_command(
        "generate",
        &[
            ".",
            "--only",
            exercise_name,
            "--spec-path",
            problem_specifications_path
                .to_str()
                .ok_or(format_err!("path inexpressable as str"))?,
        ],
    )?;

    Ok(())
}

// Generate a new exercise with specified name and flags
pub fn generate_exercise(exercise_name: &str, use_maplit: bool) -> Result<()> {
    if exercise::exercise_exists(exercise_name) {
        return Err(format_err!("exercise with the name {} already exists", exercise_name,).into());
    }

    let exercise_path = Path::new(&*exercise::TRACK_ROOT)
        .join("exercises")
        .join(exercise_name);

    println!(
        "Generating a new exercise at the following path: {}",
        exercise_path
            .to_str()
            .ok_or(format_err!("path inexpressable as str"))?
    );

    let _cargo_new_output = Command::new("cargo")
        .arg("new")
        .arg("--lib")
        .arg(
            exercise_path
                .to_str()
                .ok_or(format_err!("path inexpressable as str"))?,
        ).output()?;

    fs::write(exercise_path.join(".gitignore"), GITIGNORE_CONTENT)?;

    if use_maplit {
        let mut cargo_toml_file = OpenOptions::new()
            .append(true)
            .open(exercise_path.join("Cargo.toml"))?;

        cargo_toml_file.write_all(b"maplit = \"1.0.1\"")?;
    }

    fs::create_dir(exercise_path.join("tests"))?;

    let mut test_file = File::create(
        exercise_path
            .join("tests")
            .join(format!("{}.rs", exercise_name)),
    )?;

    if use_maplit {
        test_file.write_all(b"#[macro_use]\nextern crate maplit;\n")?;
    }

    test_file
        .write_all(&format!("extern crate {};\n", exercise_name.replace("-", "_")).into_bytes())?;

    test_file
        .write_all(&format!("use {}::*;\n\n", exercise_name.replace("-", "_")).into_bytes())?;

    fs::write(exercise_path.join("example.rs"), EXAMPLE_RS_CONTENT)?;

    match exercise::get_canonical_data(exercise_name) {
        Ok(canonical_data) => {
            println!("Generating tests from canonical data");

            generate_tests_from_canonical_data(
                &exercise_name,
                &exercise_path,
                &canonical_data,
                use_maplit,
            )?;
        }
        Err(e) => {
            eprintln!("Failed to get canonical data: {}", e);
            println!(
                "No canonical data for exercise '{}' found. Generating standard exercise template.",
                &exercise_name
            );

            generate_default_meta(&exercise_name, &exercise_path)?;
        }
    }

    generate_readme(&exercise_name)?;

    Ok(())
}
