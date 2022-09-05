// ATTENTION: tests should be run with a single thread:
//
// ```bash
// cargo test -- --test-threads 1
// ```

use itertools::*;

use should_color::*;

fn setup_env(no_color: Option<&str>, clicolor: Option<&str>, clicolor_force: Option<&str>) {
    no_color.map_or_else(
        || std::env::remove_var(NO_COLOR),
        |s| std::env::set_var(NO_COLOR, s),
    );
    clicolor.map_or_else(
        || std::env::remove_var(CLICOLOR),
        |s| std::env::set_var(CLICOLOR, s),
    );
    clicolor_force.map_or_else(
        || std::env::remove_var(CLICOLOR_FORCE),
        |s| std::env::set_var(CLICOLOR_FORCE, s),
    );
}

#[test]
fn test_clicolor_force() {
    let any_env = [
        None,
        Some(""),
        Some("0"),
        Some("1"),
        Some("false"),
        Some("true"),
    ];
    let any_default = [ColorChoice::Never, ColorChoice::Auto, ColorChoice::Always];
    let any_cli = [
        None,
        Some(ColorChoice::Never),
        Some(ColorChoice::Auto),
        Some(ColorChoice::Always),
    ];
    let any_set_clicolor_force = [Some("1"), Some("="), Some("false"), Some("true")];

    for (default, no_color, clicolor, cli, clicolor_force) in iproduct!(
        any_default,
        any_env,
        any_env,
        any_cli,
        any_set_clicolor_force
    ) {
        setup_env(no_color, clicolor, clicolor_force);
        assert_eq!(resolve(default, cli), ColorChoice::Always);
    }
}

#[test]
fn test_cli() {
    let any_env = [
        None,
        Some(""),
        Some("0"),
        Some("1"),
        Some("false"),
        Some("true"),
    ];
    let any_default = [ColorChoice::Never, ColorChoice::Auto, ColorChoice::Always];
    let any_set_cli = [ColorChoice::Never, ColorChoice::Auto, ColorChoice::Always];
    let any_unset_clicolor_force = [None, Some(""), Some("0")];

    for (default, no_color, clicolor, cli, clicolor_force) in iproduct!(
        any_default,
        any_env,
        any_env,
        any_set_cli,
        any_unset_clicolor_force
    ) {
        setup_env(no_color, clicolor, clicolor_force);
        assert_eq!(resolve(default, Some(cli)), cli);
    }
}
