/*!
Determine whether output should use colors or not.

The resulting color choice is determined by taking into account,
in order of priority from higher to lower, the following settings:

- [`CLICOLOR_FORCE`](#clicolor_force) environment variable (requires `clicolor_force` feature),
- explicit user preference (for instance command line arguments),
- [`CLICOLOR`](#clicolor) environment variable (requires `clicolor` feature),
- [`NO_COLOR`](#no_color) environment variable (requires `no_color` feature),
- application default choice.

The specification of `CLICOLOR`, `CLICOLOR_FORCE`, and `NO_COLOR` is inspired by:

- <https://bixense.com/clicolors/>,
- <https://no-color.org>,

with the exception that variables which are set to the empty string `""`
are treated as if they were unset.
The reason is that it is common to override environment variables by executing programs as
`VAR= cmd args...` and expect that `VAR` is unset.

# `CLICOLOR_FORCE`

Requires the <span class="stab portability" title="Available on crate feature `clicolor_force` only"><code>clicolor_force</code></span> feature.

The meaning of the environment variable is the following:

- if not set or `CLICOLOR_FORCE == ""` or `CLICOLOR_FORCE == "0"`: ignore;
- if set and `CLICOLOR_FORCE != ""` and `CLICOLOR_FORCE != "0"`: [`ColorChoice::Always`].

# `CLICOLOR`

Requires the <span class="stab portability" title="Available on crate feature `clicolor` only"><code>clicolor</code></span> feature.

The meaning of the environment variable is the following:

- if not set or `CLICOLOR == ""`: ignore;
- if set and `CLICOLOR == "0"`: [`ColorChoice::Never`];
- if set and `CLICOLOR != ""` and `CLICOLOR != "0"`: [`ColorChoice::Auto`].

# `NO_COLOR`

Requires the <span class="stab portability" title="Available on crate feature `no_color` only"><code>no_color</code></span> feature.

The meaning of the environment variable is the following:

- if not set or `NO_COLOR == ""`: ignore;
- if set and `NO_COLOR != ""`: [`ColorChoice::Never`].

# Compatibility

The goal of this crate is to implement the standards proposed in
<https://no-color.org> and <https://bixense.com/clicolors/>.

Please note that the proposals in the latter are slightly ambiguous and undesirable
(see [this issue](https://github.com/jhasse/clicolors/issues/8)),
hence they are merely taken as an inspiration and not followed too strictly.

Relevant quote from <https://no-color.org>:

> Command-line software which adds ANSI color to its output by default should
  check for a `NO_COLOR` environment variable that, when present and not an
  empty string (regardless of its value), prevents the addition of ANSI color.

Relevant quote from <https://bixense.com/clicolors/>:

> The idea is to have the environment variables `CLICOLOR` and `CLICOLOR_FORCE`
> (which are currently already used for this exact reason on some UNIX systems).
> When set, the following rules should apply:
> - `CLICOLOR != 0`: ANSI colors are supported and should be used when the program isn’t piped,
> - `CLICOLOR == 0`: don’t output ANSI color escape codes,
> - `CLICOLOR_FORCE != 0`: ANSI colors should be enabled no matter what.
*/

#![deny(missing_docs, missing_debug_implementations, warnings)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

/// Name of the `NO_COLOR` environment variable.
#[cfg(feature = "no_color")]
pub const NO_COLOR: &str = "NO_COLOR";
/// Name of the `CLICOLOR` environment variable.
#[cfg(feature = "clicolor")]
pub const CLICOLOR: &str = "CLICOLOR";
/// Name of the `CLICOLOR_FORCE` environment variable.
#[cfg(feature = "clicolor_force")]
pub const CLICOLOR_FORCE: &str = "CLICOLOR_FORCE";

/**
Possible color choices for the output.
*/
#[cfg_attr(
    feature = "clap",
    doc = r#"

# Clap interoperability

