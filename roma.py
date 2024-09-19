import os
import sys
import subprocess
import argparse
import shutil
from typing import Type
import signal


class BaseCompiler:
    def __init__(self, args: argparse.Namespace) -> None:
        """Initialize the compiler with the given arguments."""
        self.project_dir = args.project_dir.rstrip("/")
        self.compiler = args.compiler
        self.flags = args.flags
        self.target = args.target
        self.target_options = args.target_options
        self.initialize_paths()
        self.setup(args)
    
    def initialize_paths(self) -> None:
        """Initialize paths for source, include, binary, and log directories."""
        self.src_dir = os.path.join(self.project_dir, "src")
        self.src_dir = self.src_dir if os.path.isdir(self.src_dir) else self.project_dir
        self.include_dir = os.path.join(self.project_dir, "include")
        self.include_flags = f"-I{self.include_dir}" if os.path.isdir(self.include_dir) else ""

        self.binary_dir = "./" + os.path.join(self.project_dir, "bin") if self.project_dir != "." else "./bin"
        self.log_dir = "./" + os.path.join(self.project_dir, "log") if self.project_dir != "." else "./log"

    def setup(self, args: argparse.Namespace) -> None:
        """Override this method in derived classes to set up language-specific configurations."""
        pass

    def build(self) -> None:
        """Build the project. Implemented in derived classes."""
        raise NotImplementedError("Build method not implemented for this language.")

    def valgrind_test(self) -> None:
        """Run Valgrind on the built project. Implemented in derived classes."""
        raise NotImplementedError("Valgrind test not implemented for this language.")

    def clean(self) -> None:
        """Clean the build and log directories."""
        cleaned = False

        for directory in [self.binary_dir, self.log_dir]:
            if os.path.isdir(directory):
                shutil.rmtree(directory)
                cleaned = True

        if cleaned:
            print("Clean completed.")
        else:
            print("Nothing to clean.")


class CCompiler(BaseCompiler):
    def setup(self, args: argparse.Namespace) -> None:
        """Set up default values and configurations for C language."""
        self.compilers = ["gcc", "clang"]
        self.cflags = "-Wall -Wextra -Werror -Wpedantic -g -std=c11"
        self.valgrind_log = "valgrind.txt"
        self.compiler = self.compiler if self.compiler in self.compilers else self.compilers[0]
        self.flags = self.flags if self.flags else self.cflags
        if self.project_dir != ".":
            self.target = self.target if self.target else os.path.basename(self.project_dir)
        else:
            self.target = self.target if self.target else "a.out"
        self.valgrind_flags = "--leak-check=full --show-leak-kinds=all --log-file=" + os.path.join(self.log_dir, self.valgrind_log)

    def build(self) -> None:
        """Build the C project."""
        if not os.path.exists(self.binary_dir):
            os.makedirs(self.binary_dir)

        src_files = [os.path.join(root, file) for root, _, files in os.walk(self.src_dir) for file in files if file.endswith(".c")]
        if not src_files:
            raise FileNotFoundError("No source files found.")

        compile_command = [self.compiler] + self.flags.split() + self.include_flags.split() + ["-o", os.path.join(self.binary_dir, self.target)] + src_files
        result = subprocess.run(compile_command)
        if result.returncode != 0:
            raise RuntimeError("Build failed.")
        print(f"Build completed. Run with {os.path.join(self.binary_dir, self.target)}")

    def valgrind_test(self) -> None:
        """Run Valgrind to test the C project for memory leaks."""
        if not os.path.exists(self.log_dir):
            os.makedirs(self.log_dir)

        self.build()

        valgrind_command = ["valgrind"] + self.valgrind_flags.split() + [os.path.join(self.binary_dir, self.target)] + self.target_options.split()
        result = subprocess.run(valgrind_command)
        if result.returncode != 0:
            raise RuntimeError("Valgrind failed.")
        print(f"Valgrind completed. Check {os.path.join(self.log_dir, self.valgrind_log)}")

# TODO: Implement Assembly
class AssemblyCompiler(BaseCompiler):
    def setup(self, args: argparse.Namespace) -> None:
        raise NotImplementedError("Assembly support is not implemented yet.")
    
    def build(self) -> None:
        raise NotImplementedError("Assembly support is not implemented yet.")
    
    def valgrind_test(self) -> None:
        raise NotImplementedError("Assembly support is not implemented yet.")

# TODO: Implement C++
class CppCompiler(BaseCompiler):
    def setup(self, args: argparse.Namespace) -> None:
        raise NotImplementedError("C++ support is not implemented yet.")
    
    def build(self) -> None:
        raise NotImplementedError("C++ support is not implemented yet.")
    
    def valgrind_test(self) -> None:
        raise NotImplementedError("C++ support is not implemented yet.")


# TODO: Implement Python
class PythonInterpreter(BaseCompiler):
    def setup(self, args: argparse.Namespace) -> None:
        raise NotImplementedError("Python support is not implemented yet.")
    
    def build(self) -> None:
        raise NotImplementedError("Python support is not implemented yet.")
    
    def valgrind_test(self) -> None:
        raise NotImplementedError("Python support is not implemented yet.")


def signal_handler(sig, frame):
    script_name = os.path.basename(__file__)

    if sig == signal.SIGINT:
        print(f"\n{script_name}: {sig} signal received.")
    elif sig == signal.SIGQUIT:
        print(f"\n{script_name}: {sig} signal received.")
        sys.exit(0)

def main():
    signal.signal(signal.SIGINT, signal_handler)

    parser = argparse.ArgumentParser(description="Roma - Runtime Optimization and Memory Analysis")

    parser.add_argument("project_dir", type=str, help="Path to the project directory.")
    parser.add_argument("-l", "--language", type=str, help="Specify the language of the project.", required=True, default=None)

    parser.add_argument("-b", "--build", action="store_const", const="-b", dest="option", help="Build the project.")
    parser.add_argument("-v", "--valgrind", action="store_const", const="-v", dest="option", help="Run valgrind.")
    parser.add_argument("-c", "--clean", action="store_const", const="-c", dest="option", help="Clean the project.")

    parser.add_argument("--compiler", type=str, help="Specify the compiler to use.", default=None)
    parser.add_argument("--flags", type=str, help="Specify the flags to use.", default=None)
    parser.add_argument("--target", type=str, help="Specify the target to use.", default=None)
    parser.add_argument("--target-options", type=str, help="Specify the target options to use.", default="")
    
    args = parser.parse_args()

    compilers: dict[str, Type[BaseCompiler]] = {
        "c": CCompiler,
        "asm": AssemblyCompiler,
        "cpp": CppCompiler,
        "py": PythonInterpreter
    }

    try:
        if args.language not in compilers:
            raise ValueError(f"Language \"{args.language}\" not supported.")
        
        compiler_class = compilers[args.language]
        compiler = compiler_class(args)
        if args.option == "-b":
            compiler.build()
        elif args.option == "-v":
            compiler.valgrind_test()
        elif args.option == "-c":
            compiler.clean()
        else:
            parser.error("Invalid option or language.")
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
