# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.4.1] - 2021-12-31

- fix addressBlock usage
- update svd2rust to 0.20
- `SVDTOOLS` env value for specifying patching tool

## [v0.4.0] - 2021-05-02

- update to svd2rust
- use CMSIS PACK (directly downloadable) instead of SDK as SVD source
- experimental support for the ROM Patch Unit

## [v0.3.0] - 2021-02-26

- adds some patching to remove bare-metal, and implement
  cortex_m::interrupt::Nr instead

## [v0.2.0] - 2020-12-30

- republish previous v0.1.1 as v0.2.0, as the cortex-m bump
  breaks e.g. RTIC which is still on cortex-m 0.6

## [v0.1.1] - 2020-12-29

- SAU fixes @samueltardieu
- cortex-m bump to v0.7 @labott
- move to lpc55 organization
- remove Circle CI

## [v0.1.0] - 2020-04-28

- Derive USB0 from USB1 @conorpp
- Rename from `lpc55s6x-pac` to `lpc55-pac`
- SDK 2.7.1 has no changes, skipping

## [v0.0.8] - 2020-02-20

- INPUTMUX is back

## [v0.0.7] - 2020-02-16

- Actually build the new patched SVD...

## [v0.0.6] - 2020-02-20

- Update SVD to version from SDK 2.7.0
- Use svd2rust 0.17.0 (changes most files somewhat)
- Use svdtools 0.1.0
- Bump dependencies
- ADC and PUF fixes

## [v0.0.5] - 2019-09-19

### Changed

- Update SVD to version from SDK 2.6.3, this is breaking
  change that fixes a bunch of things
- Revert `UTICK0` rename from v0.0.3
- Update dependencies

### Added
- New SVD files from SDK 2.6.3

### Removed
- Unnecessary documentation generation
- Old SDK 2.5.1 files
- Unnecessary SDK 2.6.2 files

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

