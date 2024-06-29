# GlazeWM Extra

Additional features for the Glaze Window Manager (glazewm). 
Currently, it provides support for `translucent unfocused windows` and `hide title bar`.

#### Translucent windows
![transparency](assets/transparency.gif)

#### Hide Title Bar
![hide_titlebar](assets/hide_titlebar.png)

## Installation
To install, use cargo:
```shell
cargo install glazewm-extra
```

## Usage

After installing the application, run it with an alpha argument:
```shell
# Unfocused windows will have a translucent appearance with an alpha value of 128/255.
# All windows' title bar will be hidden except those who draw title bar by themself like Google Chrome
glazewm-extra --enable-transparency --hide-titlebar --alpha 128
```

## License

This repository is licensed under the [MIT License](LICENSE).


