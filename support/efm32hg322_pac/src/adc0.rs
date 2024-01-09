#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    cmd: CMD,
    status: STATUS,
    singlectrl: SINGLECTRL,
    scanctrl: SCANCTRL,
    ien: IEN,
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    singledata: SINGLEDATA,
    scandata: SCANDATA,
    singledatap: SINGLEDATAP,
    scandatap: SCANDATAP,
    cal: CAL,
    _reserved14: [u8; 0x04],
    biasprog: BIASPROG,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x08 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x0c - Single Sample Control Register"]
    #[inline(always)]
    pub const fn singlectrl(&self) -> &SINGLECTRL {
        &self.singlectrl
    }
    #[doc = "0x10 - Scan Control Register"]
    #[inline(always)]
    pub const fn scanctrl(&self) -> &SCANCTRL {
        &self.scanctrl
    }
    #[doc = "0x14 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0x18 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0x1c - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0x20 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
    #[doc = "0x24 - Single Conversion Result Data"]
    #[inline(always)]
    pub const fn singledata(&self) -> &SINGLEDATA {
        &self.singledata
    }
    #[doc = "0x28 - Scan Conversion Result Data"]
    #[inline(always)]
    pub const fn scandata(&self) -> &SCANDATA {
        &self.scandata
    }
    #[doc = "0x2c - Single Conversion Result Data Peek Register"]
    #[inline(always)]
    pub const fn singledatap(&self) -> &SINGLEDATAP {
        &self.singledatap
    }
    #[doc = "0x30 - Scan Sequence Result Data Peek Register"]
    #[inline(always)]
    pub const fn scandatap(&self) -> &SCANDATAP {
        &self.scandatap
    }
    #[doc = "0x34 - Calibration Register"]
    #[inline(always)]
    pub const fn cal(&self) -> &CAL {
        &self.cal
    }
    #[doc = "0x3c - Bias Programming Register"]
    #[inline(always)]
    pub const fn biasprog(&self) -> &BIASPROG {
        &self.biasprog
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "SINGLECTRL (rw) register accessor: Single Sample Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`singlectrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`singlectrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singlectrl`]
module"]
pub type SINGLECTRL = crate::Reg<singlectrl::SINGLECTRL_SPEC>;
#[doc = "Single Sample Control Register"]
pub mod singlectrl;
#[doc = "SCANCTRL (rw) register accessor: Scan Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scanctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scanctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scanctrl`]
module"]
pub type SCANCTRL = crate::Reg<scanctrl::SCANCTRL_SPEC>;
#[doc = "Scan Control Register"]
pub mod scanctrl;
#[doc = "IEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`]
module"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "IF (r) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`]
module"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS (w) register accessor: Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifs`]
module"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifc`]
module"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "SINGLEDATA (r) register accessor: Single Conversion Result Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`singledata::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singledata`]
module"]
pub type SINGLEDATA = crate::Reg<singledata::SINGLEDATA_SPEC>;
#[doc = "Single Conversion Result Data"]
pub mod singledata;
#[doc = "SCANDATA (r) register accessor: Scan Conversion Result Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scandata::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scandata`]
module"]
pub type SCANDATA = crate::Reg<scandata::SCANDATA_SPEC>;
#[doc = "Scan Conversion Result Data"]
pub mod scandata;
#[doc = "SINGLEDATAP (r) register accessor: Single Conversion Result Data Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`singledatap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singledatap`]
module"]
pub type SINGLEDATAP = crate::Reg<singledatap::SINGLEDATAP_SPEC>;
#[doc = "Single Conversion Result Data Peek Register"]
pub mod singledatap;
#[doc = "SCANDATAP (r) register accessor: Scan Sequence Result Data Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scandatap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scandatap`]
module"]
pub type SCANDATAP = crate::Reg<scandatap::SCANDATAP_SPEC>;
#[doc = "Scan Sequence Result Data Peek Register"]
pub mod scandatap;
#[doc = "CAL (rw) register accessor: Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal`]
module"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "BIASPROG (rw) register accessor: Bias Programming Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`biasprog::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`biasprog::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@biasprog`]
module"]
pub type BIASPROG = crate::Reg<biasprog::BIASPROG_SPEC>;
#[doc = "Bias Programming Register"]
pub mod biasprog;
