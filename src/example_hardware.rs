
// https://github.com/ellbur/totalmapper/issues/2
#[cfg(test)]
pub const GAMING_MOUSE_SETUP_1: &'static str = r#"I: Bus=0019 Vendor=0000 Product=0005 Version=0000
N: Name="Lid Switch"
P: Phys=PNP0C0D/button/input0
S: Sysfs=/devices/LNXSYSTM:00/LNXSYBUS:00/PNP0C0D:00/input/input0
U: Uniq=
H: Handlers=event0 
B: PROP=0
B: EV=21
B: SW=1

I: Bus=0019 Vendor=0000 Product=0001 Version=0000
N: Name="Power Button"
P: Phys=PNP0C0C/button/input0
S: Sysfs=/devices/LNXSYSTM:00/LNXSYBUS:00/PNP0C0C:00/input/input1
U: Uniq=
H: Handlers=kbd event1 
B: PROP=0
B: EV=3
B: KEY=10000000000000 0

I: Bus=0011 Vendor=0001 Product=0001 Version=ab41
N: Name="AT Translated Set 2 keyboard"
P: Phys=isa0060/serio0/input0
S: Sysfs=/devices/platform/i8042/serio0/input/input2
U: Uniq=
H: Handlers=sysrq kbd event2 leds 
B: PROP=0
B: EV=120013
B: KEY=1100f02902000 8380307cf910f001 feffffdfffefffff fffffffffffffffe
B: MSC=10
B: LED=7

I: Bus=0019 Vendor=0000 Product=0006 Version=0000
N: Name="Video Bus"
P: Phys=LNXVIDEO/video/input0
S: Sysfs=/devices/LNXSYSTM:00/LNXSYBUS:00/PNP0A08:00/LNXVIDEO:00/input/input5
U: Uniq=
H: Handlers=kbd event3 
B: PROP=0
B: EV=3
B: KEY=3e000b00000000 0 0 0

I: Bus=0011 Vendor=0002 Product=0001 Version=0000
N: Name="PS/2 Generic Mouse"
P: Phys=isa0060/serio1/input0
S: Sysfs=/devices/platform/i8042/serio1/input/input4
U: Uniq=
H: Handlers=mouse3 event7 
B: PROP=1
B: EV=7
B: KEY=70000 0 0 0 0
B: REL=3

I: Bus=0003 Vendor=0c76 Product=161e Version=0100
N: Name="USB PnP Audio Device"
P: Phys=usb-0000:00:14.0-2/input2
S: Sysfs=/devices/pci0000:00/0000:00:14.0/usb1/1-2/1-2:1.2/0003:0C76:161E.0004/input/input11
U: Uniq=
H: Handlers=kbd event8 
B: PROP=0
B: EV=13
B: KEY=7800000000 e000000000000 0
B: MSC=10

I: Bus=0003 Vendor=145f Product=01bc Version=0110
N: Name="GXT 4155 Gaming Mouse"
P: Phys=usb-0000:00:14.0-1.3/input0
S: Sysfs=/devices/pci0000:00/0000:00:14.0/usb1/1-1/1-1.3/1-1.3:1.0/0003:145F:01BC.0005/input/input12
U: Uniq=
H: Handlers=mouse4 event9 
B: PROP=0
B: EV=17
B: KEY=ffff0000 0 0 0 0
B: REL=1943
B: MSC=10

I: Bus=0003 Vendor=145f Product=01bc Version=0110
N: Name="GXT 4155 Gaming Mouse"
P: Phys=usb-0000:00:14.0-1.3/input1
S: Sysfs=/devices/pci0000:00/0000:00:14.0/usb1/1-1/1-1.3/1-1.3:1.1/0003:145F:01BC.0006/input/input13
U: Uniq=
H: Handlers=sysrq kbd event10 
B: PROP=0
B: EV=100013
B: KEY=1000000000007 ff9f207ac14057ff febeffdfffefffff fffffffffffffffe
B: MSC=10

