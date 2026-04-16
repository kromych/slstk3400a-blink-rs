# efm32hg322_usb — DWC2 USB Device Driver for EFM32HG322

Bare-metal USB device driver for the EFM32 Happy Gecko (Cortex-M0+, EFM32HG322F64).
Uses the on-chip DWC2 OTG core in slave (FIFO) mode by default, with an optional
`dma` feature for the DWC2 internal DMA engine.

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
| DMA support | Internal DMA via `feature = "dma"` |
| VBUS detection | `USB->CTRL.VREGOSEN` + `USB->STATUS.VREGOS` |
| OTG | No — device mode only |
| D+ pull-up | `USB->ROUTE.DMPUPEN` (explicit pull-up bit) |

### EFM32HG-specific init

```mermaid
sequenceDiagram
    participant FW as Firmware
    participant CMU as CMU (Clocks)
    participant USB as USB / DWC2

    FW->>CMU: Enable HFCORECLKEN0.USBC + USB + LE
    FW->>CMU: Enable LFCCLKEN0.USBLE, LFC = LFRCO
    FW->>CMU: USHFRCO 48 MHz band + clock recovery
    FW->>CMU: CMD.USBCCLKSEL = USHFRCO
    FW->>USB: CTRL.LEMOSCCTRL = NONE
    FW->>USB: ROUTE = PHYPEN | DMPUPEN
    FW->>USB: Clear PCGCCTL
    FW->>USB: GRSTCTL.CSFTRST (soft-reset)
    USB-->>FW: GRSTCTL.AHBIDLE
    FW->>USB: GAHBCFG.GLBLINTRMSK = 1
    FW->>USB: DCFG.DEVSPD = FS, DEVADDR = 0
    FW->>USB: Set EP interrupt masks, allocate FIFOs
    FW->>USB: GINTMSK (USBRST, ENUMDONE, USBSUSP, IEPINT, OEPINT, RXFLVL)
    FW->>USB: GINTSTS = 0xFFFFFFFF (clear pending)
    FW->>USB: Toggle DCTL.PWRONPRGDONE
    FW->>USB: Clear DCTL.SFTDISCON (connect)
```

No VBUS detection needed — the HG is typically bus-powered with VBUS always present.

## SETUP Packet Flow — FIFO (Slave) Mode

```mermaid
sequenceDiagram
    participant Host
    participant DWC2
    participant ISR as poll()
    participant Class as UsbClass

    Host->>DWC2: SETUP token + 8 data bytes
    DWC2->>ISR: GINTSTS.RXFLVL
    ISR->>DWC2: Read GRXSTSP (pktsts=0x6 SETUP_DATA)
    ISR->>DWC2: Read 2 words from RX FIFO
    Note over ISR: Parse SetupPacket

    DWC2->>ISR: GINTSTS.RXFLVL (pktsts=0x4 SETUP_COMPL)
    ISR->>DWC2: ep0_prepare_out()

    DWC2->>ISR: GINTSTS.OEPINT (DOEP0INT.SETUP)
    Note over ISR: Ignored (already handled in RXFLVL)

    ISR->>Class: handle_setup()

    alt Handled (e.g. SET_ADDRESS)
        ISR->>DWC2: ep0_write_packet(ZLP)
    else DataIn (e.g. GET_DESCRIPTOR)
        Class->>DWC2: ep0_write_packet(data)
    else DataOut (e.g. SET_LINE_CODING)
        ISR->>DWC2: ep0_prepare_out()
        Host->>DWC2: DATA OUT payload
        DWC2->>ISR: RXFLVL (pktsts=0x2 OUT_DATA)
        DWC2->>ISR: OEPINT.XFERCOMPL
        ISR->>Class: ep0_data_out(data)
        ISR->>DWC2: ep0_write_packet(ZLP)
    else Unhandled
        ISR->>DWC2: STALL EP0
    end
```

