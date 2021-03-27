
# About

`totalmapper` is a simple utility for remapping keys using the Linux event handling system.

It is more flexible than tools like `xmodmap` and `xkb` in that it lets you use any key as a modifier, enabling more complex layouts than can be achieved with the usual combination of alts, shifts, and controls.

## Features

* Use any key as a modifier
* Use any number of modifiers
* Write layouts in a simple JSON syntax that is easy to generate programmatically
* Run it on any Linux platform, including Chrome OS (in developer mode)
* Run it on X and Wayland and with all window managers and GUI frameworks
* Use a consistent layout across remote desktops and virtual machines
* Change repeat behavior per-key (e.g., disable repeat or repeat with a different code than the initial press)
* Prevent TYping LIke THis by making Shift only apply to one key

# Installation

## Packages

* Ubuntu amd64:
    * [`totalmapper_1.3.4-focal_amd64.deb`](https://github.com/ellbur/totalmapper/releases/download/v1.3.4/totalmapper_1.3.4-focal_amd64.deb)
    * [`totalmapper_1.3.4-groovy_amd64.deb`](https://github.com/ellbur/totalmapper/releases/download/v1.3.4/totalmapper_1.3.4-groovy_amd64.deb)
* Self-contained Linux binaries (useful for Chrome OS):
    * [`totalmapper-static-linux-amd64-1.3.4.tar.gz`](https://github.com/ellbur/totalmapper/releases/download/v1.3.4/totalmapper-static-linux-amd64-1.3.4.tar.gz)
    * [`totalmapper-static-linux-aarch64-1.3.4.tar.gz`](https://github.com/ellbur/totalmapper/releases/download/v1.3.4/totalmapper-static-linux-aarch64-1.3.4.tar.gz)

## From source

With [`cargo`](https://doc.rust-lang.org/cargo/):

    cargo build --release
    sudo cp ./target/release/totalmapper /usr/bin

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

If your system uses `systemd`, you can add a `udev` rule that will automatically run `totalmapper` whenever a new keyboard is plugged in:

    sudo totalmapper add_systemd_service --default-layout caps-for-movement

# Defining layouts

Layouts are defined with a simple JSON syntax:

```json
{
  "mappings": [
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
}
```

The names of keys are taken from [the Linux header](https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h), minus the `KEY_` prefix.

You can use any key as a modifier. You don't have to tell totalmapper which keys are modifiers; simply creating a mapping that uses the key in combination with another makes it act like a modifier.

Be careful that if you want to use a key as a modifier that normally has another function, you will want to map the key by itself to `[]`, as in the example above.

## Remapping

A basic mapping maps some combination of keys to another combination of keys:

```json
{ "from" : ["LEFTCTRL", "C"], "to": ["LEFTALT", "1"] }
```

The above mapping will be triggered when the user presses the left control key and then taps the 'C' key, and will make it as if the left alt were pressed while tapping '1'.

## Custom repeat

### Disabling repeat

You can use `totalmapper` to make specific keys or combinations of keys not repeat:

```json
{ "from": [ "RIGHTSHIFT", "SLASH" ], "to": [ "LEFTSHIFT", "Z" ], "repeat": "Disabled" }
```

### Changing the repeat code

You can make a key repeat with a different code than the initial press:

```json
{ "from": [ "SEMICOLON" ], "to": [ "S" ], "repeat": { "Special": { "keys": ["F21"], "delay_ms": 180, "interval_ms": 30 } } }
```

This will cause the first press of the <kbd>;</kbd> key to generate the code for <kbd>S</kbd>, but, if held down, the repeat code will be <kbd>F21</kbd>. This can be used to make a key that repeats in some apps but not others by configuring how those apps treat the repeat code. I personally use it to make Vim movement letters (h, j, k, l) only repeat in Vim normal mode.  

## Preventing extra Shift

If you're like me, you have a tendancy to HOld DOwn SHift TOo LOong, resulting in WOrds LIke THis. `totalmapper` can be used to make a modifier only apply to a single key stroke:

```json
 { "from": [ "LEFTSHIFT", "L" ], "to": [ "LEFTSHIFT", "N" ], "absorbing": [ "LEFTSHIFT" ] }
```

The `absorbing` option tells `totalmapper` that after it applies this mapping, it should "absorb" the `LEFTSHIFT` modifier so that it is not used for any subsequent keypresses.

# On Chrome OS

The self-contained packages will run on Intel or ARM chromebooks in developer mode. There is no need to install crouton. The binary must be copied to a filesystem that allows code execution, such as `/usr/local/bin`.

The `chronos` user is part of the `input` group but not the `uinput` group. You can fix this problem with:

```bash
sudo chown root:input /dev/uinput
```

On some devices, the remapped keyboard will not automatically disable in tablet mode, which is annoying. Use the `--tablet-mode-switch-device` option to have totalmapper read the tablet mode switch device and turn itself off:

    totalmapper remap --dev-file /dev/input/event2 --tablet-mode-switch-device /dev/input/event5 --default-layout caps-for-movement

You can inspect devices under `/dev/input` with `evtest` to find your tablet mode switch.
