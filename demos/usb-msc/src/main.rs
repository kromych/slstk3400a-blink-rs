//! USB Mass Storage Class (Bulk-Only Transport) demo for the SLSTK3400A.
//!
//! Presents a tiny read-only FAT16 volume (128 KB, 256 sectors) containing a
//! single `README.TXT` file.  The entire filesystem is generated on-the-fly
//! from const data — no RAM image required.
//!
//! Endpoint mapping:
//!   EP0  – Control (SETUP / status)
//!   EP1  – Bulk IN + OUT (SCSI over BBB)

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use core::cell::RefCell;
use core::ops::DerefMut;
use critical_section::Mutex;
use efm32hg322_pac as pac;
use efm32hg322_usb::{
    usb_string, EpConfig, EpType, SetupPacket, SetupResult, UsbBus, UsbClass, UsbConfig,
    UsbDevice,
};
use pac::interrupt;
use portable_atomic::{AtomicBool, Ordering};

// ---------------------------------------------------------------------------
// Virtual FAT16 filesystem layout (128 KB = 256 sectors of 512 bytes)
//
//   LBA 0   : Boot sector (BPB)
//   LBA 1   : FAT #1  (1 sector)
//   LBA 2   : FAT #2  (1 sector)
//   LBA 3   : Root directory (1 sector, 16 entries max)
//   LBA 4-7 : Data cluster 2 (readme.txt, 4 sectors = 1 cluster)
//   LBA 8+  : Zero
// ---------------------------------------------------------------------------

const SECTOR_SIZE: u32 = 512;
const TOTAL_SECTORS: u32 = 256; // 128 KB
const LAST_LBA: u32 = TOTAL_SECTORS - 1;

static README_TXT: &[u8] = b"Hello from the EFM32HG322 SLSTK3400A!\r\n\
This tiny USB drive is generated entirely in firmware.\r\n\
No flash storage is used -- sectors are synthesised on the fly.\r\n";

