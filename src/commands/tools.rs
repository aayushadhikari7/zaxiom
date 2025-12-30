//! External development tools
//!
//! Thin wrappers around common dev tools that execute them directly.
//! These run the actual system binaries for fast execution.

use std::process::Command;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use anyhow::{Result, anyhow};

use crate::terminal::state::TerminalState;
use super::traits::Command as CommandTrait;

/// Generic tool executor - runs a command with args
fn run_tool(program: &str, args: &[String], cwd: &std::path::Path) -> Result<String> {
    // On Windows, use cmd /C to handle both .exe and .cmd/.bat files
    // CREATE_NO_WINDOW (0x08000000) prevents console window flash and speeds up execution
    #[cfg(windows)]
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    #[cfg(windows)]
    let output = Command::new("cmd")
        .args(["/C", program])
        .args(args)
        .current_dir(cwd)
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                anyhow!("'{}' not found. Is it installed and in PATH?", program)
            } else {
                anyhow!("{}: {}", program, e)
            }
        })?;

    #[cfg(not(windows))]
    let output = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                anyhow!("'{}' not found. Is it installed and in PATH?", program)
            } else {
                anyhow!("{}: {}", program, e)
            }
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Combine stdout and stderr (many tools output to stderr for status)
    let mut result = stdout.to_string();
    if !stderr.is_empty() {
        if !result.is_empty() && !result.ends_with('\n') {
            result.push('\n');
        }
        result.push_str(&stderr);
    }

    Ok(result.trim().to_string())
}

// ============ Node.js Tools ============

/// npm - Node Package Manager
pub struct NpmCommand;

impl CommandTrait for NpmCommand {
    fn name(&self) -> &'static str { "npm" }
    fn description(&self) -> &'static str { "Node Package Manager" }
    fn usage(&self) -> &'static str { "npm <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Ok("Usage: npm <command>\n\nCommon commands:\n  npm install    Install dependencies\n  npm run        Run scripts\n  npm start      Start the app\n  npm test       Run tests\n  npm init       Create package.json".to_string());
        }
        run_tool("npm", args, state.cwd())
    }
}

/// npx - Node Package Executor
pub struct NpxCommand;

impl CommandTrait for NpxCommand {
    fn name(&self) -> &'static str { "npx" }
    fn description(&self) -> &'static str { "Execute npm packages" }
    fn usage(&self) -> &'static str { "npx <package> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow!("Usage: npx <package> [args...]"));
        }
        run_tool("npx", args, state.cwd())
    }
}

/// yarn - Yarn Package Manager
pub struct YarnCommand;

impl CommandTrait for YarnCommand {
    fn name(&self) -> &'static str { "yarn" }
    fn description(&self) -> &'static str { "Yarn Package Manager" }
    fn usage(&self) -> &'static str { "yarn [command] [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("yarn", args, state.cwd())
    }
}

/// pnpm - Fast, disk space efficient package manager
pub struct PnpmCommand;

impl CommandTrait for PnpmCommand {
    fn name(&self) -> &'static str { "pnpm" }
    fn description(&self) -> &'static str { "Fast, disk space efficient package manager" }
    fn usage(&self) -> &'static str { "pnpm [command] [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("pnpm", args, state.cwd())
    }
}

/// bun - Fast all-in-one JavaScript runtime
pub struct BunCommand;

impl CommandTrait for BunCommand {
    fn name(&self) -> &'static str { "bun" }
    fn description(&self) -> &'static str { "Fast JavaScript runtime & toolkit" }
    fn usage(&self) -> &'static str { "bun [command] [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("bun", args, state.cwd())
    }
}

/// node - Node.js runtime
pub struct NodeCommand;

impl CommandTrait for NodeCommand {
    fn name(&self) -> &'static str { "node" }
    fn description(&self) -> &'static str { "Node.js JavaScript runtime" }
    fn usage(&self) -> &'static str { "node [script.js] [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            // Show version
            run_tool("node", &["--version".to_string()], state.cwd())
        } else {
            run_tool("node", args, state.cwd())
        }
    }
}

/// deno - Secure JavaScript/TypeScript runtime
pub struct DenoCommand;

impl CommandTrait for DenoCommand {
    fn name(&self) -> &'static str { "deno" }
    fn description(&self) -> &'static str { "Secure JavaScript/TypeScript runtime" }
    fn usage(&self) -> &'static str { "deno [command] [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("deno", args, state.cwd())
    }
}

// ============ Python Tools ============

/// python - Python interpreter
pub struct PythonCommand;

impl CommandTrait for PythonCommand {
    fn name(&self) -> &'static str { "python" }
    fn description(&self) -> &'static str { "Python interpreter" }
    fn usage(&self) -> &'static str { "python [script.py] [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            run_tool("python", &["--version".to_string()], state.cwd())
        } else {
            run_tool("python", args, state.cwd())
        }
    }
}

/// python3 - Python 3 interpreter
pub struct Python3Command;

impl CommandTrait for Python3Command {
    fn name(&self) -> &'static str { "python3" }
    fn description(&self) -> &'static str { "Python 3 interpreter" }
    fn usage(&self) -> &'static str { "python3 [script.py] [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            run_tool("python3", &["--version".to_string()], state.cwd())
        } else {
            run_tool("python3", args, state.cwd())
        }
    }
}

