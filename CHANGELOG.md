# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

The crate was regenerated several times with various commits of msp430_svd
and svdrust, up to and including svd2rust v0.20.0. Non-breaking changes
include:

- Add [commit hash information](https://github.com/rust-embedded/svd2rust/pull/439)
  to emitted crate from svd2rust.
- Add [commit hash information](https://github.com/pftbest/msp430_svd/pull/14)
  to output svd from msp430_svd generation.
- [Add](https://github.com/pftbest/msp430_svd/tree/master/overrides) a large
  number of missing register fields from DSLite files to generated SVD files
  via patching using svdtools.

Breaking changes include:

- Many breaking changes to svd2rust API between svd2rust v0.17.0 plus msp-fix
  branch changes and v0.18.0 (see commits f635f19 and 272a6d2).
  - Changes from regeneration with v0.20.0 were less breaking (see commit
    f3e0e99); some generated `bits()` functions became safe after svd2rust
    improvements.
- Reorganize timer peripheral register names (see commit 358c0d9).
- Using patching, merge multiple individual bit fields from peripherals into
  larger single fields as appropriate, such as USCI_B0_I2C_MODE
  (see commit 92f9892).
- Update msp430 and msp430-rt to version 0.3.0.

With the additional register fields added via patching, _it should now be
possible to write completely safe applications using this PAC._

## [v0.2.0] - 2020-01-03

Regenerate crate with svd2rust v0.17.0 plus msp-fix branch changes.

Update msp430 and msp430-rt to version 0.2.0

## [v0.1.3] - 2018-05-06

Fix for macro_reexport feature.

## [v0.1.2] - 2018-04-08

Updated msp430-rt to version 0.1.2

## [v0.1.1] - 2018-02-02

Updated msp430-rt to version 0.1.1

## v0.1.0 - 2017-07-22

Initial release

[Unreleased]: https://github.com/pftbest/msp430g2553/compare/v0.2.0...HEAD
[v0.2.0]: https://github.com/pftbest/msp430g2553/compare/v0.1.3...v0.2.0
[v0.1.3]: https://github.com/pftbest/msp430g2553/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/pftbest/msp430g2553/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/pftbest/msp430g2553/compare/v0.1.0...v0.1.1
