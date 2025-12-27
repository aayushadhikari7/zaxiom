//! coffee command - brew some ASCII coffee
//!
//! Because every programmer needs coffee!

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct CoffeeCommand;

const COFFEE_ART: &str = r#"
           (  )   (   )  )
            ) (   )  (  (
            ( )  (    ) )
            _____________
           <_____________> ___
           |             |/ _ \
           |               | | |
           |   COFFEE     |_| |
           |   ~~~~~~        |_/
           |  Best served   |
           |    STRONG!     |
           |_______________|
           \_____________/"#;

const ESPRESSO_ART: &str = r#"
         . . . .
          )   (
        .--'---'--.
       (  ESPRESSO )
        '-.______.-'
           |    |
           |____|
          (____)
"#;

const TEA_ART: &str = r#"
              )  (
             (   ) )
              ) ( (
            _______)_
         .-'---------|
        ( C|/\/\/\/\/|
         '-./\/\/\/\/|
           '_________'
            '-------'
              TEA
"#;

impl Command for CoffeeCommand {
    fn name(&self) -> &'static str {
        "coffee"
    }

    fn description(&self) -> &'static str {
        "Brew some ASCII art coffee"
    }

    fn usage(&self) -> &'static str {
        "coffee [--espresso|--tea]"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        for arg in args {
            if arg == "-h" || arg == "--help" {
                return Ok("Usage: coffee [--espresso|--tea]\n\
                    Brew a warm beverage in ASCII art.\n\n\
                    Options:\n  \
                      --espresso  Small but mighty\n  \
                      --tea       For the tea lovers\n\n\
                    Take a break, you deserve it!".to_string());
            }

            if arg == "--espresso" || arg == "-e" {
                return Ok(format!("{}\n\n  A quick shot to keep you going!", ESPRESSO_ART));
            }

            if arg == "--tea" || arg == "-t" {
                return Ok(format!("{}\n\n  A calming cup of tea...", TEA_ART));
            }
        }

        Ok(format!("{}\n\n  ☕ Fresh hot coffee, just for you!", COFFEE_ART))
    }
}
