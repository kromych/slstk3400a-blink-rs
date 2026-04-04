//! USB Mass Storage demo for the SLSTK3400A.
//!
//! Presents a tiny read-only FAT16 drive with `README.TXT`.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32hg322_pac as pac;
use efm32hg322_usb::msc::{self, MscClass, MscHandler};
use efm32hg322_usb::UsbDevice;

// ---------------------------------------------------------------------------
// Virtual FAT16 filesystem (128 KB = 256 sectors of 512 bytes)
//
//   LBA 0   : Boot sector (BPB)
//   LBA 1   : FAT #1  (1 sector)
//   LBA 2   : FAT #2  (1 sector)
//   LBA 3   : Root directory (1 sector, 16 entries max)
//   LBA 4-7 : Data cluster 2 (readme.txt, 4 sectors = 1 cluster)
//   LBA 8+  : Zero
// ---------------------------------------------------------------------------

const TOTAL_SECTORS: u32 = 256;
const SECTOR_SIZE: u32 = 512;

static README_TXT: &[u8] = b"Hello from the EFM32HG322 SLSTK3400A!\r\n\
This tiny USB drive is generated entirely in firmware.\r\n\
No flash storage is used -- sectors are synthesised on the fly.\r\n";

struct Fat16Volume;

impl MscHandler for Fat16Volume {
    fn capacity(&self) -> u32 {
        TOTAL_SECTORS
    }

    fn read_sector(&mut self, lba: u32, buf: &mut [u8; 512]) {
        *buf = [0u8; 512];
        match lba {
            // ---- Boot sector / BPB ----
            0 => {
                buf[0] = 0xEB;
                buf[1] = 0x3C;
                buf[2] = 0x90;
                buf[3..11].copy_from_slice(b"MSDOS5.0");
                buf[11] = 0x00;
                buf[12] = 0x02; // 512 bytes/sector
                buf[13] = 4; // sectors/cluster
                buf[14] = 1; // reserved sectors
                buf[16] = 2; // FAT count
                buf[17] = 16; // root entries
                buf[19] = (TOTAL_SECTORS & 0xFF) as u8;
                buf[20] = (TOTAL_SECTORS >> 8) as u8;
                buf[21] = 0xF8; // media
                buf[22] = 1; // FAT size
                buf[24] = 32; // sectors/track
                buf[26] = 2; // heads
                buf[36] = 0x80; // drive number
                buf[38] = 0x29; // boot sig
                buf[39..43].copy_from_slice(&[0x78, 0x56, 0x34, 0x12]);
                buf[43..54].copy_from_slice(b"EFM32 DRIVE");
                buf[54..62].copy_from_slice(b"FAT16   ");
                buf[510] = 0x55;
                buf[511] = 0xAA;
            }
            // ---- FAT #1 and FAT #2 ----
            1 | 2 => {
                buf[0] = 0xF8;
                buf[1] = 0xFF;
                buf[2] = 0xFF;
                buf[3] = 0xFF;
                buf[4] = 0xFF;
                buf[5] = 0xFF; // cluster 2 EOF
            }
            // ---- Root directory ----
            3 => {
                buf[0..11].copy_from_slice(b"EFM32 DRIVE");
                buf[11] = 0x08; // volume label
                let e = &mut buf[32..64];
                e[0..11].copy_from_slice(b"README  TXT");
                e[11] = 0x01; // read-only
                e[26] = 2; // first cluster
                let sz = README_TXT.len() as u32;
                e[28..32].copy_from_slice(&sz.to_le_bytes());
            }
            // ---- Data: cluster 2 at LBA 4 ----
            4..=7 => {
                let offset = ((lba - 4) * SECTOR_SIZE) as usize;
                if offset < README_TXT.len() {
                    let end = (offset + SECTOR_SIZE as usize).min(README_TXT.len());
                    buf[..end - offset].copy_from_slice(&README_TXT[offset..end]);
                }
            }
            _ => {}
        }
    }
}

efm32hg322_usb::usb_device!(MscClass<Fat16Volume>);

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    p.wdog.ctrl().write(|w| w.en().clear_bit());

    let dev = UsbDevice::init(
        &p.cmu,
        &p.usb,
        MscClass::new(Fat16Volume),
        msc::usb_config(),
    );

    defmt::info!("USB Mass Storage ready - connect cable now");
    usb_start(dev);
    efm32hg322_usb::idle();
}
