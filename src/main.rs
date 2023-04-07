#![no_std]
#![no_main]

use uefi::prelude::*;


#[entry]
fn efi_main(image: Handle, st: SystemTable<Boot>) 
    -> Status
    {
        Status::SUCCESS
    }
