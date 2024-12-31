use clap::{Parser, ValueEnum};
use std::{
    fs,
    path::PathBuf,
    process::{Command, exit, Stdio},
    fmt,
};
use ctrlc;
use colored::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Language {
    C,
    Asm,
    Cpp,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Language::C => write!(f, "C"),
            Language::Asm => write!(f, "Assembly"),
            Language::Cpp => write!(f, "C++"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Action {
    Build,
    Valgrind,
    Clean,
}

#[derive(Parser, Debug)]
#[command(
    name = "roma",
    about = "Runtime Optimization and Memory Analysis",
    version = "1.0.0"
)]
struct Args {
    /// Path to the project directory
    #[arg(value_name = "PROJECT_DIR")]
    project_dir: PathBuf,

    /// Programming language
    #[arg(value_enum, short = 'l', long)]
    language: Language,

    /// Action to perform
    #[arg(value_enum, short = 'a', long)]
    action: Action,

    /// Compiler to use
    #[arg(long, default_value = None)]
    compiler: Option<String>,

    /// Target name
    #[arg(long, default_value = None)]
    target: Option<String>,

    /// Target options for valgrind
    #[arg(long, default_value = "")]
    target_options: String,
}

trait Compiler {
    fn new(args: &Args) -> Self where Self: Sized;
    fn build(&self) -> Result<(), String>;
    fn valgrind_test(&self) -> Result<(), String>;
    fn clean(&self) -> Result<(), String>;
}

struct CCompiler {
    src_dir: PathBuf,
    include_dir: PathBuf,
    binary_dir: PathBuf,
    log_dir: PathBuf,
    compiler: String,
    target: String,
    target_options: String,
}

impl CCompiler {
    fn get_include_flags(&self) -> String {
        if self.include_dir.exists() {
            format!("-I{}", self.include_dir.display())
        } else {
            String::new()
        }
    }
}

impl Compiler for CCompiler {
    fn new(args: &Args) -> Self {
        let project_dir = args.project_dir.clone();
        let src_dir = project_dir.join("src");
        let src_dir = if src_dir.is_dir() { src_dir } else { project_dir.clone() };
        let include_dir = project_dir.join("include");
        let binary_dir = project_dir.join("bin");
        let log_dir = project_dir.join("log");
        
        let compiler = args.compiler.clone()
            .unwrap_or_else(|| String::from("gcc"));
        
        let target = args.target.clone()
            .unwrap_or_else(|| {
                project_dir.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("a.out")
                    .to_string()
            });

        CCompiler {
            src_dir,
            include_dir,
            binary_dir,
            log_dir,
            compiler,
            target,
            target_options: args.target_options.clone(),
        }
    }

    fn build(&self) -> Result<(), String> {
        fs::create_dir_all(&self.binary_dir)
            .map_err(|e| format!("Failed to create binary directory: {}", e))?;

        let src_files: Vec<PathBuf> = walkdir::WalkDir::new(&self.src_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "c"))
            .map(|e| e.path().to_path_buf())
            .collect();

        if src_files.is_empty() {
            return Err("No source files found".to_string());
        }

        let cflags = "-Wall -Wextra -Werror -Wpedantic -g";
        let clibs = "-lm -lpthread";
        let include_flags = self.get_include_flags();
        let target_path = self.binary_dir.join(&self.target);

        let output = Command::new(&self.compiler)
            .args(cflags.split_whitespace())
            .args(include_flags.split_whitespace())
            .arg("-o")
            .arg(&target_path)
            .args(src_files)
            .args(clibs.split_whitespace())
            .output()
            .map_err(|e| format!("Failed to execute compiler: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Build failed:\n{}", error));
        }

        println!("Build completed. Run with ./{}", target_path.display());
        Ok(())
    }

    fn valgrind_test(&self) -> Result<(), String> {
        self.build()?;

        fs::create_dir_all(&self.log_dir)
            .map_err(|e| format!("Failed to create log directory: {}", e))?;

        let valgrind_log = self.log_dir.join("valgrind.txt");
        let target_path = self.binary_dir.join(&self.target);
        let valgrind_flags = format!(
            "--leak-check=full --show-leak-kinds=all --log-file={}",
            valgrind_log.display()
        );

        // Create the valgrind command
        let mut valgrind_cmd = Command::new("valgrind")
            .args(valgrind_flags.split_whitespace())
            .arg(&target_path)
            .args(self.target_options.split_whitespace())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| format!("Failed to start valgrind: {}", e))?;

        // Wait for the command to complete
        let status = valgrind_cmd.wait()
            .map_err(|e| format!("Failed to wait for valgrind: {}", e))?;

        if !status.success() {
            return Err(format!(
                "Program exited with code: {}", 
                status.code().unwrap_or(-1)
            ));
        }

        println!("Valgrind completed. Check ./{}", valgrind_log.display());
        Ok(())
    }

    fn clean(&self) -> Result<(), String> {
        let mut cleaned = false;

        for dir in &[&self.binary_dir, &self.log_dir] {
            if dir.exists() {
                fs::remove_dir_all(dir)
                    .map_err(|e| format!("Failed to clean directory {}: {}", dir.display(), e))?;
                cleaned = true;
            }
        }

        if cleaned {
            println!("Clean completed.");
        } else {
            println!("Nothing to clean.");
        }
        Ok(())
    }
}

macro_rules! impl_unimplemented_compiler {
    ($name:ident) => {
        struct $name;
        impl Compiler for $name {
            fn new(_args: &Args) -> Self {
                $name
            }

            fn build(&self) -> Result<(), String> {
                Err(format!("{} support is not implemented yet", stringify!($name)))
            }

            fn valgrind_test(&self) -> Result<(), String> {
                Err(format!("{} support is not implemented yet", stringify!($name)))
            }

            fn clean(&self) -> Result<(), String> {
                Err(format!("{} support is not implemented yet", stringify!($name)))
            }
        }
    };
}

impl_unimplemented_compiler!(AsmCompiler);
impl_unimplemented_compiler!(CppCompiler);

fn setup_signal_handlers() {
    ctrlc::set_handler(move || {
        eprintln!("\nroma: Interrupted by Ctrl+C");
        exit(1);
    }).expect("Error setting Ctrl-C handler");
}

fn main() {
    setup_signal_handlers();
    
    let args = Args::parse();
    
    let result = match args.language {
        Language::C => {
            let compiler = CCompiler::new(&args);
            match args.action {
                Action::Build => compiler.build(),
                Action::Valgrind => compiler.valgrind_test(),
                Action::Clean => compiler.clean(),
            }
        },
        Language::Asm => {
            let compiler = AsmCompiler::new(&args);
            match args.action {
                Action::Build => compiler.build(),
                Action::Valgrind => compiler.valgrind_test(),
                Action::Clean => compiler.clean(),
            }
        },
        Language::Cpp => {
            let compiler = CppCompiler::new(&args);
            match args.action {
                Action::Build => compiler.build(),
                Action::Valgrind => compiler.valgrind_test(),
                Action::Clean => compiler.clean(),
            }
        },
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        exit(1);
    }
}