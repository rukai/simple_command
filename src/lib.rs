//! When writing a `build.rs` to run some commands you naturally want to see the output of these commands.
//! This is however impossible because `build.rs` cannot display stdout or stderr.
//! The next best option is to display the output when the command goes wrong for any reason.
//! The `simple_command` function does exactly that, panicking if anything at all goes wrong and
//! displaying the combined stderr and stdout.
//!
//! Possible reasons for panicking include:
//! *   No command specified
//! *   Command does not exist
//! *   Non-zero return value
//!
//! DO NOT use this function in your actual application, you should be properly handling error cases!

use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead, Write};
use std::str;

pub fn simple_command(cmd: &str) {
    let words: Vec<_> = cmd.split_whitespace().collect();
    if words.len() == 0 {
        panic!("No command specified");
    }

    let mut command = Command::new(words[0]);
    for word in &words[1..] {
        command.arg(word);
    }
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut output = Vec::new();
    let mut child = command.spawn().unwrap();
    {
        let stdout = child.stdout.as_mut().expect("Wasn't stdout");
        let stderr = child.stderr.as_mut().expect("Wasn't stderr");

        let mut stdout = BufReader::new(stdout);
        let mut stderr = BufReader::new(stderr);

        loop {
            let (stdout_bytes, stderr_bytes) = match (stdout.fill_buf(), stderr.fill_buf()) {
                (Ok(stdout), Ok(stderr)) => {
                    output.write_all(stdout).expect("Couldn't write");
                    output.write_all(stderr).expect("Couldn't write");

                    (stdout.len(), stderr.len())
                }
                other => panic!("Failed to read stdout or stderr... {:?}", other)
            };

            if stdout_bytes == 0 && stderr_bytes == 0 {
                // Seems less-than-ideal; should be some way of
                // telling if the child has actually exited vs just
                // not outputting anything.
                break;
            }

            stdout.consume(stdout_bytes);
            stderr.consume(stderr_bytes);
        }
    }
    let output = String::from_utf8_lossy(&output);

    let status = match child.wait() {
        Ok(status) => status,
        Err(err) => panic!("{:?}", err)
    };

    if !status.success() {
        if let Some(status) = status.code() {
            panic!("\nCommand \"{}\" failed with return value {}\n{}", cmd, status, output);
        }
        else {
            panic!("\nCommand \"{}\" failed with no return value\n{}", cmd, output);
        }
    }
}