/// Generate sector `lba` into `buf` (exactly 512 bytes).
fn generate_sector(lba: u32, buf: &mut [u8; 512]) {
    *buf = [0u8; 512];
    match lba {
        // ---- Boot sector / BPB ----
        0 => {
            // Jump boot code
            buf[0] = 0xEB;
            buf[1] = 0x3C;
            buf[2] = 0x90;
            // OEM name
            buf[3..11].copy_from_slice(b"MSDOS5.0");
            // Bytes per sector = 512
            buf[11] = 0x00;
            buf[12] = 0x02;
            // Sectors per cluster = 4
            buf[13] = 4;
            // Reserved sectors = 1
            buf[14] = 1;
            buf[15] = 0;
            // Number of FATs = 2
            buf[16] = 2;
            // Root entry count = 16 (1 sector worth)
            buf[17] = 16;
            buf[18] = 0;
            // Total sectors 16-bit = 256
            buf[19] = (TOTAL_SECTORS & 0xFF) as u8;
            buf[20] = (TOTAL_SECTORS >> 8) as u8;
            // Media type
            buf[21] = 0xF8;
            // FAT size in sectors = 1
            buf[22] = 1;
            buf[23] = 0;
            // Sectors per track
            buf[24] = 32;
            buf[25] = 0;
            // Number of heads
            buf[26] = 2;
            buf[27] = 0;
            // Hidden sectors = 0
            // Total sectors 32-bit = 0 (using 16-bit field)
            // Drive number
            buf[36] = 0x80;
            // Boot signature
            buf[38] = 0x29;
            // Volume serial
            buf[39..43].copy_from_slice(&[0x78, 0x56, 0x34, 0x12]);
            // Volume label (11 bytes)
            buf[43..54].copy_from_slice(b"EFM32 DRIVE");
            // Filesystem type (8 bytes)
            buf[54..62].copy_from_slice(b"FAT16   ");
            // Boot signature
            buf[510] = 0x55;
            buf[511] = 0xAA;
        }
        // ---- FAT #1 and FAT #2 ----
        1 | 2 => {
            // Cluster 0: media type
            buf[0] = 0xF8;
            buf[1] = 0xFF;
            // Cluster 1: end-of-chain marker
            buf[2] = 0xFF;
            buf[3] = 0xFF;
            // Cluster 2: readme.txt (EOF)
            buf[4] = 0xFF;
            buf[5] = 0xFF;
        }
        // ---- Root directory ----
        3 => {
            // Volume label entry
            buf[0..11].copy_from_slice(b"EFM32 DRIVE");
            buf[11] = 0x08; // Attribute: volume label

            // README.TXT entry at offset 32
            let e = &mut buf[32..64];
            e[0..11].copy_from_slice(b"README  TXT");
            e[11] = 0x01; // Attribute: read-only
            // Creation time/date (zero = unset)
            // First cluster low = 2
            e[26] = 2;
            e[27] = 0;
            // File size (little-endian u32)
            let sz = README_TXT.len() as u32;
            e[28] = (sz & 0xFF) as u8;
            e[29] = ((sz >> 8) & 0xFF) as u8;
            e[30] = ((sz >> 16) & 0xFF) as u8;
            e[31] = ((sz >> 24) & 0xFF) as u8;
        }
        // ---- Data: cluster 2 starts at LBA 4 ----
        4..=7 => {
            let offset = ((lba - 4) * SECTOR_SIZE) as usize;
            if offset < README_TXT.len() {
                let end = (offset + SECTOR_SIZE as usize).min(README_TXT.len());
                buf[..end - offset].copy_from_slice(&README_TXT[offset..end]);
            }
        }
        // Everything else is zero
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// USB descriptors
// ---------------------------------------------------------------------------

#[rustfmt::skip]
static DEVICE_DESC: [u8; 18] = [
    18,         // bLength
    0x01,       // bDescriptorType (Device)
    0x00, 0x02, // bcdUSB 2.00
    0x00,       // bDeviceClass    (defined at interface level)
    0x00,       // bDeviceSubClass
    0x00,       // bDeviceProtocol
    64,         // bMaxPacketSize0
    0xFE, 0xCA, // idVendor  0xCAFE (test VID)
    0x05, 0x00, // idProduct 0x0005
    0x00, 0x01, // bcdDevice 1.00
    1,          // iManufacturer
    2,          // iProduct
    0,          // iSerialNumber
    1,          // bNumConfigurations
];

// Config: Configuration(9) + Interface(9) + EP OUT(7) + EP IN(7) = 32
const CONFIG_TOTAL_LEN: u16 = 32;

#[rustfmt::skip]
static CONFIG_DESC: [u8; CONFIG_TOTAL_LEN as usize] = [
    // ---- Configuration Descriptor ----
    9, 0x02,
    (CONFIG_TOTAL_LEN & 0xFF) as u8, (CONFIG_TOTAL_LEN >> 8) as u8,
    1,              // bNumInterfaces
    1,              // bConfigurationValue
    0,              // iConfiguration
    0x80,           // bmAttributes (bus-powered)
    50,             // bMaxPower (100 mA)

    // ---- Interface 0: Mass Storage ----
    9, 0x04,
    0,              // bInterfaceNumber
    0,              // bAlternateSetting
    2,              // bNumEndpoints
    0x08,           // bInterfaceClass (Mass Storage)
    0x06,           // bInterfaceSubClass (SCSI Transparent)
    0x50,           // bInterfaceProtocol (Bulk-Only)
    0,              // iInterface

    // Endpoint 1 OUT – Bulk (host → device, CBW + write data)
    7, 0x05,
    0x01,           // bEndpointAddress (EP1 OUT)
    0x02,           // bmAttributes (Bulk)
    0x40, 0x00,     // wMaxPacketSize (64)
    0,              // bInterval

    // Endpoint 1 IN – Bulk (device → host, read data + CSW)
    7, 0x05,
    0x81,           // bEndpointAddress (EP1 IN)
    0x02,           // bmAttributes (Bulk)
    0x40, 0x00,     // wMaxPacketSize (64)
    0,              // bInterval
];

// ---- String descriptors ----

static STRING1: [u8; 2 + 10 * 2] = usb_string!("SLSTK3400A");
static STRING2: [u8; 2 + 15 * 2] = usb_string!("EFM32 USB Drive");

// ---------------------------------------------------------------------------
// SCSI / BBB constants
// ---------------------------------------------------------------------------

const CBW_SIGNATURE: u32 = 0x43425355; // "USBC"
const CSW_SIGNATURE: u32 = 0x53425355; // "USBS"

const CSW_STATUS_PASSED: u8 = 0;
const CSW_STATUS_FAILED: u8 = 1;

// SCSI opcodes
const SCSI_TEST_UNIT_READY: u8 = 0x00;
const SCSI_REQUEST_SENSE: u8 = 0x03;
const SCSI_INQUIRY: u8 = 0x12;
const SCSI_MODE_SENSE_6: u8 = 0x1A;
const SCSI_START_STOP_UNIT: u8 = 0x1B;
const SCSI_PREVENT_ALLOW_MEDIUM_REMOVAL: u8 = 0x1E;
const SCSI_READ_CAPACITY_10: u8 = 0x25;
const SCSI_READ_10: u8 = 0x28;
const SCSI_WRITE_10: u8 = 0x2A;
const SCSI_READ_FORMAT_CAPACITIES: u8 = 0x23;

// ---------------------------------------------------------------------------
// MSC class driver
// ---------------------------------------------------------------------------

static CONFIGURED: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy, PartialEq)]
enum MscState {
    /// Waiting for a CBW on EP1 OUT.
    Idle,
    /// Sending data to host (READ response), then CSW.
    DataIn,
    /// Receiving data from host (WRITE), then CSW.
    DataOut,
    /// Sending CSW.
    SendCsw,
}

