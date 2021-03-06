pub use self::character::*;
pub use self::ctl::*;
pub use self::direction::*;
pub use self::item::*;
pub use self::position::*;

mod character;
mod ctl;
mod direction;
mod item;
mod position;

// The in-game ID for an object.
pub type ObjectId = u16;
