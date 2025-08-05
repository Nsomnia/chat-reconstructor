# TODO

## Bugs

- [ ] The parser sometimes creates an extra file named `ImplementationCargo.toml` when it should be updating the existing `Cargo.toml`. This seems to be caused by the parser misinterpreting the context around a `Cargo.toml` code block.

## Features

- [ ] Add a `--dry-run` flag to show what changes *would* be made without actually touching the filesystem.
- [ ] Support for JSON logs from LLMs.
- [ ] More intelligent patching for other common config files (`package.json`, `pom.xml`, etc.).
- [ ] Direct API integration with LLMs to bypass the need for a text file export.
- [ ] Add a `--verbose` flag to show more details about the parsing and file operations.
- [ ] Add more tests to cover different scenarios and edge cases.
- [ ] Improve the fuzzy patching logic to be more robust.
- [ ] Add a GUI to make the tool easier to use for non-technical users.
