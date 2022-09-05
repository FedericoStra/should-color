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

## `CLICOLOR_FORCE`

Requires the <span class="stab portability" title="Available on crate feature `clicolor_force` only"><code>clicolor_force</code></span> feature.

The meaning of the environment variable is the following:

- if not set or `CLICOLOR_FORCE == ""` or `CLICOLOR_FORCE == "0"`: ignore;
- if set and `CLICOLOR_FORCE != ""` and `CLICOLOR_FORCE != "0"`: [`ColorChoice::Always`].

## `CLICOLOR`

Requires the <span class="stab portability" title="Available on crate feature `clicolor` only"><code>clicolor</code></span> feature.

The meaning of the environment variable is the following:

- if not set or `CLICOLOR == ""`: ignore;
- if set and `CLICOLOR == "0"`: [`ColorChoice::Never`];
- if set and `CLICOLOR != ""` and `CLICOLOR != "0"`: [`ColorChoice::Auto`].

## `NO_COLOR`

Requires the <span class="stab portability" title="Available on crate feature `no_color` only"><code>no_color</code></span> feature.

The meaning of the environment variable is the following:

- if not set or `NO_COLOR == ""`: ignore;
- if set and `NO_COLOR != ""`: [`ColorChoice::Never`].

## Compatibility

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

/// Possible color choices for the output.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorChoice {
    /// The output will not be colorized.
    Never,
    /// The output will be colorized if the output device is a tty,
    /// i.e. when the output goes directly to a text screen or terminal emulator window.
    Auto,
    /// The output will be colorized.
    Always,
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

    // std::env::var_os(NO_COLOR).and_then(|s| {
    //     if s.is_empty() {
    //         None
    //     } else {
    //         Some(ColorChoice::Never)
    //     }
    // })
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

/// Resolve the output color choice from the environment variables and explicit CLI choice.
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

// https://medium.com/@ericdreichert/test-setup-and-teardown-in-rust-without-a-framework-ba32d97aa5ab

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
