# Updates

This document describes steps to follow to update the app version and its dependencies

## App version

TODO

```bash
curl -sL "https://github.com/DeimosHall/Metamorphosis/archive/refs/tags/v0.1.0.tar.gz" | sha256sum
```

## Exiftool

1. Verify the latest `exiftool` version here: [https://exiftool.org/](https://exiftool.org/)
2. Update the version number in the url field on the [dev.deimoshall.Metamorphosis.js](dev.deimoshall.Metamorphosis) file
3. Updte the sha256 field, check the corresponding the the `exiftool` version here: [https://exiftool.org/checksums.txt](https://exiftool.org/checksums.txt)

## Flatpak release

When releasing a new version for flathub, I must create a source tarball that includes the vendored Rust dependencies:

1. Vendor the Rust dependencies (if not already done):

```bash
cargo vendor
```

2. Setup the build directory (if not already done):

```bash
meson setup build --reconfigure
```

3. Create the distribution tarball with vendored deps:

```bash
meson dist -C build --allow-dirty --no-tests
```

4. This generates a tarball in `build/meson-dist/` named `metamorphosis-X.Y.Z.tar.xz`

5. Compute the SHA256 of this vendored tarball:

```bash
sha256sum build/meson-dist/metamorphosis-X.Y.Z.tar.xz
```

6. Upload this tarball to your GitHub release

**Why this is needed:** Flatpak builds run in offline mode, so cargo must be able to find all dependencies locally in the `vendor/` directory that's bundled in the tarball. The project's `build-aux/dist-vendor.sh` script automatically handles vendoring during distribution.

**Updating the manifest:** After tagging the release, update the flathub manifest:

1. Update the `url` in the metamorphosis module to point to my new release tarball
2. Update the `sha256` with the value computed in step 5
3. Commit to my flathub fork and open a PR
