/*
This example requires the `clap` and `stream` features.

Run it with:
```bash
cargo run -q -F=clap --example debug
```

Try running it:
- with/without the NO_COLOR, CLICOLOR, CLICOLOR_FORCE environment variables
- with/without the `--color={always,auto,never}` option
- piping the stdout `| cat` or piping both stdout/stderr `|& cat`

For example:
```bash
CLICOLOR_FORCE=1 cargo run -q -F=clap --example debug -- --color=never | cat
```
*/

use clap::Parser;
use colored::{control::set_override, Colorize};
use should_color::{clap_color, resolve, ColorChoice};

#[derive(Debug, Parser)]
#[clap(
    name = "app-exe",
    author,
    version,
    about,
    color = clap_color()
)]
struct Cli {
    /// Coloring
    #[clap(long, value_name = "WHEN", arg_enum, global = true)]
    color: Option<ColorChoice>,
}

fn main() {
    let cli = Cli::parse();

    // resolve from cli preference, environment variables, default value
    let color_choice = resolve(cli.color).unwrap_or(ColorChoice::Auto);

    let color_stdout = color_choice.for_stream(atty::Stream::Stdout);
    let color_stderr = color_choice.for_stream(atty::Stream::Stderr);

    println!("         cli = {cli:?}");
    println!("color_choice = {color_choice:?}");

    set_override(color_stdout);
    println!(
        "{}: {}",
        "Colorize stdout".bright_green().italic(),
        format!("{}", color_stdout).bright_yellow()
    );

    set_override(color_stderr);
    eprintln!(
        "{}: {}",
        "Colorize stderr".bright_red().underline(),
        format!("{}", color_stderr).bright_yellow()
    );
}
