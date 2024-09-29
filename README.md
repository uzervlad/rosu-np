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

## Customize commands

You can customize replies to existing commands in the config (`templates`), as described below

You can also add your own custom commands to `templates`

Default replies can be found [here](src/config.rs)

<details>
  <summary>Supported tokens</summary>
  
  - `artist`
  - `title`
  - `version` - difficulty name
  - `creator`
  - `mods` - has a `+` in front when mods are selected
  - `skin`
  - `map_id`
  - `link` - beatmap link, empty when `map_id` is 0
  - `pp_98`
  - `pp_99`
  - `pp_ss`
  - `gamemode` - `osu`/`taiko`/`catch`/`mania`
</details>

## Config example

```ron
(
  username: "username",         // Twitch username
  token: "qwertyasdfgh123456",  // OAuth token (don't touch)
  channel: "other_username",    // [Optional] Twitch channel to join
  source: Tosu,                 // osu! data source (currently only Tosu)
  timeout: 5                    // Command timeout in seconds (default: 5)
  templates: {                  // [Optional] Customizable reply templates
    "np": "{artist} - {title} [{version}] by {creator} {link}",
    "pp": "PP {mods} (98/99/100): {pp_98}/{pp_99}/{pp_ss}",
    "skin": "Skin: {skin}",
    "test": "{map_id}"          // example custom command
  }
)
```