/// pip - Python package installer
pub struct PipCommand;

impl CommandTrait for PipCommand {
    fn name(&self) -> &'static str { "pip" }
    fn description(&self) -> &'static str { "Python package installer" }
    fn usage(&self) -> &'static str { "pip <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Ok("Usage: pip <command>\n\nCommon commands:\n  pip install <pkg>    Install package\n  pip uninstall <pkg>  Remove package\n  pip list             List installed packages\n  pip freeze           Output requirements format\n  pip search <query>   Search packages".to_string());
        }
        run_tool("pip", args, state.cwd())
    }
}

/// pip3 - Python 3 package installer
pub struct Pip3Command;

impl CommandTrait for Pip3Command {
    fn name(&self) -> &'static str { "pip3" }
    fn description(&self) -> &'static str { "Python 3 package installer" }
    fn usage(&self) -> &'static str { "pip3 <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("pip3", args, state.cwd())
    }
}

/// uv - Fast Python package installer
pub struct UvCommand;

impl CommandTrait for UvCommand {
    fn name(&self) -> &'static str { "uv" }
    fn description(&self) -> &'static str { "Fast Python package installer" }
    fn usage(&self) -> &'static str { "uv <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("uv", args, state.cwd())
    }
}

/// poetry - Python dependency management
pub struct PoetryCommand;

impl CommandTrait for PoetryCommand {
    fn name(&self) -> &'static str { "poetry" }
    fn description(&self) -> &'static str { "Python dependency management" }
    fn usage(&self) -> &'static str { "poetry <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("poetry", args, state.cwd())
    }
}

// ============ Rust Tools ============

/// cargo - Rust package manager
pub struct CargoCommand;

impl CommandTrait for CargoCommand {
    fn name(&self) -> &'static str { "cargo" }
    fn description(&self) -> &'static str { "Rust package manager" }
    fn usage(&self) -> &'static str { "cargo <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Ok("Usage: cargo <command>\n\nCommon commands:\n  cargo build      Compile the project\n  cargo run        Build and run\n  cargo test       Run tests\n  cargo check      Check for errors\n  cargo clippy     Run linter\n  cargo fmt        Format code\n  cargo add <pkg>  Add dependency".to_string());
        }
        run_tool("cargo", args, state.cwd())
    }
}

/// rustc - Rust compiler
pub struct RustcCommand;

impl CommandTrait for RustcCommand {
    fn name(&self) -> &'static str { "rustc" }
    fn description(&self) -> &'static str { "Rust compiler" }
    fn usage(&self) -> &'static str { "rustc [options] <input>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            run_tool("rustc", &["--version".to_string()], state.cwd())
        } else {
            run_tool("rustc", args, state.cwd())
        }
    }
}

/// rustup - Rust toolchain manager
pub struct RustupCommand;

impl CommandTrait for RustupCommand {
    fn name(&self) -> &'static str { "rustup" }
    fn description(&self) -> &'static str { "Rust toolchain manager" }
    fn usage(&self) -> &'static str { "rustup <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("rustup", args, state.cwd())
    }
}

// ============ Go Tools ============

/// go - Go programming language
pub struct GoCommand;

impl CommandTrait for GoCommand {
    fn name(&self) -> &'static str { "go" }
    fn description(&self) -> &'static str { "Go programming language" }
    fn usage(&self) -> &'static str { "go <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Ok("Usage: go <command>\n\nCommon commands:\n  go build       Compile packages\n  go run         Compile and run\n  go test        Run tests\n  go get         Download packages\n  go mod init    Initialize module\n  go fmt         Format code".to_string());
        }
        run_tool("go", args, state.cwd())
    }
}

// ============ Java/JVM Tools ============

/// java - Java runtime
pub struct JavaCommand;

impl CommandTrait for JavaCommand {
    fn name(&self) -> &'static str { "java" }
    fn description(&self) -> &'static str { "Java runtime" }
    fn usage(&self) -> &'static str { "java [options] <class> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            run_tool("java", &["--version".to_string()], state.cwd())
        } else {
            run_tool("java", args, state.cwd())
        }
    }
}

/// javac - Java compiler
pub struct JavacCommand;

impl CommandTrait for JavacCommand {
    fn name(&self) -> &'static str { "javac" }
    fn description(&self) -> &'static str { "Java compiler" }
    fn usage(&self) -> &'static str { "javac [options] <source files>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("javac", args, state.cwd())
    }
}

/// mvn - Apache Maven
pub struct MvnCommand;

impl CommandTrait for MvnCommand {
    fn name(&self) -> &'static str { "mvn" }
    fn description(&self) -> &'static str { "Apache Maven build tool" }
    fn usage(&self) -> &'static str { "mvn [options] [<goal>...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("mvn", args, state.cwd())
    }
}

/// gradle - Gradle build tool
pub struct GradleCommand;

impl CommandTrait for GradleCommand {
    fn name(&self) -> &'static str { "gradle" }
    fn description(&self) -> &'static str { "Gradle build tool" }
    fn usage(&self) -> &'static str { "gradle [options] [tasks...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("gradle", args, state.cwd())
    }
}

