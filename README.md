<p align="center">
    <img src=".github/imgs/roma-removebg.png" width="250">
    <br>
    <strong>Runtime Optimization and Memory Analysis</strong>
</p>

# roma

![GitHub License](https://img.shields.io/github/license/AntonioBerna/roma)
![GitHub Created At](https://img.shields.io/github/created-at/antonioberna/roma)

```
 ____           ___                  __  __           _
|  _ \         / _ \                |  \/  |         / \ 
| |_) |       | | | |               | |\/| |        / _ \ 
|  _ <        | |_| |               | |  | |       / ___ \
|_| \_\untime  \___/ptimization and |_|  |_|emory /_/   \_\nalysis
```

> [!TIP]
> Would you like to create these custom texts using `figlet`? For more information click [here](https://github.com/AntonioBerna/maximus.git).

> [!WARNING]
> This repository is unfinished. Keep your expectations low.

> [!WARNING]
> At the moment there is only support for the `C` language but in the future I plan to add support for `Assembly`, and `C++` as well.

## Activity

<img src="https://repobeats.axiom.co/api/embed/0d58ecee9de031a6ccc14b5bf32d4c7fd390982e.svg" width="100%" />

## Why `roma`?

I know what you're thinking: why create a software to compile `C` (or `Assembly`, and `C++`) code if `Makefile` or `CMakeLists.txt` already exist? The answer is simple and now I will explain my reasoning with a simple example.

Imagine you have many small projects as shown below:

```
.
├── 01
│   ├── Makefile
│   └── main.c
├── 02
│   ├── Makefile
│   └── main.c
├── 03
│   ├── Makefile
│   └── main.c

...
```

Each individual project consists of a `main.c` file and a `Makefile`. Of course each project has a different purpose so it is normal that the `C` code of each individual `main.c` is different, but this does not apply to `Makefiles` which tend to be copied and pasted from one project to the next.

By the way, if for project number `x` I modified the `Makefile` to optimize something, then I have to copy and paste for all the other projects, generating lots of uncontrolled `Makefiles`.

It is for this very reason that I came up with the idea of ​​creating `roma`, an all-in-one software written in `Rust` that allows me to compile the `C` (or `Assembly` and `C++`) code.

As will become clearer later, it is possible to compile the examples in the `examples/` directory arranged as follows:

```
.
├── Cargo.lock
├── Cargo.toml
├── examples
│   └── C
│       ├── complex-hello
│       │   ├── include
│       │   │   └── log.h
│       │   └── src
│       │       ├── log.c
│       │       └── main.c
│       ├── print-args
│       │   └── main.c
│       └── simple-hello
│           └── main.c
├── legacy-python-version
│   ├── install.sh
│   ├── README.md
│   ├── requirements.txt
│   └── roma.py
├── LICENSE
├── README.md
└── src
    └── main.rs
```

> [!NOTE]
> Of course, depending on your needs you can also use `roma` inside an `x` project, for example inside a `client-server` application to compile the client and the server separately using the same script and not having two separate `Makefiles` or `CMakeLists.txt`.

## Download & Installation

First of all, you have to clone the repository using the following command:

```
git clone https://github.com/AntonioBerna/roma.git
```

subsequently, using the command `cd roma/` you will be able to access the `roma/` directory.

Now you can install the program in your system with the following command:

```
cargo install --path .
```

> [!NOTE]
> I recommend installing the software inside the system so that it can be invoked from any point of your computer.

## Uninstall

If you want uninstall the software you can use the command:

```
cargo uninstall roma
```

which will remove the software from the system.

## Usage

After the installation, you can run the program with the following command:

```
roma --help
```

which will give us the following output:

```
Runtime Optimization and Memory Analysis

Usage: roma [OPTIONS] --language <LANGUAGE> --action <ACTION> <PROJECT_DIR>

Arguments:
  <PROJECT_DIR>  Path to the project directory

Options:
  -l, --language <LANGUAGE>              Programming language [possible values: c, asm, cpp]
  -a, --action <ACTION>                  Action to perform [possible values: build, valgrind, clean]
      --compiler <COMPILER>              Compiler to use
      --target <TARGET>                  Target name
      --target-options <TARGET_OPTIONS>  Target options for valgrind [default: ]
  -h, --help                             Print help
  -V, --version                          Print version
```

## Examples

### simple-hello

Consider the project `examples/C/simple-hello/`. Inside this project there is only one `main.c` file, so let's specify the project path as an option of `roma` and then specify that it is a project created in `C` and that we want to build it to generate the ELF file:

```
roma --language c --action build examples/C/simple-hello/
```

the output of this command is as follows:

```
Build completed. Run with ./examples/C/simple-hello/bin/simple-hello/
```

then using the command `./examples/C/simple-hello/bin/simple-hello/` we run the ELF file obtaining:

```
Hello, World!
```

> [!NOTE]
> Since the `--target` option was not specified, the ELF file took the name of the project directory.

So let's try using `valgrind` using the following command:

```
roma --language c --action valgrind examples/C/simple-hello/
```

in order to obtain:

```
Build completed. Run with ./examples/C/simple-hello/bin/simple-hello
Hello, World!
Valgrind completed. Check ./examples/C/simple-hello/log/valgrind.txt
```

> [!NOTE]
> Using the command `cat ./examples/C/simple-hello/log/valgrind.txt` you can view the output of `valgrind` to see if the code reports any vulnerabilities.

Finally using the command:

```
roma --language c --action clean examples/C/simple-hello/
```

in order to obtain:

```
Clean completed.
```

> [!NOTE]
> The interesting thing about the `roma` project is that it can automatically understand the structure of the `C` code and therefore "knows" where to get the source files. Typically, `C` projects are divided into the `src/` and `include/` directories and it is for this reason that if we try to execute the commands already seen previously with the example `examples/C/complex-hello/` we get the same result.

### print-args

Consider the project `examples/C/print-args/`. This example is to understand the use of the `--compiler`, `--target` and `--target-options` options. In particular, using the following command:

```
roma --language c --action valgrind --compiler clang --target pippo --target-options "pluto paperino" examples/C/print-args/
```

you get the following output:

```
Build completed. Run with ./examples/C/print-args/bin/pippo
Number of arguments: 3
Program name: examples/C/print-args/bin/pippo
Arguments:
  1: pluto
  2: paperino
Valgrind completed. Check ./examples/C/print-args/log/valgrind.txt
```
