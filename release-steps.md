# Updates

This document describes steps to follow to deploy a new version on Flathub.

## App version

The app version must be updated in two files:

1. [Cargo.toml](Cargo.toml)
2. [meson.build](meson.build)

## Release notes

Add release notes on the [meta info](dev.deimoshall.Metamorphosis.metainfo.xml.in.in) file.

## Screenshots

Update app screenshots if required.

> New screenshots or modification of existing ones must be added/edited in the [meta info](dev.deimoshall.Metamorphosis.metainfo.xml.in.in) file.

## Exiftool

1. Verify the latest `exiftool` version here: [https://exiftool.org/](https://exiftool.org/)
2. Update the version number in the url field on the [manifest](dev.deimoshall.Metamorphosis.json) file.
3. Update the sha256 field, check the corresponding the the `exiftool` version here: [https://exiftool.org/checksums.txt](https://exiftool.org/checksums.txt)

## GitHub release

When releasing a new version for flathub, I must create a source tarball that includes the vendored Rust dependencies:

2. Vendor the dependencies and setup the build directory:

```bash
meson setup build --reconfigure
```

3. Create the distribution tarball with vendored deps (takes a couple of minutes):

```bash
meson dist -C build --allow-dirty --no-tests
```

> This generates a tarball in `build/meson-dist/` named `metamorphosis-X.Y.Z.tar.xz`

6. Upload this tarball to the GitHub release

> Flatpak builds run in offline mode, so cargo must be able to find all dependencies locally in the `vendor/` directory that's bundled in the tarball. The project's `build-aux/dist-vendor.sh` script automatically handles vendoring during distribution.

## Deploy steps

After tagging the release, update the flathub manifest:

1. Go to my flathub repository:

```bash
cd ~/Projects/infra/dev.deimoshall.Metamorphosis
```

1. Update the `url` in the metamorphosis module to point to my new release tarball

2. Update the `sha256` with the value from the file created by step 3

3. Commit and open a PR
