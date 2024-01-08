#!/usr/bin/env python3

import os
import toml
import argparse

def find_cargo_toml_files(start_path):
    for root, dirs, files in os.walk(start_path):
        if 'Cargo.toml' in files:
            yield os.path.join(root, 'Cargo.toml')

def parse_cargo_toml(file_path):
    try:
        with open(file_path, 'r') as file:
            cargo_data = toml.load(file)
            package_info = cargo_data.get('package', {})
            return package_info.get('name'), package_info.get('description')
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return None, None

def create_markdown_table(cargo_tomls):
    table = "Package Name | Description\n--- | ---\n"
    for file_path in cargo_tomls:
        name, description = parse_cargo_toml(file_path)
        if name and description:
            table += f"{name} | {description}\n"
    return table

def parse_arguments():
    parser = argparse.ArgumentParser(description="Generate a Markdown table from Cargo.toml files in a directory.")
    parser.add_argument("directory", help="The directory to search for Cargo.toml files")
    return parser.parse_args()

def main():
    args = parse_arguments()
    cargo_files = find_cargo_toml_files(args.directory)
    markdown_table = create_markdown_table(cargo_files)
    print(markdown_table)

if __name__ == "__main__":
    main()
