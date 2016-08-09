# Habitat template previewer

This is a simple app that takes in a templated config file that is part of a
[habitat](https://www.habitat.sh) plan and a toml file (such as default.toml,
also part of a habitat plan), and renders the template, printing it out.

This lets you test quickly if you have the correct template syntax and that
it's doing what you want without having to rebuild the habitat package
multiple times.

## Usage

    hab_template_preview file.conf default.toml

## Building

Install rust (e.g. `brew install rust` if you have homebrew on a mac), and run
the following to install it to `~/bin`:

    cargo install --root ~ --force

## Features/Limitations

* The same helpers as used in habitat are available, including json and toml
  helpers.
* Only `cfg.*` variables are set, any additional variables (such as
  `svc.leader`/`svc.follower`) are not currently available.
