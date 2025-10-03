use std::env;
use std::option::Option;
use std::process::{Command, ExitCode, ExitStatus};

const HELP_MESSAGE: &str = "\
USAGE: cargo-update-shim [--cutoff DATE] [--help] [--] [arguments to be forwarded to cargo update…]

cargo-update-shim is a wrapper that sometimes runs “cargo update”. You can use
cargo-update-shim to help make sure that you run “cargo update” every so often.
For example, you can use cargo-update-shim to help ensure that you run
cargo-update once every two weeks or once every three months.

Options:

    --cutoff DATE    Run “cargo update” only if Cargo.lock hasn’t been changed
                     since DATE. Changes to Cargo.lock only count if they have
                     been committed. If there are multiple Cargo.lock files in
                     the current Git repository, then “cargo update” will only
                     be run if none of them have been updated since DATE. DATE
                     can be any date string that’s accepted by git-log’s --since
                     option.

    --help           Display this help message and then exit without doing
                     anything.

    --               Any arguments that come before the “--” argument will be
                     parsed by cargo-update-shim. If any of those arguments
                     aren’t recognized, then cargo-update-shim will forward them
                     to “cargo update”. Any arguments that come after the “--”
                     argument will be ignored by cargo-update-shim. They will be
                     forwarded to “cargo update” no matter what.

Examples:

    Run “cargo update” unconditionally:

        cargo-update-shim

    Run “cargo update” only if the last change to Cargo.lock was committed more
    than one week ago:

        cargo-update-shim --cutoff \"1 week ago\"

    Run “cargo update” only if the last change to Cargo.lock was committed more
    than three months ago:

        cargo-update-shim --cutoff \"3 months ago\"

    Run “cargo update --verbose” only if the last change to Cargo.lock was
    committed more than three months ago:

        cargo-update-shim --verbose --cutoff \"3 months ago\"

    Show documentation for cargo-update-shim:

        cargo-update-shim --help

    Run “cargo update --help” only if the last change to Cargo.lock was
    committed more than 5 days ago:

        cargo-update-shim --cutoff \"5 days ago\" -- --help

    (That last example is probably not very useful, but I needed a way to
    showcase the “--” argument).
";

fn command_failed_error(command: Command, exit_status: ExitStatus) -> ExitCode {
    match exit_status.code() {
        Some(exit_code_number) => {
            eprintln!(
                "ERROR: Attempted to run this command, but it failed with exit status {exit_code_number}: {command:?}"
            );
            match TryInto::<u8>::try_into(exit_code_number) {
                Ok(exit_code_number_u8) => ExitCode::from(exit_code_number_u8),
                Err(_) => ExitCode::FAILURE,
            }
        }
        None => {
            eprintln!(
                "ERROR: Started running this command, but it was terminated by a signal: {command:?}"
            );
            ExitCode::FAILURE
        }
    }
}

fn main() -> ExitCode {
    // Step 1: Parse command-line arguments and generate a “cargo update” command.
    let mut cargo_command = Command::new("cargo");
    cargo_command.arg("update");
    let mut cutoff_date: Option<String> = None;
    let mut expecting_date = false;
    let mut args_iterator = env::args();
    // Skip the first argument because it’s just the name of the program.
    for argument in args_iterator.by_ref().skip(1) {
        if expecting_date {
            cutoff_date = Some(argument);
            expecting_date = false;
        } else if argument == "--cutoff" {
            expecting_date = true;
        } else if argument == "--help" {
            print!("{HELP_MESSAGE}");
            return ExitCode::SUCCESS;
        } else if argument == "--" {
            // I don’t think that there’s any situations where using “--” as a command-line
            // argument separator would actually be helpful. Supporting “--” as a command-line
            // separator allows you to do stuff like this…
            //
            // $ cargo-update-shim --cutoff='1 week ago' -- --help
            //
            // …but doing stuff like that isn’t actually helpful (why would you want to run “cargo
            // update --help” once a week?). That being said, I’m still including support for using
            // “--” as a command-line separator just in case there’s something that I haven’t
            // thought of yet. It’s better to be safe than sorry.
            break;
        } else {
            cargo_command.arg(argument);
        }
    }
    // Unconditionally forward any arguments after “--” to cargo update.
    cargo_command.args(args_iterator);

    // Step 2: Determine whether or not we need to run “cargo update”.
    let run_cargo_update = match cutoff_date {
        // If there were any commits made between the cutoff date and now that changed Cargo.lock,
        // then set run_cargo_update to false. If there were no commits made between the cutoff
        // date and now that change Cargo.lock, then set run_cargo_update to true.
        Some(date) => {
            let mut git_log_command = Command::new("git");
            git_log_command.args([
                "log",
                "--since",
                &date,
                // We would do the same thing regardless of whether git-log finds one commit or
                // more than one commit. This next argument makes it so that git-log won’t bother
                // looking for more than one commit. Hopefully, this will make git-log use less
                // time and memory.
                "--max-count=1",
                // We don’t need git-log to tell us anything about the commit that it finds (if it
                // finds a commit). We just need git-log to print something. This next argument
                // makes it so that it just prints a single “c” if it finds a commit. (“c” stands
                // for “commit”). Hopefully, this will make git-log and this program use less time
                // and memory.
                "--format=format:c",
                // These next few arguments make it so that git-log only considers commits that
                // modify a file named “Cargo.lock”.
                "--",
                "Cargo.lock",
                "*/Cargo.lock",
            ]);
            let git_log_result = git_log_command
                .output()
                .expect("git should be on your PATH and should be executable");
            if !git_log_result.status.success() {
                return command_failed_error(git_log_command, git_log_result.status);
            }
            git_log_result.stdout.len() == 0
        }
        // The user didn’t specify a cutoff date so just run “cargo update” no matter what.
        None => true,
    };

    // Step 3: Potentially run the cargo_command.
    if run_cargo_update {
        let cargo_command_status = cargo_command
            .status()
            .expect("cargo should be on your PATH and should be executable");
        if cargo_command_status.success() {
            ExitCode::SUCCESS
        } else {
            command_failed_error(cargo_command, cargo_command_status)
        }
    } else {
        ExitCode::SUCCESS
    }
}
