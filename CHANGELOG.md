# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.0.4] - 2019-08-31

### Changed

- Delete `SYSCON.PRESECTRLX?`, as `svd2rust` does not
  handle them properly

## [v0.0.3] - 2019-08-31

### Changed

- Rename `UTICK0` to `UTICK` like UM

## [v0.0.2] - 2019-08-23

### Added

- CI, via CircleCI
- Better README

### Changed

- Delete `SYSCON.AHBCLKCTRLX?`, as `svd2rust` does not
  handle them properly

## [v0.0.1] - 2019-08-18

### Added

- Initial release
- Delete `INPUTMUX` register to appease `svd2rust`.