If the <span class="stab portability" title="Available on crate feature `clap` only"><code>clap</code></span> feature is enabled then
[`ColorChoice`] can be converted to and from [`clap::ColorChoice`](https://docs.rs/clap/latest/clap/enum.ColorChoice.html).
Moreover it implements [`clap::ValueEnum`](https://docs.rs/clap/latest/clap/trait.ValueEnum.html), hence can be used as

```rust
#[derive(clap::Parser)]
struct Cli {
    /// Coloring of the output
    #[clap(long, value_name = "WHEN", arg_enum, global = true)]
    color: Option<should_color::ColorChoice>,

    // Other arguments...
}
```
"#
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum ColorChoice {
    /// The output will not be colorized.
    Never,
    /// The output will be colorized if the output device is a tty,
    /// i.e. when the output goes directly to a text screen or terminal emulator window.
    Auto,
    /// The output will be colorized.
    Always,
}

#[cfg(feature = "stream")]
impl ColorChoice {
    /**
    Determine the color setting for a specific stream.

    If the choice is [`ColorChoice::Never`] or [`ColorChoice::Always`],
    the result will be `false` and `true` respectively.

    If the choice is [`ColorChoice::Auto`], then the answer depends on whether
    the `stream` is a tty or not.
    */
    pub fn for_stream(&self, stream: atty::Stream) -> bool {
        match self {
            ColorChoice::Never => false,
            ColorChoice::Always => true,
            ColorChoice::Auto => atty::is(stream),
        }
    }
}

// #[cfg(feature = "clap")]
// /// Alias for [`clap::ColorChoice`](https://docs.rs/clap/latest/clap/enum.ColorChoice.html).
// pub type ClapColorChoice = clap::ColorChoice;

#[cfg(feature = "clap")]
impl From<ColorChoice> for clap::ColorChoice {
    fn from(color_choice: ColorChoice) -> clap::ColorChoice {
        match color_choice {
            ColorChoice::Never => clap::ColorChoice::Never,
            ColorChoice::Auto => clap::ColorChoice::Auto,
            ColorChoice::Always => clap::ColorChoice::Always,
        }
    }
}

#[cfg(feature = "clap")]
impl From<clap::ColorChoice> for ColorChoice {
    fn from(color_choice: clap::ColorChoice) -> ColorChoice {
        match color_choice {
            clap::ColorChoice::Never => ColorChoice::Never,
            clap::ColorChoice::Auto => ColorChoice::Auto,
            clap::ColorChoice::Always => ColorChoice::Always,
        }
    }
}

/**
Compute a [`clap::ColorChoice`](https://docs.rs/clap/latest/clap/enum.ColorChoice.html)
suitable for the [`clap::App::color`](https://docs.rs/clap/latest/clap/builder/struct.App.html#method.color) setting.

This is a convenience function equivalent to [`resolve`] without an explicit CLI preference
and a default value of [`clap::ColorChoice::Auto`](https://docs.rs/clap/latest/clap/enum.ColorChoice.html#variant.Auto).

```rust
#[derive(clap::Parser)]
#[clap(color = should_color::clap_color())]
struct Cli {
    // Arguments...
}
```
*/
#[cfg(feature = "clap")]
pub fn clap_color() -> clap::ColorChoice {
    resolve(None).unwrap_or(ColorChoice::Auto).into()
}

/**
Get the setting of the `NO_COLOR` environment variable.

The environment variable is treated as follows:

- if not set or `NO_COLOR == ""`: return `None`;
- if set and `NO_COLOR != ""`: return `Some(`[`ColorChoice::Never`]`)`.
*/
#[cfg(feature = "no_color")]
pub fn no_color() -> Option<ColorChoice> {
    match std::env::var_os(NO_COLOR) {
        Some(s) if !s.is_empty() => Some(ColorChoice::Never),
        _ => None,
    }
}

/**
Get the setting of the `CLICOLOR` environment variable.

The environment variable is treated as follows:

- if not set or `CLICOLOR == ""`: return `None`;
- if set and `CLICOLOR == "0"`: return `Some(`[`ColorChoice::Never`]`)`;
- if set and `CLICOLOR != ""` and `CLICOLOR != "0"`: return `Some(`[`ColorChoice::Auto`]`)`.
*/
#[cfg(feature = "clicolor")]
pub fn clicolor() -> Option<ColorChoice> {
    match std::env::var_os(CLICOLOR) {
        Some(s) if s == "0" => Some(ColorChoice::Never),
        Some(s) if !s.is_empty() => Some(ColorChoice::Auto),
        _ => None,
    }
}

/**
Get the setting of the `CLICOLOR_FORCE` environment variable.

The environment variable is treated as follows:

- if not set or `CLICOLOR_FORCE == ""` or `CLICOLOR_FORCE == "0"`: return `None`;
- if set and `CLICOLOR_FORCE != ""` and `CLICOLOR_FORCE != "0"`: return `Some`[`ColorChoice::Always`]`)`.
*/
#[cfg(feature = "clicolor_force")]
pub fn clicolor_force() -> Option<ColorChoice> {
    match std::env::var_os(CLICOLOR_FORCE) {
        Some(s) if !s.is_empty() && s != "0" => Some(ColorChoice::Always),
        _ => None,
    }
}

/**
Resolve the output color choice from the environment variables and an explicit CLI preference.

Please refer to the [crate level documentation](crate) for a detailed description of the
resolution process.

Commonly this function will be called as `resolve(cli).unwrap_or(default)` to take into account
a preference expressed through the CLI arguments and the default behavior of the application.

# Examples

The following examples assume that all the features
<span class="stab portability" title="Available on crate feature `clicolor` only"><code>clicolor</code></span>,
<span class="stab portability" title="Available on crate feature `clicolor_force` only"><code>clicolor_force</code></span>, and
<span class="stab portability" title="Available on crate feature `no_color` only"><code>no_color</code></span>
are enabled.

- ```
  # use should_color::{resolve, ColorChoice};
  std::env::set_var("CLICOLOR_FORCE", "false"); // this wins
  # #[cfg(all(feature = "clicolor_force"))]
  assert_eq!(resolve(Some(ColorChoice::Never)), Some(ColorChoice::Always));
  ```

- ```
  # use should_color::{resolve, ColorChoice};
  std::env::remove_var("CLICOLOR_FORCE");
  std::env::set_var("CLICOLOR", "1"); // this wins
  # #[cfg(all(feature = "clicolor"))]
  assert_eq!(resolve(None), Some(ColorChoice::Auto));
  # #[cfg(not(feature = "clicolor"))]
  # assert_eq!(resolve(None), None);
  ```

- ```
  # use should_color::{resolve, ColorChoice};
  std::env::remove_var("CLICOLOR_FORCE");
  std::env::set_var("CLICOLOR", "0"); // this wins
  # #[cfg(all(feature = "clicolor"))]
  assert_eq!(resolve(None), Some(ColorChoice::Never));
  # #[cfg(not(feature = "clicolor"))]
  # assert_eq!(resolve(None), None);
  ```

- ```
  # use should_color::{resolve, ColorChoice};
  std::env::remove_var("CLICOLOR_FORCE");
  std::env::remove_var("CLICOLOR");
  std::env::set_var("NO_COLOR", "1"); // this wins
  # #[cfg(feature = "no_color")]
  assert_eq!(resolve(None), Some(ColorChoice::Never));
  # #[cfg(not(feature = "no_color"))]
  # assert_eq!(resolve(None), None);
  ```

- ```
  # use should_color::{resolve, ColorChoice};
  std::env::remove_var("CLICOLOR_FORCE");
  std::env::remove_var("CLICOLOR");
  std::env::remove_var("NO_COLOR");
  assert_eq!(resolve(None), None);
  ```
*/
pub fn resolve(cli: Option<ColorChoice>) -> Option<ColorChoice> {
    #[cfg(feature = "clicolor_force")]
    let choice = clicolor_force();

    #[cfg(feature = "clicolor_force")]
    let choice = choice.or(cli);
    #[cfg(not(feature = "clicolor_force"))]
    let choice = cli;

    #[cfg(feature = "clicolor")]
    let choice = choice.or_else(clicolor);

    #[cfg(feature = "no_color")]
    let choice = choice.or_else(no_color);

    choice
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "no_color")]
    fn test_no_color() {
        use super::*;

        std::env::remove_var(NO_COLOR);
        assert_eq!(no_color(), None);

        std::env::set_var(NO_COLOR, "");
        assert_eq!(no_color(), None);

        for s in ["0", "1", "false", "true", "="] {
            std::env::set_var(NO_COLOR, s);
            assert_eq!(no_color(), Some(ColorChoice::Never));
        }
    }

    #[test]
    #[cfg(feature = "clicolor")]
    fn test_clicolor() {
        use super::*;

        std::env::remove_var(CLICOLOR);
        assert_eq!(clicolor(), None);

        std::env::set_var(CLICOLOR, "");
        assert_eq!(clicolor(), None);

        std::env::set_var(CLICOLOR, "0");
        assert_eq!(clicolor(), Some(ColorChoice::Never));

        for s in ["1", "false", "true", "="] {
            std::env::set_var(CLICOLOR, s);
            assert_eq!(clicolor(), Some(ColorChoice::Auto));
        }
    }

    #[test]
    #[cfg(feature = "clicolor_force")]
    fn test_clicolor_force() {
        use super::*;

        std::env::remove_var(CLICOLOR_FORCE);
        assert_eq!(clicolor_force(), None);

        std::env::set_var(CLICOLOR_FORCE, "");
        assert_eq!(clicolor_force(), None);

        std::env::set_var(CLICOLOR_FORCE, "0");
        assert_eq!(clicolor_force(), None);

        for s in ["1", "false", "true", "="] {
            std::env::set_var(CLICOLOR_FORCE, s);
            assert_eq!(clicolor_force(), Some(ColorChoice::Always));
        }
    }
}
