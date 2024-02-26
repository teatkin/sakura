#![no_std]

use device::Device;

mod device;

fn init() {
    let device = Device::new();
}
