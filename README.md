# GlazeWM Extra

Additional features for the Glaze Window Manager (glazewm). 

#### Translucent windows
![transparency](assets/transparency.gif)

#### Hide Title Bar
![hide_titlebar](assets/hide_titlebar.png)

#### Disable Rounded Corner
![disable_rounded_corner](assets/rounded_corner.png)

## Installation

Make sure [VCRuntime140](https://www.microsoft.com/en-us/download/details.aspx?id=48145) is installed before running.

#### Direct Download
The latest build can be found [here](https://github.com/ptazithos/glazewm-extra/releases/).  

#### Scoop

```cmd
scoop install https://github.com/ptazithos/glazewm-extra/releases/latest/download/glazewm-extra.json
```

## Configuration
The config file located at `~/.config/glazewm-extra.toml`. The default config is as follows:

```toml
# Rules applied for every windows once the window creates.
[[window_rules]]
command = "set title false"
match_process_name = ".*"

[[window_rules]]
command = "set rounded false"
match_process_name = ".*"

# Rules applied for the focused window when focus changes.
[[focused_window_rules]]
command = "set translucent 255"
match_process_name = ".*"

# Rule applied for unfocused windows when focus change.
[[unfocused_window_rules]]
command = "set translucent 220"
match_process_name = ".*"
```
Except `match_process_name`, `match_class_name` and `match_title` are also available.

## License

This repository is licensed under the [MIT License](LICENSE).


