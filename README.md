# Bison
This is a Rust program that reads a BSON file and converts it into a pretty-printed JSON format. It replaces binary data with the string "BINARY".

# Usage
To use the program, run the binary file with the path to a BSON file as an argument:

```bash
bison <file.bson>
```

# Dependencies
This program depends on the following Rust crates:

- `serde_json for JSON serialization`
- `bson for BSON parsing`

# Contributing
Contributions to this program are welcome. Please fork the repository, make your changes, and submit a pull request.

# License
This program is licensed under the MIT License. See the LICENSE file for more information.