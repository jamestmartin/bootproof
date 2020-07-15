# Timeline
A tentative short-term timeline for what to do next.

## Leaving UEFI
* Transition from UEFI stdout to UEFI GOP
* Transition from UEFI GOP to real graphics drivers:
    * VirtIO GPU
    * Intel HD Graphics 630
* Transition from UEFI stdin to a PS/2 keyboard
* Transition from UEFI allocation to a custom allocator
* RTC support continues even after exiting UEFI boot services.

## Multiprocessing
* Support for basic relocatable executables
* Single-processor scheduler
* Multi-processor scheduling

## Accessing storage
* NVMe driver
* FAT32 support
