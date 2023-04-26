# should-color

Determine whether output should use colors or not.

[![crates.io](https://img.shields.io/crates/v/should-color?logo=rust)](https://crates.io/crates/should-color)
[![docs.rs](https://img.shields.io/docsrs/should-color?logo=docsdotrs)](https://docs.rs/should-color)
[![GitHub](https://img.shields.io/static/v1?label=github&message=FedericoStra/should-color&color=brightgreen&logo=github)](https://github.com/FedericoStra/should-color)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/FedericoStra/should-color/rust.yml?logo=githubactions&logoColor=white)](https://github.com/FedericoStra/should-color/actions/workflows/rust.yml)
[![Dependencies status](https://deps.rs/repo/github/FedericoStra/should-color/status.svg)](https://deps.rs/repo/github/FedericoStra/should-color)
[![MIT license](https://img.shields.io/crates/l/should-color)](https://choosealicense.com/licenses/mit/)

The [`should-color`](https://crates.io/crates/should-color) crate helps determine the color choice for an application output based on the command line arguments and environment variables.

The resulting color choice is determined by taking into account, in order of priority from higher to lower, the following settings:

- [`CLICOLOR_FORCE`] environment variable (requires `clicolor_force` feature),
- explicit user preference (for instance command line arguments),
- [`CLICOLOR`] environment variable (requires `clicolor` feature),
- [`NO_COLOR`] environment variable (requires `no_color` feature),
- application default choice.

Please refer to the [documentation](https://docs.rs/should-color/latest/should_color/index.html) for more details on the resolution process.

[`CLICOLOR_FORCE`]: https://docs.rs/should-color/latest/should_color/index.html#clicolor_force
[`CLICOLOR`]: https://docs.rs/should-color/latest/should_color/index.html#clicolor
[`NO_COLOR`]: https://docs.rs/should-color/latest/should_color/index.html#no_color

## Alternatives and comparison

- [`concolor`](https://crates.io/crates/concolor)
- [`clicolors-control`](https://crates.io/crates/clicolors-control)
