//! Command registry
//!
//! Stores and looks up built-in commands.

#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;

use crate::terminal::state::TerminalState;
use super::traits::Command;
use super::nav::{LsCommand, CdCommand, PwdCommand, ClearCommand, TreeCommand, HelpCommand};
use super::files::{CatCommand, TouchCommand, RmCommand, MkdirCommand, CpCommand, MvCommand, LnCommand, StatCommand, FileCommand, BasenameCommand, DirnameCommand, RealpathCommand, ChmodCommand, ReadlinkCommand, MktempCommand, NanoCommand, VimCommand, ViCommand, EditCommand};
use super::text::{EchoCommand, HeadCommand, TailCommand, WcCommand, SortCommand, UniqCommand, TacCommand, CutCommand, PasteCommand, DiffCommand, TrCommand, SedCommand, AwkCommand, RevCommand, NlCommand, PrintfCommand, XargsCommand, ColumnCommand, StringsCommand, SplitCommand, JoinCommand, CommCommand};
use super::search::{GrepCommand, FindCommand};
use super::system::{ExitCommand, WhichCommand, DuCommand, DfCommand, PsCommand, KillCommand, WhoamiCommand, HostnameCommand, UnameCommand, UptimeCommand, FreeCommand, DateCommand, CalCommand, IdCommand, NeofetchCommand, PrintenvCommand, LscpuCommand, HistoryCommand, TestCommand, ManCommand, ThemeCommand};
use super::net::{CurlCommand, WgetCommand, PingCommand, NetstatCommand, TracerouteCommand, NslookupCommand, HostCommand, IfconfigCommand};
use super::hash::{Md5sumCommand, Sha1sumCommand, Sha224sumCommand, Sha256sumCommand, Sha384sumCommand, Sha512sumCommand, Blake3sumCommand, Crc32Command, Base64Command, XxdCommand};
use super::compress::{TarCommand, ZipCommand, UnzipCommand, GzipCommand, GunzipCommand};
use super::fun::{FortuneCommand, CowsayCommand, CoffeeCommand, MatrixCommand, PetCommand};
use super::shell::{AliasCommand, EnvCommand, ExportCommand, SleepCommand, WatchCommand, SeqCommand, YesCommand, TrueCommand, FalseCommand, ExprCommand, BcCommand, TeeCommand, TimeoutCommand, TypeCommand, CommandCommand, PushdCommand, PopdCommand, DirsCommand};
use super::ai::OllamaCommand;
use super::tools::{
    // Node.js
    NpmCommand, NpxCommand, YarnCommand, PnpmCommand, BunCommand, NodeCommand, DenoCommand,
    // Python
    PythonCommand, Python3Command, PipCommand, Pip3Command, UvCommand, PoetryCommand,
    // Rust
    CargoCommand, RustcCommand, RustupCommand,
    // Go
    GoCommand,
    // Java/JVM
    JavaCommand, JavacCommand, MvnCommand, GradleCommand,
    // .NET
    DotnetCommand,
    // Containers
    DockerCommand, KubectlCommand,
    // Build tools
    MakeCommand, CmakeCommand,
    // Version control
    GitCommand,
    // Other languages
    RubyCommand, GemCommand, BundleCommand, PhpCommand, ComposerCommand,
    SwiftCommand, ZigCommand, GccCommand, GppCommand, ClangCommand,
    // Utilities
    CodeCommand, CursorCommand, SublCommand, SshCommand, ScpCommand, RsyncCommand,
    GhCommand, AwsCommand, AzCommand, GcloudCommand, TerraformCommand, AnsibleCommand,
    FfmpegCommand, ConvertCommand,
    // More C/C++ tools
    ClangppCommand, LdCommand, ArCommand, NmCommand, ObjdumpCommand,
    GdbCommand, LldbCommand, ValgrindCommand,
    // Assembly
    NasmCommand, AsCommand,
    // Haskell
    GhcCommand, CabalCommand, StackCommand,
    // Elixir/Erlang
    ElixirCommand, MixCommand, IexCommand, ErlCommand,
    // Scala/Kotlin
    ScalaCommand, SbtCommand, KotlinCommand, KotlincCommand,
    // More languages
    LuaCommand, LuarocksCommand, PerlCommand, RCommand, RscriptCommand,
    JuliaCommand, OcamlCommand, OpamCommand, DuneCommand,
    RacketCommand, SbclCommand, GfortranCommand, CobolCommand,
    // Linters & formatters
    PrettierCommand, EslintCommand, BlackCommand, RuffCommand, MypyCommand,
    PytestCommand, JestCommand, VitestCommand,
};

