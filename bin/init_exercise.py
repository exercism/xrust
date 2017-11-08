#!/usr/bin/env python
"""
Script to initialize an exercise for Exercism's Rust track

Why a Python script in the Rust track repo? Distribution.
A rust program would either need to be precompiled for various
platforms, and available for download (in which case it wouldn't
conveniently work in the repository), or would need to be included
as a sub-crate and compiled locally. A python script can simply
be a single file and depend on the user's system Python, if desired.

This module requires Python3.5 or newer
"""
from __future__ import print_function

try:
    import collections
    import json
    import os
    import shlex
    import subprocess
    import string
    import sys
    import urllib.request

    from contextlib import contextmanager
    from uuid import uuid4
except ImportError:
    print("This script requires Python 3.5 or higher", file=sys.stderr)
    # exiting like this isn't great for library use, but at least it's a quick fail
    sys.exit(1)

# check version info
if sys.version_info[0] != 3 or sys.version_info[1] < 5:
    print("This script requires Python 3.5 or higher", file=sys.stderr)
    # exiting like this isn't great for library use, but at least it's a quick fail
    sys.exit(1)


def output_of(cmd, check_returncode=True):
    "Return the stdout of the given command"
    sp = subprocess.run(shlex.split(cmd),
                        stdout=subprocess.PIPE,
                        universal_newlines=True)
    if check_returncode:
        sp.check_returncode()
    return sp.stdout.strip()


REPO_ROOT = output_of('git rev-parse --show-toplevel')
EXERCISES = os.path.join(REPO_ROOT, 'exercises')
ITEM_NAME_CHARS = {c for c in string.ascii_lowercase + string.digits + '_'}
VERSION = "0.1.0"


def to_item_name(description):
    "Produce a valid rust item name from arbitrary inputs"
    item = description.lower().replace(' ', '_')
    item = [c for c in item if c in ITEM_NAME_CHARS]
    while len(item) > 0 and item[0] in string.digits:
        item = item[1:]
    if len(item) == 0:
        raise ValueError("Could not produce an item name from " + description)
    return ''.join(item)


def to_crate_name(name):
    return name.replace('-', '_')


def url_for(name, file):
    return (
        "https://raw.githubusercontent.com/exercism/problem-specifications"
        f"/master/exercises/{name}/{file}"
    )


def get_problem_specification(name):
    """
    Try to get problem specifications for the exercise of the given name.

    If the problem specifications repo doesn't exist or the exercise does not
    exist within the specifications repo, returns None.
    Otherwise, returns a dict, of which the values might be None or str.
    """
    try:
        with urllib.request.urlopen(url_for(name, 'canonical-data.json')) as response:
            return json.loads(response.read())
    except (urllib.request.URLError, json.JSONDecodeError):
        pass


@contextmanager
def inside(path):
    cwd = os.getcwd()
    os.chdir(path)
    try:
        yield
    finally:
        os.chdir(cwd)


