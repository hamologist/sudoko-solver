# Sudoko Solver
**Sudoko Solver** is a Rust project for solving Sudoko puzzles.
The package includes a CLI frontend for running the puzzle solving logic.
The project also provides a library for interfacing the project with other Rust projects.
Lastly, wasm support is included so the puzzle solving logic can be run on browser.

## Installation

### Local Installation
You can install the CLI solver directly to your machine using cargo.

```bash
$ cargo install --git https://github.com/hamologist/sudoko-solver.git --branch main sudoko-solver-cli
```

Likewise, you can uninstall the application using:
```bash
$ cargo uninstall sudoko-solver-cli
```

### Docker
You can also install and run the package using Docker like so:

First, build the image for the package:
```bash
$ docker build -t sudoko-solver https://github.com/hamologist/sudoko-solver.git#main
```

You can then run the `sudoko-solver` CLI, using the following:
```bash
$ docker run --rm -it sudoko-solver
```
This will connect you to an interactive shell on the `sudoko-solver` container.
You can then run `sudoko-solver` using the following:
```bash
$ echo '
?,?,?,6,7,2,?,?,?
4,5,?,?,?,8,?,?,?
?,1,?,?,?,5,?,?,?
2,?,?,8,?,?,?,6,?
7,?,?,?,?,?,?,?,?
?,?,5,?,?,?,?,2,7
?,9,3,?,?,?,4,?,?
?,?,?,?,6,?,8,?,?
?,?,?,3,?,?,?,?,5
' | sudoko-solver
```

## Usage
Once the sudoko-solver CLI frontend has been installed, you can start interfacing with it.
### CLI
A `sudoko-solver` command will be intalled on your system.
Help can be pulled up using the help flag:
```bash
$ sudoko-solver --help
```

The command takes a message sent via STDIN or using a file on your local machine.
Here is an example of what using the tool looks like:
```bash
$ echo '
?,?,?,6,7,2,?,?,?
4,5,?,?,?,8,?,?,?
?,1,?,?,?,5,?,?,?
2,?,?,8,?,?,?,6,?
7,?,?,?,?,?,?,?,?
?,?,5,?,?,?,?,2,7
?,9,3,?,?,?,4,?,?
?,?,?,?,6,?,8,?,?
?,?,?,3,?,?,?,?,5
' | sudoko-solver
```
The above will return the solved board (as a CSV):
```bash
9,3,8,6,7,2,1,5,4
4,5,7,9,1,8,6,3,2
6,1,2,4,3,5,7,8,9
2,4,9,8,5,7,3,6,1
7,6,1,2,9,3,5,4,8
3,8,5,1,4,6,9,2,7
8,9,3,5,2,1,4,7,6
5,2,4,7,6,9,8,1,3
1,7,6,3,8,4,2,9,5
```

## Building the project for wasm
For those interested in generating the project's wasm a `Makefile` is provided.
Assuming you've already installed `wasm-bindgen` and `wasm-opt` you can generate a wasm `pkg` directory using:
```bash
make wasm
```
