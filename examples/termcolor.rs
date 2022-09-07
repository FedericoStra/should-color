/**
This example requires the `clap` and `stream` features.

Run it with:
```bash
cargo run -q -F=clap --example termcolor
```

Try running it:
- with/without the NO_COLOR, CLICOLOR, CLICOLOR_FORCE environment variables
- with/without the `--color={always,auto,never}` option
- piping the stdout `| cat` or piping both stdout/stderr `|& cat`

For example:
```bash
CLICOLOR_FORCE=1 cargo run -q -F=clap --example termcolor -- --color=never | cat
```
*/
use clap::Parser;
use should_color::{clap_color, resolve, ColorChoice};
use std::io::Write;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

#[derive(Debug, Parser)]
#[clap(version, color = clap_color())]
struct Cli {
    /// Coloring
    #[clap(long, value_name = "WHEN", arg_enum, global = true)]
    color: Option<ColorChoice>,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    // resolve from cli preference, environment variables, default value
    let color_choice = resolve(cli.color).unwrap_or(ColorChoice::Auto);

    let color_stdout = color_choice.for_stream(atty::Stream::Stdout);
    let color_stderr = color_choice.for_stream(atty::Stream::Stderr);

    println!("         cli = {cli:?}");
    println!("color_choice = {color_choice:?}");

    let mut stdout = StandardStream::stdout(if color_stdout {
        termcolor::ColorChoice::Always
    } else {
        termcolor::ColorChoice::Never
    });

    let mut stderr = StandardStream::stderr(if color_stderr {
        termcolor::ColorChoice::Always
    } else {
        termcolor::ColorChoice::Never
    });

    stdout.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::Green))
            .set_intense(true)
            .set_italic(true),
    )?;
    write!(stdout, "Colorize stdout")?;
    stdout.reset()?;
    write!(stdout, ": ")?;
    stdout.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::Yellow))
            .set_intense(true),
    )?;
    writeln!(stdout, "{}", color_stdout)?;
    stdout.reset()?;

    stderr.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::Red))
            .set_intense(true)
            .set_underline(true),
    )?;
    write!(stderr, "Colorize stderr")?;
    stderr.reset()?;
    write!(stderr, ": ")?;
    stderr.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::Yellow))
            .set_intense(true),
    )?;
    writeln!(stderr, "{}", color_stderr)?;
    stderr.reset()?;

    Ok(())
}
