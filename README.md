# GlazeWM Extra

Additional features for the Glaze Window Manager (glazewm). 

#### Translucent windows
![transparency](assets/transparency.gif)

#### Hide Title Bar
![hide_titlebar](assets/hide_titlebar.png)

#### Workspaces Preview
After glazewm-extra starts, run `glazewm-extra.exe workspaces` to trigger the worksapces preview.
![workspace_preview](assets/workspace_preview.png)

## Installation

#### Direct Download
The latest build can be found [here](https://github.com/ptazithos/glazewm-extra/releases/).  
Make sure [Webview2](https://developer.microsoft.com/en-us/microsoft-edge/webview2) and [VCRuntime140](https://www.microsoft.com/en-us/download/details.aspx?id=48145) are installed before running.

## Configuration
The config file located at "${UserFolder}/.config/glazewm-extra.toml". The default config is as follows:

```toml
[translucent_window]
enable = true
alpha = 220


[title_bar]
# True for hiding all titlebars
enable = true
```

## License

This repository is licensed under the [MIT License](LICENSE).


