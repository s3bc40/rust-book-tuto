//Commenting Contained Items
// @dev for crates and modules to explain context
//! # RustCh14 Cargo CratesIo
//!
//! `rust_ch_14_cargo_crates_io` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--

// Making Useful Documentation Comments
// @dev support md

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = rust_ch_14_cargo_crates_io::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Commonly Used Sections
// Examples

// Panics: The scenarios in which the function being documented could panic.
// Callers of the function who don’t want their programs to panic should make sure they don’t call the function in these situations.

// Errors: If the function returns a Result, describing the kinds of errors that might occur and what conditions
// might cause those errors to be returned can be helpful to callers so they can write code to handle the different kinds of errors in different ways.

// Safety: If the function is unsafe to call (we discuss unsafety in Chapter 20), there should be a section explaining
// why the function is unsafe and covering the invariants that the function expects callers to uphold.

// Exporting a Convenient Public API with pub use
// //! # Art
// //!
// //! A library for modeling artistic concepts.

// pub mod kinds {
//     /// The primary colors according to the RYB color model.
//     pub enum PrimaryColor {
//         Red,
//         Yellow,
//         Blue,
//     }

//     /// The secondary colors according to the RYB color model.
//     pub enum SecondaryColor {
//         Orange,
//         Green,
//         Purple,
//     }
// }

// pub mod utils {
//     use crate::kinds::*;

//     /// Combines two primary colors in equal amounts to create
//     /// a secondary color.
//     pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
//         // --snip--
//     }
// }

// Crate using art module
// use art::kinds::PrimaryColor;
// use art::utils::mix;

// fn main() {
//     let red = PrimaryColor::Red;
//     let yellow = PrimaryColor::Yellow;
//     mix(red, yellow);
// }

// Remove internal organization from public API
// //! # Art
// //!
// //! A library for modeling artistic concepts.

// pub use self::kinds::PrimaryColor;
// pub use self::kinds::SecondaryColor;
// pub use self::utils::mix;

// pub mod kinds {
//     // --snip--
// }

// pub mod utils {
//     // --snip--
// }

// Now users can use internal struct or convenient one
// use art::mix;
// use art::PrimaryColor;

// fn main() {
//     // --snip--
// }
