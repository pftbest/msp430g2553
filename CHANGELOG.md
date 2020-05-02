# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

Regenerate crate with svd2rust v0.17.0 plus [commit hash information](https://github.com/rust-embedded/svd2rust/pull/439),
and msp430_svd v0.2.0.

No breaking changes were introduced by bumping the svd2rust commit.

msp430_svd v0.2.0 [adds](https://github.com/pftbest/msp430_svd/tree/master/overrides)
a large number of missing register fields to generated SVD files, and merges
multiple individual bit fields into larger single fields as appropriate.
_Merging bit fields is a breaking change._ The msp430_svd commit hash is now
also stored in the output SVD file.

With the additional register fields, _it should now be possible to write
completely safe applications using this PAC._

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
