# stripper

A command-line tool to flag files in the working directory that would be
ignored by `.gitignore` or `.gcloudignore` rules.

This exists to help in cases where `gcloud` CLI and friends do not cull files on
deployment adequately.  This tool should ultimately be a bug filed against
Google App Engine instead.

## Synopsis

`stripper` scans the current directory and lists files that match ignore rules,
helping you identify files that would be excluded by Git or Google Cloud.

## Build

```sh
cargo build --release
```

## Install

```sh
cargo install --git https://github.com/matttproud/stripper.git
```

## Usage

```sh
stripper .gcloudignore
```

This will print the list of ignored files in the current directory.

Hypothetically this can be aggressively used as such:

```sh
stripper .gcloudignore | xargs -n1 -P1 rm
```