struct MscClass {
    state: MscState,
    // Current CBW fields
    tag: u32,
    data_len: u32,
    // READ(10) progress
    read_lba: u32,
    read_remaining: u32, // sectors remaining
    bytes_sent: u32,
    // Sense data for REQUEST SENSE
    sense_key: u8,
    sense_asc: u8,
    sense_ascq: u8,
    // Sector buffer
    sector_buf: [u8; 512],
    sector_offset: usize,
}

impl MscClass {
    fn new() -> Self {
        Self {
            state: MscState::Idle,
            tag: 0,
            data_len: 0,
            read_lba: 0,
            read_remaining: 0,
            bytes_sent: 0,
            sense_key: 0,
            sense_asc: 0,
            sense_ascq: 0,
            sector_buf: [0u8; 512],
            sector_offset: 0,
        }
    }

    fn set_sense(&mut self, key: u8, asc: u8, ascq: u8) {
        self.sense_key = key;
        self.sense_asc = asc;
        self.sense_ascq = ascq;
    }

    fn send_csw(&mut self, usb: &UsbBus, status: u8) {
        let residue = self.data_len.saturating_sub(self.bytes_sent);
        let mut csw = [0u8; 13];
        csw[0..4].copy_from_slice(&CSW_SIGNATURE.to_le_bytes());
        csw[4..8].copy_from_slice(&self.tag.to_le_bytes());
        csw[8..12].copy_from_slice(&residue.to_le_bytes());
        csw[12] = status;
        usb.ep_write(1, &csw);
        self.state = MscState::Idle;
    }

    fn send_data_in(&mut self, usb: &UsbBus, data: &[u8]) {
        let len = data.len().min(self.data_len as usize);
        usb.ep_write(1, &data[..len]);
        self.bytes_sent = len as u32;
        self.state = MscState::SendCsw;
    }

