//! fortune command - display a random fortune
//!
//! Classic Unix fortune-cookie program

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct FortuneCommand;

const FORTUNES: &[&str] = &[
    "The best way to predict the future is to invent it. - Alan Kay",
    "Talk is cheap. Show me the code. - Linus Torvalds",
    "Any sufficiently advanced technology is indistinguishable from magic. - Arthur C. Clarke",
    "First, solve the problem. Then, write the code. - John Johnson",
    "Code is like humor. When you have to explain it, it's bad. - Cory House",
    "Simplicity is the soul of efficiency. - Austin Freeman",
    "Make it work, make it right, make it fast. - Kent Beck",
    "The only way to learn a new programming language is by writing programs in it. - Dennis Ritchie",
    "Perfection is achieved not when there is nothing more to add, but when there is nothing left to take away. - Antoine de Saint-Exupéry",
    "Programming today is a race between software engineers striving to build bigger and better idiot-proof programs, and the Universe trying to produce bigger and better idiots. So far, the Universe is winning. - Rick Cook",
    "The computer was born to solve problems that did not exist before. - Bill Gates",
    "In theory, there is no difference between theory and practice. But in practice, there is. - Jan L. A. van de Snepscheut",
    "Debugging is twice as hard as writing the code in the first place. - Brian Kernighan",
    "The most disastrous thing that you can ever learn is your first programming language. - Alan Kay",
    "Always code as if the guy who ends up maintaining your code will be a violent psychopath who knows where you live. - John Woods",
    "There are only two kinds of languages: the ones people complain about and the ones nobody uses. - Bjarne Stroustrup",
    "Programming is the art of telling another human being what one wants the computer to do. - Donald Knuth",
    "Any fool can write code that a computer can understand. Good programmers write code that humans can understand. - Martin Fowler",
    "The best error message is the one that never shows up. - Thomas Fuchs",
    "A language that doesn't affect the way you think about programming is not worth knowing. - Alan Perlis",
    "Rust: Where the compiler is your frenemy. - Anonymous",
    "Fear not the borrow checker, for it guards against the chaos of the void. - Rustacean Proverb",
    "In Rust we trust. - Community Motto",
    "Memory safety isn't a feature, it's a lifestyle. - Anonymous Rustacean",
    "Why did the Rust programmer break up with C++? Too many issues with commitment (and lifetimes). - r/rustjerk",
];

impl Command for FortuneCommand {
    fn name(&self) -> &'static str {
        "fortune"
    }

    fn description(&self) -> &'static str {
        "Display a random programming fortune"
    }

    fn usage(&self) -> &'static str {
        "fortune"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        for arg in args {
            if arg == "-h" || arg == "--help" {
                return Ok("Usage: fortune\n\
                    Display a random programming fortune or quote."
                    .to_string());
            }
        }

        // Simple pseudo-random selection based on current time
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let idx = (now.as_nanos() as usize) % FORTUNES.len();

        Ok(format!("  \n  {}\n", FORTUNES[idx]))
    }
}
