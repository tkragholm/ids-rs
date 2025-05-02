console is a library for Rust that provides access to various terminal features so you can build nicer looking command line interfaces. It comes with various tools and utilities for working with Terminals and formatting text.

Best paired with other libraries in the family:

    dialoguer
    indicatif

Terminal Access

The terminal is abstracted through the console::Term type. It can either directly provide access to the connected terminal or by buffering up commands. A buffered terminal will however not be completely buffered on windows where cursor movements are currently directly passed through.

Example usage:

use std::thread;

use std::time::Duration;


use console::Term;


let term = Term::stdout();

term.write_line("Hello World!")?;

thread::sleep(Duration::from_millis(2000));

term.clear_line()?;

Colors and Styles

console automatically detects when to use colors based on the tty flag. It also provides higher level wrappers for styling text and other things that can be displayed with the style function and utility types.

Example usage:

use console::style;


println!("This is {} neat", style("quite").cyan());

You can also store styles and apply them to text later:

use console::Style;


let cyan = Style::new().cyan();

println!("This is {} neat", cyan.apply_to("quite"));

Working with ANSI Codes

The crate provides the function strip_ansi_codes to remove ANSI codes from a string as well as measure_text_width to calculate the width of a string as it would be displayed by the terminal. Both of those together are useful for more complex formatting.
Unicode Width Support

By default this crate depends on the unicode-width crate to calculate the width of terminal characters. If you do not need this you can disable the unicode-width feature which will cut down on dependencies.
Features

By default all features are enabled. The following features exist:

    unicode-width: adds support for unicode width calculations
    ansi-parsing: adds support for parsing ansi codes (this adds support for stripping and taking ansi escape codes into account for length calculations).

Structs

AnsiCodeIterator
    An iterator over ansi codes in a string.
Emoji
    “Intelligent” emoji formatter.
Style
    A stored style that can be applied.
StyledObject
    A formatting wrapper that can be styled for a terminal.
Term
    Abstraction around a terminal.
TermFeatures
    Gives access to the terminal features.

Enums

Alignment
    Defines the alignment for padding operations.
Attribute
    A terminal style attribute.
Color
    A terminal color.
Key
    Key mapping
TermFamily
    The family of the terminal.
TermTarget
    Where the term is writing.

Functions

colors_enabled
    Returns true if colors should be enabled for stdout.
colors_enabled_stderr
    Returns true if colors should be enabled for stderr.
measure_text_width
    Measure the width of a string in terminal characters.
pad_str
    Pads a string to fill a certain number of characters.
pad_str_with
    Pads a string with specific padding to fill a certain number of characters.
set_colors_enabled
    Forces colorization on or off for stdout.
set_colors_enabled_stderr
    Forces colorization on or off for stderr.
strip_ansi_codes
    Helper function to strip ansi codes.
style
    Wraps an object for formatting for styling.
truncate_str
    Truncates a string to a certain number of characters.
user_attended
    A fast way to check if the application has a user attended for stdout.
user_attended_stderr
    A fast way to check if the application has a user attended for stderr.