// ============ .NET Tools ============

/// dotnet - .NET CLI
pub struct DotnetCommand;

impl CommandTrait for DotnetCommand {
    fn name(&self) -> &'static str { "dotnet" }
    fn description(&self) -> &'static str { ".NET CLI" }
    fn usage(&self) -> &'static str { "dotnet <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Ok("Usage: dotnet <command>\n\nCommon commands:\n  dotnet new        Create new project\n  dotnet build      Build project\n  dotnet run        Run project\n  dotnet test       Run tests\n  dotnet add        Add package/reference".to_string());
        }
        run_tool("dotnet", args, state.cwd())
    }
}

// ============ Container Tools ============

/// docker - Docker CLI
pub struct DockerCommand;

impl CommandTrait for DockerCommand {
    fn name(&self) -> &'static str { "docker" }
    fn description(&self) -> &'static str { "Docker container CLI" }
    fn usage(&self) -> &'static str { "docker <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Ok("Usage: docker <command>\n\nCommon commands:\n  docker ps          List containers\n  docker images      List images\n  docker run         Run container\n  docker build       Build image\n  docker compose     Docker Compose".to_string());
        }
        run_tool("docker", args, state.cwd())
    }
}

/// kubectl - Kubernetes CLI
pub struct KubectlCommand;

impl CommandTrait for KubectlCommand {
    fn name(&self) -> &'static str { "kubectl" }
    fn description(&self) -> &'static str { "Kubernetes CLI" }
    fn usage(&self) -> &'static str { "kubectl <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("kubectl", args, state.cwd())
    }
}

// ============ Build Tools ============

/// make - GNU Make
pub struct MakeCommand;

impl CommandTrait for MakeCommand {
    fn name(&self) -> &'static str { "make" }
    fn description(&self) -> &'static str { "GNU Make build tool" }
    fn usage(&self) -> &'static str { "make [target...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("make", args, state.cwd())
    }
}

/// cmake - CMake build system
pub struct CmakeCommand;

impl CommandTrait for CmakeCommand {
    fn name(&self) -> &'static str { "cmake" }
    fn description(&self) -> &'static str { "CMake build system" }
    fn usage(&self) -> &'static str { "cmake [options] <path>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("cmake", args, state.cwd())
    }
}

// ============ Version Control ============

/// git - Git version control (full access)
pub struct GitCommand;

impl CommandTrait for GitCommand {
    fn name(&self) -> &'static str { "git" }
    fn description(&self) -> &'static str { "Git version control" }
    fn usage(&self) -> &'static str { "git <command> [args...]" }

    fn extended_help(&self) -> String {
        r#"git - Git version control system

USAGE:
  git <command> [options] [args...]

================== GETTING STARTED ==================

FIRST TIME SETUP (do this once):
  git config --global user.name "Your Name"
  git config --global user.email "you@email.com"

START A NEW PROJECT:
  mkdir my-project && cd my-project
  git init                    # Initialize git repo
  git add .                   # Stage all files
  git commit -m "Initial commit"

CLONE EXISTING PROJECT:
  git clone <url>             # Download a repository
  git clone <url> folder-name # Clone into specific folder

================== DAILY WORKFLOW ==================

THE BASIC CYCLE:
  1. git pull                 # Get latest changes
  2. (make your changes)
  3. git status               # See what changed
  4. git add .                # Stage changes
  5. git commit -m "message"  # Save changes
  6. git push                 # Upload to remote

CHECKING STATUS:
  git status            What files changed?
  git diff              What exactly changed? (unstaged)
  git diff --staged     What's about to be committed?
  git log               View commit history
  git log --oneline     Compact history view

STAGING CHANGES:
  git add <file>        Stage specific file
  git add .             Stage ALL changes
  git add -p            Interactive staging (pick chunks)
  git reset <file>      Unstage a file

COMMITTING:
  git commit -m "message"     Save with message
  git commit -am "message"    Add + commit (tracked files)
  git commit --amend          Fix last commit message

================== BRANCHES ==================

WHY BRANCHES?
  Work on features without breaking main code!

BRANCH COMMANDS:
  git branch                  List local branches
  git branch -a               List all branches
  git branch <name>           Create new branch
  git checkout <name>         Switch to branch
  git checkout -b <name>      Create AND switch
  git switch <name>           Modern way to switch
  git switch -c <name>        Modern create + switch
  git merge <branch>          Merge branch into current
  git branch -d <name>        Delete branch

COMMON BRANCH WORKFLOW:
  git checkout -b feature     # Create feature branch
  (make changes)
  git add . && git commit -m "Add feature"
  git checkout main           # Switch back to main
  git merge feature           # Merge feature in
  git branch -d feature       # Clean up

================== REMOTE (GitHub, etc.) ==================

SETUP REMOTE:
  git remote add origin <url>       Add remote
  git remote -v                     List remotes

SYNC WITH REMOTE:
  git push                     Push to remote
  git push -u origin main      First push (sets upstream)
  git pull                     Fetch + merge
  git fetch                    Download without merging

================== FIXING MISTAKES ==================

UNDO UNCOMMITTED CHANGES:
  git checkout -- <file>      Discard file changes
  git restore <file>          Modern discard
  git reset --hard            Discard ALL changes (!)

UNDO COMMITS:
  git reset --soft HEAD~1     Undo last commit, keep changes
  git reset --hard HEAD~1     Undo last commit, DELETE changes
  git revert <commit>         Create undo commit (safe)

ACCIDENTALLY COMMITTED TO WRONG BRANCH:
  git stash                   Save changes temporarily
  git checkout correct-branch
  git stash pop               Restore changes

================== COMMON ERRORS & FIXES ==================

"fatal: not a git repository"
  You're not in a git folder. Run: git init

"error: failed to push some refs"
  Remote has changes you don't have.
  Fix: git pull --rebase then git push

"CONFLICT (content): Merge conflict"
  Same lines changed in both versions.
  Fix: Edit file, remove <<<<< ===== >>>>> markers
       Then: git add <file> && git commit

"fatal: refusing to merge unrelated histories"
  Fix: git pull origin main --allow-unrelated-histories

"Your branch is behind"
  Fix: git pull

"Your branch is ahead"
  You have commits to push: git push

"Changes not staged for commit"
  Run: git add . before committing

"nothing to commit, working tree clean"
  Everything is already committed. You're good!

================== USEFUL SHORTCUTS ==================

VIEW CHANGES:
  git log --oneline --graph     Pretty history
  git show <commit>             Details of commit
  git blame <file>              Who changed what?

CLEANUP:
  git clean -fd                 Remove untracked files
  git gc                        Garbage collect

SAVE WORK TEMPORARILY:
  git stash                     Save changes
  git stash list                See stashes
  git stash pop                 Restore latest
  git stash drop                Delete stash

COMPARE:
  git diff main..feature        Compare branches
  git diff HEAD~3               Last 3 commits

================== .GITIGNORE ==================

Create a .gitignore file to exclude files:

# Common ignores:
node_modules/
*.log
.env
.DS_Store
__pycache__/
*.pyc
target/
build/
dist/

RELATED COMMANDS:
  gh       GitHub CLI
"#.to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Ok("Usage: git <command>\n\nCommon commands:\n  git status       Show status\n  git add          Stage changes\n  git commit       Commit changes\n  git push         Push to remote\n  git pull         Pull from remote\n  git log          Show history\n  git diff         Show changes\n  git branch       List branches\n  git checkout     Switch branches\n\nRun 'git --help' for complete beginner guide!".to_string());
        }
        run_tool("git", args, state.cwd())
    }
}

