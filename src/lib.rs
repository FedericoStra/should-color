/*!
Determine whether output should use colors or not.

The resulting color choice is determined by taking into account,
in order of priority from higher to lower, the following settings:

- `CLICOLOR_FORCE` environment variable (requires `clicolor_force` feature),
- explicit user preference (for instance command line arguments),
- `CLICOLOR` environment variable (requires `clicolor` feature),
- `NO_COLOR` environment variable (requires `no_color` feature),
- application default choice.

The specification of `CLICOLOR`, `CLICOLOR_FORCE`, and `NO_COLOR` is inspired by:

- <https://bixense.com/clicolors/>,
- <https://no-color.org>.

*/

#![deny(
    dead_code,
    missing_docs,
    missing_debug_implementations,
    unused_imports,
    unused_qualifications
)]
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
`NO_COLOR` environment variable setting.

From <https://no-color.org>:

> Command-line software which adds ANSI color to its output by default should
  check for a `NO_COLOR` environment variable that, when present and not an
  empty string (regardless of its value), prevents the addition of ANSI
  color.
*/
#[cfg(feature = "no_color")]
// #[cfg_attr(docsrs, doc(cfg(feature = "no_color")))]
pub fn no_color() -> Option<bool> {
    std::env::var_os(NO_COLOR).map(|s| !s.is_empty())
}

/**
`CLICOLOR` environment variable setting.

From <https://bixense.com/clicolors/>:

> The idea is to have the environment variables `CLICOLOR` and `CLICOLOR_FORCE`
> (which are currently already used for this exact reason on some UNIX systems).
> When set, the following rules should apply:
> - `CLICOLOR != 0`
>   - ANSI colors are supported and should be used when the program isn’t piped.
> - `CLICOLOR == 0`
>   - Don’t output ANSI color escape codes.
*/
#[cfg(feature = "clicolor")]
pub fn clicolor() -> Option<bool> {
    std::env::var_os(CLICOLOR)
        .and_then(|s| if s.is_empty() { None } else { Some(s) })
        .map(|s| s != "0")
}

/**
`CLICOLOR_FORCE` environment variable setting.

From <https://bixense.com/clicolors/>:

> The idea is to have the environment variables `CLICOLOR` and `CLICOLOR_FORCE`
> (which are currently already used for this exact reason on some UNIX systems).
> When set, the following rules should apply:
> - `CLICOLOR_FORCE != 0`
>   - ANSI colors should be enabled no matter what.
*/
#[cfg(feature = "clicolor_force")]
pub fn clicolor_force() -> Option<bool> {
    // std::env::var_os(CLICOLOR_FORCE).map(|s| !s.is_empty() && s != "0")
    std::env::var_os(CLICOLOR_FORCE)
        .and_then(|s| if s.is_empty() { None } else { Some(s) })
        .map(|s| s != "0")
}

/// Resolve the output color choice from default value, environment variables
/// and explicit CLI choice.
pub fn resolve(default: ColorChoice, cli: Option<ColorChoice>) -> ColorChoice {
    #[cfg(feature = "clicolor_force")]
    if let Some(true) = clicolor_force() {
        return ColorChoice::Always;
    }

    if let Some(c) = cli {
        return c;
    }

    #[cfg(feature = "clicolor")]
    match clicolor() {
        Some(true) => return ColorChoice::Auto,
        Some(false) => return ColorChoice::Never,
        _ => (),
    }

    #[cfg(feature = "no_color")]
    if let Some(true) = no_color() {
        return ColorChoice::Never;
    }

    default
}

/// Resolve the output color choice from default value, environment variables
/// and explicit CLI choice.
pub fn resolve_all(
    default: ColorChoice,
    no_color: Option<bool>,
    clicolor: Option<bool>,
    clicolor_force: Option<bool>,
    cli: Option<ColorChoice>,
) -> ColorChoice {
    match clicolor_force {
        None | Some(false) => None,
        Some(true) => Some(ColorChoice::Always),
    }
    .or(cli)
    .or_else(|| match clicolor {
        None => None,
        Some(false) => Some(ColorChoice::Never),
        Some(true) => Some(ColorChoice::Auto),
    })
    .or_else(|| match no_color {
        None | Some(false) => None,
        Some(true) => Some(ColorChoice::Never),
    })
    .unwrap_or(default)
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
        assert_eq!(no_color(), Some(false));

        for s in ["0", "1", "false", "true", "="] {
            std::env::set_var(NO_COLOR, s);
            assert_eq!(no_color(), Some(true));
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
        assert_eq!(clicolor(), Some(false));

        for s in ["1", "false", "true", "="] {
            std::env::set_var(CLICOLOR, s);
            assert_eq!(clicolor(), Some(true));
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
        assert_eq!(clicolor_force(), Some(false));

        for s in ["1", "false", "true", "="] {
            std::env::set_var(CLICOLOR_FORCE, s);
            assert_eq!(clicolor_force(), Some(true));
        }
    }
}