def make_exercise(name, use_maplit):
    "Make a new exercise with the specified name"
    with inside(EXERCISES):
        if os.path.exists(name):
            print(f"{name} already exists; aborting", file=sys.stderr)
            sys.exit(1)
        subprocess.run(['cargo', 'new', name])
    exercise_dir = os.path.join(EXERCISES, name)
    # blank out the default lib.rs
    with inside(exercise_dir):
        with open('.gitignore', 'w') as gitignore:
            print("Cargo.lock  # We're building a library, not a binary", file=gitignore)
            print("            # http://doc.crates.io/faq.html#why-do-binaries-have-cargolock-in-version-control-but-not-libraries", file=gitignore)
        with open(os.path.join('src', 'lib.rs'), 'w') as lib_rs:
            lib_rs.truncate()
        if use_maplit:
            with open('Cargo.toml', 'a') as cargo_toml:
                print("maplit = \"1.0.0\"", file=cargo_toml)
        os.mkdir('tests')
        with inside('tests'):
            with open(f'{name}.rs', 'w') as tests_rs:
                if use_maplit:
                    print("#[macro_use] extern crate maplit;", file=tests_rs)
                print(f"extern crate {to_crate_name(name)};", file=tests_rs)
                print(f"use {to_crate_name(name)}::*;", file=tests_rs)
                print(file=tests_rs)
        with open('example.rs', 'w') as example_rs:
            print(f"//! Example implementation for {name}", file=example_rs)
            print('//!', file=example_rs)
            print("//! - Implement the solution to your exercise here.", file=example_rs)
            print("//! - Put the stubs for any tested functions in `src/lib.rs`,", file=example_rs)
            print("//!   whose variable names are `_` and", file=example_rs)
            print("//!   whose contents are `unimplemented!()`.", file=example_rs)
            print("//! - If your example implementation has dependencies, copy", file=example_rs)
            print("//!   `Cargo.toml` into `Cargo-example.toml` and then make", file=example_rs)
            print("//!   any modifications necessary to the latter so your example will run.", file=example_rs)
            print("//! - Test your example by running `../../bin/test-exercise`", file=example_rs)

        cd = get_problem_specification(name)
        if cd is None:
            print(f"No problem specification for {name} found")
            make_new_exercise(name, exercise_dir)
            generate_readme(name, get_problem_specification=False)
        else:
            make_exercise_with_specifications(name, exercise_dir, cd, use_maplit)
            generate_readme(name, get_problem_specification=True)


def make_new_exercise(name, exercise_dir):
    print("Creating new exercise from scratch...")
    with inside(exercise_dir):
        os.mkdir('.meta')
        with inside('.meta'):
            with open('description.md', 'w') as description:
                print("Describe your exercise here", file=description)
                print(file=description)
                print("Don't forget that `README.md` is automatically generated; update this within `.meta\description.md`.", file=description)
            with open('metadata.yml', 'w') as metadata:
                print("---", file=metadata)
                print(f"blurb: \"{name}\"", file=metadata)
                print("source: \"\"", file=metadata)
                print("source_url: \"\"", file=metadata)
        with inside('tests'):
            with open(f'{name}.rs', 'a') as tests_rs:
                print("// Add your tests here", file=tests_rs)


