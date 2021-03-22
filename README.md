# rusty-journal
version: 0.1.0
## description
A command line to-do app written in Rust.
The created task will be saved in json format.

### USAGE:
    cargo build
    target/debug/rusty-journal [OPTIONS] <SUBCOMMAND>

### FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

### OPTIONS:
    -j, --journal-file <journal-file>    Use a differnt journal file

### SUBCOMMANDS:
    add     Write tasks to the journak file
    done    Remove an entry from the journal file by positon
    help    Prints this message or the help of the given subcommand(s)
    list    List all tasks in the journal file