I: Bus=0003 Vendor=145f Product=01bc Version=0110
N: Name="GXT 4155 Gaming Mouse Consumer Control"
P: Phys=usb-0000:00:14.0-1.3/input2
S: Sysfs=/devices/pci0000:00/0000:00:14.0/usb1/1-1/1-1.3/1-1.3:1.2/0003:145F:01BC.0007/input/input14
U: Uniq=
H: Handlers=kbd event11 
B: PROP=0
B: EV=1f
B: KEY=3f000301ff 0 0 483ffff17aff32d bfd4444600000000 1 130ff38b17c000 677bfad9415fed 19ed68000004400 10000002
B: REL=1040
B: ABS=100000000
B: MSC=10

I: Bus=0003 Vendor=145f Product=01bc Version=0110
N: Name="GXT 4155 Gaming Mouse"
P: Phys=usb-0000:00:14.0-1.3/input2
S: Sysfs=/devices/pci0000:00/0000:00:14.0/usb1/1-1/1-1.3/1-1.3:1.2/0003:145F:01BC.0007/input/input15
U: Uniq=
H: Handlers=event12 
B: PROP=0
B: EV=9
B: ABS=10000000000

I: Bus=0019 Vendor=0000 Product=0000 Version=0000
N: Name="Intel HID events"
P: Phys=
S: Sysfs=/devices/platform/INT33D5:00/input/input16
U: Uniq=
H: Handlers=rfkill kbd event13 
B: PROP=0
B: EV=13
B: KEY=81000300000000 5000004000 1e294000000020 0
B: MSC=10

I: Bus=0019 Vendor=0000 Product=0000 Version=0000
N: Name="Intel HID 5 button array"
P: Phys=
S: Sysfs=/devices/platform/INT33D5:00/input/input17
U: Uniq=
H: Handlers=kbd event14 
B: PROP=0
B: EV=13
B: KEY=2000000000000 0 0 0 0 1000000000000 0 201c000000000000 0
B: MSC=10

I: Bus=0019 Vendor=0000 Product=0000 Version=0000
N: Name="Dell WMI hotkeys"
P: Phys=
S: Sysfs=/devices/platform/PNP0C14:03/wmi_bus/wmi_bus-PNP0C14:03/9DBB5994-A997-11DA-B012-B622A1EF5492/input/input18
U: Uniq=
H: Handlers=rfkill kbd event4 
B: PROP=0
B: EV=13
B: KEY=800000000000 0 0 1500b00000c00 4000001200300000 e000000000000 0
B: MSC=10

I: Bus=0018 Vendor=27c6 Product=01e0 Version=0100
N: Name="DELL09EC:00 27C6:01E0 Mouse"
P: Phys=i2c-DELL09EC:00
S: Sysfs=/devices/pci0000:00/0000:00:15.0/i2c_designware.0/i2c-1/i2c-DELL09EC:00/0018:27C6:01E0.0001/input/input19
U: Uniq=
H: Handlers=mouse0 event5 
B: PROP=0
B: EV=17
B: KEY=30000 0 0 0 0
B: REL=1943
B: MSC=10

I: Bus=0018 Vendor=27c6 Product=01e0 Version=0100
N: Name="DELL09EC:00 27C6:01E0 Touchpad"
P: Phys=i2c-DELL09EC:00
S: Sysfs=/devices/pci0000:00/0000:00:15.0/i2c_designware.0/i2c-1/i2c-DELL09EC:00/0018:27C6:01E0.0001/input/input20
U: Uniq=
H: Handlers=mouse1 event15 
B: PROP=5
B: EV=1b
B: KEY=e520 10000 0 0 0 0
B: ABS=2e0800000000003
B: MSC=20

