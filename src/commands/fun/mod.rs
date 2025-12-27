//! Fun and novelty commands
//!
//! fortune, cowsay, coffee, matrix, pet - because terminals should be fun!

mod fortune;
mod cowsay;
mod coffee;
mod matrix;
mod pet;

pub use fortune::FortuneCommand;
pub use cowsay::CowsayCommand;
pub use coffee::CoffeeCommand;
pub use matrix::MatrixCommand;
pub use pet::PetCommand;
