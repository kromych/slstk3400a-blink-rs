#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    readctrl: Readctrl,
    writectrl: Writectrl,
    writecmd: Writecmd,
    addrb: Addrb,
    _reserved5: [u8; 0x04],
    wdata: Wdata,
    status: Status,
    _reserved7: [u8; 0x0c],
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    lock: Lock,
    cmd: Cmd,
    cachehits: Cachehits,
    cachemisses: Cachemisses,
    _reserved15: [u8; 0x04],
    timebase: Timebase,
    masslock: Masslock,
    irqlatency: Irqlatency,
}
impl RegisterBlock {
    #[doc = "0x00 - Memory System Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Read Control Register"]
    #[inline(always)]
    pub const fn readctrl(&self) -> &Readctrl {
        &self.readctrl
    }
    #[doc = "0x08 - Write Control Register"]
    #[inline(always)]
    pub const fn writectrl(&self) -> &Writectrl {
        &self.writectrl
    }
    #[doc = "0x0c - Write Command Register"]
    #[inline(always)]
    pub const fn writecmd(&self) -> &Writecmd {
        &self.writecmd
    }
    #[doc = "0x10 - Page Erase/Write Address Buffer"]
    #[inline(always)]
    pub const fn addrb(&self) -> &Addrb {
        &self.addrb
    }
    #[doc = "0x18 - Write Data Register"]
    #[inline(always)]
    pub const fn wdata(&self) -> &Wdata {
        &self.wdata
    }
    #[doc = "0x1c - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x2c - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x30 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x34 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x38 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x3c - Configuration Lock Register"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x40 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x44 - Cache Hits Performance Counter"]
    #[inline(always)]
    pub const fn cachehits(&self) -> &Cachehits {
        &self.cachehits
    }
    #[doc = "0x48 - Cache Misses Performance Counter"]
    #[inline(always)]
    pub const fn cachemisses(&self) -> &Cachemisses {
        &self.cachemisses
    }
    #[doc = "0x50 - Flash Write and Erase Timebase"]
    #[inline(always)]
    pub const fn timebase(&self) -> &Timebase {
        &self.timebase
    }
    #[doc = "0x54 - Mass Erase Lock Register"]
    #[inline(always)]
    pub const fn masslock(&self) -> &Masslock {
        &self.masslock
    }
    #[doc = "0x58 - Irq Latency Register"]
    #[inline(always)]
    pub const fn irqlatency(&self) -> &Irqlatency {
        &self.irqlatency
    }
}
#[doc = "CTRL (rw) register accessor: Memory System Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Memory System Control Register"]
pub mod ctrl;
#[doc = "READCTRL (rw) register accessor: Read Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`readctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readctrl`] module"]
#[doc(alias = "READCTRL")]
pub type Readctrl = crate::Reg<readctrl::ReadctrlSpec>;
#[doc = "Read Control Register"]
pub mod readctrl;
#[doc = "WRITECTRL (rw) register accessor: Write Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`writectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writectrl`] module"]
#[doc(alias = "WRITECTRL")]
pub type Writectrl = crate::Reg<writectrl::WritectrlSpec>;
#[doc = "Write Control Register"]
pub mod writectrl;
#[doc = "WRITECMD (w) register accessor: Write Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writecmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writecmd`] module"]
#[doc(alias = "WRITECMD")]
pub type Writecmd = crate::Reg<writecmd::WritecmdSpec>;
#[doc = "Write Command Register"]
pub mod writecmd;
#[doc = "ADDRB (rw) register accessor: Page Erase/Write Address Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`addrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrb`] module"]
#[doc(alias = "ADDRB")]
pub type Addrb = crate::Reg<addrb::AddrbSpec>;
#[doc = "Page Erase/Write Address Buffer"]
pub mod addrb;
#[doc = "WDATA (rw) register accessor: Write Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdata`] module"]
#[doc(alias = "WDATA")]
pub type Wdata = crate::Reg<wdata::WdataSpec>;
#[doc = "Write Data Register"]
pub mod wdata;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "IF (r) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS (w) register accessor: Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifs`] module"]
#[doc(alias = "IFS")]
pub type Ifs = crate::Reg<ifs::IfsSpec>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifc`] module"]
#[doc(alias = "IFC")]
pub type Ifc = crate::Reg<ifc::IfcSpec>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "LOCK (rw) register accessor: Configuration Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "CACHEHITS (r) register accessor: Cache Hits Performance Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cachehits::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cachehits`] module"]
#[doc(alias = "CACHEHITS")]
pub type Cachehits = crate::Reg<cachehits::CachehitsSpec>;
#[doc = "Cache Hits Performance Counter"]
pub mod cachehits;
#[doc = "CACHEMISSES (r) register accessor: Cache Misses Performance Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cachemisses::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cachemisses`] module"]
#[doc(alias = "CACHEMISSES")]
pub type Cachemisses = crate::Reg<cachemisses::CachemissesSpec>;
#[doc = "Cache Misses Performance Counter"]
pub mod cachemisses;
#[doc = "TIMEBASE (rw) register accessor: Flash Write and Erase Timebase\n\nYou can [`read`](crate::Reg::read) this register and get [`timebase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timebase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timebase`] module"]
#[doc(alias = "TIMEBASE")]
pub type Timebase = crate::Reg<timebase::TimebaseSpec>;
#[doc = "Flash Write and Erase Timebase"]
pub mod timebase;
#[doc = "MASSLOCK (rw) register accessor: Mass Erase Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`masslock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`masslock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@masslock`] module"]
#[doc(alias = "MASSLOCK")]
pub type Masslock = crate::Reg<masslock::MasslockSpec>;
#[doc = "Mass Erase Lock Register"]
pub mod masslock;
#[doc = "IRQLATENCY (rw) register accessor: Irq Latency Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irqlatency::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqlatency::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqlatency`] module"]
#[doc(alias = "IRQLATENCY")]
pub type Irqlatency = crate::Reg<irqlatency::IrqlatencySpec>;
#[doc = "Irq Latency Register"]
pub mod irqlatency;
