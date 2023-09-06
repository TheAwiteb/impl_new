<!-- 
Changelog file, to keep track of changes to the project.
-->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic Versioning].

## [Unreleased]

### Added
<!-- This section is for new features and enhancements to existing features. -->
<!-- Format: `- {The feature or enhancement title}. ([#{PR number}]({PR link}))` -->

### Changed
<!-- This section is for changes in existing functionality. -->
<!-- Format: `- {The change title}. ([#{PR number}]({PR link}))` -->

### Deprecated
<!-- This section is for once-stable features removed in upcoming releases. -->
<!-- Format: `- {The deprecation title}. ([#{#PR number}]({PR link}))` -->

### Removed
<!-- This section is for deprecated features removed in this release. -->
<!-- Format: `- {The removal title}. ([#{PR number}]({PR link}))` -->

<!-- ### Fixed -->
<!-- This section is for any bug fixes. -->
<!-- Format: `- {The bug which was fixed title}. ([#{PR number}]({PR link}))` -->


## [0.2.0] - 2023-09-06

### Added
- Add `default` flag option to the `impl_new` attribute that remove the field from the `new` function arguments and use the default value of the field type instead. ([#4](https://github.com/theawiteb/impl_new/pull/4))
- Add `value` option to the `impl_new` attribute that set the field value to the given value. ([#6](https://github.com/theawiteb/impl_new/pull/6))

## [0.1.0] - 2023-08-30
- Initial release.

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html