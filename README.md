# Touch Template

touch, with a template

Available on cargo, with `cargo install touchtemplate` ([Link to crates.io](https://crates.io/crates/touchtemplate)).
Then you can use the `tt` command.

This provides the `tt` binary, which looks in `~/Templates` (or whatever
you have `$XDG_TEMPLATE_DIR` set to), and will allow you to touch
(create) files, by copying them and renaming them to whatever you wish.

This utility is inspired by KDE's usage of the templates directory when using
the right click menu to "Create New" in a directory.

## Usage

```none
touchtemplate ('tt') is a way to create a file using a template 
(if the template is in your templates directory, default ~/Templates).

Usage: tt <TEMPLATE> <FILENAME>

Arguments:
  <TEMPLATE>
          The filename of the template.

  <FILENAME>
          The name of the file to create.

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Contributing

All code is GPLv3+ licensed, and by contributing you agree to
license the contributed code under this license.

Contributions are welcome for any improvements, especially regarding
optimisations, or platform compatibility.

## License

Copyright © 2024 Thomas Dickson and other contributors

This code is license under the GNU GPL v3+. See LICENSE for more details
