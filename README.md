# bootproof
A hobby x86_64 operating system written in Rust.

![screenshot](/docs/screenshot.png)

The above screenshot from commit 538dfea (July 18th, 2020)
demonstrates bootproof's Unicode support, PSF font loader,
and generic support for graphics-based TTYs and terminals.

## Installation
bootproof runs on x86_64 and expects to be loaded by UEFI.
You can either boot it using an emulator or on your own computer.

### Building
You'll need to the Rust nightly toolchain installed
because bootproof relies heavily on Rust nightly features
(most of which are directly necessary for OS development).

You'll also need the `cargo-xbuild` crate installed
so that you can compile for the `x86_64-unknown-uefi` target.

Building bootproof is pretty straightforward:

```
cargo xbuild --target x86_64-unknown-uefi
```

You can add the `--release` flag for a release-profile build.

This will produce an executable,
`target/x86_64-unknown-efi/{profile}/bootproof.efi`.

### Running
#### With QEMU
You will need QEMU, and OVMF, which provides a UEFI implementation for QEMU.
On Debian derivatives, you can install these dependencies with:

```
apt install qemu-system-x86 ovmf
```

*After you have built the crate with `cargo xbuild`*,
you can use `./run.sh $profile` to run QEMU with some good presets.
`$profile` may be either `debug` or `release`, depending on which you built.
If you don't specify, it defaults to `debug`, just like `cargo`.

The VM's serial port will be mapped to stdio,
which you can use to interact with the OS.

#### With real hardware
I would strongly recommend against doing this.

Copy `bootproof.efi` to your system EFI partition in the EFI folder.
You may put it wherever you'd like and select it while booting.
Alternatively, you can name it `/EFI/Boot/BootX64.efi`,
and it will be loaded automatically, *instead of your regular bootloader or OS*.

You do *not* need a bootloader to run bootproof. The UEFI is all you need.

## Goals
1. **Have fun.** Ultimately, I'm doing this *because I want to*.
   Operating system development can be very difficult and tedious at times,
   but if I've turned this project into work, I've failed.

2. **Gain experience**, in particular with Rust, large-scale projects,
   and low-level programming in general. I should always be learning
   and becoming a better programmer.

3. **Show off.** I want to demonstrate my skills as a programmer,
   both to employers and to other programmers in general
   (because at least in my opinion, writing your own operating system
    gives you some serious cred!)

4. **Make something I'd want to use.**
   I should always be working towards an operating system
   that directly addresses my use cases and supports my hardware,
   so that if I ever managed to get far enough along,
   I'd actually *want* to use the OS that I ended up making.

## Philosophy
1. **Simplicity.**
   Getting a lot of stuff done in a simple way
   is better than getting very little done in an ideal way,
   especially with a scope as large as an entire operating system.

2. **Maintainability.**
   Operating system codebases are large, complex, and long-lived.
   In the long term, good maintainability is absolutely necessary.

3. **Forward-thinking.**
   Focus on what you'll need tomorrow, not what you need today.
   By the time you have it, tomorrow will be today and today will be yesterday.

4. **Iterate quickly.**
   There's a lot I don't know, and ultimately the best way to learn it
   is to explore the space through programming.
   It's *okay* to write some crappy code
   if that means my next attempt will be much better--
   as long as I don't let it build up and interfere with maintainability.
