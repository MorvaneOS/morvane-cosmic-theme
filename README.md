# morvane-cosmic-theme

The MorvaneOS look for Cosmic: a dark theme with Pastel Pink accents and
purple-tinted surfaces, candy-icons and the Twilight wallpaper, as Cosmic's defaults.

## How it works

The theme is computed by **Cosmic's own theme code**, the same code Cosmic Settings
uses when you pick colours on its Appearance page, from four MorvaneOS colours:
Pastel Pink as the accent, `#1A1220` as the background, Lavender as the tint for
Cosmic's greys and Paper for its text. So every shade Cosmic derives (hover, pressed,
text on the accent and so on) follows Cosmic's own rules. `tools/compute-theme` is a
small Rust program around `cosmic-theme`, pinned to the libcosmic commit Cosmic
Settings 1.8.0 is built with.

Cosmic looks for its defaults in each XDG data directory in turn and takes the first
that has a setting's whole folder. Its own are in `/usr/share/cosmic`, owned by its
packages, so this package installs the MorvaneOS ones in
`/usr/share/morvaneos/cosmic/` and `/etc/profile.d/morvane-cosmic-theme.sh` puts
that directory first in `XDG_DATA_DIRS`:

- `com.system76.CosmicTheme.Dark` and `.Dark.Builder`: the theme
- `com.system76.CosmicBackground`: Cosmic's wallpaper settings with the Twilight
  wallpaper. Cosmic's own copy writes booleans as `#true`, which cosmic-bg can't
  read, so it silently falls back to its built-in wallpaper; this copy writes
  `true`.
- `com.system76.CosmicTk`: only the icon theme, candy-icons

Anything set in Cosmic Settings still wins. The MorvaneOS installer installs this
package with Cosmic.

## Changing it

Edit the colours in `tools/build.py`, then run it with the root to read Cosmic's own
defaults from (`/` on a system with Cosmic installed):

```
python tools/build.py [root]
```

Needs Python and Rust (`cargo`). When Cosmic updates, update the libcosmic commit in
`tools/compute-theme/Cargo.toml` to the one in that Cosmic Settings release's
`Cargo.lock`, and rebuild. Commit the output, bump `pkgver`, build and publish.

## License

LGPL-2.1-or-later.
