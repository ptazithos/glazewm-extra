# GlazeWM Extra

## Description

This repository contains additional features for the Glaze Window Manager (glazewm) based on `ipc` and `windows api`. 

Currently, it provides support for translucent unfocused windows.

![Demo](assets/demo.gif)

## Installation
To install, use cargo:
```shell
cargo install glazewm-extra
```

## Usage

After installing the application, run it with an alpha argument:
```shell
# Unfocused windows will have a translucent appearance with an alpha value of 128/255.
glazewm-extra --alpha 128
```

## License

This repository is licensed under the [MIT License](LICENSE).