## SETUP Packet Flow — DMA Mode

EP0 OUT is always armed with **SNAK** so that only SETUP packets (which
bypass NAK on the DWC2) can arrive.  DATA OUT is gated by an explicit
`ep0_clear_out_nak()` call after the SETUP has been read and processed,
preventing a host DATA OUT from overwriting the SETUP data in the shared
DMA buffer before the ISR can parse it.

```mermaid
sequenceDiagram
    participant Host
    participant DWC2
    participant ISR as poll()
    participant Class as UsbClass

    Note over DWC2: EP0 OUT armed with SNAK<br/>(SETUP bypasses NAK)
    Host->>DWC2: SETUP token + 8 data bytes
    Note over DWC2: DMA writes 8 bytes to EP0 OUT buffer

    DWC2->>ISR: OEPINT (setup + xfercompl both set)
    ISR->>DWC2: Read SETUP from DMA buffer
    Note over ISR: Parse SetupPacket
    ISR->>Class: handle_setup()

    Note over ISR: xfercompl is from SETUP DMA,<br/>not a DATA OUT payload

    alt Handled
        ISR->>DWC2: ep0_write_packet_dma(ZLP)
    else DataIn
        Class->>DWC2: ep0_write_packet_dma(data)
    else DataOut
        Note over ISR: setup bit set → skip xfercompl
        ISR->>DWC2: ep0_prepare_out_dma() [SNAK]
        ISR->>DWC2: ep0_clear_out_nak() [CNAK]
        Note over DWC2: NAK cleared — DATA OUT can arrive
        Host->>DWC2: DATA OUT payload
        Note over DWC2: DMA writes payload to EP0 OUT buffer
        DWC2->>ISR: OEPINT.XFERCOMPL (setup NOT set)
        ISR->>DWC2: Read actual byte count from DOEP0TSIZ
        ISR->>Class: ep0_data_out(data)
        ISR->>DWC2: ep0_write_packet_dma(ZLP)
    else Unhandled
        ISR->>DWC2: STALL EP0
    end
```

## Multi-Packet EP0 IN Transfer

`DIEP0TSIZ.xfersize` is only 7 bits wide (max 127), so descriptors
larger than 64 bytes must be sent one packet at a time in both modes.

```mermaid
sequenceDiagram
    participant FW as ep0_start_in
    participant Bus as UsbBus
    participant Host

    FW->>Bus: ep0_write_packet(data[0..64])
    Note over FW: Save pointer + remaining

    loop While ep0_in_remaining > 0
        Bus-->>Host: 64-byte DATA packet
        Host-->>Bus: ACK
        Bus->>FW: IEPINT.XFERCOMPL
        FW->>Bus: ep0_continue_in → ep0_write_packet(next chunk)
    end

    Bus-->>Host: Final packet (may be short / ZLP)
    Host-->>Bus: ACK
    Bus->>FW: IEPINT.XFERCOMPL

    alt FIFO mode
        FW->>Bus: ep0_prepare_out() [CNAK]
    else DMA mode
        FW->>Bus: ep0_prepare_out() [SNAK]
        FW->>Bus: ep0_clear_out_nak() [CNAK]
        Note over FW: CNAK after prepare so status<br/>ZLP / next SETUP can arrive
    end
```

## IN Transfer Flow (EPn)

Isochronous IN endpoints are activated with `usbactep` deferred: the bit
is **not** set in `activate_endpoints()` so the DWC2 NAKs host IN tokens
until the first `ep_write()` supplies real data.  This prevents the
controller from transmitting stale/zero data (which appears as a green
flash in YUY2 video).

