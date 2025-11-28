#![no_main]
#![no_std]


#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(i: &PanicInfo) -> ! {
    println!("panic {:?}", i.location());
    loop {}
}

include!(concat!(env!("OUT_DIR"), "/generated_data.rs"));


use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::println;
use uefi::boot::OpenProtocolAttributes;
use uefi::boot::OpenProtocolParams;
use core::time::Duration;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    // Disable the watchdog timer

    boot::set_watchdog_timer(0, 0x10000, None).unwrap();

    // Get gop
    let gop_handle = boot::get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let params = OpenProtocolParams {
        handle: gop_handle,
        agent: gop_handle,
        controller: None,
    };

    let mut gop = if let Ok(gop) = unsafe {
        boot::open_protocol::<GraphicsOutput>(params, OpenProtocolAttributes::GetProtocol)
    } {
        gop
    } else {
        println!("boot::open_protocol_exclusive() failed");

        // boot::stall(3_000_000);

        return Status::ABORTED;
    };

    unsafe {
        core::ptr::copy_nonoverlapping(GENERATED_DATA as *const u8, gop.frame_buffer().as_mut_ptr(), GENERATED_DATA.len());
    }


    boot::stall(Duration::from_secs(100));

    Status::SUCCESS
}
