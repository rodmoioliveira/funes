# Changelog
All notable changes to this project will be documented in this file.

## [0.3.0](https://github.com/rodmoioliveira/funes/compare/0.2.9...0.3.0) - 2021-10-16

[485143e](https://github.com/rodmoioliveira/funes/commit/485143e3d6c32e2754c9ed2e86cdbb303c418b77)...[a289bac](https://github.com/rodmoioliveira/funes/commit/a289bac82085d2e4739f9b980003e9d55cd9d182)

### Documentation

- Fix README for examples/latency-collection ([fe053cb](https://github.com/rodmoioliveira/funes/commit/fe053cb1edd667de4e5d750786e2c03291946857))
- Improve README ([96ec95b](https://github.com/rodmoioliveira/funes/commit/96ec95b270b0eb1d3bace9fab506785ef8090914))

### Features

- Add env ASYNC_TASK_SLEEP_MODIFIER ([dd7a059](https://github.com/rodmoioliveira/funes/commit/dd7a0591e0ecf8f3817b642044ec9a3034a79d00))

### Miscellaneous Tasks

- 0.3.0 ([a289bac](https://github.com/rodmoioliveira/funes/commit/a289bac82085d2e4739f9b980003e9d55cd9d182))

### Refactor

- Map std::io::Error to error::FunesError ([e00bcc7](https://github.com/rodmoioliveira/funes/commit/e00bcc7cf2037ec7865da98506a3aef286c91e16))
- Add statics for FUNES_ envs ([f53563e](https://github.com/rodmoioliveira/funes/commit/f53563e4904e1c3e3c87a7331cfe1afa9bc3fe51))
- Return format! directly format::resource() ([7d66e49](https://github.com/rodmoioliveira/funes/commit/7d66e49f340861c75d27020fcc1c60777dfe5669))
- Add config::FUNES_LATENCY_COLLECTION in server.rs ([98aa494](https://github.com/rodmoioliveira/funes/commit/98aa494bcbd35d4e3238d682baa5e3b76c87a2e1))
- Add ASYNC_TASK_SLEEP_MODIFIER ([a258123](https://github.com/rodmoioliveira/funes/commit/a258123d85d7b622bb048ea0613d03e35bb9d66f))
- Remove FUNES_API_REGEX env ([c3e2e03](https://github.com/rodmoioliveira/funes/commit/c3e2e03a8388825c2b9faa926d0d441e96cbd052))

## [0.2.9](https://github.com/rodmoioliveira/funes/compare/0.2.8...0.2.9) - 2021-10-14

[6972b8b](https://github.com/rodmoioliveira/funes/commit/6972b8b47b9e4bfc7095cccf0d2dd1635699465f)...[485143e](https://github.com/rodmoioliveira/funes/commit/485143e3d6c32e2754c9ed2e86cdbb303c418b77)

### Bug Fixes

- Remore statics autoimport ([4af8fa2](https://github.com/rodmoioliveira/funes/commit/4af8fa2ffacef474ba306e8aab0fde8c75e3a943))

### Miscellaneous Tasks

- Remove reqwest::Client timeout ([027a538](https://github.com/rodmoioliveira/funes/commit/027a5383ad1b1db720c76e0478a81559a5e0c028))
- 0.2.9 ([485143e](https://github.com/rodmoioliveira/funes/commit/485143e3d6c32e2754c9ed2e86cdbb303c418b77))

## [0.2.8](https://github.com/rodmoioliveira/funes/compare/0.2.7...0.2.8) - 2021-10-11

[4a19cbb](https://github.com/rodmoioliveira/funes/commit/4a19cbbac0caaf483726447e31ca71fe2a7f4cd9)...[6972b8b](https://github.com/rodmoioliveira/funes/commit/6972b8b47b9e4bfc7095cccf0d2dd1635699465f)

### Bug Fixes

- Fix docker example ([95ed561](https://github.com/rodmoioliveira/funes/commit/95ed561eea2726a1d43273246c3a2fe63249fe6e))

### Documentation

- Update CHANGELOG ([3a58626](https://github.com/rodmoioliveira/funes/commit/3a58626a3500576c7cba4fbda761a1992566691a))
- Update CHANGELOG ([17c345a](https://github.com/rodmoioliveira/funes/commit/17c345a7936aa27c86c804642b4bcb328294694c))
- Update CHANGELOG ([6972b8b](https://github.com/rodmoioliveira/funes/commit/6972b8b47b9e4bfc7095cccf0d2dd1635699465f))

### Refactor

- Refactor api namespace ([e4e2c94](https://github.com/rodmoioliveira/funes/commit/e4e2c949320689109892bc30724c33fc9de86b73))
- Remove dead code ([4b15083](https://github.com/rodmoioliveira/funes/commit/4b1508304c8c1d2cfde9ccfb65c1d99a7a117668))

## [0.2.7](https://github.com/rodmoioliveira/funes/compare/0.2.6...0.2.7) - 2021-10-08

[e59f20c](https://github.com/rodmoioliveira/funes/commit/e59f20caef2b47e3ef1a7005954f051600c87fe6)...[4a19cbb](https://github.com/rodmoioliveira/funes/commit/4a19cbbac0caaf483726447e31ca71fe2a7f4cd9)

### Bug Fixes

- Revert to version 0.2.4 ([2020bc5](https://github.com/rodmoioliveira/funes/commit/2020bc58230212e0341e97d0a6c4a94b1c292bf1))

### Documentation

- Update changelog ([17439e2](https://github.com/rodmoioliveira/funes/commit/17439e2fec7bb594a8d3a5131865b8638a1022cf))

### Miscellaneous Tasks

- 0.2.7 ([4a19cbb](https://github.com/rodmoioliveira/funes/commit/4a19cbbac0caaf483726447e31ca71fe2a7f4cd9))

## [0.2.6](https://github.com/rodmoioliveira/funes/compare/0.2.5...0.2.6) - 2021-09-22

[6eb659e](https://github.com/rodmoioliveira/funes/commit/6eb659e10c44368275314286f253a9df77dd7ae4)...[e59f20c](https://github.com/rodmoioliveira/funes/commit/e59f20caef2b47e3ef1a7005954f051600c87fe6)

### Documentation

- Update docs ([5eeae7e](https://github.com/rodmoioliveira/funes/commit/5eeae7e709fccaa96a91cb7e563809046294da74))

### Features

- Add METHOD and PAYLOAD to mocks ([4b6eb72](https://github.com/rodmoioliveira/funes/commit/4b6eb72b567147921bf9e9503d784af0cf6905f4))

### Miscellaneous Tasks

- 0.2.6 ([e59f20c](https://github.com/rodmoioliveira/funes/commit/e59f20caef2b47e3ef1a7005954f051600c87fe6))

### Testing

- Add new test in handlers ([09c6cce](https://github.com/rodmoioliveira/funes/commit/09c6cceb91388b2769810e33551860ca05fb5c16))

## [0.2.5](https://github.com/rodmoioliveira/funes/compare/0.2.4...0.2.5) - 2021-09-14

[a5dd6f8](https://github.com/rodmoioliveira/funes/commit/a5dd6f88e7300cb3a50d38680459c6e3960c6588)...[6eb659e](https://github.com/rodmoioliveira/funes/commit/6eb659e10c44368275314286f253a9df77dd7ae4)

### Features

- Return status code from mock response ([16a3e37](https://github.com/rodmoioliveira/funes/commit/16a3e37974c3e438898f4234b611731f7f92bbd7))

### Miscellaneous Tasks

- 0.2.5 ([6eb659e](https://github.com/rodmoioliveira/funes/commit/6eb659e10c44368275314286f253a9df77dd7ae4))

## [0.2.4](https://github.com/rodmoioliveira/funes/compare/0.2.3...0.2.4) - 2021-09-14

[49b9a7f](https://github.com/rodmoioliveira/funes/commit/49b9a7f684a215f430243b9a6c44ef03574b2536)...[a5dd6f8](https://github.com/rodmoioliveira/funes/commit/a5dd6f88e7300cb3a50d38680459c6e3960c6588)

### Miscellaneous Tasks

- Update CHANGELOG ([ea44237](https://github.com/rodmoioliveira/funes/commit/ea44237b1c16b8b1d345a23d95a6647a847b3166))
- Alter FUNES_MOCK_DIR to .funes ([ab04f50](https://github.com/rodmoioliveira/funes/commit/ab04f504487d9733b3083e24b2c988919bb55088))
- 0.2.4 ([a5dd6f8](https://github.com/rodmoioliveira/funes/commit/a5dd6f88e7300cb3a50d38680459c6e3960c6588))

## [0.2.3](https://github.com/rodmoioliveira/funes/compare/0.2.2...0.2.3) - 2021-09-10

[d00684d](https://github.com/rodmoioliveira/funes/commit/d00684dd2a607a0259de5adcea7fd50ca10017e3)...[49b9a7f](https://github.com/rodmoioliveira/funes/commit/49b9a7f684a215f430243b9a6c44ef03574b2536)

### Bug Fixes

- Fix pipeline badges ([b1a5d7d](https://github.com/rodmoioliveira/funes/commit/b1a5d7d4b51727889a6754e63f0266b5e9b38230))

### CI

- Add recipes in Makefile ([4f8343f](https://github.com/rodmoioliveira/funes/commit/4f8343fe2e7d09278576960225c33e185b507fd0))

### Features

- Add fxhash crate ([5fcb131](https://github.com/rodmoioliveira/funes/commit/5fcb1311517de49d38f0b5eda487f608a1314d87))
- Add benchmark for api::map_range() ([be3e2fb](https://github.com/rodmoioliveira/funes/commit/be3e2fb4415a1089c46b1ec9a33148385c402f6a))

### Miscellaneous Tasks

- Update CHANGELOG ([44b02c2](https://github.com/rodmoioliveira/funes/commit/44b02c24392c4903e192db488f330fe89ee1a69a))
- Remove Ok() from io::read() ([7f89972](https://github.com/rodmoioliveira/funes/commit/7f8997212dffc4ee14a4d5aece56fa22c1c112ff))
- 0.2.3 ([49b9a7f](https://github.com/rodmoioliveira/funes/commit/49b9a7f684a215f430243b9a6c44ef03574b2536))

## [0.2.2](https://github.com/rodmoioliveira/funes/compare/0.2.1...0.2.2) - 2021-09-09

[8a04042](https://github.com/rodmoioliveira/funes/commit/8a04042455a68e05f9847347493a743e5b20a816)...[d00684d](https://github.com/rodmoioliveira/funes/commit/d00684dd2a607a0259de5adcea7fd50ca10017e3)

### Bug Fixes

- Run cargo clippy --fix ([6392785](https://github.com/rodmoioliveira/funes/commit/63927851436bc040dad009a2e3f33a3eb6e07b21))
- Fix cargo clippy ([94cc588](https://github.com/rodmoioliveira/funes/commit/94cc5881ea0c3438dbc844bb2bd318356088b359))

### CI

- Improve ci pipelines ([21faf3d](https://github.com/rodmoioliveira/funes/commit/21faf3dc93cf5dd66778f386528bae392d0a8ecb))
- Rename ci file ([14ccd3a](https://github.com/rodmoioliveira/funes/commit/14ccd3abb861fdc863840c03bec2fef86748d017))

### Miscellaneous Tasks

- Update CHANGELOG ([af88155](https://github.com/rodmoioliveira/funes/commit/af8815513f65c53b2e4d06de62e2ae6c77575fa7))
- Remove Ok(?) from fetch ([acd487f](https://github.com/rodmoioliveira/funes/commit/acd487fe435bbe6fb98e4d2213ce7373e4929cc0))
- 0.2.2 ([d00684d](https://github.com/rodmoioliveira/funes/commit/d00684dd2a607a0259de5adcea7fd50ca10017e3))

## [0.2.1](https://github.com/rodmoioliveira/funes/compare/0.2.0...0.2.1) - 2021-09-09

[b12fb1e](https://github.com/rodmoioliveira/funes/commit/b12fb1eb58ae1373057fcbfc1fad4735b0dbd862)...[8a04042](https://github.com/rodmoioliveira/funes/commit/8a04042455a68e05f9847347493a743e5b20a816)

### Documentation

- Improve docs ([483f93a](https://github.com/rodmoioliveira/funes/commit/483f93ad41c347eb8059ec6fdcb79c979a45fb39))

### Miscellaneous Tasks

- Update CHANGELOG ([5aa9aa4](https://github.com/rodmoioliveira/funes/commit/5aa9aa450f00f248bb49301b27d241613efbb4ae))
- Update Cargo.lock ([f821f5c](https://github.com/rodmoioliveira/funes/commit/f821f5c169e3d026459d4b79d52f41e807892df4))
- Update latency-collection examples ([4d1a18d](https://github.com/rodmoioliveira/funes/commit/4d1a18d9693ec16b3ebd74b531f706c928c02fa0))
- Set default FUNES_MOCK_DIR as $HOME/.mocks ([36cfba0](https://github.com/rodmoioliveira/funes/commit/36cfba06ae844f72b92d28f245ec46244cb2a3b4))
- Remove log::debug! ([3051a35](https://github.com/rodmoioliveira/funes/commit/3051a35ab8eb138ca1f82d592a0e6597eb57938f))
- Default of FUNES_LATENCY_COLLECTION is "" ([558789c](https://github.com/rodmoioliveira/funes/commit/558789cfb03e1587258daad3bf6a6679adb86cc9))
- 0.2.1 ([8a04042](https://github.com/rodmoioliveira/funes/commit/8a04042455a68e05f9847347493a743e5b20a816))

## [0.2.0](https://github.com/rodmoioliveira/funes/compare/0.1.11...0.2.0) - 2021-09-09

[5915b85](https://github.com/rodmoioliveira/funes/commit/5915b8559236ec24456570b97c68416c25d4cb21)...[b12fb1e](https://github.com/rodmoioliveira/funes/commit/b12fb1eb58ae1373057fcbfc1fad4735b0dbd862)

### Features

- Implement latency_collection ([1b82c72](https://github.com/rodmoioliveira/funes/commit/1b82c72510321923a4387e790bcca925105728e3))

### Miscellaneous Tasks

- 0.2.0 ([b12fb1e](https://github.com/rodmoioliveira/funes/commit/b12fb1eb58ae1373057fcbfc1fad4735b0dbd862))

## [0.1.11](https://github.com/rodmoioliveira/funes/compare/0.1.10...0.1.11) - 2021-09-03

[d196756](https://github.com/rodmoioliveira/funes/commit/d1967560337ef19f8295021512845544d748843f)...[5915b85](https://github.com/rodmoioliveira/funes/commit/5915b8559236ec24456570b97c68416c25d4cb21)

### Miscellaneous Tasks

- Update CHANGELOG ([6239c8c](https://github.com/rodmoioliveira/funes/commit/6239c8c9a22da8c0c11d659b79e1d47360d8e4ef))
- 0.1.11 ([5915b85](https://github.com/rodmoioliveira/funes/commit/5915b8559236ec24456570b97c68416c25d4cb21))

## [0.1.10](https://github.com/rodmoioliveira/funes/compare/0.1.9...0.1.10) - 2021-09-03

[7b611ce](https://github.com/rodmoioliveira/funes/commit/7b611ce5a4940837c31faa411873f19cf03545c4)...[d196756](https://github.com/rodmoioliveira/funes/commit/d1967560337ef19f8295021512845544d748843f)

### Miscellaneous Tasks

- Set_var RUST_LOG ([a88ee38](https://github.com/rodmoioliveira/funes/commit/a88ee38941b0c6d27455218587a539636d7f9ae5))
- 0.1.10 ([d196756](https://github.com/rodmoioliveira/funes/commit/d1967560337ef19f8295021512845544d748843f))

## [0.1.9](https://github.com/rodmoioliveira/funes/compare/0.1.8...0.1.9) - 2021-09-03

[9368c7e](https://github.com/rodmoioliveira/funes/commit/9368c7efceaf123747c3bf62117d41ea18f20f87)...[7b611ce](https://github.com/rodmoioliveira/funes/commit/7b611ce5a4940837c31faa411873f19cf03545c4)

### Documentation

- Add docs.rs docs ([a01e139](https://github.com/rodmoioliveira/funes/commit/a01e139299d013ed9af092fb250a48852080c328))

### Miscellaneous Tasks

- Update CHANGELOG ([2d98fed](https://github.com/rodmoioliveira/funes/commit/2d98fede8567f66f851ddf2baa740d99d1e3a9e8))
- Update README ([2a969bc](https://github.com/rodmoioliveira/funes/commit/2a969bc4791bcf5d45ecf3a6f79f095cf9ff8f31))
- 0.1.9 ([7b611ce](https://github.com/rodmoioliveira/funes/commit/7b611ce5a4940837c31faa411873f19cf03545c4))

## [0.1.8](https://github.com/rodmoioliveira/funes/compare/0.1.7...0.1.8) - 2021-09-03

[543f119](https://github.com/rodmoioliveira/funes/commit/543f1196e711e80c176bedf82df43b9bb8ae6526)...[9368c7e](https://github.com/rodmoioliveira/funes/commit/9368c7efceaf123747c3bf62117d41ea18f20f87)

### Miscellaneous Tasks

- Update CHANGELOG ([f9e855c](https://github.com/rodmoioliveira/funes/commit/f9e855cac5bdc222aaf19655ffcbc5f349450bb8))
- Update README ([cb5a94a](https://github.com/rodmoioliveira/funes/commit/cb5a94a381235f2df11a58111462e9afb7f7d6f2))
- 0.1.8 ([9368c7e](https://github.com/rodmoioliveira/funes/commit/9368c7efceaf123747c3bf62117d41ea18f20f87))

## [0.1.7](https://github.com/rodmoioliveira/funes/compare/0.1.6...0.1.7) - 2021-09-03

[5513065](https://github.com/rodmoioliveira/funes/commit/5513065d30edc8b9fdda8002e80a1b013b114a6c)...[543f119](https://github.com/rodmoioliveira/funes/commit/543f1196e711e80c176bedf82df43b9bb8ae6526)

### Miscellaneous Tasks

- Update CHANGELOG ([fc4a27d](https://github.com/rodmoioliveira/funes/commit/fc4a27d41fb4dbde8ab97c58758ce39c4c31faad))
- Update manifest ([33e45db](https://github.com/rodmoioliveira/funes/commit/33e45dbe235a67f0418b73943e164c4983c39eef))
- Remove criterion ([72f26ff](https://github.com/rodmoioliveira/funes/commit/72f26ff03732e3b9e033abe3d1825b0d965c17d1))
- 0.1.7 ([543f119](https://github.com/rodmoioliveira/funes/commit/543f1196e711e80c176bedf82df43b9bb8ae6526))

## [0.1.6](https://github.com/rodmoioliveira/funes/compare/0.1.5...0.1.6) - 2021-09-02

[4f844be](https://github.com/rodmoioliveira/funes/commit/4f844be0f64dd4fe252d4c86fea64ea885ab99ea)...[5513065](https://github.com/rodmoioliveira/funes/commit/5513065d30edc8b9fdda8002e80a1b013b114a6c)

### Miscellaneous Tasks

- Update CHANGELOG ([5792da7](https://github.com/rodmoioliveira/funes/commit/5792da7547d8a5ebee5da646f4f3dd7c44e3ddc4))

### Refactor

- Change modules visibility ([885f978](https://github.com/rodmoioliveira/funes/commit/885f978e1187c5c875e6c015068a2367a99dc522))
- Rename errors ([9601971](https://github.com/rodmoioliveira/funes/commit/960197119a5726ee2bfc491d27f6d191fdd25c9f))
- Rename MyError to FunesError ([5513065](https://github.com/rodmoioliveira/funes/commit/5513065d30edc8b9fdda8002e80a1b013b114a6c))

## [0.1.5](https://github.com/rodmoioliveira/funes/compare/0.1.4...0.1.5) - 2021-09-01

[8706c66](https://github.com/rodmoioliveira/funes/commit/8706c66999af01e87ec29f7b45865cbacb7d3b9d)...[4f844be](https://github.com/rodmoioliveira/funes/commit/4f844be0f64dd4fe252d4c86fea64ea885ab99ea)

### Bug Fixes

- Fix tests ([cdf4ac0](https://github.com/rodmoioliveira/funes/commit/cdf4ac0a6b5526a43e220d3c03f3285a541cdbe9))

### Features

- Create io namespace ([d9b3033](https://github.com/rodmoioliveira/funes/commit/d9b303348475e964d9f9505c2a5610bef9fed444))
- Rename functions ([71e84a1](https://github.com/rodmoioliveira/funes/commit/71e84a1c4bbf667e0102bf4898ac02be3d551979))
- Create format namespace ([98cd047](https://github.com/rodmoioliveira/funes/commit/98cd04726480b84af63dae4c76cd0e4b9308b179))

### Miscellaneous Tasks

- Update CHANGELOG ([36119a1](https://github.com/rodmoioliveira/funes/commit/36119a165104804c722d084021f2832fc3428dd1))
- Rename check_mocks_dir() to mock_dir() ([4f844be](https://github.com/rodmoioliveira/funes/commit/4f844be0f64dd4fe252d4c86fea64ea885ab99ea))

### Testing

- Add tests post_api and get_api ([19e1601](https://github.com/rodmoioliveira/funes/commit/19e16017bbf6fa6c0dd7af0169c104d123fac510))

## [0.1.4](https://github.com/rodmoioliveira/funes/compare/0.1.3...0.1.4) - 2021-09-01

[5e1c60a](https://github.com/rodmoioliveira/funes/commit/5e1c60a65e9e53b83ca93b35efaa2ca86da8dc83)...[8706c66](https://github.com/rodmoioliveira/funes/commit/8706c66999af01e87ec29f7b45865cbacb7d3b9d)

### Bug Fixes

- Remove status route ([137c205](https://github.com/rodmoioliveira/funes/commit/137c205c45ad231655160dd5d5a1409f8fe07d6d))
- Remove convert_headers() and headers from reqwest ([8706c66](https://github.com/rodmoioliveira/funes/commit/8706c66999af01e87ec29f7b45865cbacb7d3b9d))

### CI

- Create rust.yml ([b4d03b1](https://github.com/rodmoioliveira/funes/commit/b4d03b1a252c2a4c43bc506bf14cbe42e2cf2288))

### Features

- Create RUST_MOCK_DIR env ([c495255](https://github.com/rodmoioliveira/funes/commit/c495255d920808f87fe322db4c6ca74a3fa5b827))

### Miscellaneous Tasks

- Update CHANGELOG ([9d76977](https://github.com/rodmoioliveira/funes/commit/9d769771f3e84a4f9ebedc7438bce5d7848ff4b4))
- Add pipeline workflow badge ([527cb44](https://github.com/rodmoioliveira/funes/commit/527cb44a5071c5d8383224a74d378d0db98ad755))
- Remove some .clone()s ([918be20](https://github.com/rodmoioliveira/funes/commit/918be20a9fbce9069b3f52f781b71149c600d6a1))

## [0.1.3](https://github.com/rodmoioliveira/funes/compare/0.1.2...0.1.3) - 2021-08-31

[be52bc1](https://github.com/rodmoioliveira/funes/commit/be52bc18ff14306fd5ece0947a7f1302e902fc41)...[5e1c60a](https://github.com/rodmoioliveira/funes/commit/5e1c60a65e9e53b83ca93b35efaa2ca86da8dc83)

### Features

- Add docker example ([5e1c60a](https://github.com/rodmoioliveira/funes/commit/5e1c60a65e9e53b83ca93b35efaa2ca86da8dc83))

## [0.1.2](https://github.com/rodmoioliveira/funes/compare/0.1.1...0.1.2) - 2021-08-31

[13f8f4c](https://github.com/rodmoioliveira/funes/commit/13f8f4cd2218b2653072567df1a1f69e02e84d2b)...[be52bc1](https://github.com/rodmoioliveira/funes/commit/be52bc18ff14306fd5ece0947a7f1302e902fc41)

### Bug Fixes

- Fix CHANGELOG links ([05779c0](https://github.com/rodmoioliveira/funes/commit/05779c026f1a86ebda90b7c6f036dc9aaf6cbe0b))
- Fix headers for client and server ([be52bc1](https://github.com/rodmoioliveira/funes/commit/be52bc18ff14306fd5ece0947a7f1302e902fc41))

### Miscellaneous Tasks

- Update CHANGELOG ([e36c6bd](https://github.com/rodmoioliveira/funes/commit/e36c6bddabc35f4d7cc65f145b091fed22b0d1e5))

## [0.1.1](https://github.com/rodmoioliveira/funes/compare/0.1.0...0.1.1) - 2021-08-31

[d2cda58](https://github.com/rodmoioliveira/funes/commit/d2cda5867210f84aa84b6e943d23ea3016b219f4)...[13f8f4c](https://github.com/rodmoioliveira/funes/commit/13f8f4cd2218b2653072567df1a1f69e02e84d2b)

### CI

- Update ([9b64399](https://github.com/rodmoioliveira/funes/commit/9b643997dcac1296d02b6ec4e6ab3e8fc304ff7a))

### Miscellaneous Tasks

- Add README badges ([deebf84](https://github.com/rodmoioliveira/funes/commit/deebf845269c25719164cbb31250f2f323db2991))
- Add Borges quote ([b937823](https://github.com/rodmoioliveira/funes/commit/b937823af3adfc7589dd03ec5d1c90adb882dd75))
- Improve documentation ([13f8f4c](https://github.com/rodmoioliveira/funes/commit/13f8f4cd2218b2653072567df1a1f69e02e84d2b))

## [0.1.0](https://github.com/rodmoioliveira/funes/compare/...0.1.0) - 2021-08-30

### Miscellaneous Tasks

- First commit ([d2cda58](https://github.com/rodmoioliveira/funes/commit/d2cda5867210f84aa84b6e943d23ea3016b219f4))

<!-- generated by git-cliff -->