/// Registry of all built-in commands
pub struct CommandRegistry {
    commands: HashMap<&'static str, Arc<dyn Command>>,
}

impl CommandRegistry {
    /// Create a new registry with all built-in commands
    pub fn new() -> Self {
        let mut commands: HashMap<&'static str, Arc<dyn Command>> = HashMap::new();

        // Navigation commands
        commands.insert("ls", Arc::new(LsCommand));
        commands.insert("cd", Arc::new(CdCommand));
        commands.insert("pwd", Arc::new(PwdCommand));
        commands.insert("clear", Arc::new(ClearCommand));
        commands.insert("tree", Arc::new(TreeCommand));
        commands.insert("help", Arc::new(HelpCommand));

        // File commands
        commands.insert("cat", Arc::new(CatCommand));
        commands.insert("touch", Arc::new(TouchCommand));
        commands.insert("rm", Arc::new(RmCommand));
        commands.insert("mkdir", Arc::new(MkdirCommand));
        commands.insert("cp", Arc::new(CpCommand));
        commands.insert("mv", Arc::new(MvCommand));
        commands.insert("ln", Arc::new(LnCommand));
        commands.insert("stat", Arc::new(StatCommand));
        commands.insert("file", Arc::new(FileCommand));
        commands.insert("basename", Arc::new(BasenameCommand));
        commands.insert("dirname", Arc::new(DirnameCommand));
        commands.insert("realpath", Arc::new(RealpathCommand));

        // Text commands
        commands.insert("echo", Arc::new(EchoCommand));
        commands.insert("head", Arc::new(HeadCommand));
        commands.insert("tail", Arc::new(TailCommand));
        commands.insert("wc", Arc::new(WcCommand));
        commands.insert("sort", Arc::new(SortCommand));
        commands.insert("uniq", Arc::new(UniqCommand));
        commands.insert("tac", Arc::new(TacCommand));
        commands.insert("cut", Arc::new(CutCommand));
        commands.insert("paste", Arc::new(PasteCommand));
        commands.insert("diff", Arc::new(DiffCommand));
        commands.insert("tr", Arc::new(TrCommand));
        commands.insert("sed", Arc::new(SedCommand));
        commands.insert("awk", Arc::new(AwkCommand));
        commands.insert("rev", Arc::new(RevCommand));
        commands.insert("nl", Arc::new(NlCommand));
        commands.insert("printf", Arc::new(PrintfCommand));

        // Search commands
        commands.insert("grep", Arc::new(GrepCommand));
        commands.insert("find", Arc::new(FindCommand));

        // System commands
        commands.insert("exit", Arc::new(ExitCommand));
        commands.insert("quit", Arc::new(ExitCommand)); // Alias
        commands.insert("which", Arc::new(WhichCommand));
        commands.insert("du", Arc::new(DuCommand));
        commands.insert("df", Arc::new(DfCommand));
        commands.insert("ps", Arc::new(PsCommand));
        commands.insert("kill", Arc::new(KillCommand));
        commands.insert("whoami", Arc::new(WhoamiCommand));
        commands.insert("hostname", Arc::new(HostnameCommand));
        commands.insert("uname", Arc::new(UnameCommand));
        commands.insert("uptime", Arc::new(UptimeCommand));
        commands.insert("free", Arc::new(FreeCommand));
        commands.insert("date", Arc::new(DateCommand));
        commands.insert("cal", Arc::new(CalCommand));
        commands.insert("id", Arc::new(IdCommand));
        commands.insert("neofetch", Arc::new(NeofetchCommand));

        // Network commands
        commands.insert("curl", Arc::new(CurlCommand));
        commands.insert("wget", Arc::new(WgetCommand));
        commands.insert("ping", Arc::new(PingCommand));
        commands.insert("netstat", Arc::new(NetstatCommand));
        commands.insert("traceroute", Arc::new(TracerouteCommand));

        // Hash & encoding commands
        commands.insert("md5sum", Arc::new(Md5sumCommand));
        commands.insert("sha1sum", Arc::new(Sha1sumCommand));
        commands.insert("sha224sum", Arc::new(Sha224sumCommand));
        commands.insert("sha256sum", Arc::new(Sha256sumCommand));
        commands.insert("sha384sum", Arc::new(Sha384sumCommand));
        commands.insert("sha512sum", Arc::new(Sha512sumCommand));
        commands.insert("blake3sum", Arc::new(Blake3sumCommand));
        commands.insert("b3sum", Arc::new(Blake3sumCommand)); // Common alias
        commands.insert("crc32", Arc::new(Crc32Command));
        commands.insert("base64", Arc::new(Base64Command));
        commands.insert("xxd", Arc::new(XxdCommand));

        // Compression commands
        commands.insert("tar", Arc::new(TarCommand));
        commands.insert("zip", Arc::new(ZipCommand));
        commands.insert("unzip", Arc::new(UnzipCommand));
        commands.insert("gzip", Arc::new(GzipCommand));
        commands.insert("gunzip", Arc::new(GunzipCommand));

        // Shell utilities
        commands.insert("alias", Arc::new(AliasCommand));
        commands.insert("env", Arc::new(EnvCommand));
        commands.insert("export", Arc::new(ExportCommand));
        commands.insert("sleep", Arc::new(SleepCommand));
        commands.insert("watch", Arc::new(WatchCommand));
        commands.insert("seq", Arc::new(SeqCommand));
        commands.insert("yes", Arc::new(YesCommand));
        commands.insert("true", Arc::new(TrueCommand));
        commands.insert("false", Arc::new(FalseCommand));
        commands.insert("expr", Arc::new(ExprCommand));
        commands.insert("bc", Arc::new(BcCommand));
        commands.insert("tee", Arc::new(TeeCommand));
        commands.insert("timeout", Arc::new(TimeoutCommand));
        commands.insert("type", Arc::new(TypeCommand));
        commands.insert("command", Arc::new(CommandCommand));
        commands.insert("pushd", Arc::new(PushdCommand));
        commands.insert("popd", Arc::new(PopdCommand));
        commands.insert("dirs", Arc::new(DirsCommand));

        // New file commands
        commands.insert("chmod", Arc::new(ChmodCommand));
        commands.insert("readlink", Arc::new(ReadlinkCommand));
        commands.insert("mktemp", Arc::new(MktempCommand));

        // New text commands
        commands.insert("xargs", Arc::new(XargsCommand));
        commands.insert("column", Arc::new(ColumnCommand));
        commands.insert("strings", Arc::new(StringsCommand));
        commands.insert("split", Arc::new(SplitCommand));
        commands.insert("join", Arc::new(JoinCommand));
        commands.insert("comm", Arc::new(CommCommand));

        // New system commands
        commands.insert("printenv", Arc::new(PrintenvCommand));
        commands.insert("lscpu", Arc::new(LscpuCommand));
        commands.insert("history", Arc::new(HistoryCommand));
        commands.insert("test", Arc::new(TestCommand));
        commands.insert("[", Arc::new(TestCommand)); // Alias for test
        commands.insert("man", Arc::new(ManCommand));
        commands.insert("theme", Arc::new(ThemeCommand));

        // New network commands
        commands.insert("nslookup", Arc::new(NslookupCommand));
        commands.insert("host", Arc::new(HostCommand));
        commands.insert("ifconfig", Arc::new(IfconfigCommand));

        // Editor commands
        commands.insert("nano", Arc::new(NanoCommand));
        commands.insert("vim", Arc::new(VimCommand));
        commands.insert("vi", Arc::new(ViCommand));
        commands.insert("edit", Arc::new(EditCommand));

        // Fun commands
        commands.insert("fortune", Arc::new(FortuneCommand));
        commands.insert("cowsay", Arc::new(CowsayCommand));
        commands.insert("coffee", Arc::new(CoffeeCommand));
        commands.insert("matrix", Arc::new(MatrixCommand));
        commands.insert("pet", Arc::new(PetCommand));

        // AI commands
        commands.insert("ollama", Arc::new(OllamaCommand));

        // ============ External Dev Tools ============

        // Node.js ecosystem
        commands.insert("npm", Arc::new(NpmCommand));
        commands.insert("npx", Arc::new(NpxCommand));
        commands.insert("yarn", Arc::new(YarnCommand));
        commands.insert("pnpm", Arc::new(PnpmCommand));
        commands.insert("bun", Arc::new(BunCommand));
        commands.insert("node", Arc::new(NodeCommand));
        commands.insert("deno", Arc::new(DenoCommand));

        // Python ecosystem
        commands.insert("python", Arc::new(PythonCommand));
        commands.insert("python3", Arc::new(Python3Command));
        commands.insert("pip", Arc::new(PipCommand));
        commands.insert("pip3", Arc::new(Pip3Command));
        commands.insert("uv", Arc::new(UvCommand));
        commands.insert("poetry", Arc::new(PoetryCommand));

        // Rust ecosystem
        commands.insert("cargo", Arc::new(CargoCommand));
        commands.insert("rustc", Arc::new(RustcCommand));
        commands.insert("rustup", Arc::new(RustupCommand));

        // Go
        commands.insert("go", Arc::new(GoCommand));

        // Java/JVM
        commands.insert("java", Arc::new(JavaCommand));
        commands.insert("javac", Arc::new(JavacCommand));
        commands.insert("mvn", Arc::new(MvnCommand));
        commands.insert("gradle", Arc::new(GradleCommand));

        // .NET
        commands.insert("dotnet", Arc::new(DotnetCommand));

        // Containers & orchestration
        commands.insert("docker", Arc::new(DockerCommand));
        commands.insert("kubectl", Arc::new(KubectlCommand));

        // Build tools
        commands.insert("make", Arc::new(MakeCommand));
        commands.insert("cmake", Arc::new(CmakeCommand));

        // Version control
        commands.insert("git", Arc::new(GitCommand));

        // Other languages
        commands.insert("ruby", Arc::new(RubyCommand));
        commands.insert("gem", Arc::new(GemCommand));
        commands.insert("bundle", Arc::new(BundleCommand));
        commands.insert("php", Arc::new(PhpCommand));
        commands.insert("composer", Arc::new(ComposerCommand));
        commands.insert("swift", Arc::new(SwiftCommand));
        commands.insert("zig", Arc::new(ZigCommand));
        commands.insert("gcc", Arc::new(GccCommand));
        commands.insert("g++", Arc::new(GppCommand));
        commands.insert("clang", Arc::new(ClangCommand));

        // Editors & IDEs
        commands.insert("code", Arc::new(CodeCommand));
        commands.insert("cursor", Arc::new(CursorCommand));
        commands.insert("subl", Arc::new(SublCommand));

        // Remote & network
        commands.insert("ssh", Arc::new(SshCommand));
        commands.insert("scp", Arc::new(ScpCommand));
        commands.insert("rsync", Arc::new(RsyncCommand));

        // Cloud & DevOps
        commands.insert("gh", Arc::new(GhCommand));
        commands.insert("aws", Arc::new(AwsCommand));
        commands.insert("az", Arc::new(AzCommand));
        commands.insert("gcloud", Arc::new(GcloudCommand));
        commands.insert("terraform", Arc::new(TerraformCommand));
        commands.insert("ansible", Arc::new(AnsibleCommand));

        // Media tools
        commands.insert("ffmpeg", Arc::new(FfmpegCommand));
        commands.insert("convert", Arc::new(ConvertCommand));

        // More C/C++ tools
        commands.insert("clang++", Arc::new(ClangppCommand));
        commands.insert("ld", Arc::new(LdCommand));
        commands.insert("ar", Arc::new(ArCommand));
        commands.insert("nm", Arc::new(NmCommand));
        commands.insert("objdump", Arc::new(ObjdumpCommand));
        commands.insert("gdb", Arc::new(GdbCommand));
        commands.insert("lldb", Arc::new(LldbCommand));
        commands.insert("valgrind", Arc::new(ValgrindCommand));

        // Assembly
        commands.insert("nasm", Arc::new(NasmCommand));
        commands.insert("as", Arc::new(AsCommand));

        // Haskell
        commands.insert("ghc", Arc::new(GhcCommand));
        commands.insert("cabal", Arc::new(CabalCommand));
        commands.insert("stack", Arc::new(StackCommand));

        // Elixir/Erlang
        commands.insert("elixir", Arc::new(ElixirCommand));
        commands.insert("mix", Arc::new(MixCommand));
        commands.insert("iex", Arc::new(IexCommand));
        commands.insert("erl", Arc::new(ErlCommand));

        // Scala/Kotlin
        commands.insert("scala", Arc::new(ScalaCommand));
        commands.insert("sbt", Arc::new(SbtCommand));
        commands.insert("kotlin", Arc::new(KotlinCommand));
        commands.insert("kotlinc", Arc::new(KotlincCommand));

        // More languages
        commands.insert("lua", Arc::new(LuaCommand));
        commands.insert("luarocks", Arc::new(LuarocksCommand));
        commands.insert("perl", Arc::new(PerlCommand));
        commands.insert("R", Arc::new(RCommand));
        commands.insert("Rscript", Arc::new(RscriptCommand));
        commands.insert("julia", Arc::new(JuliaCommand));
        commands.insert("ocaml", Arc::new(OcamlCommand));
        commands.insert("opam", Arc::new(OpamCommand));
        commands.insert("dune", Arc::new(DuneCommand));
        commands.insert("racket", Arc::new(RacketCommand));
        commands.insert("sbcl", Arc::new(SbclCommand));
        commands.insert("gfortran", Arc::new(GfortranCommand));
        commands.insert("cobc", Arc::new(CobolCommand));

        // Linters & formatters
        commands.insert("prettier", Arc::new(PrettierCommand));
        commands.insert("eslint", Arc::new(EslintCommand));
        commands.insert("black", Arc::new(BlackCommand));
        commands.insert("ruff", Arc::new(RuffCommand));
        commands.insert("mypy", Arc::new(MypyCommand));
        commands.insert("pytest", Arc::new(PytestCommand));
        commands.insert("jest", Arc::new(JestCommand));
        commands.insert("vitest", Arc::new(VitestCommand));

        Self { commands }
    }