```mermaid
sequenceDiagram
    participant Class as UsbClass
    participant Bus as UsbBus
    participant DWC2
    participant Host

    Note over Bus: Iso IN: usbactep deferred<br/>until first ep_write

    Class->>Bus: ep_write(ep, data)

    alt DMA mode
        Note over Bus: Copy data to DMA buffer
        Note over Bus: DSB (flush write buffer)
        Bus->>DWC2: DIEPnDMAADDR
    else FIFO mode
        Note over Bus: (data written to TX FIFO after EPENA)
    end

    Bus->>DWC2: DIEPnTSIZ + DIEPnCTL (USBACTEP, EPENA, CNAK)
    Host->>DWC2: IN token
    DWC2-->>Host: Data packet
    DWC2->>Class: IEPINT.XFERCOMPL → in_complete(ep)
```

## OUT Transfer Flow (EPn)

```mermaid
sequenceDiagram
    participant Class as UsbClass
    participant Bus as UsbBus
    participant DWC2
    participant Host

    Bus->>DWC2: DOEPnTSIZ + DOEPnCTL (EPENA, CNAK)
    Note over DWC2: EP armed for reception

    Host->>DWC2: OUT token + data

    alt FIFO mode
        DWC2->>Bus: RXFLVL (pktsts=0x2)
        Bus->>Bus: Read data from RX FIFO
        Bus->>Class: data_out(ep, data)
        DWC2->>Bus: OEPINT.XFERCOMPL
        Bus->>DWC2: Re-arm EP (ep_prepare_out)
    else DMA mode
        Note over DWC2: DMA writes to EPn OUT buffer
        DWC2->>Bus: OEPINT.XFERCOMPL
        Bus->>Bus: Read byte count from DOEPnTSIZ
        Bus->>Class: data_out(ep, data)
        Bus->>DWC2: Re-arm EP (ep_prepare_out_dma)
    end
```

## Architecture

```mermaid
graph TD
    subgraph "USB ISR (poll)"
        GINTSTS["GINTSTS dispatch"]
        RXFLVL["RXFLVL<br/><i>FIFO mode only</i>"]
        OEPINT["OEPINT<br/>SETUP + OUT completion"]
        IEPINT["IEPINT<br/>IN completion"]
        RST["USBRST / ENUMDONE"]
        SUSP["USBSUSP"]
    end

    subgraph "UsbDevice&lt;C: UsbClass&gt;"
        STATE["EP0 state<br/>(pending_data_out,<br/>ep0_in_ptr, ep0_in_remaining)"]
        SETUP["handle_setup()"]
    end

    subgraph UsbBus
        direction LR
        EP0W["ep0_write_packet"]
        EP0P["ep0_prepare_out"]
        EPW["ep_write"]
        EPP["ep_prepare_out"]
    end

    subgraph "UsbClass (trait)"
        CDC["CdcAcmClass"]
        HID["HidKeyboardClass"]
        VIDEO["VideoClass"]
        MIDI["MidiClass"]
        AUDIO["AudioClass"]
        MSC["MscClass"]
    end

    GINTSTS --> RXFLVL
    GINTSTS --> OEPINT
    GINTSTS --> IEPINT
    GINTSTS --> RST
    GINTSTS --> SUSP

    OEPINT --> SETUP
    SETUP --> EP0W
    SETUP --> EP0P

    OEPINT -->|"data_out()"| CDC
    IEPINT -->|"in_complete()"| CDC

    EP0W -->|"FIFO mode"| FIFO["write_fifo()"]
    EP0W -->|"DMA mode"| DMA["DMA buffer copy"]
    EPW -->|"FIFO mode"| FIFO
    EPW -->|"DMA mode"| DMA
```

## Feature Flags

| Feature | Effect |
|---|---|
| *(default)* | FIFO (slave) mode — CPU reads/writes FIFOs directly |
| `dma` | DWC2 internal DMA mode — AHB DMA master handles transfers; uses ~1.3 KiB static RAM for DMA buffers |

In DMA mode, the public `UsbBus` API (`ep_write`, `ep_prepare_out`,
`ep0_write_packet`, `ep0_prepare_out`) dispatches to DMA internally,
so class drivers work transparently in both modes.
