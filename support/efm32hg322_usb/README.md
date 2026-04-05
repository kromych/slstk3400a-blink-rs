# efm32hg322_usb — DWC2 USB Device Driver for EFM32HG322

Bare-metal USB device driver for the EFM32 Happy Gecko (Cortex-M0+, EFM32HG322F64).
Uses the on-chip DWC2 OTG core in slave (FIFO) mode with no DMA.

## DWC2 on the EFM32HG

| Property | Value |
|---|---|
| Core | Cortex-M0+ |
| DWC2 register base | `0x400C_4000` (USB peripheral base) |
| FIFO base | `USB_BASE + 0x3D000 + ep * 0x1000` |
| Device endpoints | EP0 + EP1 + EP2 (3 total) |
| FIFO RAM | 256 words (1 KB) shared |
| Max packet size (EP0) | 64 bytes |
| PHY | Integrated FS-only |
| USB clock | USHFRCO 48 MHz with SOF clock recovery |
| DMA support | No (slave/FIFO mode only) |
| VBUS detection | `USB->CTRL.VREGOSEN` + `USB->STATUS.VREGOS` |
| OTG | No — device mode only |
| D+ pull-up | `USB->ROUTE.DMPUPEN` (explicit pull-up bit) |

### EFM32HG-specific init

1. Enable clocks: `HFCORECLKEN0.USBC + USB + LE`, `LFCCLKEN0.USBLE`, select LFC = LFRCO
2. Configure USHFRCO to 48 MHz band, enable clock recovery, wait ready
3. Select USHFRCO as USB clock: `CMD.USBCCLKSEL = USHFRCO`
4. `USB->CTRL.LEMOSCCTRL = NONE` during init
5. `USB->ROUTE = PHYPEN | DMPUPEN` (enable PHY pins and D+ pull-up)
6. Clear `PCGCCTL`, soft-reset (`GRSTCTL.CSFTRST`), wait `AHBIDLE`
7. `GAHBCFG.GLBLINTRMSK = 1` (enable DWC2 core interrupts to NVIC)
8. `DCFG.DEVSPD = FS`, `DEVADDR = 0`
9. Set EP interrupt masks, allocate FIFOs
10. Set `GINTMSK` for: USBRST, ENUMDONE, USBSUSP, WKUPINT, IEPINT, OEPINT, RXFLVL
11. Clear pending: `GINTSTS = 0xFFFFFFFF`
12. Power-on handshake: toggle `DCTL.PWRONPRGDONE`
13. Connect: clear `DCTL.SFTDISCON`

No VBUS detection needed — the HG is typically bus-powered with VBUS always present.

## SETUP Packet Flow (Slave Mode)

```
Host sends SETUP token + 8 data bytes
  |
  v
GINTSTS.RXFLVL fires
  |
  +-- Read GRXSTSP: pktsts = SETUP_DATA_RECVD (0x6), epnum = 0, bcnt = 8
  +-- Read 2 words (8 bytes) from RX FIFO -> parse SetupPacket
  |
  v
GINTSTS.RXFLVL fires again
  |
  +-- Read GRXSTSP: pktsts = SETUP_COMPL (0x4), epnum = 0
  +-- (no data to read)
  |
  v
GINTSTS.OEPINT fires -> DOEP0INT.SETUP is set
  |
  +-- Clear DOEP0INT.SETUP + XFERCOMPL
  +-- Dispatch to UsbClass::handle_setup()
  +-- Based on SetupResult:
       Handled  -> ep0_write_packet(ZLP) for status stage
       DataIn   -> class already wrote response via ep0_write_packet()
       DataOut  -> ep0_prepare_out() to receive DATA stage
       Unhandled-> stall_ep0()
```

## IN Transfer Flow (EP0 and EPn)

```
Firmware writes data:
  |
  +-- Set DIEPnTSIZ: xfersize = len, pktcnt = 1
  +-- Set DIEPnCTL: EPENA + CNAK (+ frame parity for isochronous)
  +-- Write data words to FIFO[ep] (4 bytes per write)
  |
  v
Host sends IN token, DWC2 sends data from TX FIFO
  |
  v
GINTSTS.IEPINT fires -> DIEPnINT.XFERCOMPL is set
  |
  +-- Clear DIEPnINT.XFERCOMPL
  +-- For EP0: if multi-packet, send next 64-byte chunk
  +-- For EPn: call UsbClass::in_complete(ep)
```

## OUT Transfer Flow (EPn)

```
Firmware prepares for reception:
  |
  +-- Set DOEPnTSIZ: xfersize = MPS, pktcnt = 1
  +-- Set DOEPnCTL: EPENA + CNAK
  |
  v
Host sends OUT token + data
  |
  v
GINTSTS.RXFLVL fires
  |
  +-- Read GRXSTSP: pktsts = OUT_DATA_RECVD (0x2), epnum, bcnt
  +-- Read bcnt bytes from RX FIFO
  |
  v
GINTSTS.OEPINT fires -> DOEPnINT.XFERCOMPL is set
  |
  +-- Clear DOEPnINT.XFERCOMPL
  +-- Call UsbClass::data_out(ep, data)
```

## Architecture

```
UsbDevice<C: UsbClass>
  +-- UsbBus           (FIFO read/write, EP control)
  +-- C                (class driver: CDC, HID, MIDI, ...)
  +-- ep0 state        (setup buffer, multi-packet IN pointer)

USB ISR -> poll() -> dispatch GINTSTS bits:
  RXFLVL  -> read GRXSTSP, read FIFO, buffer SETUP/OUT data
  OEPINT  -> process buffered SETUP, dispatch to class
  IEPINT  -> handle IN completions, multi-packet EP0
  USBRST  -> reset state, re-enumerate
  ENUMDONE -> configure EP0, set GINTMSK
  USBSUSP -> notify class
```
