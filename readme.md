# Simple Command [![Build Status](https://travis-ci.com/rukai/simple_command.svg?branch=master)](https://travis-ci.com/rukai/simple_command) [![Crates.io](https://img.shields.io/crates/v/simple_command.svg)](https://crates.io/crates/simple_command)

When writing a `build.rs` to run some commands you naturally want to see the output of these commands.
This is however impossible because `build.rs` cannot display stdout or stderr.
The next best option is to display the output when the command goes wrong for any reason.
The `simple_command` function does exactly that, panicking if anything at all goes wrong and
displaying the combined stderr and stdout.

Possible reasons for panicking include:
*   No command specified
*   Command does not exist
*   Non-zero return value

DO NOT use this function in your actual application, you should be properly handling error cases!

## Example build.rs
```
use simple_command::simple_command;

fn main() {
    // this should succeed
    simple_command::simple_command("ls");

    // this should panic, because `tree --foo` gives non-zero return code
    simple_command::simple_command("tree --foo");
}
```
