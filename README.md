
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
    * [`totalmapper_1.4.3-focal_amd64.deb`](https://github.com/ellbur/totalmapper/releases/download/v1.4.3/totalmapper_1.4.3-focal_amd64.deb)
    * [`totalmapper_1.4.3-jammy_amd64.deb`](https://github.com/ellbur/totalmapper/releases/download/v1.4.3/totalmapper_1.4.3-jammy_amd64.deb)
    * [`totalmapper_1.4.3-kinetic_amd64.deb`](https://github.com/ellbur/totalmapper/releases/download/v1.4.3/totalmapper_1.4.3-kinetic_amd64.deb)
* Self-contained Linux binaries (useful for Chrome OS):
    * [`totalmapper-static-linux-amd64-1.4.3.tar.gz`](https://github.com/ellbur/totalmapper/releases/download/v1.4.3/totalmapper-static-linux-amd64-1.4.3.tar.gz)
    * [`totalmapper-static-linux-aarch64-1.4.3.tar.gz`](https://github.com/ellbur/totalmapper/releases/download/v1.4.3/totalmapper-static-linux-aarch64-1.4.3.tar.gz)

## From source

With [`cargo`](https://doc.rust-lang.org/cargo/):

    cargo build --release
    sudo cp ./target/release/totalmapper /usr/bin

# Running

## Check Permissions

To run `totalmapper` manually, make sure you have write permissions to `/dev/uinput`. Below is an example of correct permissions:

```sh
$ ls -l /dev/uinput
crw-rw----+ 1 root input 10, 223 Mar 12 22:37 /dev/uinput

$ groups
sys network power lp input
```

The `add_systemd_service` command, discussed below, will automatically create a user with the correct permissions.

## Remapping your keyboard

Try one of the builtin layouts with:

```sh
totalmapper remap --default-layout caps-for-movement --all-keyboards
```

See the list of builtin layouts with:

```sh
totalmapper list_default_layouts
```

See the JSON source for a builtin layout with:

```sh
totalmapper print_default_layout caps-for-movement
```

Define your own layout (see below) and remap your keyboard with:

```sh
totalmapper remap --layout-file my-layout.json --all-keyboards
```

# Running automatically

## systemd Service

If your system uses `systemd`, you can add a `udev` rule that will automatically run `totalmapper` whenever a new keyboard is plugged in:

```sh
sudo totalmapper add_systemd_service --default-layout caps-for-movement
```

This will install a service definition in `/etc/systemd/system/totalmapper@.service` that will run `totalmapper` under a new user (`totalmapper`) in the `input` group.

If you have a keyboard that you do not want to be remapped, you can exclude it with the `--exclude` option, which takes a glob-like pattern. First, use `totalmapper list_keyboards` to find the name of the keyboard you want to exclude:

```sh
$ totalmapper list_keyboards
AT Translated Set 2 keyboard: /dev/input/event4
```

Then, use the `--exclude` option to exclude it:

```sh
sudo totalmapper add_systemd_service --default-layout caps-for-movement --exclude 'AT Translated Set 2 keyboard'
```

## Without systemd

If your system does not use `systemd` (such as Chrome OS), you can have `totalmapper` monitor for new keyboards itself:

```sh
totalmapper remap --default-layout caps-for-movement --auto-all-keyboards
```

# Defining layouts

## Examples

Examples can be found using `totalmapper list_default_layouts` and `totalmapper print_default_layout <name>`.

## Basic Structure

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

The names of keys are taken from [the Linux header](https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h), minus the `KEY_` prefix. Some keys you may not expect are:

* The backtick/tilde key is called `“GRAVE”`.
* The left and right “windows” keys are `“LEFTMETA”` and `“RIGHTMETA”`.
* The period is `“DOT”`.
* The single quote is `“APOSTROPHE”`.

You can use any key as a modifier. You don't have to tell totalmapper which keys are modifiers; simply creating a mapping that uses the key in combination with another makes it act like a modifier.

Be careful that if you want to use a key as a modifier that normally has another function, you will want to map the key by itself to `[]`, as in the example above.

## Key names on non-QWERTY keyboards

The key names used to define JSON mappings correspond to kernel constants for keycodes. Typically, these names correspond to a physical QWERTY layout even if the labels on your keyboard are not QWERTY and even if you have set a non-QWERTY layout in X or Wayland.

Because of this, JSON mappings usually *must* be defined using labels that correspond to a QWERTY layout regardless of which layout you have configured (e.g. Coleman, Dvorak).

For example, the following mapping would trigger when pressing the key that maps to the character `H` in a Dvorak layout ([keycode 36](https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h#L111)) because it maps to the character `J` in a QWERTY layout:

```json
{ "from": [ "J" ], "to": [ "DOWN" ] }
```

To figure out what keycodes your keyboard uses, you may use `evtest`.

## Remapping

A basic mapping maps some combination of keys to another combination of keys:

```json
{ "from" : ["LEFTCTRL", "C"], "to": ["LEFTALT", "1"] }
```

The above mapping will be triggered when the user presses the left control key and then taps the 'C' key, and will make it as if the left alt were pressed while tapping '1'.

## Shorthands

As of version 1.4, `totalmapper` supports shorthands for common remapping situations.

### Remapping an Entire Row

You can remap an entire row of keys using the shorthand `{ "from": {"row": <rowname>}, "to": {"letters": <letters>} }`. For example:

```json
{
  "mappings": [
    { "from": {"row": "A"}, "to": {"letters": "aoeu"} }
  ]
}
```

The above example is equivalent to the following individual mappings:

```json
{
  "mappings": [
    { "from": "A", "to": "A" },
    { "from": "S", "to": "O" },
    { "from": "D", "to": "E" },
    { "from": "F", "to": "U" }
  ]
}
```

This shorthand can be combined with modifiers like so:

```json
{
  "mappings": [
    { "from": ["CAPSLOCK", {"row": "A"}], "to": {"letters": "=+-"} }
  ]
}
```

This is equivalent to:

```json
{
  "mappings": [
    { "from": ["CAPSLOCK", "A"], "to": "EQUAL" },
    { "from": ["CAPSLOCK", "O"], "to": ["LEFTSHIFT", "EQUAL"] },
    { "from": ["CAPSLOCK", "U"], "to": "MINUS" }
  ]
}
```

Note that in the above example, using `“letters”` automatically includes the `“LEFTSHIFT”` necessary to make the `+` symbol on a US QWERTY keyboard.

The available rows are the following rows on a US QWERTY keyboard:

* ```“`”``` - The row starting with `“GRAVE”`. You can also use use `“1”` to refer to this row but starting with the `“1”` key.
* `“Q”` - The row starting with `“Q”`.
* `“A”` - The row starting with `“A”`.
* `“Z”` - The row starting with `“Z”`.

### Modifier Aliases

If you have layouts that use the <kbd>Shift</kbd> keys, it can be tedious to duplicate each mapping for `“LEFTSHIFT”` and `“RIGHTSHIFT”`. Instead, you can use an alias, which is any word starting with `@`:

```json
{
  "mappings": [
    { "from": "LEFTSHIFT", "to": "@shift" },
    { "from": "RIGHTSHIFT", "to": "@shift" },
    { "from": ["@shift", "SPACE"], "to": "BACKSPACE" }
  ]
}
```

See the `super-dvorak` default layout for an example that makes heavy use of aliases.

## If your physical keyboard has non-English symbols

`totalmapper` works with keycodes, not key symbols. There are many more symbols than keycodes. For example, `a` and `A` are separate symbols, but in keycodes, `A` is just <kbd>Shift</kbd> + <kbd>A</kbd>. 

Typically on Linux, the mapping between keycodes and symbols is defined through a system like [XKB](https://wiki.archlinux.org/title/X_keyboard_extension). You can use XKB and `totalmapper` together: `totalmapper` will remap keycodes, and XKB will apply the appropriate symbols to the resulting keycodes.

If your keyboard has non-English symbols on it (such as ñ, ü, ㄴㄷㄹㅁ, or д), `totalmapper` will remap those key *codes* the same as it remaps any other key *codes*—without regard for the symbols they stand for.

To figure out what keycodes correspond to your physical keys, you can inspect your keyboard device with `evtest`. Here is an example output from `evtest /dev/input/event2`:

    Event: time 1623709383.272708, type 4 (EV_MSC), code 4 (MSC_SCAN), value 40
    Event: time 1623709383.272708, type 1 (EV_KEY), code 64 (KEY_F6), value 0
    Event: time 1623709383.272708, -------------- SYN_REPORT ------------
    Event: time 1623709403.107286, type 4 (EV_MSC), code 4 (MSC_SCAN), value 1e
    Event: time 1623709403.107286, type 1 (EV_KEY), code 30 (KEY_A), value 1
    Event: time 1623709403.107286, -------------- SYN_REPORT ------------
    Event: time 1623709403.240861, type 4 (EV_MSC), code 4 (MSC_SCAN), value 1e
    Event: time 1623709403.240861, type 1 (EV_KEY), code 30 (KEY_A), value 0
    Event: time 1623709403.240861, -------------- SYN_REPORT ------------
    Event: time 1623709405.606143, type 4 (EV_MSC), code 4 (MSC_SCAN), value 1f
    Event: time 1623709405.606143, type 1 (EV_KEY), code 31 (KEY_S), value 1
    Event: time 1623709405.606143, -------------- SYN_REPORT ------------
    Event: time 1623709405.722982, type 4 (EV_MSC), code 4 (MSC_SCAN), value 1f
    Event: time 1623709405.722982, type 1 (EV_KEY), code 31 (KEY_S), value 0

The `KEY_` part tells you what keycode was typed. Remove the `KEY_` prefix and you can use it as a key in `totalmapper`.

## Typing non-US-QWERTY symbols

Because `totalmapper` only works with keycodes, it can’t directly produce non-US-QWERTY symbols. However, `totalmapper` can work alongside utilities like `xkb` that can translate key*codes* to a wide variety of symbols.

As an example, I like to have the section symbol (§) on my keyboard. `totalmapper` cannot directly produce this symbol because there is no `"§"` key on a standard US QWERTY keyboard. So, I use the following technique:

In my `xkb` layout file, I include an <kbd>AltGr</kbd> key (also known as an `ISO_Level3_Shift`) as the right <kbd>Alt</kbd> key:

```xkb
key <RALT> { type[Group1]="ONE_LEVEL", symbols[Group1] = [ ISO_Level3_Shift ] };
```

Also in my `xkb` layout file, I remap <kbd>AltGr</kbd> + <kbd>S</kbd> to `“$”`:

```xkb
key <AC02> {
  type = "FOUR_LEVEL_ALPHABETIC",
  symbols[Group1] = [ s, S, section, section ]
};
```

Then, in my `totalmapper` layout file, I remap the appropriate keys to produce the <kbd>AltGr</kbd> + <kbd>S</kbd> combination:

```json
{
  "mappings": [
    { "from": "LEFTSHIFT", "to": "@shift" },
    { "from": "RIGHTSHIFT", "to": "@shift" },
    { "from": "CAPSLOCK", "to": "@symbol" },
    { "from": "RIGHTALT", "to": "@symbol" },
    { "from": ["@symbol", "@shift", "S"], "to": ["RIGHTALT", "S"] }
  ]
}
```

## Customizing repeat behavior

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