def make_exercise_with_specifications(name, exercise_dir, canonical_data, use_maplit):
    print("Creating exercise from specification...")
    # Specify problem version
    if 'version' in canonical_data:
        with open('Cargo.toml', 'r') as cargo_toml:
            cargo_data = cargo_toml.read()
        with open('Cargo.toml', 'w') as cargo_toml:
            for line in cargo_data.splitlines():
                if line.lower().startswith('version'):
                    print(f"version = \"{canonical_data['version']}\"", file=cargo_toml)
                elif line.lower().startswith('name'):
                    print(f"name = \"{to_crate_name(name)}\"", file=cargo_toml)
                else:
                    print(line.strip(), file=cargo_toml)

    tests_filename = os.path.join(exercise_dir, 'tests', f'{name}.rs')
    # prepend doc comment about the nature of this file
    with open(tests_filename, 'r') as tests_rs:
        existing = tests_rs.read()
    with open(tests_filename, 'w') as tests_rs:
        print(f'//! Tests for {name}', file=tests_rs)
        print('//!', file=tests_rs)
        print('//! Generated by [script][script] using [canonical data][canonical-data]',
              file=tests_rs)
        print("//!", file=tests_rs)
        print("//! [script]: https://github.com/exercism/rust/blob/master/bin/init_exercise.py",
              file=tests_rs)
        print("//! [canonical-data]: {}".format(url_for(name, 'canonical_data.json')),
              file=tests_rs)
        if 'comments' in canonical_data:
            c = canonical_data['comments']
            print('//!', file=tests_rs)
            if isinstance(c, list) or isinstance(c, tuple):
                for l in c:
                    print(f'//! {l}', file=tests_rs)
            else:
                print(f'//! {c}', file=tests_rs)

        print(file=tests_rs)
        print(file=tests_rs)
        tests_rs.write(existing)

    # now add test data
    with open(tests_filename, 'a') as tests_rs:
        first_case = True

        # {property : {(input key names), ...}}
        PIK_MAP = {}

        def generate_pik_map(cases):
            nonlocal PIK_MAP
            for case in cases:
                if 'property' in case:
                    ikeys = get_input_keys(case)
                    if ikeys is not None:
                        pkeys = PIK_MAP.get(case['property'], set())
                        pkeys.add(ikeys)
                    PIK_MAP[case['property']] = pkeys
                if 'cases' in case:
                    generate_pik_map(case['cases'])

        def collect_properties(cases):
            properties = set()
            for case in cases:
                if 'expected' in case and 'property' in case:
                    properties.add(case['property'])
                if 'cases' in case:
                    properties |= collect_properties(case['cases'])
            return properties

        def property_processor(property):
            print(f"/// Process a single test case for the property `{property}`", file=tests_rs)
            print("///", file=tests_rs)
            print(f"/// All cases for the `{property}` property are implemented",
                  file=tests_rs)
            print("/// in terms of this function.", file=tests_rs)
            print('///', file=tests_rs)
            print("/// Note that you'll need to both name the expected transform which",
                  file=tests_rs)
            print("/// the student needs to write, and name the types of the inputs and outputs.",
                  file=tests_rs)
            print("/// While rustc _may_ be able to handle things properly given a working example,",
                  file=tests_rs)
            print("/// students will face confusing errors if the `I` and `O` types are not concrete.",
                  file=tests_rs)
            if property in PIK_MAP:
                print('///', file=tests_rs)
                if len(PIK_MAP[property]) == 1:
                    print(f"/// Expected input format: {next(iter(PIK_MAP[property]))}",
                          file=tests_rs)
                else:
                    print("/// CAUTION: Multiple input formats were detected in this test's cases:",
                          file=tests_rs)
                    for ifmt in PIK_MAP[property]:
                        print(f"///    {ifmt}")
            print(
                f"fn process_{property.lower()}_case<I, O>(input: I, expected: O) {{", file=tests_rs)
            print("    // typical implementation:", file=tests_rs)
            print("    // assert_eq!(", file=tests_rs)
            print(f"    //     student_{property}_func(input),", file=tests_rs)
            print("    //     expected", file=tests_rs)
            print("    // )", file=tests_rs)
            print("    unimplemented!()", file=tests_rs)
            print("}", file=tests_rs)
            print(file=tests_rs)

        def literal(item):
            if isinstance(item, str):
                return f'"{item}"'
            elif isinstance(item, tuple):
                return "({})".format(
                    ', '.join((literal(i) for i in item))
                )
            elif isinstance(item, list):
                return "vec![{}]".format(
                    ', '.join((literal(i) for i in item))
                )
            elif isinstance(item, dict):
                if use_maplit:
                    return "hashmap!{{{}}}".format(
                        ','.join((
                            "{}=>{}".format(literal(k), literal(v))
                            for k, v in item.items()
                        ))
                    )
                else:
                    return "{{let mut hm=::std::collections::HashMap::new();{}hm}}".format(
                        ''.join((
                            "hm.insert({}, {});".format(literal(k), literal(v))
                            for k, v in item.items()
                        ))
                    )
            else:
                return str(item)

        def write_case(case):
            nonlocal first_case

            print("#[test]", file=tests_rs)
            if first_case:
                first_case = False
            else:
                print("#[ignore]", file=tests_rs)
            print(f"/// {case['description']}", file=tests_rs)
            if 'comments' in case:
                print('///', file=tests_rs)
                if isinstance(case['comments'], list):
                    for line in case['comments']:
                        print(f"/// {line}", file=tests_rs)
                else:
                    print(f"/// {case['comments']}", file=tests_rs)
            print("fn test_{}() {{".format(to_item_name(case['description'])), file=tests_rs)
            print("    process_{}_case({}, {});".format(
                case['property'].lower(),
                literal(case['input']),
                literal(case['expected'])),
                file=tests_rs)
            print("}", file=tests_rs)
            print(file=tests_rs)

        def get_input_keys(item):
            if 'description' in item and 'expected' in item:
                return tuple(sorted(set(item.keys()) -
                                    {'comments',
                                     'description',
                                     'expected',
                                     'property'}
                                    ))
            # else None

        def write_cases(cases):
            for item in cases:
                if 'description' in item and 'expected' not in item:
                    if isinstance(item['description'], list):
                        for line in item['description']:
                            print(f"// {line}", file=tests_rs)
                    else:
                        print(f"// {item['description']}", file=tests_rs)
                if 'comments' in item and 'expected' not in item:
                    if isinstance(item['comments'], list):
                        for line in item['comments']:
                            print(f"// {line}", file=tests_rs)
                    else:
                        print(f"// {item['comments']}", file=tests_rs)
                if 'expected' not in item and 'comments' in item or 'description' in item:
                    print(file=tests_rs)
                if 'property' not in item:
                    item['property'] = ''
                if all(key in item for key in ('description', 'input', 'expected')):
                    write_case(item)
                elif 'description' in item and 'expected' in item:
                    item['input'] = tuple((item[k] for k in get_input_keys(item)))
                    write_case(item)
                if 'cases' in item:
                    write_cases(item['cases'])

        generate_pik_map(canonical_data['cases'])

        for ppty in collect_properties(canonical_data['cases']):
            property_processor(ppty)

        write_cases(canonical_data['cases'])