    /// Check if a command exists
    pub fn has_command(&self, name: &str) -> bool {
        self.commands.contains_key(name)
    }

    /// Execute a command
    pub fn execute(&self, name: &str, args: &[String], state: &mut TerminalState) -> Result<String> {
        match self.commands.get(name) {
            Some(cmd) => cmd.execute(args, state),
            None => Err(anyhow::anyhow!("Command not found: {}", name)),
        }
    }

    /// Execute a command with stdin input (for piping)
    pub fn execute_with_stdin(&self, name: &str, args: &[String], stdin: Option<&str>, state: &mut TerminalState) -> Result<String> {
        match self.commands.get(name) {
            Some(cmd) => cmd.execute_with_stdin(args, stdin, state),
            None => Err(anyhow::anyhow!("Command not found: {}", name)),
        }
    }

    /// Get a command
    pub fn get(&self, name: &str) -> Option<Arc<dyn Command>> {
        self.commands.get(name).cloned()
    }

    /// List all commands
    pub fn list(&self) -> Vec<(&'static str, &'static str)> {
        let mut list: Vec<_> = self.commands
            .iter()
            .map(|(name, cmd)| (*name, cmd.description()))
            .collect();
        list.sort_by_key(|(name, _)| *name);
        list
    }

    /// Get extended help for a command
    pub fn get_help(&self, name: &str) -> String {
        match self.commands.get(name) {
            Some(cmd) => cmd.extended_help(),
            None => format!("Command not found: {}", name),
        }
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}
