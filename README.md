# rosu-np

Simple self-host twitch chat bot for osu! streamers

> ⚠ ***Requires [tosu](https://tosu.app/)!***

## Available commands

* `!np` - show current beatmap
* `!pp` - show PP for current beatmap (+ with mods)
* `!skin` - show current skin

## How to setup

1. Place [`rosu-np.exe`](https://github.com/uzervlad/rosu-np/releases/latest) into a separate folder

2. Run the executable

  * The program will ask you to authorize through Twitch OAuth

## Config example

```ron
(
  username: "username",         // Twitch username
  token: "qwertyasdfgh123456",  // OAuth token (don't touch)
  source: Tosu,                 // osu! data source (currently only Tosu)
  timeout: 5                    // Command timeout in seconds
)
```