// ============ Other Languages ============

/// ruby - Ruby interpreter
pub struct RubyCommand;

impl CommandTrait for RubyCommand {
    fn name(&self) -> &'static str { "ruby" }
    fn description(&self) -> &'static str { "Ruby interpreter" }
    fn usage(&self) -> &'static str { "ruby [options] [script.rb] [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            run_tool("ruby", &["--version".to_string()], state.cwd())
        } else {
            run_tool("ruby", args, state.cwd())
        }
    }
}

/// gem - Ruby package manager
pub struct GemCommand;

impl CommandTrait for GemCommand {
    fn name(&self) -> &'static str { "gem" }
    fn description(&self) -> &'static str { "Ruby package manager" }
    fn usage(&self) -> &'static str { "gem <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("gem", args, state.cwd())
    }
}

/// bundle - Ruby Bundler
pub struct BundleCommand;

impl CommandTrait for BundleCommand {
    fn name(&self) -> &'static str { "bundle" }
    fn description(&self) -> &'static str { "Ruby Bundler" }
    fn usage(&self) -> &'static str { "bundle <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("bundle", args, state.cwd())
    }
}

/// php - PHP interpreter
pub struct PhpCommand;

impl CommandTrait for PhpCommand {
    fn name(&self) -> &'static str { "php" }
    fn description(&self) -> &'static str { "PHP interpreter" }
    fn usage(&self) -> &'static str { "php [options] [script.php] [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            run_tool("php", &["--version".to_string()], state.cwd())
        } else {
            run_tool("php", args, state.cwd())
        }
    }
}

/// composer - PHP package manager
pub struct ComposerCommand;

impl CommandTrait for ComposerCommand {
    fn name(&self) -> &'static str { "composer" }
    fn description(&self) -> &'static str { "PHP package manager" }
    fn usage(&self) -> &'static str { "composer <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("composer", args, state.cwd())
    }
}

/// swift - Swift compiler
pub struct SwiftCommand;

impl CommandTrait for SwiftCommand {
    fn name(&self) -> &'static str { "swift" }
    fn description(&self) -> &'static str { "Swift compiler" }
    fn usage(&self) -> &'static str { "swift <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("swift", args, state.cwd())
    }
}

/// zig - Zig compiler
pub struct ZigCommand;

impl CommandTrait for ZigCommand {
    fn name(&self) -> &'static str { "zig" }
    fn description(&self) -> &'static str { "Zig compiler" }
    fn usage(&self) -> &'static str { "zig <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("zig", args, state.cwd())
    }
}

/// gcc - GNU C Compiler
pub struct GccCommand;

