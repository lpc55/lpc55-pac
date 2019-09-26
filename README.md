# LPC55s6x PAC (peripheral access crate)

Low-level register mappings for the NXP LPC55S6x family of ARM Cortex-M33 microcontrollers, written in Rust.
The code is generated automatically from a patched version of the vendor-supplied SVD file, using `svd2rust`.

The purpose of this crate is to give embedded programs or libraries written in Rust access
to the complete functionality of LPC55S6x microcontrollers.

[![crates.io][crates-image]][crates-link]
[![Documentation][docs-image]][docs-link]
![LICENSE][license-image]
[![Build Status][build-image]][build-link]

## Status

Very much work-in-progress!

See also the higher-level companion library [LPC55S6x HAL][lpc55s6x-hal].

## Documentation

The API documentation is located at <https://docs.rs/lpc55s6x-pac>.

To get an understanding of `svd2rust`-generated APIs, checkout its documentation at <https://docs.rs/svd2rust>.

In addition, `make fetch-docs` downloads various vendor-supplied documentation:

- [LPC55S6x Data Sheet][datasheet]
- [LPC55S6x User Manual][usermanual]
- [LPC55S6x Errata][errata]
- [Cortex-M33 Generic User Guide][genericuserguide]

## License

[Apache-2.0][apache2-link] or [MIT][mit-link].

The SVD files are from <https://mcuxpresso.nxp.com> and licensed under the [BSD-3-Clause][bsd3-link].

[//]: # (links)

[crates-image]: https://img.shields.io/crates/v/lpc55s6x-pac.svg
[crates-link]: https://crates.io/crates/lpc55s6x-pac
[build-image]: https://img.shields.io/circleci/build/github/nickray/lpc55s6x-pac/master.svg
[build-link]: https://circleci.com/gh/nickray/lpc55s6x-pac/tree/master
[docs-image]: https://docs.rs/lpc55s6x-pac/badge.svg
[docs-link]: https://docs.rs/lpc55s6x-pac
[lpc55s6x-hal]: https://libs.rs/lpc55s6x-hal
[svd-docs-link]: https://docs.rs/svd2rust
[license-image]: https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg
[apache2-link]: https://spdx.org/licenses/Apache-2.0.html
[bsd3-link]: https://spdx.org/licenses/BSD-3-Clause.html
[mit-link]: https://spdx.org/licenses/MIT.html
[mcuxpresso]: https://mcuxpresso.nxp.com
[datasheet]: https://www.nxp.com/docs/en/data-sheet/LPC55S6x.pdf
[usermanual]: https://www.nxp.com/webapp/Download?colCode=UM11126
[errata]: https://www.nxp.com/docs/en/errata/ES_LPC55S6x.pdf
[genericuserguide]: https://static.docs.arm.com/100235/0004/arm_cortex_m33_dgug_100235_0004_00_en.pdf