def prompt(prompt, validator):
    """
    Prompt the user for a value

    Validator is a function which accepts the user's input and either
    returns a (possibly transformed) value, or raises an exception.
    On an exception, the user is asked again.
    """
    while True:
        try:
            return validator(input(prompt).strip())
        except Exception as e:
            print(f"Problem: {e}")


def update_config(name):
    "Update the configuration based on user input"
    with inside(REPO_ROOT):
        with open('config.json') as config_json:
            config = json.load(config_json, object_pairs_hook=collections.OrderedDict)

    while True:
        conf_values = collections.OrderedDict()
        conf_values['uuid'] = str(uuid4())
        conf_values['slug'] = name
        conf_values['core'] = False

        def unlock_validator(v):
            if len(v) == 0:
                return None
            if not any(v == ex['slug'] for ex in config['exercises']):
                raise ValueError(f"{v} is not an existing exercise slug")
            return v
        conf_values['unlocked_by'] = prompt(
            "Exercise slug which unlocks this (blank for None): ", unlock_validator)

        def difficulty_validator(v):
            i = int(v)
            if i <= 0 or i > 10:
                raise ValueError("difficulty must be > 0 and <= 10")
            return i
        conf_values['difficulty'] = prompt(
            "Difficulty for this exercise([1...10]): ", difficulty_validator)

        def topics_validator(v):
            topics = [t.strip() for t in v.split(',') if len(t.strip()) > 0]
            if len(topics) == 0:
                raise ValueError("must enter at least one topic")
            return topics
        conf_values['topics'] = prompt(
            "List of topics for this exercise, comma-separated: ", topics_validator)

        print("You have configured this exercise as follows:")
        print(json.dumps(conf_values, sort_keys=True, indent=4))

        yn = input('Is this correct? (y/N): ').strip().lower()
        if len(yn) > 0 and yn[0] == 'y':
            break

    if not any(conf_values['difficulty'] == ex['difficulty'] for ex in config['exercises']):
        config['exercises'].append(conf_values)
        config['exercises'].sort(key=lambda ex: ex['difficulty'])
    else:
        # find the index bounds before which we might insert this
        first_idx = None
        last_idx = None
        for idx, exercise in enumerate(config['exercises']):
            if 'difficulty' in exercise and exercise['difficulty'] == conf_values['difficulty'] and first_idx is None:
                first_idx = idx
            if 'difficulty' in exercise and exercise['difficulty'] != conf_values['difficulty'] and first_idx is not None:
                last_idx = idx
        if last_idx is None:
            last_idx = len(config['exercises'])

        def binary_search(start_idx, end_idx):
            if start_idx == end_idx:
                return start_idx
            mid_idx = start_idx + ((end_idx - start_idx) // 2)

            def easy_hard_validator(v):
                v = v.lower()[0]
                if v not in {'e', 'h'}:
                    raise ValueError("must enter 'easier' or 'harder' or a substring")
                return v
            relative_difficulty = prompt(
                f"Is {name} easier or harder than {config['exercises'][mid_idx]['slug']}: ",
                easy_hard_validator
            )

            if relative_difficulty == 'e':
                return binary_search(start_idx, mid_idx)
            else:
                return binary_search(mid_idx + 1, end_idx)

        while True:
            insert_idx = binary_search(first_idx, last_idx)
            if insert_idx == 0:
                ptext = f"{name} is the easiest exercise in the track."
            elif insert_idx == len(config['exercises']):
                ptext = f"{name} is the hardest exercise in the track."
            else:
                ptext = "{} fits between {} and {} in difficulty.".format(
                    name,
                    config['exercises'][insert_idx - 1]['slug'],
                    config['exercises'][insert_idx]['slug'],
                )
            print(f"You have indicated that {ptext}")
            yn = input('Is this correct? (y/N): ').strip().lower()
            if len(yn) > 0 and yn[0] == 'y':
                break

        config['exercises'].insert(insert_idx, conf_values)

    with inside(REPO_ROOT):
        with open('config.json', 'w') as config_json:
            json.dump(
                config,
                config_json,
                sort_keys=False,
                indent=2,
            )


@contextmanager
def git_master(git_path):
    "A context inside of which you are on the clean master branch"
    with inside(git_path):
        dirty = len(output_of('git status --porcelain')) > 0
        if dirty:
            subprocess.run(['git', 'stash'])
        branch = output_of('git rev-parse --abbrev-ref HEAD')
        if branch != 'master':
            subprocess.run(['git', 'checkout', 'master'])
        subprocess.run(['git', 'pull'])

        try:
            yield
        finally:
            if branch != 'master':
                subprocess.run(['git', 'checkout', branch])
            if dirty:
                subprocess.run(['git', 'stash', 'pop'])


def generate_readme(exercise_name, get_problem_specification):
    configlet = None
    with inside(os.path.join(REPO_ROOT, 'bin')):
        if not os.path.exists('configlet') and not os.path.exists('configlet.exe'):
            with inside(REPO_ROOT):
                subprocess.run('fetch-configlet')
        for configlet_name in ('configlet', 'configlet.exe'):
            if os.path.exists(configlet_name):
                configlet = configlet_name
                break
        if configlet is None:
            print("Could not locate configlet; aborting", file=sys.stderr)
            sys.exit(1)
    if get_problem_specification:
        with inside(os.path.join(REPO_ROOT, '..')):
            if os.path.exists('problem-specifications'):
                with git_master('problem-specifications'):
                    with inside(REPO_ROOT):
                        subprocess.run([
                            os.path.join('bin', configlet),
                            'generate', '.',
                            '--only', exercise_name,
                            '--spec-path',
                            os.path.join('..', 'problem-specifications')
                        ])
            else:
                subprocess.run(
                    ['git', 'clone', 'https://github.com/exercism/problem-specifications.git']
                )
                with inside(REPO_ROOT):
                    subprocess.run([
                        os.path.join('bin', configlet),
                        'generate', '.',
                        '--only', exercise_name,
                        '--spec-path',
                        os.path.join('..', 'problem-specifications')
                    ])
    else:
        with inside(REPO_ROOT):
            subprocess.run([
                os.path.join('bin', configlet),
                'generate', '.',
                '--only', exercise_name,
            ])


if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser(description='Create a Rust Track exercise for Exercism')
    parser.add_argument('name', help='name of the exercise to create')
    parser.add_argument('--dont-create-exercise', action='store_true',
                        help='Don\'t create the exercise. Useful when just updating config.json')
    parser.add_argument('--dont-update-config', action='store_true',
                        help='Don\'t update config.json. Useful when you don\'t yet '
                        'have a sense of exercise difficulty.')
    parser.add_argument('--version', action='version', version=VERSION)
    parser.add_argument('--use-maplit', action='store_true',
                        help='Use the maplit crate to improve readability of tests with lots of map literals')

    args = parser.parse_args()

    if not args.dont_create_exercise:
        make_exercise(args.name, args.use_maplit)

    if not args.dont_update_config:
        update_config(args.name)
