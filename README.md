# rosu-np

Simple self-host twitch chat bot for osu! streamers

> ⚠ ***Requires StreamCompanion!***

## Available commands

* `!np` - show current beatmap
* `!skin` - show current skin

## How to setup

1. Place [`rosu-np.exe`](https://github.com/uzervlad/rosu-np/releases/latest) into a separate folder

2. Create `config.json` in the same folder and fill it with [your configuration](#config)

## Config

```json
{
  "username": "username",         // Twitch username
  "token": "qwertyasdfgh123456",  // OAuth token (see below)
  "timeout": 5                    // Command timeout in seconds
}
```

## Obtaining an OAuth token

1. Open [this page](https://id.twitch.tv/oauth2/authorize?response_type=token&client_id=ci2s72rvzqny52t3sn1fdxd4vaa8uc&redirect_uri=http://localhost:9727&scope=chat%3Aread+chat%3Aedit) and confirm

2. In your address bar find `#access_token=<token>&scope...` and copy the `<token>`
