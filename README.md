
# About

`totalmapper` is a simple utility for remapping keys using the Linux event handling system.

It is more flexible than tools like `xmodmap` and `xkb` in that it lets you use any key as a modifier, enabling more complex layouts than can be achieved with the usual combination of alts, shifts, and controls.

# Installation

## From source

With [`cargo`](https://doc.rust-lang.org/cargo/):

    cargo build --release
    sudo cp ./target/release/totalmapper /usr/bin

## Packages

* Ubuntu: [`totalmapper_1.0.deb`](https://github.com/ellbur/totalmapper/releases/download/1.0/totalmapper_1.0.deb)

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

The names of keys are taken from [the Linux header](https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h), minus the `KEY_` prefix.

You can use any key as a modifier. You don't have to tell totalmapper which keys are modifiers; simply creating a mapping that uses the key in combination with another makes it act like a modifier.

But be careful that if you want to use a key as a modifier that normally has another function, you will want to map the key by itself to `[]`, as in the example above.

