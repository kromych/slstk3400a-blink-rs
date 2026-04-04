#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    cmd: Cmd,
    status: Status,
    singlectrl: Singlectrl,
    scanctrl: Scanctrl,
    ien: Ien,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    singledata: Singledata,
    scandata: Scandata,
    singledatap: Singledatap,
    scandatap: Scandatap,
    cal: Cal,
    _reserved14: [u8; 0x04],
    biasprog: Biasprog,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x08 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x0c - Single Sample Control Register"]
    #[inline(always)]
    pub const fn singlectrl(&self) -> &Singlectrl {
        &self.singlectrl
    }
    #[doc = "0x10 - Scan Control Register"]
    #[inline(always)]
    pub const fn scanctrl(&self) -> &Scanctrl {
        &self.scanctrl
    }
    #[doc = "0x14 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x18 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x1c - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x20 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x24 - Single Conversion Result Data"]
    #[inline(always)]
    pub const fn singledata(&self) -> &Singledata {
        &self.singledata
    }
    #[doc = "0x28 - Scan Conversion Result Data"]
    #[inline(always)]
    pub const fn scandata(&self) -> &Scandata {
        &self.scandata
    }
    #[doc = "0x2c - Single Conversion Result Data Peek Register"]
    #[inline(always)]
    pub const fn singledatap(&self) -> &Singledatap {
        &self.singledatap
    }
    #[doc = "0x30 - Scan Sequence Result Data Peek Register"]
    #[inline(always)]
    pub const fn scandatap(&self) -> &Scandatap {
        &self.scandatap
    }
    #[doc = "0x34 - Calibration Register"]
    #[inline(always)]
    pub const fn cal(&self) -> &Cal {
        &self.cal
    }
    #[doc = "0x3c - Bias Programming Register"]
    #[inline(always)]
    pub const fn biasprog(&self) -> &Biasprog {
        &self.biasprog
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "SINGLECTRL (rw) register accessor: Single Sample Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`singlectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singlectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singlectrl`] module"]
#[doc(alias = "SINGLECTRL")]
pub type Singlectrl = crate::Reg<singlectrl::SinglectrlSpec>;
#[doc = "Single Sample Control Register"]
pub mod singlectrl;
#[doc = "SCANCTRL (rw) register accessor: Scan Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scanctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scanctrl`] module"]
#[doc(alias = "SCANCTRL")]
pub type Scanctrl = crate::Reg<scanctrl::ScanctrlSpec>;
#[doc = "Scan Control Register"]
pub mod scanctrl;
#[doc = "IEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
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
#[doc = "SINGLEDATA (r) register accessor: Single Conversion Result Data\n\nYou can [`read`](crate::Reg::read) this register and get [`singledata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@singledata`] module"]
#[doc(alias = "SINGLEDATA")]
pub type Singledata = crate::Reg<singledata::SingledataSpec>;
#[doc = "Single Conversion Result Data"]
pub mod singledata;
#[doc = "SCANDATA (r) register accessor: Scan Conversion Result Data\n\nYou can [`read`](crate::Reg::read) this register and get [`scandata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@scandata`] module"]
#[doc(alias = "SCANDATA")]
pub type Scandata = crate::Reg<scandata::ScandataSpec>;
#[doc = "Scan Conversion Result Data"]
pub mod scandata;
#[doc = "SINGLEDATAP (r) register accessor: Single Conversion Result Data Peek Register\n\nYou can [`read`](crate::Reg::read) this register and get [`singledatap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singledatap`] module"]
#[doc(alias = "SINGLEDATAP")]
pub type Singledatap = crate::Reg<singledatap::SingledatapSpec>;
#[doc = "Single Conversion Result Data Peek Register"]
pub mod singledatap;
#[doc = "SCANDATAP (r) register accessor: Scan Sequence Result Data Peek Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scandatap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scandatap`] module"]
#[doc(alias = "SCANDATAP")]
pub type Scandatap = crate::Reg<scandatap::ScandatapSpec>;
#[doc = "Scan Sequence Result Data Peek Register"]
pub mod scandatap;
#[doc = "CAL (rw) register accessor: Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal`] module"]
#[doc(alias = "CAL")]
pub type Cal = crate::Reg<cal::CalSpec>;
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "BIASPROG (rw) register accessor: Bias Programming Register\n\nYou can [`read`](crate::Reg::read) this register and get [`biasprog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`biasprog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@biasprog`] module"]
#[doc(alias = "BIASPROG")]
pub type Biasprog = crate::Reg<biasprog::BiasprogSpec>;
#[doc = "Bias Programming Register"]
pub mod biasprog;
