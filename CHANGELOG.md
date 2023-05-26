# Changelog

## [0.4.10](https://github.com/fluencelabs/decider/compare/connector-v0.4.9...connector-v0.4.10) (2023-05-26)


### Features

* trigger connector update ([#41](https://github.com/fluencelabs/decider/issues/41)) ([d17d0aa](https://github.com/fluencelabs/decider/commit/d17d0aa1d6c6fd7e0787f4d3b074a66b6fe2f00b))

## [0.4.9](https://github.com/fluencelabs/decider/compare/connector-v0.4.8...connector-v0.4.9) (2023-05-08)


### Features

* **builtin-package:** use new blueprint ([#36](https://github.com/fluencelabs/decider/issues/36)) ([224978d](https://github.com/fluencelabs/decider/commit/224978d3e22137447d751ce416f465dd41172182))

## [0.4.8](https://github.com/fluencelabs/decider/compare/connector-v0.4.7...connector-v0.4.8) (2023-04-27)


### Features

* **builtin:** set net and contract address via env variables [NET-460] ([#33](https://github.com/fluencelabs/decider/issues/33)) ([9164529](https://github.com/fluencelabs/decider/commit/9164529d6ff9d5b7d30db11110cf8e4295e33a8c))


### Bug Fixes

* **deps:** update rust crate marine-rs-sdk-test to 0.9.0 ([#24](https://github.com/fluencelabs/decider/issues/24)) ([8a06acf](https://github.com/fluencelabs/decider/commit/8a06acf6f90ea966cfe9bed00fd7d63146cda55e))

## [0.4.7](https://github.com/fluencelabs/decider/compare/connector-v0.4.6...connector-v0.4.7) (2023-04-20)


### Bug Fixes

* **deps:** update config ([#31](https://github.com/fluencelabs/decider/issues/31)) ([8de8f7b](https://github.com/fluencelabs/decider/commit/8de8f7b4232a03f1d86046ed1445219540a5b731))

## [0.4.6](https://github.com/fluencelabs/decider/compare/connector-v0.4.5...connector-v0.4.6) (2023-04-12)


### Features

* update network to aurora testnet ([#26](https://github.com/fluencelabs/decider/issues/26)) ([efefa71](https://github.com/fluencelabs/decider/commit/efefa710c39c7cda111b4344b98782d279dede73))

## [0.4.5](https://github.com/fluencelabs/decider/compare/connector-v0.4.4...connector-v0.4.5) (2023-03-31)


### Features

* **decider:** add env variable to include/exclude decider [NET-426] ([#22](https://github.com/fluencelabs/decider/issues/22)) ([9cf1309](https://github.com/fluencelabs/decider/commit/9cf13091978a53c387e44b018e1cdaa983020175))

## [0.4.4](https://github.com/fluencelabs/decider/compare/connector-v0.4.3...connector-v0.4.4) (2023-03-23)


### Features

* **decider, connector:** poll new-app-cid events [fixes NET-384] ([#8](https://github.com/fluencelabs/decider/issues/8)) ([bc0d221](https://github.com/fluencelabs/decider/commit/bc0d22117750af0fe9eb1d9c23d247de48d6c85f))
* **decider:** update installation spell to 0.5.6 ([#21](https://github.com/fluencelabs/decider/issues/21)) ([4fbf58f](https://github.com/fluencelabs/decider/commit/4fbf58ff59e2915ca0fb6b47fa8b60578190a290))
* remove decider.json ([a161b97](https://github.com/fluencelabs/decider/commit/a161b970e5907c948c3e096336de0c07713fa33c))

## [0.4.3](https://github.com/fluencelabs/decider/compare/connector-v0.4.2...connector-v0.4.3) (2023-02-28)


### Bug Fixes

* **decider:** Fix decider monitoring deals ([#18](https://github.com/fluencelabs/decider/issues/18)) ([57793fe](https://github.com/fluencelabs/decider/commit/57793fe6e2b9b7c2b3248114282716a6b266a991))

## [0.4.2](https://github.com/fluencelabs/decider/compare/connector-v0.4.1...connector-v0.4.2) (2023-02-27)


### Bug Fixes

* **builtin:** build builtin package with a single level directory structure ([#16](https://github.com/fluencelabs/decider/issues/16)) ([6beb2f7](https://github.com/fluencelabs/decider/commit/6beb2f7e6d1304e04ad21fac8cc55a520c7ab1e2))

## [0.4.1](https://github.com/fluencelabs/decider/compare/connector-v0.4.0...connector-v0.4.1) (2023-02-27)


### Bug Fixes

* **builtin-package:** install new decider after removing ([#14](https://github.com/fluencelabs/decider/issues/14)) ([76db95c](https://github.com/fluencelabs/decider/commit/76db95cb90ca5af7691314bfdbc18b5dbc878b19))

## [0.4.0](https://github.com/fluencelabs/decider/compare/connector-v0.3.0...connector-v0.4.0) (2023-02-27)


### ⚠ BREAKING CHANGES

* **decider, connector:** Get latest block on start; update from_block on over limit #12

### Features

* **decider, connector:** Get latest block on start; update from_block on over limit [#12](https://github.com/fluencelabs/decider/issues/12) ([cec584a](https://github.com/fluencelabs/decider/commit/cec584acd97b118a8dced9d802d556a264a56117))
