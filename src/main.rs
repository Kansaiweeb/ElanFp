use std::time::Duration;

use rusb::{open_device_with_vid_pid, DeviceHandle, GlobalContext, UsbContext, Context, Device};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match Context::new() {
        Ok(mut context) => match open_scanner
    }
}

fn main1() {
    // let a = [(0x04f3, 0x0c00), (0x04f2, 0x0c4c), (0x04f3, 0x0c5e)];
    // for (vid ,pid)in a {
    //     let b = open_device_with_vid_pid(pid, vid).is_some();
    //     println!("{}", b);
    // }
    // let scanner = open_device_with_vid_pid(1267, 3148).is_some();
    // println!("scanner is {}", scanner);
    let devices = rusb::DeviceList::new();
    let fp_scanner = devices
        .unwrap()
        .iter()
        .filter(
            |device| 
            device.device_descriptor().unwrap().product_id() == 0xc4c)
        .collect::<Vec<Device<GlobalContext>>>();
    let fp_scanner = fp_scanner.first().unwrap();
    // let descriptor = fp_scanner.device_descriptor().unwrap();
    // println!("pid {}, vid {}", descriptor.product_id(), descriptor.vendor_id());
    // println!("{}", fp_scanner.open().is_ok());
    let mut opened_scanner = fp_scanner.open().unwrap();
    opened_scanner.claim_interface(0).unwrap();
    opened_scanner
    .write_bulk(1, &[0x40,0x19], Duration::from_secs(1) ).unwrap();
let read_result = opened_scanner.read_bulk(3, &mut[], Duration::from_secs(1)).unwrap();
println!("{}", read_result);
    // open_device_with_vid_pid(vendor_id, product_id)
    // let device_descriptor = get_fingerprint_handle();
    // device_descriptor.handle.write_bulk(0xff,&[0xa],Duration::from_secs(1));
}

struct ElanFpHandle<T: UsbContext> {
    pub handle: DeviceHandle<T>
}

impl<T: UsbContext> ElanFpHandle<T> {

    fn new (handle: DeviceHandle<T>)-> ElanFpHandle<T> {
        ElanFpHandle { handle }
    }

}

fn  get_fingerprint_handle()-> ElanFpHandle<GlobalContext> {
    ElanFpHandle::new(open_device_with_vid_pid(0x04f3, 0x0c5e).unwrap())
}