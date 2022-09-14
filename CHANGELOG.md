# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com),
and this project adheres to [Semantic Versioning](https://semver.org).

<!-- next-header -->
## [Unreleased]
### Added
- CHANGELOG.md

### Fixed
- Links to the documentation in README.md.

## [0.5.0] - 2022-09-07
### Added
- Feature `stream` which allows to determine the correct color choice
  for a specific stream (`ColorChoice::for_stream`).
- Example binary which can be used to experiment with the crate.
- Explanation of the resolution process in the README.md.

## [0.4.0] - 2022-09-06
### Added
- Feature `clap` which allows to:
    + parse `ColorChoice` as a CLI argument;
    + convert it from/to `clap::ColorChoice`;
    + determine the correct color choice for `clap` with `clap_color`.

### Fixed
- Improved badges in README.md.

## [0.3.0] - 2022-09-05
### Added
- Documentation explaining the compatibility with <https://no-color.org>
  and <https://bixense.com/clicolors/>.
- Documentation of `resolve`, with examples.

### Changed
- The return type of the functions `clicolor`, `clicolor_force`, `no_color`
  is now `Option<ColorChoice>`.

### Removed
- `resolve_all`.

## [0.2.1] - 2022-09-05
### Added
- Documentation of the meaning and priority of the various settings.

### Fixed
- The documentation now shows the required features for the items.

## [0.2.0] - 2022-09-05
### Added
- Features `clicolor`, `clicolor_force`, `no_color` that control the availability
  of the respective functions.

### Deprecated
- `resolve_all`.

## [0.1.0] - 2022-09-05
### Added
- Initial implementation:
    + `enum`: `ColorChoice`;
    + `fn`: `clicolor`, `clicolor_force`, `no_color`, `resolve`, `resolve_all`.

<!-- next-url -->
[Unreleased]: https://github.com/FedericoStra/should-color/compare/v0.1.0...HEAD
[0.5.0]: https://github.com/FedericoStra/should-color/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/FedericoStra/should-color/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/FedericoStra/should-color/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/FedericoStra/should-color/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/FedericoStra/should-color/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/FedericoStra/should-color/releases/tag/v0.1.0
