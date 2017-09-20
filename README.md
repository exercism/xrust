# Exercism Rust Track

[![Build Status](https://travis-ci.org/exercism/rust.svg?branch=master)](https://travis-ci.org/exercism/rust)
[![Join the chat at https://gitter.im/exercism/rust](https://badges.gitter.im/exercism/rust.svg)](https://gitter.im/exercism/rust?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Exercism exercises in Rust

## Contributing

Thank you so much for contributing! :tada:

Please read about how to [get involved in a track](https://github.com/exercism/docs/tree/master/contributing-to-language-tracks). Be sure to read the Exercism [Code of Conduct](https://github.com/exercism/exercism.io/blob/master/CODE_OF_CONDUCT.md).

We welcome pull requests of all kinds. No contribution is too small.

We encourage contributions that provide fixes and improvements to existing exercises. Please note that this track's exercises must conform to the Exercism-wide standards described in the [documentation](https://github.com/exercism/docs/tree/master/language-tracks/exercises). If you're unsure about how to make a change, then go ahead and open a GitHub issue, and we'll discuss it.

## Exercise Tests

At the most basic level, Exercism is all about the tests. You can read more about how we think about test suites in [the Exercism documentation](https://github.com/exercism/docs/blob/master/language-tracks/exercises/anatomy/test-suites.md).

Test files should use the following format:

```
extern crate exercise_name;

use exercise_name::*;

#[test]
fn test_descriptive_name() {
    assert_eq!(exercise_function(1), 1);
}

#[test]
#[ignore]
fn test_second_and_past_tests_ignored() {
    assert_ne!(exercise_function(1), 2);
}
```

## Opening an Issue

If you plan to make significant or breaking changes, please open an issue so we can discuss it first. If this is a discussion that is relevant to more than just the Rust track, please open an issue in [exercism/discussions](https://github.com/exercism/discussions/issues).

## Submitting a Pull Request

Pull requests should be focused on a single exercise, issue, or conceptually cohesive change. Please refer to Exercism's [pull request guidelines](https://github.com/exercism/docs/blob/master/contributing/pull-request-guidelines.md).

- Follow the coding standards for Rust.  [rustfmt](https://github.com/nrc/rustfmt) may help with this
and can be installed with `cargo install rustfmt`.

### Verifying your Change

Before submitting your pull request, you'll want to verify the changes in two ways:

* Run all the tests for the Rust exercises
* Run an Exercism-specific linter to verify the track

All the tests for Rust exercises can be run from the top level of the repo with `_test/check-exercises.sh`. If you are on a Windows machine, there are additional [Windows-specific instructions](_test/WINDOWS_README.md) for running this.

For the Exercism-specific linting, please see [the documentation](https://github.com/exercism/docs/blob/master/language-tracks/configuration/linting.md).

## Contributing a New Exercise

- If a new exercise would potentially be suitable for a track other than the Rust track, it should be defined in [problem-specifications](https://github.com/exercism/problem-specifications/tree/master/exercises) so that other tracks may benefit from it.

- If the exercise is commonly defined, please make sure the exercise conforms to specifications in the [exercism/problem-specifications](https://github.com/exercism/problem-specifications) repo.  (Note if slight changes are needed for Rust specific issues, see `.meta/hints.md` below.)

- Each exercise must stand on its own. Do not reference files outside the exercise directory. They will not be included when the user fetches the exercise.

- Exercises should use only the Rust core libraries.  If you must use an external crate, see note below about `Cargo-example.toml`.  `Cargo.toml` should not have external dependencies as we don't want to make the user assume required crates.

- Each exercise should have:

---
    exercises/exercise-name/
                            tests/exercise-name.rs  <- a test suite
                            src/lib.rs              <- an empty file or with exercise stubs
                            example.rs              <- example solution that satisfies tests
                            Cargo.toml              <- with version equal to exercise defintion
                            Cargo.lock              <- Auto generated 
                            README.md               <- Instructions for the exercise (see notes below)
---

- Note: If publishing an existing exercise from problem-specifications, use the version `canonical-data.json` for that exercise as your `Cargo.toml` version.  Otherwise, use "0.0.0"

- An exercise may contain `.meta/hints.md`.  This is optional and will appear after the normal exercise
  instructions if present.  Rust is different in many ways from other languages.  This is a place where the differences required for Rust are explained.  If it is a large change, you may want to call this out as a comment at the top of `src/lib.rs`, so the user recognises to read this section before starting.

- If an `example.rs` uses external crates, include `Cargo-example.toml` so that `_tests/check-exercises.sh` can compile with these when testing.

- `README.md` may be regenerated from exercism data.  The top section above `## Rust Installation` should contain `description.md` from the exercise directory in the [problem-specifications repository.](https://github.com/exercism/problem-specifications/tree/master/exercises)  The `## Source` section comes from the `metadata.yml` in the same directory.  Convention is that the description of the source remains text and the link is both name and hyperlink of the markdown link.

- Be sure to add the exercise to an appropriate place in the `config.json` file.  The position in the file determines the order exercises are sent.   Generate a unique UUID for the exercise.  Current difficuly levels in use are 1, 4, 7 and 10.

## Rust icon
The Rust Logo is created by the Mozilla Corporation, and has been released under the [Creative Commons Attribution 4.0 International license](https://creativecommons.org/licenses/by/4.0/).
We tweaked the color from black to charcoal (#212121).
