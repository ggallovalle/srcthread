# vX.Y.Z — short title for the releases list

<!--
Copy to release-notes/<version>.md and fill in before publishing.
Inspired by jdx/mise and jdx/usage release notes: short prose lead-in,
then Added / Changed / Fixed with bold one-line titles and PR links.
-->

One or two sentences summarizing the release for someone scanning the
GitHub releases page. Lead with the user-facing outcome, not the
implementation.

## Added

- **Short bold title of the feature.** One or two sentences on what
  changed and why it matters. Optional tiny example:

  ```sh
  srcthread integration install cursor --user
  ```

  [#N](https://github.com/ggallovalle/srcthread/pull/N)

## Changed

- **Short bold title.** What behavior or defaults moved, and who should
  care. [#N](https://github.com/ggallovalle/srcthread/pull/N)

## Fixed

- **Short bold title.** What was broken and what happens now.
  [#N](https://github.com/ggallovalle/srcthread/pull/N)

## Install

PC via packslip:

```toml
[tools]
"packslip:github.com/ggallovalle/srcthread" = {
  version = "X.Y.Z",
  variant = "desktop",
}
```

Android: download `srcthread-X.Y.Z-android.apk` from this release.

**Full Changelog**:
<https://github.com/ggallovalle/srcthread/compare/vPREV...vX.Y.Z>
