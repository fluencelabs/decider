# Changelog

## [0.5.9](https://github.com/fluencelabs/decider/compare/connector-v0.5.8...connector-v0.5.9) (2023-12-21)


### Bug Fixes

* update spell api ([#161](https://github.com/fluencelabs/decider/issues/161)) ([0025dc8](https://github.com/fluencelabs/decider/commit/0025dc899381c5211f3c1098f5a23cc08d129a6a))

## [0.5.8](https://github.com/fluencelabs/decider/compare/connector-v0.5.7...connector-v0.5.8) (2023-12-21)


### Features

* update marine sdk's and configs ([#154](https://github.com/fluencelabs/decider/issues/154)) ([c006b42](https://github.com/fluencelabs/decider/commit/c006b42b7e94c4f50868dbf4f6257b2310ee58ec))

## [0.5.7](https://github.com/fluencelabs/decider/compare/connector-v0.5.6...connector-v0.5.7) (2023-12-20)


### Features

* **decider,chain_connector:** activate/deactivate deals ([#153](https://github.com/fluencelabs/decider/issues/153)) ([963ef5a](https://github.com/fluencelabs/decider/commit/963ef5aeb52069fb35bd04d0f8a07d900587acae))
* **decider,chain_connector:** add deal removal by compute unit [fixes NET-659, fixes NET-696] ([#155](https://github.com/fluencelabs/decider/issues/155)) ([ef231ac](https://github.com/fluencelabs/decider/commit/ef231aca9d8eabf6d08c793f4b4e661ca169f786))
* **decider:** retrigger worker-spell on deal update [NET-649] ([#148](https://github.com/fluencelabs/decider/issues/148)) ([ff9f826](https://github.com/fluencelabs/decider/commit/ff9f826f7cbc48aa627c8ba721e3e59452e328fd))

## [0.5.6](https://github.com/fluencelabs/decider/compare/connector-v0.5.5...connector-v0.5.6) (2023-12-07)


### Features

* **decider,chain-connector:** support deal removal [fixes NET-515] ([#144](https://github.com/fluencelabs/decider/issues/144)) ([ba849eb](https://github.com/fluencelabs/decider/commit/ba849eb32b2bb3c05fe30cba574293ae1134d318))

## [0.5.5](https://github.com/fluencelabs/decider/compare/connector-v0.5.4...connector-v0.5.5) (2023-11-16)


### Features

* bump aqua-lib to 0.8.0 + [NET-623] ([#109](https://github.com/fluencelabs/decider/issues/109)) ([de473e5](https://github.com/fluencelabs/decider/commit/de473e58c6423d5993c8ed17cc579aa2f015a81b))
* bump spell to 0.5.30 ([#111](https://github.com/fluencelabs/decider/issues/111)) ([089be2c](https://github.com/fluencelabs/decider/commit/089be2cd2e11bd2fa270abe5a60d60dee552c10d))
* update deps ([#112](https://github.com/fluencelabs/decider/issues/112)) ([308bafd](https://github.com/fluencelabs/decider/commit/308bafdd699cd51be5de28acac230fd72e16e9f5))


### Bug Fixes

* **decider:** Move poll window of deal updates when there are no updates [NET-625] ([#108](https://github.com/fluencelabs/decider/issues/108)) ([16ae574](https://github.com/fluencelabs/decider/commit/16ae5747c63fc14a44415834761daf183be4ac79))

## [0.5.4](https://github.com/fluencelabs/decider/compare/connector-v0.5.3...connector-v0.5.4) (2023-11-07)


### Features

* **decider,chain_connector,tests:** Worker registration tx tracking [NET-575, NET-538] ([#90](https://github.com/fluencelabs/decider/issues/90)) ([22c1065](https://github.com/fluencelabs/decider/commit/22c1065ab3508374635076175078683988904a5e))
* **decider:** Use streams instead of options [LNG-277] ([#106](https://github.com/fluencelabs/decider/issues/106)) ([9e92d8f](https://github.com/fluencelabs/decider/commit/9e92d8f78eedee4c93b23ac07c7b5b39c30712f4))

## [0.5.3](https://github.com/fluencelabs/decider/compare/connector-v0.5.2...connector-v0.5.3) (2023-10-28)


### Bug Fixes

* **deps:** update rust crate marine-rs-sdk-test to 0.11.0 ([#91](https://github.com/fluencelabs/decider/issues/91)) ([68cab1d](https://github.com/fluencelabs/decider/commit/68cab1d2b498d1706e0c6f9a8a4c51ff8f6088f5))

## [0.5.2](https://github.com/fluencelabs/decider/compare/connector-v0.5.1...connector-v0.5.2) (2023-10-25)


### Bug Fixes

* **connector:** Rename connector to ChainConnector [NET-537] ([#73](https://github.com/fluencelabs/decider/issues/73)) ([26cde44](https://github.com/fluencelabs/decider/commit/26cde44acdd9d5e678141d1c482a88b7ee9037b8))
* **decider:** update AppCIDChanged topic [NET-573] ([#94](https://github.com/fluencelabs/decider/issues/94)) ([2261def](https://github.com/fluencelabs/decider/commit/2261defa3ac7fc0b14f3f41199512e348f097a0a))
* **decider:** use new logs api [NET-543] ([#79](https://github.com/fluencelabs/decider/issues/79)) ([3dabea9](https://github.com/fluencelabs/decider/commit/3dabea9903ca450b6d93fa111949bdd18395929f))

## [0.5.1](https://github.com/fluencelabs/decider/compare/connector-v0.5.0...connector-v0.5.1) (2023-09-06)


### Bug Fixes

* **decider:** add alias for worker spell ([#71](https://github.com/fluencelabs/decider/issues/71)) ([860ba6c](https://github.com/fluencelabs/decider/commit/860ba6c31d0a0dba9c29a4c34fe05afda256ce7a))

## [0.5.0](https://github.com/fluencelabs/decider/compare/connector-v0.4.17...connector-v0.5.0) (2023-09-05)


### ⚠ BREAKING CHANGES

* **match:** Create workers on Match, register, resolve subnet ([#68](https://github.com/fluencelabs/decider/issues/68))

### Features

* **builtin:** remove builtin package ([#63](https://github.com/fluencelabs/decider/issues/63)) ([0be2b0d](https://github.com/fluencelabs/decider/commit/0be2b0db38e45463922f8a08a394fc570b883212))
* **match:** Create workers on Match, register, resolve subnet ([#68](https://github.com/fluencelabs/decider/issues/68)) ([b40d042](https://github.com/fluencelabs/decider/commit/b40d0421fe7558f531bdfb0090df26d01d09d89b))

## [0.4.17](https://github.com/fluencelabs/decider/compare/connector-v0.4.16...connector-v0.4.17) (2023-07-13)


### Bug Fixes

* sort imports to trigger release ([#61](https://github.com/fluencelabs/decider/issues/61)) ([3a74db6](https://github.com/fluencelabs/decider/commit/3a74db6e211475ffb803573e1d45b71d511ec715))

## [0.4.16](https://github.com/fluencelabs/decider/compare/connector-v0.4.15...connector-v0.4.16) (2023-06-26)


### Bug Fixes

* find latest block when from_block is latest ([#57](https://github.com/fluencelabs/decider/issues/57)) ([0bded86](https://github.com/fluencelabs/decider/commit/0bded86f808aa0d2b71ba2a1967b0021f40cecfb))

## [0.4.15](https://github.com/fluencelabs/decider/compare/connector-v0.4.14...connector-v0.4.15) (2023-06-09)


### Features

* add distro crate [fixes NET-464]  ([#51](https://github.com/fluencelabs/decider/issues/51)) ([f240ca4](https://github.com/fluencelabs/decider/commit/f240ca4fc1f63e36e7a85b72ced098dc1fe28ed4))

## [0.4.14](https://github.com/fluencelabs/decider/compare/connector-v0.4.13...connector-v0.4.14) (2023-06-06)


### Bug Fixes

* **decider:** fix args naming ([#48](https://github.com/fluencelabs/decider/issues/48)) ([a8dea61](https://github.com/fluencelabs/decider/commit/a8dea61d5b2f706c71e04371990dd4e0f4562655))
* **deps:** update installation-spell to 0.5.14 ([#50](https://github.com/fluencelabs/decider/issues/50)) ([5501c4d](https://github.com/fluencelabs/decider/commit/5501c4def4d75ba374e24254b1b0050f99717b4e))
* **deps:** update rust crate marine-rs-sdk-test to 0.10.0 ([#38](https://github.com/fluencelabs/decider/issues/38)) ([bb8712b](https://github.com/fluencelabs/decider/commit/bb8712b7504235bae00d76a22eba5970d86a8dbd))

## [0.4.13](https://github.com/fluencelabs/decider/compare/connector-v0.4.12...connector-v0.4.13) (2023-05-30)


### Features

* **decider:** add FLUENCE_ENV_CONNECTOR_FROM_BLOCK env variable ([#46](https://github.com/fluencelabs/decider/issues/46)) ([cd79d62](https://github.com/fluencelabs/decider/commit/cd79d62f461609cbd3a92afcc6ffd02e0225c5c7))

## [0.4.12](https://github.com/fluencelabs/decider/compare/connector-v0.4.11...connector-v0.4.12) (2023-05-30)


### Features

* pass api endpoint via env [NET-479] ([#39](https://github.com/fluencelabs/decider/issues/39)) ([75aea7e](https://github.com/fluencelabs/decider/commit/75aea7e8dc6e4d098ff1efe48d700006677a19bb))

## [0.4.11](https://github.com/fluencelabs/decider/compare/connector-v0.4.10...connector-v0.4.11) (2023-05-26)


### Features

* **installation-spell:** ACTUALLY UPDATE IT ([#43](https://github.com/fluencelabs/decider/issues/43)) ([bbb3809](https://github.com/fluencelabs/decider/commit/bbb3809eb414363fd9a93727c05a69c2c98b7fd0))

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
