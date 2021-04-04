# Changelog

## 0.4.1 (unreleased)

### Added

* `CredentialsFileStorer::store_many` stores multiple profiles. [#11]

[#11]: https://github.com/azriel91/credent/pull/11

## 0.4.0 (2021-03-21)

### Added

* Support switching between `"backend-smol"` and `"backend-tokio"`. [#9]
* `CredentialsCliReader` arbitrary prompt support via `prompt_plain_text` and `prompt_secret`. [#10]

[#9]: https://github.com/azriel91/credent/pull/9
[#10]: https://github.com/azriel91/credent/pull/10

## 0.3.0 (2021-03-20)

### Changed

* `credent_model::Profile` is now type parameterized. ([#8])
* `credent_fs_model::Error` underlying errors are renamed to `error`. ([#8])

[#8]: https://github.com/azriel91/credent/pull/8

## 0.2.1 (2021-03-20)

### Changed

* `credent_fs::AppName` is moved to `credent_fs_model::AppName`. ([#7])

[#7]: https://github.com/azriel91/credent/pull/7

## 0.2.0 (2021-03-20)

### Added

* `credent_fs::Error` type that better indicates what error occurs. ([#6])

[#6]: https://github.com/azriel91/credent/pull/6

## 0.1.0 (2020-11-05)

### Added

* Support for loading credentials from command line.
* Support for loading and storing credentials from a file.
* Store credentials encoded as base64.
* Support for loading and storing credentials per profile. ([#1])

[#1]: https://github.com/azriel91/credent/pull/1
