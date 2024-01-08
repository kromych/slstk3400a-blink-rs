#!/usr/bin/env python3

import argparse
import subprocess
import json
import os
import sys

def get_rust_host_target():
    result = subprocess.run(["rustc", "-Vv"], capture_output=True, text=True)
    if result.returncode != 0:
        raise RuntimeError("rustc command failed")

    for line in result.stdout.splitlines():
        if "host: " in line:
            return line.split("host: ")[1]
    raise RuntimeError("Could not find host target in rustc output")

def read_config():
    try:
        with open('x.json', 'r') as file:
            return json.load(file)
    except FileNotFoundError:
        raise RuntimeError("Could not find x.json file (the config file)")

def parse_arguments():
    parser = argparse.ArgumentParser(description="Run cargo commands with specific settings.")
    parser.add_argument("command", help="The command to run, taken from the x.json file")
    parser.add_argument("binary", help="The binary to flash, run or attach to")
    parser.add_argument("--target", help="Override the target specified in config.json")
    parser.add_argument("--release", action="store_true", help="Build artifacts in release mode")
    return parser.parse_args()

def construct_binary_path(target, release, binary_name):
    build_profile = "release" if release else "debug"
    return os.path.join("target", target, build_profile, binary_name)

def replace_binary_placeholder(params, binary_path):
    return [param.replace("$binary", binary_path) for param in params]

def run_subprocess(command, args):
    cmdline = [command]
    cmdline.extend(args)

    process = subprocess.run(cmdline)
    if process.returncode != 0:
        raise RuntimeError(f"'{command}' failed")

def main():
    args = parse_arguments()
    config = read_config()

    if not args.binary:
        print("Error: No binary specified.")
        sys.exit(1)
    if not args.command:
        print("Error: No command specified.")
        sys.exit(1)

    command_config = config.get(args.command, {})
    target = args.target if args.target else command_config.get("target", get_rust_host_target())
    release = args.release if args.release else command_config.get("release", False)

    binary_path = construct_binary_path(target, release, args.binary)

    cargo_params = ["build", "--target", target]
    if release:
        cargo_params.append("--release")
    cargo_params += ["-p", args.binary]
    run_subprocess("cargo", cargo_params)

    params = replace_binary_placeholder(command_config.get("params", []), binary_path)
    try:
        run_subprocess(config[args.command]["command"], params)
    except KeyError:
        print(f"Error: No command specified for {args.command} in x.json file")
        sys.exit(1)
    except subprocess.CalledProcessError as e:
        print(f"Error: {e}")
        sys.exit(1)
    except KeyboardInterrupt:
        print("Interrupted, exiting...")
        sys.exit(1)

if __name__ == "__main__":
    main()
