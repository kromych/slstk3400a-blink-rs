//! USB Mass Storage Class (Bulk-Only Transport) driver.
//!
//! Handles the SCSI Transparent command set over BBB. The actual storage
//! medium is provided through the [`MscHandler`] trait - the demo supplies
//! sector data; the library handles the protocol.

use crate::{usb_string, EpConfig, EpType, SetupPacket, SetupResult, UsbBus, UsbClass, UsbConfig};
use portable_atomic::{AtomicBool, Ordering};

/// Set once the host completes SET_CONFIGURATION.
pub static CONFIGURED: AtomicBool = AtomicBool::new(false);

/// Non-blocking check.
pub fn is_configured() -> bool {
    CONFIGURED.load(Ordering::Acquire)
}

const SECTOR_SIZE: u32 = 512;

// ---------------------------------------------------------------------------
// Handler trait
// ---------------------------------------------------------------------------

/// Application-provided storage medium for the MSC device.
pub trait MscHandler {
    /// Total number of 512-byte sectors.
    fn capacity(&self) -> u32;

    /// Fill `buf` with the contents of sector `lba`.
    fn read_sector(&mut self, lba: u32, buf: &mut [u8; 512]);
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

static STRING1: [u8; 2 + 10 * 2] = usb_string!("SLSTK3400A");
static STRING2: [u8; 2 + 15 * 2] = usb_string!("EFM32 USB Drive");

// ---------------------------------------------------------------------------
// SCSI / BBB constants
// ---------------------------------------------------------------------------

const CBW_SIGNATURE: u32 = 0x43425355; // "USBC"
const CSW_SIGNATURE: u32 = 0x53425355; // "USBS"

const CSW_STATUS_PASSED: u8 = 0;
const CSW_STATUS_FAILED: u8 = 1;

const SCSI_TEST_UNIT_READY: u8 = 0x00;
const SCSI_REQUEST_SENSE: u8 = 0x03;
const SCSI_INQUIRY: u8 = 0x12;
const SCSI_MODE_SENSE_6: u8 = 0x1A;
const SCSI_START_STOP_UNIT: u8 = 0x1B;
const SCSI_PREVENT_ALLOW_MEDIUM_REMOVAL: u8 = 0x1E;
const SCSI_READ_FORMAT_CAPACITIES: u8 = 0x23;
const SCSI_READ_CAPACITY_10: u8 = 0x25;
const SCSI_READ_10: u8 = 0x28;
const SCSI_WRITE_10: u8 = 0x2A;

// ---------------------------------------------------------------------------
// Class driver
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum MscState {
    Idle,
    DataIn,
    DataOut,
    SendCsw,
}

pub struct MscClass<H: MscHandler> {
    handler: H,
    state: MscState,
    tag: u32,
    data_len: u32,
    // READ(10) progress
    read_lba: u32,
    read_remaining: u32,
    bytes_sent: u32,
    // Sense data
    sense_key: u8,
    sense_asc: u8,
    sense_ascq: u8,
    // Sector buffer
    sector_buf: [u8; 512],
    sector_offset: usize,
}

impl<H: MscHandler> MscClass<H> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
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

        defmt::debug!(
            "SCSI cmd={:02x} len={} dir_in={}",
            opcode,
            self.data_len,
            dir_in
        );

        let total_sectors = self.handler.capacity();

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
                let last_lba = total_sectors - 1;
                let mut resp = [0u8; 8];
                resp[0..4].copy_from_slice(&last_lba.to_be_bytes());
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

                self.send_next_read_chunk(usb);
            }

            SCSI_WRITE_10 => {
                // Read-only device: fail with write-protect sense.
                self.set_sense(0x07, 0x27, 0x00);
                if self.data_len > 0 {
                    self.state = MscState::DataOut;
                } else {
                    self.send_csw(usb, CSW_STATUS_FAILED);
                }
            }

            SCSI_MODE_SENSE_6 => {
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
                let mut resp = [0u8; 12];
                resp[3] = 8; // capacity list length
                resp[4..8].copy_from_slice(&total_sectors.to_be_bytes());
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

    fn send_next_read_chunk(&mut self, usb: &UsbBus) {
        if self.read_remaining == 0 {
            self.state = MscState::SendCsw;
            self.send_csw(usb, CSW_STATUS_PASSED);
            return;
        }

        if self.sector_offset == 0 {
            self.handler
                .read_sector(self.read_lba, &mut self.sector_buf);
        }

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

        if self.read_remaining == 0 && self.sector_offset == 0 {
            self.state = MscState::SendCsw;
        }
    }
}

/// Recommended USB peripheral / FIFO configuration for MSC.
pub fn usb_config() -> UsbConfig {
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
    }
}

impl<H: MscHandler> UsbClass for MscClass<H> {
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

    fn handle_setup(&mut self, setup: &SetupPacket, usb: &UsbBus) -> SetupResult {
        const MASS_STORAGE_RESET: u8 = 0xFF;
        const GET_MAX_LUN: u8 = 0xFE;

        match (setup.bm_request_type, setup.b_request) {
            (0x21, MASS_STORAGE_RESET) => {
                defmt::info!("Mass Storage Reset");
                self.state = MscState::Idle;
                SetupResult::Handled
            }
            (0xA1, GET_MAX_LUN) => {
                defmt::info!("GET_MAX_LUN");
                usb.ep0_write_packet(&[0x00]); // LUN 0 only
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
                self.process_cbw(data, usb);
            }
            MscState::DataOut => {
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
