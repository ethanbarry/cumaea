//! # Cumaea
//!
//! `cumaea` is a crate which handles prompting for user input.
//! It's named after the Cumaean Sibyl, who sold the Sibylline
//! books to the last king of Rome.

use colored::*;
use std::io::{stdout, Write};

/// An enum that represents colors from the `colored` crate.
pub enum ChoiceColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

/// An enum that represents stylings from the `colored` crate.
pub enum Choice {
    Normal(ChoiceColor),
    On(ChoiceColor),
    Bright(ChoiceColor),
    OnBright(ChoiceColor),
}

/// Prompts for a true/false value given a prompt, color option, and default value.
/// Loops until the input is valid.
///
/// # Examples
///
/// ```rust
/// let the_bool = prompt_tf_default(
///        "Approved? (Y/n) >>> ",
///        Some(Choice::Normal(ChoiceColor::Green)),
///        true,
///    );
/// ```
///
/// Notice how the default option in the prompt is capitalized. The caller has
/// complete responsibility for formatting the prompt; the crate makes no changes
/// besides the color.
///
/// # Panics
///
/// Panics on failure of `stdin().read_line()` or `stdout().flush()`.
pub fn prompt_tf_default(prompt: &str, colored: Option<Choice>, default: bool) -> bool {
    let mut input = String::new();
    loop {
        match colored {
            Some(ref color_choice) => match color_choice {
                Choice::Normal(color) => match color {
                    ChoiceColor::Black => print!("{}", prompt.to_string().black()),
                    ChoiceColor::Red => print!("{}", prompt.to_string().red()),
                    ChoiceColor::Green => print!("{}", prompt.to_string().green()),
                    ChoiceColor::Yellow => print!("{}", prompt.to_string().yellow()),
                    ChoiceColor::Blue => print!("{}", prompt.to_string().blue()),
                    ChoiceColor::Magenta => print!("{}", prompt.to_string().magenta()),
                    ChoiceColor::Cyan => print!("{}", prompt.to_string().cyan()),
                    ChoiceColor::White => print!("{}", prompt.to_string().white()),
                },
                Choice::On(color) => match color {
                    ChoiceColor::Black => print!("{}", prompt.to_string().on_black()),
                    ChoiceColor::Red => print!("{}", prompt.to_string().on_red()),
                    ChoiceColor::Green => print!("{}", prompt.to_string().on_green()),
                    ChoiceColor::Yellow => print!("{}", prompt.to_string().on_yellow()),
                    ChoiceColor::Blue => print!("{}", prompt.to_string().on_blue()),
                    ChoiceColor::Magenta => print!("{}", prompt.to_string().on_magenta()),
                    ChoiceColor::Cyan => print!("{}", prompt.to_string().on_cyan()),
                    ChoiceColor::White => print!("{}", prompt.to_string().on_white()),
                },
                Choice::Bright(color) => match color {
                    ChoiceColor::Black => print!("{}", prompt.to_string().bright_black()),
                    ChoiceColor::Red => print!("{}", prompt.to_string().bright_red()),
                    ChoiceColor::Green => print!("{}", prompt.to_string().bright_green()),
                    ChoiceColor::Yellow => print!("{}", prompt.to_string().bright_yellow()),
                    ChoiceColor::Blue => print!("{}", prompt.to_string().bright_blue()),
                    ChoiceColor::Magenta => print!("{}", prompt.to_string().bright_magenta()),
                    ChoiceColor::Cyan => print!("{}", prompt.to_string().bright_cyan()),
                    ChoiceColor::White => print!("{}", prompt.to_string().bright_white()),
                },
                Choice::OnBright(color) => match color {
                    ChoiceColor::Black => print!("{}", prompt.to_string().on_bright_black()),
                    ChoiceColor::Red => print!("{}", prompt.to_string().on_bright_red()),
                    ChoiceColor::Green => print!("{}", prompt.to_string().on_bright_green()),
                    ChoiceColor::Yellow => print!("{}", prompt.to_string().on_bright_yellow()),
                    ChoiceColor::Blue => print!("{}", prompt.to_string().on_bright_blue()),
                    ChoiceColor::Magenta => print!("{}", prompt.to_string().on_bright_magenta()),
                    ChoiceColor::Cyan => print!("{}", prompt.to_string().on_bright_cyan()),
                    ChoiceColor::White => print!("{}", prompt.to_string().on_bright_white()),
                },
            },
            None => {
                print!("{}", prompt.trim())
            }
        }

        stdout().flush().expect("Flushing line failed.");
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        input = input.trim().to_string();
        if input.eq_ignore_ascii_case("y") || input.eq_ignore_ascii_case("n") || input.is_empty() {
            break;
        }
    }

    // Loop cannot have exited w/o input being valid.
    match input.as_str() {
        "Y" | "y" => true,
        "N" | "n" => false,
        _ => default,
    }
}

