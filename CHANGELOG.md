# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

---
## [prost-helper-v0.8.3](https://github.com/tyrchen/prost-helper/compare/prost-helper-v0.8.2..prost-helper-v0.8.3) - 2023-12-22

### Bug Fixes

- fix deserializer issue with `expected a borrowed string` - ([47776ec](https://github.com/tyrchen/prost-helper/commit/47776ecb51e6a17409b3559c71cec5308e635e95)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([210bfc3](https://github.com/tyrchen/prost-helper/commit/210bfc3ae120a03116266262d9d07b17cebdf3be)) - Tyr Chen

---
## [prost-helper-v0.8.2](https://github.com/tyrchen/prost-helper/compare/prost-helper-v0.8.1..prost-helper-v0.8.2) - 2023-12-21

### Miscellaneous Chores

- fix dep - ([90e3521](https://github.com/tyrchen/prost-helper/commit/90e35215f853326e072c9f92deefbfe74a56e39d)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([6844365](https://github.com/tyrchen/prost-helper/commit/6844365c2edb61209909a8091fcdf6218a79b8a3)) - Tyr Chen

---
## [prost-helper-v0.8.1](https://github.com/tyrchen/prost-helper/compare/prost-build-config-v0.6.1..prost-helper-v0.8.1) - 2023-12-21

### Bug Fixes

- fix default config - ([ce442cc](https://github.com/tyrchen/prost-helper/commit/ce442ccf82dcda531f0058538b9c0fa8cf7ca751)) - Tyr Chen

### Features

- 1) add base_path 2) add uuid7 serialize/deserialize to/from string - ([53c5d58](https://github.com/tyrchen/prost-helper/commit/53c5d58813a8903be83da89b1676f1a99eb00ff6)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([270310c](https://github.com/tyrchen/prost-helper/commit/270310cdf4ebf6bfde54898bf18fc71b1cdc4480)) - Tyr Chen

---
## [prost-build-config-v0.6.1](https://github.com/tyrchen/prost-helper/compare/prost-helper-v0.8.0..prost-build-config-v0.6.1) - 2023-12-20

### Bug Fixes

- use prettyplease - ([0be00ec](https://github.com/tyrchen/prost-helper/commit/0be00ec5e0a5f938fa612fe2fd1bfe6cab9ad165)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([98b707c](https://github.com/tyrchen/prost-helper/commit/98b707c9d6395d38e691b6d5fb82309cc023af7f)) - Tyr Chen

---
## [prost-helper-v0.8.0](https://github.com/tyrchen/prost-helper/compare/prost-build-config-v0.4.1..prost-helper-v0.8.0) - 2023-12-18

### Bug Fixes

- need protoc for gh action - ([81f9125](https://github.com/tyrchen/prost-helper/commit/81f91258324c5837916d3822fabe708d59638b19)) - Tyr Chen

### Features

- update dependencies and use latest base64 - ([ccb1470](https://github.com/tyrchen/prost-helper/commit/ccb147004848b19e7387f0215393d91c0577d689)) - Tyr Chen

### Other

- (cargo-release) start next development iteration 0.4.2-alpha.0 - ([0834af1](https://github.com/tyrchen/prost-helper/commit/0834af1d93dcea73fe37d338f9bd4465c15f58dd)) - Tyr Chen
- bump the prost version - ([f7b78d5](https://github.com/tyrchen/prost-helper/commit/f7b78d55760b3d16d542f26e6e09ca6a22e315fd)) - Tyr Chen
- update documents (#3) - ([4f9ec28](https://github.com/tyrchen/prost-helper/commit/4f9ec2845064fec7bc48a1f72b8470914404fab6)) - Li Xiaobin
- typo (#6) - ([e1de445](https://github.com/tyrchen/prost-helper/commit/e1de4453b2ba6efcd426f79083757ca9411e79d1)) - Bryan Stenson
- update readme paths (#5)

Co-authored-by: Tyr Chen <tyr.chen@gmail.com> - ([4b9b83f](https://github.com/tyrchen/prost-helper/commit/4b9b83fc4277dc4452e6eab27816a7319fbaa6f5)) - Jacek Chmielewski
- Update CHANGELOG.md - ([3b51e35](https://github.com/tyrchen/prost-helper/commit/3b51e3524ec99dbfa90a5d2ab3ffc92610a2e4ca)) - Tyr Chen
- Update CHANGELOG.md - ([0b518f8](https://github.com/tyrchen/prost-helper/commit/0b518f88fe97dd5d8475ff7696d0bde95846850c)) - Tyr Chen

---
## [prost-build-config-v0.4.1](https://github.com/tyrchen/prost-helper/compare/prost-build-config-v0.4.0..prost-build-config-v0.4.1) - 2022-01-22

### Other

- (cargo-release) start next development iteration 0.4.1-alpha.0 - ([af8c4e7](https://github.com/tyrchen/prost-helper/commit/af8c4e726693a7eeaf86aae473ab6c4a8c321145)) - Tyr Chen
- expose service_generator interface - ([86ae9d5](https://github.com/tyrchen/prost-helper/commit/86ae9d5d89f23959f59a8a0dc759981f31f8d9c9)) - Tyr Chen

---
## [prost-build-config-v0.4.0] - 2022-01-22

### Bug Fixes

- fix cargo.toml for publishing - ([de5202c](https://github.com/tyrchen/prost-helper/commit/de5202cbf465187d03a3d4b82fce79ce9df1bce2)) - Tyr Chen
- fix category - ([e44c824](https://github.com/tyrchen/prost-helper/commit/e44c82402afb7126f90248028590ed5d9bd2cfb2)) - Tyr Chen
- fix github action - ([311f2d5](https://github.com/tyrchen/prost-helper/commit/311f2d53e259bb1873c75a7a184bedd848191011)) - Tyr Chen
- fix macro error - ([bf6fc13](https://github.com/tyrchen/prost-helper/commit/bf6fc13eca8c2eb24028f4c3993622626469af7b)) - Tyr Chen
- fix build issue - ([53add7a](https://github.com/tyrchen/prost-helper/commit/53add7a84fd76ca58c7d6777a385e60fdc1e03df)) - Tyr Chen

### Other

- init the project - ([0229102](https://github.com/tyrchen/prost-helper/commit/022910282285d6885fab8e348a66cd227364755d)) - Tyr Chen
- update doc - ([f19c22b](https://github.com/tyrchen/prost-helper/commit/f19c22b579b661e687435e2dfcd9a246a51046a6)) - Tyr Chen
- add github actions - ([d2e6795](https://github.com/tyrchen/prost-helper/commit/d2e67953a973ecc545dcf74680e833215715c40a)) - Tyr Chen
- bump prost-helper version - ([7ae4b0c](https://github.com/tyrchen/prost-helper/commit/7ae4b0c5f78cdddda0789ca6ef07c145d647d7c8)) - Tyr Chen
- use prost-helper in prost-serde - ([64c20db](https://github.com/tyrchen/prost-helper/commit/64c20db44d2e26f50ed36c7d33bdefb2656c66c0)) - Tyr Chen
- update documentation - ([2b466a9](https://github.com/tyrchen/prost-helper/commit/2b466a95b41d8c2846b566ea2ec0bf8c58d821db)) - Tyr Chen
- allow build_with_serde to return BuildConfig - ([83e4bf1](https://github.com/tyrchen/prost-helper/commit/83e4bf1558b635bf720d0655568a9a09589d2b4b)) - Tyr Chen
- support try_into for u8 - ([3d83f19](https://github.com/tyrchen/prost-helper/commit/3d83f1971811abd33a0dbe32985a565c09b5ee2f)) - Tyr Chen
- support to_json for prost message. - ([a312738](https://github.com/tyrchen/prost-helper/commit/a3127384c4f5157bc4483646795c2fd4cea0590c)) - Tyr Chen
- make macros support repetition - ([fde2d8f](https://github.com/tyrchen/prost-helper/commit/fde2d8f185c1fe047f92a1eb39ee4803d52f917b)) - Tyr Chen
- support serde Vec<u8> to base64 - ([4c5fce6](https://github.com/tyrchen/prost-helper/commit/4c5fce6d78bab62dad09ba0db67fea48202ed02f)) - Tyr Chen
- add test for enum support - ([709b250](https://github.com/tyrchen/prost-helper/commit/709b25087be2694a6e4585f620e10f2d188e5c03)) - Tyr Chen
- support bytes and treemap - ([5de8a6a](https://github.com/tyrchen/prost-helper/commit/5de8a6af96323b277fa85016474465fb8d17f484)) - Tyr Chen
- support message into/try_from with bytes - ([a20f699](https://github.com/tyrchen/prost-helper/commit/a20f6994cb3b57f6bdf456db41a8c6db1c6eb396)) - Tyr Chen
- update to prost 0.9 - ([7dab63d](https://github.com/tyrchen/prost-helper/commit/7dab63dd3b25cf0bca70a8b489877236b044ffc2)) - Danni Moiseyev
- add github workflow - ([0f3ef96](https://github.com/tyrchen/prost-helper/commit/0f3ef96f6ada6f20a0cf5a9d5e0fe024c4f04b4c)) - Tyr Chen
- Merge pull request #2 from danni-m/master

Update prost dependency to 0.9 - ([c03f8c4](https://github.com/tyrchen/prost-helper/commit/c03f8c4e87ea478eff7b5a4d7a53dd74ef333859)) - Tyr Chen
- output_dir defaults to Cargo OUT_DIR (#1) - ([a8c2ab0](https://github.com/tyrchen/prost-helper/commit/a8c2ab0254c5f094feae69853f22251c449a25d4)) - graelo
- bump version for prost-helper to 0.6.0 - ([d35a915](https://github.com/tyrchen/prost-helper/commit/d35a915829ee764773ecc86a967c7596947d1c08)) - Tyr Chen
- bump version for prost-serde to 0.3.0 - ([edc4eb4](https://github.com/tyrchen/prost-helper/commit/edc4eb4fd5a9851f378c319383be39e962549d4d)) - Tyr Chen
- update README.md - ([d807655](https://github.com/tyrchen/prost-helper/commit/d807655687de0dd7f1fd323a063f990722ef0778)) - Tyr Chen
- rename prost-serde to prost-build-config and change the config to yaml
format to simplify it. - ([8740ec7](https://github.com/tyrchen/prost-helper/commit/8740ec72d9016fa5b2f9c5bd01ff26fa49470c17)) - Tyr Chen

<!-- generated by git-cliff -->
