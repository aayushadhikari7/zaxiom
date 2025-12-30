//! Fun and novelty commands
//!
//! fortune, cowsay, coffee, matrix, pet - because terminals should be fun!

mod coffee;
mod cowsay;
mod fortune;
mod matrix;
mod pet;

pub use coffee::CoffeeCommand;
pub use cowsay::CowsayCommand;
pub use fortune::FortuneCommand;
pub use matrix::MatrixCommand;
pub use pet::PetCommand;