/// Prompts for a selection given a prompt, list of choices, color option, and default value.
/// No looping occurs.
///
/// # Examples
///
/// ```rust
/// let the_string = prompt_selection(
///     "Choose something",
///     "(a)pples, (b)ananas, (c)arrots, (D)oughnuts",
///     Some(Choice::Normal(ChoiceColor::Cyan)),
///     "D",
/// );```
///
/// Notice how the default option in the prompt is capitalized. The caller has
/// partial responsibility for formatting the prompt; the crate prints the
/// question in default colors, followed by a colon,
/// with the list in brackets & colorized follwed by another colon and a space.
/// For example:
///
/// ```bash
/// Choose something: [(a)pples, (b)ananas, (c)arrots, (D)oughnuts]:
/// ```
///
/// # Panics
///
/// Panics on failure of `stdin().read_line()` or `stdout().flush()`.
pub fn prompt_selection(
    prompt: &str,
    list: &str,
    colored: Option<Choice>,
    default: &str,
) -> String {
    let mut input = String::new();
    match colored {
        Some(ref color_choice) => match color_choice {
            Choice::Normal(color) => match color {
                ChoiceColor::Black => {
                    print!("{}: [{}]: ", prompt, list.to_string().black())
                }
                ChoiceColor::Red => {
                    print!("{}: [{}]: ", prompt, list.to_string().red())
                }
                ChoiceColor::Green => {
                    print!("{}: [{}]: ", prompt, list.to_string().green())
                }
                ChoiceColor::Yellow => {
                    print!("{}: [{}]: ", prompt, list.to_string().yellow())
                }
                ChoiceColor::Blue => {
                    print!("{}: [{}]: ", prompt, list.to_string().blue())
                }
                ChoiceColor::Magenta => {
                    print!("{}: [{}]: ", prompt, list.to_string().magenta())
                }
                ChoiceColor::Cyan => {
                    print!("{}: [{}]: ", prompt, list.to_string().cyan())
                }
                ChoiceColor::White => {
                    print!("{}: [{}]: ", prompt, list.to_string().white())
                }
            },
            Choice::On(color) => match color {
                ChoiceColor::Black => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().on_black()
                ),
                ChoiceColor::Red => {
                    print!("{}: [{}]: ", prompt, list.to_string().on_red())
                }
                ChoiceColor::Green => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().on_green()
                ),
                ChoiceColor::Yellow => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().on_yellow()
                ),
                ChoiceColor::Blue => {
                    print!("{}: [{}]: ", prompt, list.to_string().on_blue())
                }
                ChoiceColor::Magenta => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().on_magenta()
                ),
                ChoiceColor::Cyan => {
                    print!("{}: [{}]: ", prompt, list.to_string().on_cyan())
                }
                ChoiceColor::White => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().on_white()
                ),
            },
            Choice::Bright(color) => match color {
                ChoiceColor::Black => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().bright_black()
                ),
                ChoiceColor::Red => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().bright_red()
                ),
                ChoiceColor::Green => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().bright_green()
                ),
                ChoiceColor::Yellow => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().bright_yellow()
                ),
                ChoiceColor::Blue => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().bright_blue()
                ),
                ChoiceColor::Magenta => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().bright_magenta()
                ),
                ChoiceColor::Cyan => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().bright_cyan()
                ),
                ChoiceColor::White => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().bright_white()
                ),
            },
            Choice::OnBright(color) => match color {
                ChoiceColor::Black => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().on_bright_black()
                ),
                ChoiceColor::Red => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().on_bright_red()
                ),
                ChoiceColor::Green => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().on_bright_green()
                ),
                ChoiceColor::Yellow => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().on_bright_yellow()
                ),
                ChoiceColor::Blue => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().on_bright_blue()
                ),
                ChoiceColor::Magenta => {
                    print!(
                        "{}: [{}]: ",
                        prompt,
                        list.to_string().on_bright_magenta()
                    )
                }
                ChoiceColor::Cyan => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().on_bright_cyan()
                ),
                ChoiceColor::White => print!(
                    "{}: [{}]: ",
                    prompt,
                    list.to_string().on_bright_white()
                ),
            },
        },
        None => {
            print!("{}: [{}]: ", prompt.trim(), list.trim())
        }
    }

    stdout().flush().expect("Flushing line failed.");
    input.clear();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    if input.trim().is_empty() {
        default.to_string()
    } else {
        input.trim().to_string()
    }
}
