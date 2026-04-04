//! USB CDC ACM (virtual serial port) demo for the SLSTK3400A.
//!
//! Echoes data received from the host.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32hg322_pac as pac;
use efm32hg322_usb::cdc_acm::{self, CdcAcmClass, CdcAcmHandler};
use efm32hg322_usb::{UsbBus, UsbDevice};

struct EchoHandler;

impl CdcAcmHandler for EchoHandler {
    fn data_received(&mut self, data: &[u8], usb: &UsbBus) {
        usb.ep_write(1, data);
    }

    fn tx_complete(&mut self, usb: &UsbBus) {
        usb.ep_prepare_out(1, 64);
    }
}

efm32hg322_usb::usb_device!(CdcAcmClass<EchoHandler>);

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    p.wdog.ctrl().write(|w| w.en().clear_bit());

    let dev = UsbDevice::init(
        &p.cmu,
        &p.usb,
        CdcAcmClass::new(EchoHandler),
        cdc_acm::usb_config(),
    );

    defmt::info!("USB CDC ACM serial port ready - connect cable now");
    usb_start(dev);
    efm32hg322_usb::idle();
}
