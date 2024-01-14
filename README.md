# cumaea

`cumaea` is a crate that handles prompting for user input. It's a personal project; if you found it somehow, DO NOT USE IT. I'll probably break it a week from now. :)

## Usage

If you're still here, this crate exports two functions and a couple of `[enum]`s that govern the coloration and decoration of your prompts.
I suggest bringing into scope the individual variants of the enums, for less typing. Or, you can type `Some(cumaea::ColorChoice::Normal(cumaea::Color::Magenta))` in your function calls!