impl CommandTrait for GccCommand {
    fn name(&self) -> &'static str { "gcc" }
    fn description(&self) -> &'static str { "GNU C Compiler" }
    fn usage(&self) -> &'static str { "gcc [options] <source files>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("gcc", args, state.cwd())
    }
}

/// g++ - GNU C++ Compiler
pub struct GppCommand;

impl CommandTrait for GppCommand {
    fn name(&self) -> &'static str { "g++" }
    fn description(&self) -> &'static str { "GNU C++ Compiler" }
    fn usage(&self) -> &'static str { "g++ [options] <source files>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("g++", args, state.cwd())
    }
}

/// clang - Clang C/C++ Compiler
pub struct ClangCommand;

impl CommandTrait for ClangCommand {
    fn name(&self) -> &'static str { "clang" }
    fn description(&self) -> &'static str { "Clang C/C++ Compiler" }
    fn usage(&self) -> &'static str { "clang [options] <source files>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("clang", args, state.cwd())
    }
}

// ============ Utility Tools ============

/// code - Visual Studio Code
pub struct CodeCommand;

impl CommandTrait for CodeCommand {
    fn name(&self) -> &'static str { "code" }
    fn description(&self) -> &'static str { "Open in VS Code" }
    fn usage(&self) -> &'static str { "code [path]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let args = if args.is_empty() {
            vec![".".to_string()]
        } else {
            args.to_vec()
        };
        run_tool("code", &args, state.cwd())?;
        Ok("Opening in VS Code...".to_string())
    }
}

/// cursor - Cursor AI Editor
pub struct CursorCommand;

impl CommandTrait for CursorCommand {
    fn name(&self) -> &'static str { "cursor" }
    fn description(&self) -> &'static str { "Open in Cursor" }
    fn usage(&self) -> &'static str { "cursor [path]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let args = if args.is_empty() {
            vec![".".to_string()]
        } else {
            args.to_vec()
        };
        run_tool("cursor", &args, state.cwd())?;
        Ok("Opening in Cursor...".to_string())
    }
}

/// subl - Sublime Text
pub struct SublCommand;

impl CommandTrait for SublCommand {
    fn name(&self) -> &'static str { "subl" }
    fn description(&self) -> &'static str { "Open in Sublime Text" }
    fn usage(&self) -> &'static str { "subl [path]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("subl", args, state.cwd())?;
        Ok("Opening in Sublime Text...".to_string())
    }
}

/// ssh - SSH client
pub struct SshCommand;

impl CommandTrait for SshCommand {
    fn name(&self) -> &'static str { "ssh" }
    fn description(&self) -> &'static str { "SSH client" }
    fn usage(&self) -> &'static str { "ssh [user@]host" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("ssh", args, state.cwd())
    }
}

/// scp - Secure copy
pub struct ScpCommand;

impl CommandTrait for ScpCommand {
    fn name(&self) -> &'static str { "scp" }
    fn description(&self) -> &'static str { "Secure copy" }
    fn usage(&self) -> &'static str { "scp <source> <dest>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("scp", args, state.cwd())
    }
}

/// rsync - Remote sync
pub struct RsyncCommand;

impl CommandTrait for RsyncCommand {
    fn name(&self) -> &'static str { "rsync" }
    fn description(&self) -> &'static str { "Remote sync" }
    fn usage(&self) -> &'static str { "rsync [options] <source> <dest>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("rsync", args, state.cwd())
    }
}

/// gh - GitHub CLI
pub struct GhCommand;

impl CommandTrait for GhCommand {
    fn name(&self) -> &'static str { "gh" }
    fn description(&self) -> &'static str { "GitHub CLI" }
    fn usage(&self) -> &'static str { "gh <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("gh", args, state.cwd())
    }
}

/// aws - AWS CLI
pub struct AwsCommand;

impl CommandTrait for AwsCommand {
    fn name(&self) -> &'static str { "aws" }
    fn description(&self) -> &'static str { "AWS CLI" }
    fn usage(&self) -> &'static str { "aws <service> <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("aws", args, state.cwd())
    }
}

/// az - Azure CLI
pub struct AzCommand;

impl CommandTrait for AzCommand {
    fn name(&self) -> &'static str { "az" }
    fn description(&self) -> &'static str { "Azure CLI" }
    fn usage(&self) -> &'static str { "az <group> <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("az", args, state.cwd())
    }
}

/// gcloud - Google Cloud CLI
pub struct GcloudCommand;

impl CommandTrait for GcloudCommand {
    fn name(&self) -> &'static str { "gcloud" }
    fn description(&self) -> &'static str { "Google Cloud CLI" }
    fn usage(&self) -> &'static str { "gcloud <group> <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("gcloud", args, state.cwd())
    }
}

/// terraform - Terraform CLI
pub struct TerraformCommand;

impl CommandTrait for TerraformCommand {
    fn name(&self) -> &'static str { "terraform" }
    fn description(&self) -> &'static str { "Terraform infrastructure as code" }
    fn usage(&self) -> &'static str { "terraform <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("terraform", args, state.cwd())
    }
}

/// ansible - Ansible automation
pub struct AnsibleCommand;

