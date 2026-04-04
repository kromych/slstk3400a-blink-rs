#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    cmd: Cmd,
    status: Status,
    ien: Ien,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    top: Top,
    topb: Topb,
    cnt: Cnt,
    route: Route,
    _reserved11: [u8; 0x04],
    cc0_ctrl: Cc0Ctrl,
    cc0_ccv: Cc0Ccv,
    cc0_ccvp: Cc0Ccvp,
    cc0_ccvb: Cc0Ccvb,
    cc1_ctrl: Cc1Ctrl,
    cc1_ccv: Cc1Ccv,
    cc1_ccvp: Cc1Ccvp,
    cc1_ccvb: Cc1Ccvb,
    cc2_ctrl: Cc2Ctrl,
    cc2_ccv: Cc2Ccv,
    cc2_ccvp: Cc2Ccvp,
    cc2_ccvb: Cc2Ccvb,
    _reserved23: [u8; 0x10],
    dtctrl: Dtctrl,
    dttime: Dttime,
    dtfc: Dtfc,
    dtogen: Dtogen,
    dtfault: Dtfault,
    dtfaultc: Dtfaultc,
    dtlock: Dtlock,
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
    #[doc = "0x0c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x10 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x14 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x18 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x1c - Counter Top Value Register"]
    #[inline(always)]
    pub const fn top(&self) -> &Top {
        &self.top
    }
    #[doc = "0x20 - Counter Top Value Buffer Register"]
    #[inline(always)]
    pub const fn topb(&self) -> &Topb {
        &self.topb
    }
    #[doc = "0x24 - Counter Value Register"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x28 - I/O Routing Register"]
    #[inline(always)]
    pub const fn route(&self) -> &Route {
        &self.route
    }
    #[doc = "0x30 - CC Channel Control Register"]
    #[inline(always)]
    pub const fn cc0_ctrl(&self) -> &Cc0Ctrl {
        &self.cc0_ctrl
    }
    #[doc = "0x34 - CC Channel Value Register"]
    #[inline(always)]
    pub const fn cc0_ccv(&self) -> &Cc0Ccv {
        &self.cc0_ccv
    }
    #[doc = "0x38 - CC Channel Value Peek Register"]
    #[inline(always)]
    pub const fn cc0_ccvp(&self) -> &Cc0Ccvp {
        &self.cc0_ccvp
    }
    #[doc = "0x3c - CC Channel Buffer Register"]
    #[inline(always)]
    pub const fn cc0_ccvb(&self) -> &Cc0Ccvb {
        &self.cc0_ccvb
    }
    #[doc = "0x40 - CC Channel Control Register"]
    #[inline(always)]
    pub const fn cc1_ctrl(&self) -> &Cc1Ctrl {
        &self.cc1_ctrl
    }
    #[doc = "0x44 - CC Channel Value Register"]
    #[inline(always)]
    pub const fn cc1_ccv(&self) -> &Cc1Ccv {
        &self.cc1_ccv
    }
    #[doc = "0x48 - CC Channel Value Peek Register"]
    #[inline(always)]
    pub const fn cc1_ccvp(&self) -> &Cc1Ccvp {
        &self.cc1_ccvp
    }
    #[doc = "0x4c - CC Channel Buffer Register"]
    #[inline(always)]
    pub const fn cc1_ccvb(&self) -> &Cc1Ccvb {
        &self.cc1_ccvb
    }
    #[doc = "0x50 - CC Channel Control Register"]
    #[inline(always)]
    pub const fn cc2_ctrl(&self) -> &Cc2Ctrl {
        &self.cc2_ctrl
    }
    #[doc = "0x54 - CC Channel Value Register"]
    #[inline(always)]
    pub const fn cc2_ccv(&self) -> &Cc2Ccv {
        &self.cc2_ccv
    }
    #[doc = "0x58 - CC Channel Value Peek Register"]
    #[inline(always)]
    pub const fn cc2_ccvp(&self) -> &Cc2Ccvp {
        &self.cc2_ccvp
    }
    #[doc = "0x5c - CC Channel Buffer Register"]
    #[inline(always)]
    pub const fn cc2_ccvb(&self) -> &Cc2Ccvb {
        &self.cc2_ccvb
    }
    #[doc = "0x70 - DTI Control Register"]
    #[inline(always)]
    pub const fn dtctrl(&self) -> &Dtctrl {
        &self.dtctrl
    }
    #[doc = "0x74 - DTI Time Control Register"]
    #[inline(always)]
    pub const fn dttime(&self) -> &Dttime {
        &self.dttime
    }
    #[doc = "0x78 - DTI Fault Configuration Register"]
    #[inline(always)]
    pub const fn dtfc(&self) -> &Dtfc {
        &self.dtfc
    }
    #[doc = "0x7c - DTI Output Generation Enable Register"]
    #[inline(always)]
    pub const fn dtogen(&self) -> &Dtogen {
        &self.dtogen
    }
    #[doc = "0x80 - DTI Fault Register"]
    #[inline(always)]
    pub const fn dtfault(&self) -> &Dtfault {
        &self.dtfault
    }
    #[doc = "0x84 - DTI Fault Clear Register"]
    #[inline(always)]
    pub const fn dtfaultc(&self) -> &Dtfaultc {
        &self.dtfaultc
    }
    #[doc = "0x88 - DTI Configuration Lock Register"]
    #[inline(always)]
    pub const fn dtlock(&self) -> &Dtlock {
        &self.dtlock
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
#[doc = "TOP (rw) register accessor: Counter Top Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`top::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`top::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@top`] module"]
#[doc(alias = "TOP")]
pub type Top = crate::Reg<top::TopSpec>;
#[doc = "Counter Top Value Register"]
pub mod top;
#[doc = "TOPB (rw) register accessor: Counter Top Value Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`topb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`topb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@topb`] module"]
#[doc(alias = "TOPB")]
pub type Topb = crate::Reg<topb::TopbSpec>;
#[doc = "Counter Top Value Buffer Register"]
pub mod topb;
#[doc = "CNT (rw) register accessor: Counter Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "Counter Value Register"]
pub mod cnt;
#[doc = "ROUTE (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@route`] module"]
#[doc(alias = "ROUTE")]
pub type Route = crate::Reg<route::RouteSpec>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "CC0_CTRL (rw) register accessor: CC Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_ctrl`] module"]
#[doc(alias = "CC0_CTRL")]
pub type Cc0Ctrl = crate::Reg<cc0_ctrl::Cc0CtrlSpec>;
#[doc = "CC Channel Control Register"]
pub mod cc0_ctrl;
#[doc = "CC0_CCV (rw) register accessor: CC Channel Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_ccv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_ccv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_ccv`] module"]
#[doc(alias = "CC0_CCV")]
pub type Cc0Ccv = crate::Reg<cc0_ccv::Cc0CcvSpec>;
#[doc = "CC Channel Value Register"]
pub mod cc0_ccv;
#[doc = "CC0_CCVP (r) register accessor: CC Channel Value Peek Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_ccvp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_ccvp`] module"]
#[doc(alias = "CC0_CCVP")]
pub type Cc0Ccvp = crate::Reg<cc0_ccvp::Cc0CcvpSpec>;
#[doc = "CC Channel Value Peek Register"]
pub mod cc0_ccvp;
#[doc = "CC0_CCVB (rw) register accessor: CC Channel Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_ccvb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_ccvb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_ccvb`] module"]
#[doc(alias = "CC0_CCVB")]
pub type Cc0Ccvb = crate::Reg<cc0_ccvb::Cc0CcvbSpec>;
#[doc = "CC Channel Buffer Register"]
pub mod cc0_ccvb;
#[doc = "CC1_CTRL (rw) register accessor: CC Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_ctrl`] module"]
#[doc(alias = "CC1_CTRL")]
pub type Cc1Ctrl = crate::Reg<cc1_ctrl::Cc1CtrlSpec>;
#[doc = "CC Channel Control Register"]
pub mod cc1_ctrl;
#[doc = "CC1_CCV (rw) register accessor: CC Channel Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_ccv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1_ccv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_ccv`] module"]
#[doc(alias = "CC1_CCV")]
pub type Cc1Ccv = crate::Reg<cc1_ccv::Cc1CcvSpec>;
#[doc = "CC Channel Value Register"]
pub mod cc1_ccv;
#[doc = "CC1_CCVP (r) register accessor: CC Channel Value Peek Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_ccvp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_ccvp`] module"]
#[doc(alias = "CC1_CCVP")]
pub type Cc1Ccvp = crate::Reg<cc1_ccvp::Cc1CcvpSpec>;
#[doc = "CC Channel Value Peek Register"]
pub mod cc1_ccvp;
#[doc = "CC1_CCVB (rw) register accessor: CC Channel Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_ccvb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1_ccvb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_ccvb`] module"]
#[doc(alias = "CC1_CCVB")]
pub type Cc1Ccvb = crate::Reg<cc1_ccvb::Cc1CcvbSpec>;
#[doc = "CC Channel Buffer Register"]
pub mod cc1_ccvb;
#[doc = "CC2_CTRL (rw) register accessor: CC Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2_ctrl`] module"]
#[doc(alias = "CC2_CTRL")]
pub type Cc2Ctrl = crate::Reg<cc2_ctrl::Cc2CtrlSpec>;
#[doc = "CC Channel Control Register"]
pub mod cc2_ctrl;
#[doc = "CC2_CCV (rw) register accessor: CC Channel Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_ccv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2_ccv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2_ccv`] module"]
#[doc(alias = "CC2_CCV")]
pub type Cc2Ccv = crate::Reg<cc2_ccv::Cc2CcvSpec>;
#[doc = "CC Channel Value Register"]
pub mod cc2_ccv;
#[doc = "CC2_CCVP (r) register accessor: CC Channel Value Peek Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_ccvp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2_ccvp`] module"]
#[doc(alias = "CC2_CCVP")]
pub type Cc2Ccvp = crate::Reg<cc2_ccvp::Cc2CcvpSpec>;
#[doc = "CC Channel Value Peek Register"]
pub mod cc2_ccvp;
#[doc = "CC2_CCVB (rw) register accessor: CC Channel Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_ccvb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2_ccvb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2_ccvb`] module"]
#[doc(alias = "CC2_CCVB")]
pub type Cc2Ccvb = crate::Reg<cc2_ccvb::Cc2CcvbSpec>;
#[doc = "CC Channel Buffer Register"]
pub mod cc2_ccvb;
#[doc = "DTCTRL (rw) register accessor: DTI Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtctrl`] module"]
#[doc(alias = "DTCTRL")]
pub type Dtctrl = crate::Reg<dtctrl::DtctrlSpec>;
#[doc = "DTI Control Register"]
pub mod dtctrl;
#[doc = "DTTIME (rw) register accessor: DTI Time Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dttime::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dttime::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dttime`] module"]
#[doc(alias = "DTTIME")]
pub type Dttime = crate::Reg<dttime::DttimeSpec>;
#[doc = "DTI Time Control Register"]
pub mod dttime;
#[doc = "DTFC (rw) register accessor: DTI Fault Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtfc`] module"]
#[doc(alias = "DTFC")]
pub type Dtfc = crate::Reg<dtfc::DtfcSpec>;
#[doc = "DTI Fault Configuration Register"]
pub mod dtfc;
#[doc = "DTOGEN (rw) register accessor: DTI Output Generation Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtogen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtogen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtogen`] module"]
#[doc(alias = "DTOGEN")]
pub type Dtogen = crate::Reg<dtogen::DtogenSpec>;
#[doc = "DTI Output Generation Enable Register"]
pub mod dtogen;
#[doc = "DTFAULT (r) register accessor: DTI Fault Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtfault::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtfault`] module"]
#[doc(alias = "DTFAULT")]
pub type Dtfault = crate::Reg<dtfault::DtfaultSpec>;
#[doc = "DTI Fault Register"]
pub mod dtfault;
#[doc = "DTFAULTC (w) register accessor: DTI Fault Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtfaultc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtfaultc`] module"]
#[doc(alias = "DTFAULTC")]
pub type Dtfaultc = crate::Reg<dtfaultc::DtfaultcSpec>;
#[doc = "DTI Fault Clear Register"]
pub mod dtfaultc;
#[doc = "DTLOCK (rw) register accessor: DTI Configuration Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtlock`] module"]
#[doc(alias = "DTLOCK")]
pub type Dtlock = crate::Reg<dtlock::DtlockSpec>;
#[doc = "DTI Configuration Lock Register"]
pub mod dtlock;
