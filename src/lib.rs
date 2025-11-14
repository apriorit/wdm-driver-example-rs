#![no_std]

#[cfg(not(test))]
extern crate wdk_panic;

#[cfg(not(test))]
use wdk_alloc::WdkAllocator;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;

use wdk::println;
use wdk_sys::{
   DRIVER_OBJECT,
   PDRIVER_OBJECT,
   NTSTATUS,
   PCUNICODE_STRING,
   STATUS_SUCCESS,
};

#[unsafe(export_name = "DriverEntry")]
pub unsafe extern "system" fn driver_entry(
   driver_object: &mut DRIVER_OBJECT,
   _registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    println!("Hello World!");

    driver_object.DriverUnload = Some(driver_unload);

    STATUS_SUCCESS
}

extern "C" fn driver_unload(_driver_object: PDRIVER_OBJECT) {
    println!("Goodbye World!");
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