impl CommandTrait for AnsibleCommand {
    fn name(&self) -> &'static str { "ansible" }
    fn description(&self) -> &'static str { "Ansible automation" }
    fn usage(&self) -> &'static str { "ansible <host-pattern> [options]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("ansible", args, state.cwd())
    }
}

/// ffmpeg - Media converter
pub struct FfmpegCommand;

impl CommandTrait for FfmpegCommand {
    fn name(&self) -> &'static str { "ffmpeg" }
    fn description(&self) -> &'static str { "Media converter" }
    fn usage(&self) -> &'static str { "ffmpeg [options] -i <input> <output>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("ffmpeg", args, state.cwd())
    }
}

/// imagemagick/convert - Image manipulation
pub struct ConvertCommand;

impl CommandTrait for ConvertCommand {
    fn name(&self) -> &'static str { "convert" }
    fn description(&self) -> &'static str { "ImageMagick convert" }
    fn usage(&self) -> &'static str { "convert <input> [options] <output>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("convert", args, state.cwd())
    }
}

// ============ More C/C++ Tools ============

/// clang++ - Clang C++ Compiler
pub struct ClangppCommand;

impl CommandTrait for ClangppCommand {
    fn name(&self) -> &'static str { "clang++" }
    fn description(&self) -> &'static str { "Clang C++ Compiler" }
    fn usage(&self) -> &'static str { "clang++ [options] <source files>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("clang++", args, state.cwd())
    }
}

/// ld - GNU Linker
pub struct LdCommand;

impl CommandTrait for LdCommand {
    fn name(&self) -> &'static str { "ld" }
    fn description(&self) -> &'static str { "GNU Linker" }
    fn usage(&self) -> &'static str { "ld [options] <object files>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("ld", args, state.cwd())
    }
}

/// ar - Archive tool
pub struct ArCommand;

impl CommandTrait for ArCommand {
    fn name(&self) -> &'static str { "ar" }
    fn description(&self) -> &'static str { "Create/manage archives" }
    fn usage(&self) -> &'static str { "ar [options] <archive> <files>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("ar", args, state.cwd())
    }
}

/// nm - List symbols
pub struct NmCommand;

impl CommandTrait for NmCommand {
    fn name(&self) -> &'static str { "nm" }
    fn description(&self) -> &'static str { "List symbols from object files" }
    fn usage(&self) -> &'static str { "nm [options] <file>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("nm", args, state.cwd())
    }
}

/// objdump - Object file dumper
pub struct ObjdumpCommand;

impl CommandTrait for ObjdumpCommand {
    fn name(&self) -> &'static str { "objdump" }
    fn description(&self) -> &'static str { "Display object file info" }
    fn usage(&self) -> &'static str { "objdump [options] <file>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("objdump", args, state.cwd())
    }
}

/// gdb - GNU Debugger
pub struct GdbCommand;

impl CommandTrait for GdbCommand {
    fn name(&self) -> &'static str { "gdb" }
    fn description(&self) -> &'static str { "GNU Debugger" }
    fn usage(&self) -> &'static str { "gdb [options] <program>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("gdb", args, state.cwd())
    }
}

/// lldb - LLVM Debugger
pub struct LldbCommand;

impl CommandTrait for LldbCommand {
    fn name(&self) -> &'static str { "lldb" }
    fn description(&self) -> &'static str { "LLVM Debugger" }
    fn usage(&self) -> &'static str { "lldb [options] <program>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("lldb", args, state.cwd())
    }
}

/// valgrind - Memory debugger
pub struct ValgrindCommand;

impl CommandTrait for ValgrindCommand {
    fn name(&self) -> &'static str { "valgrind" }
    fn description(&self) -> &'static str { "Memory debugger" }
    fn usage(&self) -> &'static str { "valgrind [options] <program>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("valgrind", args, state.cwd())
    }
}

// ============ Assembly Tools ============

/// nasm - Netwide Assembler
pub struct NasmCommand;

impl CommandTrait for NasmCommand {
    fn name(&self) -> &'static str { "nasm" }
    fn description(&self) -> &'static str { "Netwide Assembler" }
    fn usage(&self) -> &'static str { "nasm [options] <source>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("nasm", args, state.cwd())
    }
}

/// as - GNU Assembler
pub struct AsCommand;

impl CommandTrait for AsCommand {
    fn name(&self) -> &'static str { "as" }
    fn description(&self) -> &'static str { "GNU Assembler" }
    fn usage(&self) -> &'static str { "as [options] <source>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("as", args, state.cwd())
    }
}

// ============ Haskell ============

/// ghc - Glasgow Haskell Compiler
pub struct GhcCommand;

impl CommandTrait for GhcCommand {
    fn name(&self) -> &'static str { "ghc" }
    fn description(&self) -> &'static str { "Glasgow Haskell Compiler" }
    fn usage(&self) -> &'static str { "ghc [options] <source>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("ghc", args, state.cwd())
    }
}

/// cabal - Haskell build tool
pub struct CabalCommand;

impl CommandTrait for CabalCommand {
    fn name(&self) -> &'static str { "cabal" }
    fn description(&self) -> &'static str { "Haskell build tool" }
    fn usage(&self) -> &'static str { "cabal <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("cabal", args, state.cwd())
    }
}

