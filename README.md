## Minigrep: A Simplified grep Clone

**Description:**

Minigrep is a Rust project that implements a simplified version of the `grep` command. It searches for lines that match a specified search pattern in one or more files.

**Features:**

- Simple text pattern searching.
- Support for plain text files.
- Outputs lines matching the pattern.
- Options to ignore case and display line numbers.

**Installation Instructions:**

1. Ensure you have Rust installed on your system.
2. Clone this repository: `git clone https://github.com/adannup/minigrep.git`
3. Navigate to the project directory: `cd minigrep`
4. Compile the project: `cargo build`
5. Run the command `./target/debug/minigrep` with desired options and arguments.

**Usage Example:**

```
./target/debug/minigrep "Hello" file.txt
```

Insensitive case:

```
IGNORE_CASE=1 ./target/debug/minigrep "hElLo" file.txt
```

This command will search for the word "Hello" in the file `file.txt` and display any lines that match.

**Contributions:**

This project is open-source and contributions are welcome. If you wish to improve Minigrep, you can fork the repository, make your changes, and submit a pull request.

**License:**

This project is licensed under the MIT License.

**Acknowledgements:**

This project is inspired by the Rust tutorial "The Rust Programming Language".

**Additional Resources:**

- The Rust Programming Language: [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)
- Rust Cookbook: [https://rust-lang-nursery.github.io/rust-cookbook/](https://rust-lang-nursery.github.io/rust-cookbook/)
- Rust API Reference: [https://doc.rust-lang.org/std/](https://doc.rust-lang.org/std/)

**Contact:**

If you have any questions or feedback about this project, you can reach me through GitHub or email.

**I hope you find Minigrep useful!**