    fn process_cbw(&mut self, data: &[u8], usb: &UsbBus) {
        if data.len() < 31 {
            return;
        }
        let sig = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        if sig != CBW_SIGNATURE {
            return;
        }

        self.tag = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        self.data_len = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
        let dir_in = data[12] & 0x80 != 0;
        let cb_len = data[14] & 0x1F;
        let cb = &data[15..15 + cb_len as usize];
        let opcode = cb[0];

        self.bytes_sent = 0;

        defmt::debug!("SCSI cmd={:02x} len={} dir_in={}", opcode, self.data_len, dir_in);

        match opcode {
            SCSI_TEST_UNIT_READY => {
                self.set_sense(0, 0, 0);
                self.send_csw(usb, CSW_STATUS_PASSED);
            }

            SCSI_REQUEST_SENSE => {
                let alloc = (cb[4] as usize).min(18);
                let mut resp = [0u8; 18];
                resp[0] = 0x70; // current errors
                resp[2] = self.sense_key;
                resp[7] = 10; // additional sense length
                resp[12] = self.sense_asc;
                resp[13] = self.sense_ascq;
                self.send_data_in(usb, &resp[..alloc]);
            }

            SCSI_INQUIRY => {
                #[rustfmt::skip]
                let resp: [u8; 36] = [
                    0x00,       // peripheral type: direct access
                    0x80,       // RMB: removable
                    0x04,       // version: SPC-2
                    0x02,       // response data format
                    0x1F,       // additional length (36 - 5)
                    0x00, 0x00, 0x00,
                    // Vendor (8 bytes)
                    b'E', b'F', b'M', b'3', b'2', b' ', b' ', b' ',
                    // Product (16 bytes)
                    b'U', b'S', b'B', b' ', b'D', b'r', b'i', b'v',
                    b'e', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
                    // Revision (4 bytes)
                    b'1', b'.', b'0', b'0',
                ];
                self.send_data_in(usb, &resp);
            }

            SCSI_READ_CAPACITY_10 => {
                let mut resp = [0u8; 8];
                resp[0..4].copy_from_slice(&LAST_LBA.to_be_bytes());
                resp[4..8].copy_from_slice(&SECTOR_SIZE.to_be_bytes());
                self.send_data_in(usb, &resp);
            }

            SCSI_READ_10 => {
                let lba = u32::from_be_bytes([cb[2], cb[3], cb[4], cb[5]]);
                let count = u16::from_be_bytes([cb[7], cb[8]]) as u32;
                defmt::debug!("READ(10) lba={} count={}", lba, count);

                self.read_lba = lba;
                self.read_remaining = count;
                self.state = MscState::DataIn;

                // Start sending the first chunk.
                self.send_next_read_chunk(usb);
            }

            SCSI_WRITE_10 => {
                // Read-only device: fail with write-protect sense.
                self.set_sense(0x07, 0x27, 0x00);
                if self.data_len > 0 {
                    // Must accept (and discard) the data phase before CSW.
                    self.state = MscState::DataOut;
                } else {
                    self.send_csw(usb, CSW_STATUS_FAILED);
                }
            }

            SCSI_MODE_SENSE_6 => {
                // 4-byte header with write-protect bit set.
                let resp: [u8; 4] = [
                    0x03, // mode data length (3 bytes follow)
                    0x00, // medium type
                    0x80, // device-specific: write-protected
                    0x00, // block descriptor length
                ];
                self.send_data_in(usb, &resp);
            }

            SCSI_START_STOP_UNIT | SCSI_PREVENT_ALLOW_MEDIUM_REMOVAL => {
                self.send_csw(usb, CSW_STATUS_PASSED);
            }

            SCSI_READ_FORMAT_CAPACITIES => {
                // 12-byte response: 4-byte header + 8-byte descriptor.
                let mut resp = [0u8; 12];
                resp[3] = 8; // capacity list length
                resp[4..8].copy_from_slice(&TOTAL_SECTORS.to_be_bytes());
                resp[8] = 0x02; // formatted media
                // Block size 512 in bytes 9-11 (big-endian, 3 bytes)
                resp[10] = 0x02;
                resp[11] = 0x00;
                self.send_data_in(usb, &resp);
            }

            _ => {
                defmt::warn!("Unknown SCSI cmd {:02x}", opcode);
                self.set_sense(0x05, 0x20, 0x00); // illegal request, invalid command
                if dir_in && self.data_len > 0 {
                    // Host expects data we can't provide — send zeros.
                    let zero = [0u8; 64];
                    let len = (self.data_len as usize).min(64);
                    usb.ep_write(1, &zero[..len]);
                    self.bytes_sent = len as u32;
                    self.state = MscState::SendCsw;
                } else if !dir_in && self.data_len > 0 {
                    self.state = MscState::DataOut;
                } else {
                    self.send_csw(usb, CSW_STATUS_FAILED);
                }
            }
        }
    }

    /// Continue a READ(10) transfer: generate and send the next 64-byte chunk.
    fn send_next_read_chunk(&mut self, usb: &UsbBus) {
        if self.read_remaining == 0 {
            self.state = MscState::SendCsw;
            self.send_csw(usb, CSW_STATUS_PASSED);
            return;
        }

        // Generate sector if we're at the start.
        if self.sector_offset == 0 {
            generate_sector(self.read_lba, &mut self.sector_buf);
        }

        // Send up to 64 bytes from the current sector.
        let chunk_end = (self.sector_offset + 64).min(512);
        let chunk = &self.sector_buf[self.sector_offset..chunk_end];
        usb.ep_write(1, chunk);
        self.bytes_sent += chunk.len() as u32;
        self.sector_offset = chunk_end;

        if self.sector_offset >= 512 {
            self.sector_offset = 0;
            self.read_lba += 1;
            self.read_remaining -= 1;
        }

        // If more data remains, stay in DataIn; in_complete will call us again.
        if self.read_remaining == 0 && self.sector_offset == 0 {
            self.state = MscState::SendCsw;
        }
    }
}

