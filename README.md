<div align="center">
<h1>Metamorphosis</h1>

Edit EXIF metadata.

<img src="data/resources/icons/hicolor/scalable/apps/io.dev.deimoshall.Metamorphosis.svg" width="128" height="128" alt="Metamorphosis icon">

[![Installs](https://img.shields.io/flathub/downloads/io.dev.deimoshall.Metamorphosis?style=for-the-badge)](https://flathub.org/apps/details/io.dev.deimoshall.Metamorphosis)
[![Latest Tag](https://img.shields.io/gitlab/v/tag/deimoshall/Metamorphosis?sort=date&style=for-the-badge)](https://gitlab.com/deimoshall/Metamorphosis/-/tags)
[![License](https://img.shields.io/gitlab/license/deimoshall/Metamorphosis?style=for-the-badge)](https://gitlab.com/deimoshall/Metamorphosis/-/raw/main/COPYING)


</div>

## Installation
<a href='https://flathub.org/apps/details/io.dev.deimoshall.Metamorphosis'><img width='240' alt='Download on Flathub' src='https://flathub.org/api/badge?svg&locale=en'/></a>

## About

Metamorphosis is designed to help you edit exif metadata in a simple and reliable way.

<img src="data/resources/screenshots/0.png" alt="View of Metamorphosis">

Metamorphosis supports editing the following datatypes:

- [x] **Images:**
  - [x] JPEG, JPG, JPE
  - [ ] PNG, JNG, MNG
  - [ ] TIFF, TIF
  - [ ] GIF
  - [ ] BMP, DIB
  - [ ] HEIC, HEIF, HIF
  - [ ] AVIF
  - [ ] WebP
  - [ ] SVG
  - [ ] PSD, PSB
  - [ ] ICO, CUR
- [ ] **RAW formats:** CR2, CR3, NEF, ARW, RAF, ORF, RW2, DNG, and more
- [ ] **Videos:** MOV, MP4, M4V, AVI, WebM
- [ ] **Audio:** MP3, M4A, FLAC, OGG
- [ ] **Documents:** PDF
- [ ] **XMP sidecar** files

You can also drag and drop as well as paste the copied image into the app!

## Contributing

Issues and merge requests are more than welcome. However, please take the following into consideration:

- This project follows the [GNOME Code of Conduct](https://wiki.gnome.org/Foundation/CodeOfConduct)
- Only Flatpak is supported

## Development

### GNOME Builder

The recommended method is to use GNOME Builder:

1. Install [GNOME Builder](https://apps.gnome.org/app/org.gnome.Builder/) from Flathub
2. Open Builder and select "Clone Repository..."
3. Clone `https://github.com/DeimosHall/Metamorphosis.git` (or your fork)
4. Press "Run Project" (▶) at the top, or `Ctrl`+`Shift`+`[Spacebar]`.

### Flatpak

You can install Metamorphosis from the latest commit:

1. Install [`org.flatpak.Builder`](https://github.com/flathub/org.flatpak.Builder) from Flathub
2. Clone `https://github.com/DeimosHall/Metamorphosis.git` (or your fork)
3. Run `flatpak run org.flatpak.Builder --install --user --force-clean build-dir io.dev.deimoshall.Metamorphosis.json` in the terminal from the root of the repository.

### Meson

You can build and install on your host system by directly using the Meson buildsystem:

1. Install `blueprint-compiler`
2. Run the following commands (with `/usr` prefix):

```
meson --prefix=/usr build
ninja -C build
sudo ninja -C build install
```

## Credits

Actively developed by Deimos Hall.

Based on [`Switcheroo`](https://gitlab.com/adhami3310/Switcheroo.git) by Khaleel Al-Adhami, an app to convert and manipulate images.

This app uses a variety of libraries but the main one is [ImageMagick](https://github.com/ImageMagick/ImageMagick), which without their generous contribution to the open source community this would be nearly impossible to make.
