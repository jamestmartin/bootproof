# Hardware
## Minimum viable product
* CPU: any x86_64
* Storage: NVMe
* Graphics: VGA
* PnP: PS/2 keyboard
    * My laptop keyboard is represented as a PS/2 keyboard, so this is actual hardware!

## Very important
* PnP: Clock
* Sound: Intel HD Audio
* Network:
    * QEMU: VirtIO network
    * Physical:
        * Qualcomm Atheros QCA6174 802.11ac
        * USB wireless adapter
            * I have a few of these. This would require USB support first obviously.
        * RTL8153 Ethernet via Thunderbolt 3 dock
            * Probably not a viable option because it'd require supporting my dock.

## Important
* PnP: PS/2 mouse
* USB:
    * Controllers: xHCI
    * Devices: HID, audio
* Graphics: Intel HD Graphics 630
    * Not supported by QEMU.

## Less important
* USB devices: storage, bluetooth, hub
* Bluetooth devices: keyboard, mouse
* Thunderbolt 3: controller, bridge, NHI (what is this?)
    * Not supported by QEMU.

## Unimportant
* CPU security mitigations (as listed by lscpu)
* Graphics: AMD Polaris 22 XL [Radeon RX Vega M GL]
    * Not supported by QEMU.
    * Not very important thanks to Intel HD graphics. I don't expect to be doing a lot of gaming!
* Graphics: VirtIO GPU
    * Necessary to get better than VGA graphics on QEMU (I think?)
    * Possibly easier than supporting real graphics cards?
        * If this is true, maybe it'd be worth implementing first?
* Power management: ACPI
* USB devices:
    * billboard
        * To my understanding this is just an error reporting mechanism.
    * MIDI over USB
        * Not very useful without a good synthesizer (might be a fun toy anyway)
        * Maybe not supported by QEMU?
    * wacom
        * Not very useful without a good drawing program
    * webcam
        * It's something I'd use in theory, at least.

## Very unimportant
Hardware I don't even have, but is very common and makes it on the wishlist at least.

* Storage: ACPI, SCSI & UAS, eSATA, ATAPI
    * ACPI especially. Most drives aren't SSD drives!
* USB controllers: OHCI, UHCI, EHCI
    * Probably not too difficult. I wouldn't be surprised if they end up getting implemented while working my way up to xHCI anyway.
    * A computer with literally no USB 2.0 ports like mine is still pretty niche, probably!
* Network: USB ethernet & wireless adapters
    * Good because any computer can use them.

## Don't care
Hardware I *do* have, but don't care about supporting for the foreseeable future.

* SD card reader
* Fingerprint reader
* Intel ME

## Needs research
* USB device family: billboard
    * lsusb reports at least one but I don't know what it does
* USB:
    * Microchip Technology, Inc. (formerly SMSC) USB5537B
    * (totally empty): Pretty sure this is my USB drive. Don't know why it's shwoing up empty.
* Thunderbolt 3:
    * I'm guessing I need to support all of these for my thunderbolt 3 dock to work,
      but I don't actually know what does what, specifically.
    * I'm pretty sure I don't need this for the ports to work as USB 3 ports. (And if it does, I'm fucked!!)
    * PCI bridge: Intel Corporation JHL6540 Thunderbolt 3 Bridge (C step) [Alpine Ridge 4C 2016] (rev 02)
    * System peripheral: Intel Corporation JHL6540 Thunderbolt 3 NHI (C step) [Alpine Ridge 4C 2016] (rev 02)
    * PCI bridge: Intel Corporation DSL6540 Thunderbolt 3 Bridge [Alpine Ridge 4C 2015]
    * USB controller: Intel Corporation JHL6540 Thunderbolt 3 USB Controller (C step) [Alpine Ridge 4C 2016] (rev 02)
        * Does this need a different, non-xHCI driver?
* Apparently power-related:
    * I'm not sure what drivers for these would actually do.
      Am I supposed to read out data from them, or control them in some way, or..?
    * I think I can probably get away with ignoring these for now.
    * Signal processing controller: Intel Corporation Xeon E3-1200 v5/E3-1500 v5/6th Gen Core Processor Thermal Subsystem
    * Signal processing controller: Intel Corporation 100 Series/C230 Series Chipset Family Thermal Subsystem
    * Memory controller: Intel Corporation 100 Series/C230 Series Chipset Family Power Management Controller
* Apparently input-related:
    * Signal processing controller: Intel Corporation 100 Series/C230 Series Chipset Family Serial IO I2C Controller
        * I think this has to do with the touch screen and touch pad (I have 2 controllers shown)
        * Might actually be worth implementing if it means freeing up a USB port until I get thunderbolt 3 working
        * MIght actually be worth implementing if it means I can defer implementing USB entirely
* Don't know:
    * SMBus: Intel Corporation 100 Series/C230 Series Chipset Family SMBus
        * A bus controller obviously, but I don't know what uses this bus that I need to support.
    * Non-VGA unclassified device: Intel Corporation 100 Series/C230 Series Chipset Family Integrated Sensor Hub
        * kernel.org says this is some kind of co-processor but I don't know
          if it's just optional to save power or if it's necessary to support some hardware,
          and for that matter, what hardware it's actually applicable to