impl UsbClass for MscClass {
    fn device_descriptor(&self) -> &[u8] {
        &DEVICE_DESC
    }
    fn config_descriptor(&self) -> &[u8] {
        &CONFIG_DESC
    }
    fn string_descriptor(&self, index: u8) -> Option<&[u8]> {
        match index {
            1 => Some(&STRING1),
            2 => Some(&STRING2),
            _ => None,
        }
    }

    fn handle_setup(&mut self, setup: &SetupPacket, _usb: &UsbBus) -> SetupResult {
        // MSC Bulk-Only Mass Storage Reset (class request, interface recipient)
        const MASS_STORAGE_RESET: u8 = 0xFF;
        // Get Max LUN
        const GET_MAX_LUN: u8 = 0xFE;

        match (setup.bm_request_type, setup.b_request) {
            (0x21, MASS_STORAGE_RESET) => {
                defmt::info!("Mass Storage Reset");
                self.state = MscState::Idle;
                SetupResult::Handled
            }
            (0xA1, GET_MAX_LUN) => {
                defmt::info!("GET_MAX_LUN");
                _usb.ep0_write(&[0x00], 1); // LUN 0 only
                SetupResult::DataIn
            }
            _ => SetupResult::Unhandled,
        }
    }

    fn data_out(&mut self, ep: u8, data: &[u8], usb: &UsbBus) {
        if ep != 1 {
            return;
        }

        match self.state {
            MscState::Idle => {
                // Expect a CBW.
                self.process_cbw(data, usb);
            }
            MscState::DataOut => {
                // Discard write data, then send failing CSW.
                self.bytes_sent += data.len() as u32;
                if self.bytes_sent >= self.data_len {
                    self.send_csw(usb, CSW_STATUS_FAILED);
                }
            }
            _ => {}
        }
    }

    fn in_complete(&mut self, ep: u8, usb: &UsbBus) {
        if ep != 1 {
            return;
        }

        match self.state {
            MscState::DataIn => {
                self.send_next_read_chunk(usb);
            }
            MscState::SendCsw => {
                self.send_csw(usb, CSW_STATUS_PASSED);
            }
            _ => {}
        }
    }

    fn configured(&mut self, _usb: &UsbBus) {
        CONFIGURED.store(true, Ordering::Release);
        defmt::info!("MSC device configured");
    }

    fn reset(&mut self) {
        CONFIGURED.store(false, Ordering::Release);
        self.state = MscState::Idle;
    }

    fn suspended(&mut self) {
        CONFIGURED.store(false, Ordering::Release);
    }
}

// ---------------------------------------------------------------------------
// Global state
// ---------------------------------------------------------------------------

static USB_DEV: Mutex<RefCell<Option<UsbDevice<MscClass>>>> = Mutex::new(RefCell::new(None));

// ---------------------------------------------------------------------------
// USB interrupt handler
// ---------------------------------------------------------------------------

#[interrupt]
fn USB() {
    critical_section::with(|lock| {
        if let Some(dev) = USB_DEV.borrow(lock).borrow_mut().deref_mut() {
            dev.poll();
        }
    });
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    p.wdog.ctrl().write(|w| w.en().clear_bit());

    let dev = UsbDevice::init(
        &p.cmu,
        &p.usb,
        MscClass::new(),
        UsbConfig {
            rx_fifo_words: 64,
            tx0_fifo_words: 24,
            tx1_fifo_words: 64,
            tx2_fifo_words: 0,
            ep1: Some(EpConfig {
                ep_type: EpType::Bulk,
                mps: 64,
                has_in: true,
                has_out: true,
            }),
            ep2: None,
        },
    );

    critical_section::with(|lock| {
        USB_DEV.borrow(lock).replace(Some(dev));
    });

    pac::NVIC::unpend(pac::Interrupt::USB);
    unsafe { pac::NVIC::unmask(pac::Interrupt::USB) };

    defmt::info!("USB Mass Storage ready — connect cable now");

    loop {
        cortex_m::asm::wfi();
    }
}