/// stack - Haskell tool stack
pub struct StackCommand;

impl CommandTrait for StackCommand {
    fn name(&self) -> &'static str { "stack" }
    fn description(&self) -> &'static str { "Haskell Tool Stack" }
    fn usage(&self) -> &'static str { "stack <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("stack", args, state.cwd())
    }
}

// ============ Elixir/Erlang ============

/// elixir - Elixir language
pub struct ElixirCommand;

impl CommandTrait for ElixirCommand {
    fn name(&self) -> &'static str { "elixir" }
    fn description(&self) -> &'static str { "Elixir language" }
    fn usage(&self) -> &'static str { "elixir [options] <script>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("elixir", args, state.cwd())
    }
}

/// mix - Elixir build tool
pub struct MixCommand;

impl CommandTrait for MixCommand {
    fn name(&self) -> &'static str { "mix" }
    fn description(&self) -> &'static str { "Elixir build tool" }
    fn usage(&self) -> &'static str { "mix <task> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("mix", args, state.cwd())
    }
}

/// iex - Elixir REPL
pub struct IexCommand;

impl CommandTrait for IexCommand {
    fn name(&self) -> &'static str { "iex" }
    fn description(&self) -> &'static str { "Elixir interactive shell" }
    fn usage(&self) -> &'static str { "iex [options]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("iex", args, state.cwd())
    }
}

/// erl - Erlang shell
pub struct ErlCommand;

impl CommandTrait for ErlCommand {
    fn name(&self) -> &'static str { "erl" }
    fn description(&self) -> &'static str { "Erlang shell" }
    fn usage(&self) -> &'static str { "erl [options]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("erl", args, state.cwd())
    }
}

// ============ Scala/Kotlin ============

/// scala - Scala REPL/compiler
pub struct ScalaCommand;

impl CommandTrait for ScalaCommand {
    fn name(&self) -> &'static str { "scala" }
    fn description(&self) -> &'static str { "Scala language" }
    fn usage(&self) -> &'static str { "scala [options] <source>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("scala", args, state.cwd())
    }
}

/// sbt - Scala build tool
pub struct SbtCommand;

impl CommandTrait for SbtCommand {
    fn name(&self) -> &'static str { "sbt" }
    fn description(&self) -> &'static str { "Scala build tool" }
    fn usage(&self) -> &'static str { "sbt [command]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("sbt", args, state.cwd())
    }
}

/// kotlin - Kotlin compiler
pub struct KotlinCommand;

impl CommandTrait for KotlinCommand {
    fn name(&self) -> &'static str { "kotlin" }
    fn description(&self) -> &'static str { "Kotlin compiler" }
    fn usage(&self) -> &'static str { "kotlin [options] <source>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("kotlin", args, state.cwd())
    }
}

/// kotlinc - Kotlin compiler CLI
pub struct KotlincCommand;

impl CommandTrait for KotlincCommand {
    fn name(&self) -> &'static str { "kotlinc" }
    fn description(&self) -> &'static str { "Kotlin compiler CLI" }
    fn usage(&self) -> &'static str { "kotlinc [options] <source>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("kotlinc", args, state.cwd())
    }
}

// ============ Other Languages ============

/// lua - Lua interpreter
pub struct LuaCommand;

impl CommandTrait for LuaCommand {
    fn name(&self) -> &'static str { "lua" }
    fn description(&self) -> &'static str { "Lua interpreter" }
    fn usage(&self) -> &'static str { "lua [script.lua]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("lua", args, state.cwd())
    }
}

/// luarocks - Lua package manager
pub struct LuarocksCommand;

impl CommandTrait for LuarocksCommand {
    fn name(&self) -> &'static str { "luarocks" }
    fn description(&self) -> &'static str { "Lua package manager" }
    fn usage(&self) -> &'static str { "luarocks <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("luarocks", args, state.cwd())
    }
}

/// perl - Perl interpreter
pub struct PerlCommand;

impl CommandTrait for PerlCommand {
    fn name(&self) -> &'static str { "perl" }
    fn description(&self) -> &'static str { "Perl interpreter" }
    fn usage(&self) -> &'static str { "perl [options] [script]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("perl", args, state.cwd())
    }
}

/// R - R language
pub struct RCommand;

impl CommandTrait for RCommand {
    fn name(&self) -> &'static str { "R" }
    fn description(&self) -> &'static str { "R statistical language" }
    fn usage(&self) -> &'static str { "R [options]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("R", args, state.cwd())
    }
}

/// Rscript - R script runner
pub struct RscriptCommand;

impl CommandTrait for RscriptCommand {
    fn name(&self) -> &'static str { "Rscript" }
    fn description(&self) -> &'static str { "Run R scripts" }
    fn usage(&self) -> &'static str { "Rscript <script.R>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("Rscript", args, state.cwd())
    }
}

/// julia - Julia language
pub struct JuliaCommand;

impl CommandTrait for JuliaCommand {
    fn name(&self) -> &'static str { "julia" }
    fn description(&self) -> &'static str { "Julia language" }
    fn usage(&self) -> &'static str { "julia [options] [script]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("julia", args, state.cwd())
    }
}

