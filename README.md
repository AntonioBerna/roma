# roma

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
> At the moment there is only support for the `C` language but in the future I plan to add support for `Assembly`, `C++` and `Python` as well.

## Why `roma`?

I know what you're thinking: why create a software to compile `C` (or `Assembly`, `C++` and `Python`) code if `Makefile` or `CMakeLists.txt` already exist? The answer is simple and now I will explain my reasoning with a simple example.

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

It is for this very reason that I came up with the idea of ​​creating `roma`, an all-in-one software written in `Python` that allows me to compile the `C` (or `Assembly`, `C++` and `Python`) code.

As will become clearer later, it is possible to compile the examples in the `examples` directory arranged as follows:

```
.
├── examples
│   ├── Assembly
│   ├── C
│   │   ├── complex-hello
│   │   │   ├── include
│   │   │   │   └── log.h
│   │   │   └── src
│   │   │       ├── log.c
│   │   │       └── main.c
│   │   └── simple-hello
│   │       └── main.c
│   ├── Cpp
│   └── Python
├── install.sh
├── LICENSE
├── README.md
├── requirements.txt
└── roma.py
```

even if on the terminal I have a path that has nothing to do with that of a general project called `x`.

> [!NOTE]
> Of course, depending on your needs you can also use `roma` inside an `x` project, for example inside a `client-server` application to compile the client and the server separately using the same script and not having two separate `Makefiles` or `CMakeLists.txt`.

## mini docs

First of all, you have to clone the repository using the following command:

```
git clone https://github.com/AntonioBerna/roma.git
```

subsequently, using the command `cd roma` you will be able to access the `roma` directory.

Now we have two possibilities: install `roma` inside our computer using the `install.sh` script or use `roma.py` calling `python roma.py` each time.

> [!WARNING]
> If you use another distro than `Manjaro` or `Arch` you may have to modify the `install.sh` script. For any problems open an issue or pull-request on the repository.

> [!WARNING]
> For generate a `roma` ELF file using `install.sh` script you need to install `requirements.txt` using the command `pip install -r requirements.txt`. Otherwise, you can install `pyinstaller` using `pipx install pyinstaller`.

I recommend installing the software inside the system so that it can be invoked from any point of your computer and to do this you need to use this command:

```
./install.sh
```

which will give us the following output:

```
Usage: ./install.sh [ -i | -r ]
Options:
  -i  Install the program.
  -r  Remove the program.
```

> [!NOTE]
> If you want uninstall the software you can use the command `./install.sh -r` which will remove the software from the system.

i.e. a detailed explanation to install/uninstall the `roma` software. Then running:

```
./install.sh -i
```

will start the installation procedure which will end with the following message:

```
...

The directory .../roma/dist has been added to the PATH.
```

Now we are ready to understand how to use `roma` and in particular we use the following command:

```
roma
```

in order to obtain:

```
usage: roma [-h] -l LANGUAGE [-b] [-v] [-c] [--compiler COMPILER]
            [--flags FLAGS] [--target TARGET]
            [--target-options TARGET_OPTIONS] [--version]
            project_dir

Roma - Runtime Optimization and Memory Analysis

positional arguments:
  project_dir           path to the project directory.

options:
  -h, --help            show this help message and exit
  -l LANGUAGE, --language LANGUAGE
                        specify the language of the project.
  -b, --build           build the project.
  -v, --valgrind        run valgrind.
  -c, --clean           clean the project.
  --compiler COMPILER   specify the compiler to use.
  --flags FLAGS         specify the flags to use.
  --target TARGET       specify the target to use.
  --target-options TARGET_OPTIONS
                        specify the target options to use.
  --version             show program's version number and exit
```

But let's see it in action. 

### `roma` outside projects

Consider the project `examples/C/simple-hello`. Inside this project there is only one `main.c` file, so let's specify the project path as an option of `roma` and then specify that it is a project created in `C` and that we want to build it to generate the ELF file:

```
roma examples/C/simple-hello/ --language c -b
```

the output of this command is as follows:

```
Build completed. Run with ./examples/C/simple-hello/bin/simple-hello
```

then using the command `./examples/C/simple-hello/bin/simple-hello` we run the ELF file obtaining:

```
Hello, World!
```

> [!NOTE]
> Since the `--target` option was not specified, the ELF file took the name of the project directory.

So let's try using `valgrind` using the following command:

```
roma examples/C/simple-hello/ --language c -v
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
roma examples/C/simple-hello/ --language c -c
```

in order to obtain:

```
Clean completed.
```

> [!NOTE]
> The interesting thing about the `roma` project is that it can automatically understand the structure of the `C` code and therefore "knows" where to get the source files. Typically, `C` projects are divided into the `src` and `include` directories and it is for this reason that if we try to execute the commands already seen previously with the example `examples/C/complex-hello` we get the same result.

### `roma` inside projects

If you want to use `roma` inside a specific project you can do it simply by specifying the path `"."`. The operation remains unchanged compared to the previous examples.

> [!NOTE]
> If `roma` is used with the path `"."` then the `target` is by default `a.out`.
