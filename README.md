# bootproof
Messing around with UEFI apps.

I don't have a specific goal here.
My general direction is to work towards a bootable programming language environment,
preferably one where security and allocation etc. are handled through the programming language
rather than through a traditional operating system.
I don't seriously expect to ever accomplish that, so for now I'm probably just going to...
make a forth or something.

## System Requirements
Other configurations may work, but only these systems are regularly tested.
* CPU: x86_64 QEMU, OVMF UEFI.
* Memory: 128 MB. (64 MB appears to be the minimum required to load OVMF at all. Real hardware might require less?)

## Running
bootproof runs on x86_64 UEFI. You may either boot the program directly on your own computer or use an emulator.

Make sure you have the `cargo-xbuild` crate installed and nightly Rust so you can compile to the UEFI target.

First, build with:
```
cargo xbuild --target x86_64-unknown-uefi
```

And to run, `./run.sh` will launch bootproof in QEMU.
