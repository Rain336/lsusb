use rusb::{has_hotplug, Device, GlobalContext, Hotplug, HotplugBuilder, UsbContext};
use std::error::Error;
use std::marker::PhantomData;

struct HotplugList<T: UsbContext>(PhantomData<T>);

impl<T: UsbContext> Hotplug<T> for HotplugList<T> {
    fn device_arrived(&mut self, device: Device<T>) {
        let descriptor = match device.device_descriptor() {
            Ok(x) => x,
            Err(e) => {
                println!("Device Connected - Error: {}", e);
                return;
            }
        };
        println!(
            "Device Connected - Vendor Id: {:x} Product Id: {:x}",
            descriptor.vendor_id(),
            descriptor.product_id()
        );
    }

    fn device_left(&mut self, device: Device<T>) {
        let descriptor = match device.device_descriptor() {
            Ok(x) => x,
            Err(e) => {
                println!("Device Disconnected - Error: {}", e);
                return;
            }
        };
        println!(
            "Device Disconnected - Vendor Id: {:x} Product Id: {:x}",
            descriptor.vendor_id(),
            descriptor.product_id()
        );
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let global = GlobalContext::default();
    if !has_hotplug() {
        println!("Hotplug not supported!");
        for device in global.devices()?.iter() {
            let descriptor = device.device_descriptor()?;
            println!(
                "Device - Vendor Id: {:x} Product Id: {:x}",
                descriptor.vendor_id(),
                descriptor.product_id()
            );
        }
        return Ok(());
    }
    let _reg = HotplugBuilder::new()
        .enumerate(true)
        .register(global, Box::new(HotplugList(PhantomData)))?;
    loop {
        global.handle_events(None)?
    }
}