/// ocaml - OCaml REPL
pub struct OcamlCommand;

impl CommandTrait for OcamlCommand {
    fn name(&self) -> &'static str { "ocaml" }
    fn description(&self) -> &'static str { "OCaml REPL" }
    fn usage(&self) -> &'static str { "ocaml [options]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("ocaml", args, state.cwd())
    }
}

/// opam - OCaml package manager
pub struct OpamCommand;

impl CommandTrait for OpamCommand {
    fn name(&self) -> &'static str { "opam" }
    fn description(&self) -> &'static str { "OCaml package manager" }
    fn usage(&self) -> &'static str { "opam <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("opam", args, state.cwd())
    }
}

/// dune - OCaml build system
pub struct DuneCommand;

impl CommandTrait for DuneCommand {
    fn name(&self) -> &'static str { "dune" }
    fn description(&self) -> &'static str { "OCaml build system" }
    fn usage(&self) -> &'static str { "dune <command> [args...]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("dune", args, state.cwd())
    }
}

/// racket - Racket language
pub struct RacketCommand;

impl CommandTrait for RacketCommand {
    fn name(&self) -> &'static str { "racket" }
    fn description(&self) -> &'static str { "Racket language" }
    fn usage(&self) -> &'static str { "racket [options] [script]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("racket", args, state.cwd())
    }
}

/// sbcl - Steel Bank Common Lisp
pub struct SbclCommand;

impl CommandTrait for SbclCommand {
    fn name(&self) -> &'static str { "sbcl" }
    fn description(&self) -> &'static str { "Steel Bank Common Lisp" }
    fn usage(&self) -> &'static str { "sbcl [options]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("sbcl", args, state.cwd())
    }
}

/// gfortran - GNU Fortran compiler
pub struct GfortranCommand;

impl CommandTrait for GfortranCommand {
    fn name(&self) -> &'static str { "gfortran" }
    fn description(&self) -> &'static str { "GNU Fortran compiler" }
    fn usage(&self) -> &'static str { "gfortran [options] <source>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("gfortran", args, state.cwd())
    }
}

/// cobol - GnuCOBOL compiler
pub struct CobolCommand;

impl CommandTrait for CobolCommand {
    fn name(&self) -> &'static str { "cobc" }
    fn description(&self) -> &'static str { "GnuCOBOL compiler" }
    fn usage(&self) -> &'static str { "cobc [options] <source>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("cobc", args, state.cwd())
    }
}

// ============ Linters & Formatters ============

/// prettier - Code formatter
pub struct PrettierCommand;

impl CommandTrait for PrettierCommand {
    fn name(&self) -> &'static str { "prettier" }
    fn description(&self) -> &'static str { "Code formatter" }
    fn usage(&self) -> &'static str { "prettier [options] <files>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("prettier", args, state.cwd())
    }
}

/// eslint - JavaScript linter
pub struct EslintCommand;

impl CommandTrait for EslintCommand {
    fn name(&self) -> &'static str { "eslint" }
    fn description(&self) -> &'static str { "JavaScript linter" }
    fn usage(&self) -> &'static str { "eslint [options] <files>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("eslint", args, state.cwd())
    }
}

/// black - Python formatter
pub struct BlackCommand;

impl CommandTrait for BlackCommand {
    fn name(&self) -> &'static str { "black" }
    fn description(&self) -> &'static str { "Python code formatter" }
    fn usage(&self) -> &'static str { "black [options] <files>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("black", args, state.cwd())
    }
}

/// ruff - Fast Python linter
pub struct RuffCommand;

impl CommandTrait for RuffCommand {
    fn name(&self) -> &'static str { "ruff" }
    fn description(&self) -> &'static str { "Fast Python linter" }
    fn usage(&self) -> &'static str { "ruff [command] [options]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("ruff", args, state.cwd())
    }
}

/// mypy - Python type checker
pub struct MypyCommand;

impl CommandTrait for MypyCommand {
    fn name(&self) -> &'static str { "mypy" }
    fn description(&self) -> &'static str { "Python type checker" }
    fn usage(&self) -> &'static str { "mypy [options] <files>" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("mypy", args, state.cwd())
    }
}

/// pytest - Python testing
pub struct PytestCommand;

impl CommandTrait for PytestCommand {
    fn name(&self) -> &'static str { "pytest" }
    fn description(&self) -> &'static str { "Python testing framework" }
    fn usage(&self) -> &'static str { "pytest [options] [files]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("pytest", args, state.cwd())
    }
}

/// jest - JavaScript testing
pub struct JestCommand;

impl CommandTrait for JestCommand {
    fn name(&self) -> &'static str { "jest" }
    fn description(&self) -> &'static str { "JavaScript testing framework" }
    fn usage(&self) -> &'static str { "jest [options] [files]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("jest", args, state.cwd())
    }
}

/// vitest - Fast Vite-native testing
pub struct VitestCommand;

impl CommandTrait for VitestCommand {
    fn name(&self) -> &'static str { "vitest" }
    fn description(&self) -> &'static str { "Fast Vite-native testing" }
    fn usage(&self) -> &'static str { "vitest [command] [options]" }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        run_tool("vitest", args, state.cwd())
    }
}
