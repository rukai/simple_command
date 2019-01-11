fn main() {
    // this should succeed
    simple_command::simple_command("ls");

    // this should panic, because `tree --foo` gives non-zero return code
    simple_command::simple_command("tree --foo");
}
