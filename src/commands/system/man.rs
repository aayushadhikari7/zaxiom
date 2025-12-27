//! man command - display command documentation
//!
//! Shows detailed help for built-in commands

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct ManCommand;

impl Command for ManCommand {
    fn name(&self) -> &'static str {
        "man"
    }

    fn description(&self) -> &'static str {
        "Display manual pages for commands"
    }

    fn usage(&self) -> &'static str {
        "man <command>"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Ok("What manual page do you want?\nUsage: man <command>".to_string());
        }

        let cmd = args[0].to_lowercase();

        if cmd == "-h" || cmd == "--help" {
            return Ok("Usage: man <command>\n\
                Display the manual page for a command.\n\n\
                Examples:\n  \
                  man ls\n  \
                  man grep\n  \
                  man git".to_string());
        }

        let manual = get_manual(&cmd);
        Ok(manual)
    }
}

fn get_manual(cmd: &str) -> String {
    match cmd {
        "ls" => r#"LS(1)                           User Commands                          LS(1)

NAME
       ls - list directory contents

SYNOPSIS
       ls [OPTION]... [FILE]...

DESCRIPTION
       List information about the FILEs (the current directory by default).

       -a, --all
              do not ignore entries starting with .

       -l     use a long listing format

       -h, --human-readable
              with -l, print sizes in human readable format (e.g., 1K 234M 2G)

       -R, --recursive
              list subdirectories recursively

       -t     sort by modification time, newest first

       -S     sort by file size, largest first

       -r     reverse order while sorting

EXAMPLES
       ls -la
              List all files including hidden, in long format

       ls -lhS
              List files sorted by size, human readable"#.to_string(),

        "cd" => r#"CD(1)                           User Commands                          CD(1)

NAME
       cd - change the current directory

SYNOPSIS
       cd [DIR]

DESCRIPTION
       Change the current directory to DIR. The default DIR is the value of
       the HOME environment variable.

       -      Go to the previous directory (OLDPWD)

       ~      Go to the home directory

       ..     Go to the parent directory

EXAMPLES
       cd /tmp
              Change to /tmp directory

       cd -
              Go back to the previous directory

       cd ~/Documents
              Change to Documents in home directory"#.to_string(),

        "grep" => r#"GREP(1)                         User Commands                         GREP(1)

NAME
       grep - print lines that match patterns

SYNOPSIS
       grep [OPTION]... PATTERN [FILE]...

DESCRIPTION
       grep searches for PATTERN in each FILE. PATTERN is a regular expression.

       -i, --ignore-case
              ignore case distinctions in patterns and input data

       -v, --invert-match
              select non-matching lines

       -c, --count
              print only a count of matching lines per FILE

       -n, --line-number
              prefix each line of output with line number

       -r, --recursive
              read all files under each directory, recursively

       -l, --files-with-matches
              print only names of FILEs containing matches

       -E, --extended-regexp
              interpret PATTERN as an extended regular expression

EXAMPLES
       grep "error" log.txt
              Find lines containing "error" in log.txt

       grep -r "TODO" src/
              Recursively search for TODO in src/ directory

       grep -in "warning" *.log
              Case-insensitive search with line numbers"#.to_string(),

        "find" => r#"FIND(1)                         User Commands                         FIND(1)

NAME
       find - search for files in a directory hierarchy

SYNOPSIS
       find [PATH]... [EXPRESSION]

DESCRIPTION
       find searches the directory tree rooted at each given PATH for files
       matching the given expression.

       -name PATTERN
              file name matches glob pattern

       -type TYPE
              file is of type (f=file, d=directory)

       -size N
              file uses N units of space

       -mtime N
              file was modified N days ago

       -maxdepth N
              descend at most N levels of directories

EXAMPLES
       find . -name "*.rs"
              Find all Rust files in current directory tree

       find /tmp -type f -size +10M
              Find files larger than 10MB in /tmp

       find . -name "*.log" -mtime +7
              Find log files older than 7 days"#.to_string(),

        "cat" => r#"CAT(1)                          User Commands                          CAT(1)

NAME
       cat - concatenate files and print on the standard output

SYNOPSIS
       cat [OPTION]... [FILE]...

DESCRIPTION
       Concatenate FILE(s) to standard output.

       -n, --number
              number all output lines

       -b, --number-nonblank
              number nonempty output lines

       -s, --squeeze-blank
              suppress repeated empty output lines

       -E, --show-ends
              display $ at end of each line

EXAMPLES
       cat file.txt
              Display contents of file.txt

       cat file1.txt file2.txt
              Concatenate and display two files

       cat -n file.txt
              Display with line numbers"#.to_string(),

        "git" => r#"GIT(1)                          User Commands                          GIT(1)

NAME
       git - the stupid content tracker

SYNOPSIS
       git <command> [options]

DESCRIPTION
       Git is a fast, scalable, distributed revision control system.

COMMON COMMANDS
       init        Create an empty Git repository
       clone       Clone a repository
       add         Add file contents to the index
       commit      Record changes to the repository
       push        Update remote refs
       pull        Fetch and integrate with another repository
       status      Show the working tree status
       log         Show commit logs
       diff        Show changes between commits
       branch      List, create, or delete branches
       checkout    Switch branches or restore files
       merge       Join two or more development histories

ZAXIOM SHORTCUTS
       gs          git status
       gd          git diff
       gl          git log --oneline -20
       gp          git push
       ga          git add
       gc          git commit

EXAMPLES
       git status
              Show changes in working directory

       git add .
              Stage all changes

       git commit -m "feat: add new feature"
              Commit with a message"#.to_string(),

        "fortune" => r#"FORTUNE(1)                      User Commands                      FORTUNE(1)

NAME
       fortune - display a random programming quote or fortune

SYNOPSIS
       fortune

DESCRIPTION
       Print a random, hopefully interesting, programming-related quote or
       adage. A classic Unix tradition for terminal entertainment.

EXAMPLES
       fortune
              Display a random fortune

       fortune | cowsay
              Pipe fortune to cowsay for extra fun"#.to_string(),

        "cowsay" => r#"COWSAY(1)                       User Commands                       COWSAY(1)

NAME
       cowsay - generate ASCII pictures of a cow with a message

SYNOPSIS
       cowsay [-f cowfile] [message]

DESCRIPTION
       Cowsay generates an ASCII picture of a cow saying something provided
       by the user. If run with no arguments, it will say "Moo!"

       -f cowfile
              Select a different creature (cow, robot, tux)

CREATURES
       cow      The classic cow (default)
       robot    A friendly robot
       tux      Tux the Linux penguin

EXAMPLES
       cowsay Hello World!
              Have the cow say "Hello World!"

       cowsay -f robot Beep boop!
              Have a robot say "Beep boop!"

       fortune | cowsay
              Have the cow tell you a fortune"#.to_string(),

        "coffee" => r#"COFFEE(1)                       User Commands                       COFFEE(1)

NAME
       coffee - brew some ASCII art coffee

SYNOPSIS
       coffee [--espresso|--tea]

DESCRIPTION
       Display a comforting ASCII art beverage. Because every programmer
       needs a coffee break.

       --espresso, -e
              Brew a small but mighty espresso

       --tea, -t
              Brew a calming cup of tea instead

EXAMPLES
       coffee
              Brew a regular cup of coffee

       coffee --espresso
              Get a quick espresso shot

       coffee --tea
              For the tea lovers"#.to_string(),

        "matrix" => r#"MATRIX(1)                       User Commands                       MATRIX(1)

NAME
       matrix - display Matrix-style digital rain

SYNOPSIS
       matrix

DESCRIPTION
       Display a Matrix-style digital rain animation.
       Wake up, Neo...

EXAMPLES
       matrix
              Enter the Matrix"#.to_string(),

        "neofetch" => r#"NEOFETCH(1)                     User Commands                     NEOFETCH(1)

NAME
       neofetch - display system information with ASCII art

SYNOPSIS
       neofetch

DESCRIPTION
       Neofetch displays information about your operating system, software
       and hardware in an aesthetic and visually pleasing way.

       Displays:
           - Operating system and version
           - Hostname
           - Kernel version
           - Shell (Zaxiom!)
           - CPU information
           - Memory usage
           - Uptime
           - Zaxiom robot mascot

EXAMPLES
       neofetch
              Display system information"#.to_string(),

        _ => format!(r#"No manual entry for {}

Try 'help' to see all available commands.
Or use '{} --help' to see command-specific help."#, cmd, cmd),
    }
}
