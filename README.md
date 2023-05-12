# Driver Installer

This script helps to automate the installation of drivers in a Windows environment. It uses the `pnputil.exe` utility to install all driver packages (*.inf files) in the current directory and its subdirectories.

## Dependencies

The script is written in Rust and depends on several libraries:

- `indicatif`: For progress bar rendering.
- `std`: For various standard functionalities.
- `walkdir`: For directory traversal.
- `term_size`: To get the terminal window size.

## Features

- It traverses the current directory and all subdirectories, finds all the `.inf` files (driver packages), and attempts to install them using `pnputil.exe`.
- It shows a progress bar during the installation process.
- It counts the number of successful and failed installations and prints the counts at the end.
- It supports logging. If you run the program with `log` argument, it logs the output of `pnputil.exe` commands into a file called `log.txt`.

## Usage

### Compile the script

First, you need to compile the script using Rust. If you have Rust installed, you can compile the script with:

```bash
cargo build --release
```

The executable will be located in `target/release`.

### Run the script

To run the script, navigate to the directory containing your drivers and run:

```bash
path/to/your/executable
```

Replace `path/to/your/executable` with the actual path to the compiled executable.

To enable logging, add `log` as an argument:

```bash
path/to/your/executable log
```

## Contributing

If you have suggestions for improving the script, please create an issue or a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
