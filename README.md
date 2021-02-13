
# About

`totalmapper` is a simple utility for remapping keys using the Linux event handling system.

It is more flexible than tools like `xmodmap` and `xkb` in that it lets you use any key as a modifier, enabling more complex layouts than can be achieved with the usual combination of alts, shifts, and controls.

# Installation

## From source

With [`cargo`](https://doc.rust-lang.org/cargo/):

    cargo build --release
    sudo cp ./target/release/totalmapper /usr/bin

## Packages

* Ubuntu amd64: [`totalmapper_1.1.1_amd64.deb`](https://github.com/ellbur/totalmapper/releases/download/v1.1.1/totalmapper_1.1.1_amd64.deb)
* Self-contained Linux amd64 (useful for Chrome OS): [`totalmapper-static-linux-amd64-1.1.1.tar.gz`](https://github.com/ellbur/totalmapper/releases/download/v1.1.1/totalmapper-static-linux-amd64-1.1.1.tar.gz)

# Running

## Remapping your keyboard

Try one of the builtin layouts with:

    totalmapper remap --default-layout caps-for-movement --all-keyboards

See the list of builtin layouts with:

    totalmapper list_default_layouts

See the JSON source for a builtin layout with:

    totalmapper print_default_layout caps-for-movement

Define your own layout (see below) and remap your keyboard with:

    totalmapper remap --layout-file my-layout.json --all-keyboards

# Running automatically

    sudo totalmapper add_systemd_service

# Defining layouts

Layouts are defined with a simple JSON syntax:

```json
[
  { "from": [ "CAPSLOCK" ], "to": [] },
  { "from": [ "CAPSLOCK", "J" ], "to": [ "LEFT" ] },
  { "from": [ "CAPSLOCK", "I" ], "to": [ "UP" ] },
  { "from": [ "CAPSLOCK", "K" ], "to": [ "DOWN" ] },
  { "from": [ "CAPSLOCK", "L" ], "to": [ "RIGHT" ] },
  { "from": [ "CAPSLOCK", "H" ], "to": [ "HOME" ] },
  { "from": [ "CAPSLOCK", "SEMICOLON" ], "to": [ "END" ] },
  { "from": [ "CAPSLOCK", "U" ], "to": [ "PAGEUP" ] },
  { "from": [ "CAPSLOCK", "M" ], "to": [ "PAGEDOWN" ] },
  { "from": [ "CAPSLOCK", "N" ], "to": [ "LEFTCTRL", "LEFT" ] },
  { "from": [ "CAPSLOCK", "COMMA" ], "to": [ "LEFTCTRL", "RIGHT" ] }
]
```

The names of keys are taken from [the Linux header](https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h), minus the `KEY_` prefix.

You can use any key as a modifier. You don't have to tell totalmapper which keys are modifiers; simply creating a mapping that uses the key in combination with another makes it act like a modifier.

But be careful that if you want to use a key as a modifier that normally has another function, you will want to map the key by itself to `[]`, as in the example above.

# On Chrome OS

The self-contained package will run on Intel chromebooks in developer mode. There is no need to install crouton. The binary must be copied to a filesystem that allows code execution, such as `/usr/local/bin`.

The `chronos` user is part of the `input` group but not the `uinput` group. You can fix this problem with:

```bash
sudo chown root:input /dev/uinput
```

On some devices, the remapped keyboard will not automatically disable in tablet mode, which is annoying. Use the `--tablet-mode-switch-device` option to have totalmapper read the tablet mode switch device and turn itself off:

    totalmapper remap --dev-file /dev/input/event2 --tablet-mode-switch-device /dev/input/event5 --default-layout caps-for-movement

You can test devices under `/dev/input` with `evtest` to find your tablet mode switch.