I: Bus=0018 Vendor=1fd2 Product=8002 Version=0100
N: Name="CUST0000:00 1FD2:8002"
P: Phys=i2c-CUST0000:00
S: Sysfs=/devices/pci0000:00/0000:00:15.1/i2c_designware.1/i2c-8/i2c-CUST0000:00/0018:1FD2:8002.0002/input/input22
U: Uniq=
H: Handlers=mouse2 event6 
B: PROP=2
B: EV=1b
B: KEY=400 0 0 0 0 0
B: ABS=260800000000003
B: MSC=20

I: Bus=0000 Vendor=0000 Product=0000 Version=0000
N: Name="sof-hda-dsp Headphone Mic"
P: Phys=ALSA
S: Sysfs=/devices/pci0000:00/0000:00:1f.3/skl_hda_dsp_generic/sound/card0/input24
U: Uniq=
H: Handlers=event16 
B: PROP=0
B: EV=21
B: SW=4

I: Bus=0000 Vendor=0000 Product=0000 Version=0000
N: Name="sof-hda-dsp HDMI/DP,pcm=3"
P: Phys=ALSA
S: Sysfs=/devices/pci0000:00/0000:00:1f.3/skl_hda_dsp_generic/sound/card0/input25
U: Uniq=
H: Handlers=event17 
B: PROP=0
B: EV=21
B: SW=140

I: Bus=0000 Vendor=0000 Product=0000 Version=0000
N: Name="sof-hda-dsp HDMI/DP,pcm=4"
P: Phys=ALSA
S: Sysfs=/devices/pci0000:00/0000:00:1f.3/skl_hda_dsp_generic/sound/card0/input26
U: Uniq=
H: Handlers=event18 
B: PROP=0
B: EV=21
B: SW=140

I: Bus=0000 Vendor=0000 Product=0000 Version=0000
N: Name="sof-hda-dsp HDMI/DP,pcm=5"
P: Phys=ALSA
S: Sysfs=/devices/pci0000:00/0000:00:1f.3/skl_hda_dsp_generic/sound/card0/input27
U: Uniq=
H: Handlers=event19 
B: PROP=0
B: EV=21
B: SW=140

I: Bus=0003 Vendor=1bcf Product=2283 Version=1217
N: Name="NexiGo N930AF FHD Webcam: NexiG"
P: Phys=usb-0000:00:14.0-3/button
S: Sysfs=/devices/pci0000:00/0000:00:14.0/usb1/1-3/1-3:1.0/input/input28
U: Uniq=
H: Handlers=kbd event20 
B: PROP=0
B: EV=3
B: KEY=100000 0 0 0

I: Bus=0003 Vendor=0bda Product=5532 Version=8264
N: Name="Integrated_Webcam_HD: Integrate"
P: Phys=usb-0000:00:14.0-6/button
S: Sysfs=/devices/pci0000:00/0000:00:14.0/usb1/1-6/1-6:1.0/input/input29
U: Uniq=
H: Handlers=kbd event21 
B: PROP=0
B: EV=3
B: KEY=100000 0 0 0

I: Bus=0003 Vendor=0bda Product=5532 Version=8264
N: Name="Integrated_Webcam_HD: Integrate"
P: Phys=usb-0000:00:14.0-6/button
S: Sysfs=/devices/pci0000:00/0000:00:14.0/usb1/1-6/1-6:1.2/input/input30
U: Uniq=
H: Handlers=kbd event22 
B: PROP=0
B: EV=3
B: KEY=100000 0 0 0

I: Bus=0005 Vendor=000a Product=ffff Version=ffff
N: Name="TaoTronics TT-BH22 (AVRCP)"
P: Phys=e0:2b:e9:ef:f7:30
S: Sysfs=/devices/virtual/input/input33
U: Uniq=
H: Handlers=kbd event24 
B: PROP=0
B: EV=100007
B: KEY=2fc800 145200000000 0 10300 49e800000c00 e16800000000f f810000010000ffc
B: REL=0
"#;

