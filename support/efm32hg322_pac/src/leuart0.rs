#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    cmd: CMD,
    status: STATUS,
    clkdiv: CLKDIV,
    startframe: STARTFRAME,
    sigframe: SIGFRAME,
    rxdatax: RXDATAX,
    rxdata: RXDATA,
    rxdataxp: RXDATAXP,
    txdatax: TXDATAX,
    txdata: TXDATA,
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    ien: IEN,
    pulsectrl: PULSECTRL,
    freeze: FREEZE,
    syncbusy: SYNCBUSY,
    _reserved18: [u8; 0x0c],
    route: ROUTE,
    _reserved19: [u8; 0x54],
    input: INPUT,
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
    #[doc = "0x0c - Clock Control Register"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &CLKDIV {
        &self.clkdiv
    }
    #[doc = "0x10 - Start Frame Register"]
    #[inline(always)]
    pub const fn startframe(&self) -> &STARTFRAME {
        &self.startframe
    }
    #[doc = "0x14 - Signal Frame Register"]
    #[inline(always)]
    pub const fn sigframe(&self) -> &SIGFRAME {
        &self.sigframe
    }
    #[doc = "0x18 - Receive Buffer Data Extended Register"]
    #[inline(always)]
    pub const fn rxdatax(&self) -> &RXDATAX {
        &self.rxdatax
    }
    #[doc = "0x1c - Receive Buffer Data Register"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &RXDATA {
        &self.rxdata
    }
    #[doc = "0x20 - Receive Buffer Data Extended Peek Register"]
    #[inline(always)]
    pub const fn rxdataxp(&self) -> &RXDATAXP {
        &self.rxdataxp
    }
    #[doc = "0x24 - Transmit Buffer Data Extended Register"]
    #[inline(always)]
    pub const fn txdatax(&self) -> &TXDATAX {
        &self.txdatax
    }
    #[doc = "0x28 - Transmit Buffer Data Register"]
    #[inline(always)]
    pub const fn txdata(&self) -> &TXDATA {
        &self.txdata
    }
    #[doc = "0x2c - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0x30 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0x34 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
    #[doc = "0x38 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0x3c - Pulse Control Register"]
    #[inline(always)]
    pub const fn pulsectrl(&self) -> &PULSECTRL {
        &self.pulsectrl
    }
    #[doc = "0x40 - Freeze Register"]
    #[inline(always)]
    pub const fn freeze(&self) -> &FREEZE {
        &self.freeze
    }
    #[doc = "0x44 - Synchronization Busy Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &SYNCBUSY {
        &self.syncbusy
    }
    #[doc = "0x54 - I/O Routing Register"]
    #[inline(always)]
    pub const fn route(&self) -> &ROUTE {
        &self.route
    }
    #[doc = "0xac - LEUART Input Register"]
    #[inline(always)]
    pub const fn input(&self) -> &INPUT {
        &self.input
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
#[doc = "CLKDIV (rw) register accessor: Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`]
module"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock Control Register"]
pub mod clkdiv;
#[doc = "STARTFRAME (rw) register accessor: Start Frame Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`startframe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`startframe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@startframe`]
module"]
pub type STARTFRAME = crate::Reg<startframe::STARTFRAME_SPEC>;
#[doc = "Start Frame Register"]
pub mod startframe;
#[doc = "SIGFRAME (rw) register accessor: Signal Frame Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigframe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigframe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigframe`]
module"]
pub type SIGFRAME = crate::Reg<sigframe::SIGFRAME_SPEC>;
#[doc = "Signal Frame Register"]
pub mod sigframe;
#[doc = "RXDATAX (r) register accessor: Receive Buffer Data Extended Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdatax::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdatax`]
module"]
pub type RXDATAX = crate::Reg<rxdatax::RXDATAX_SPEC>;
#[doc = "Receive Buffer Data Extended Register"]
pub mod rxdatax;
#[doc = "RXDATA (r) register accessor: Receive Buffer Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`]
module"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "Receive Buffer Data Register"]
pub mod rxdata;
#[doc = "RXDATAXP (r) register accessor: Receive Buffer Data Extended Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdataxp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdataxp`]
module"]
pub type RXDATAXP = crate::Reg<rxdataxp::RXDATAXP_SPEC>;
#[doc = "Receive Buffer Data Extended Peek Register"]
pub mod rxdataxp;
#[doc = "TXDATAX (w) register accessor: Transmit Buffer Data Extended Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdatax::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdatax`]
module"]
pub type TXDATAX = crate::Reg<txdatax::TXDATAX_SPEC>;
#[doc = "Transmit Buffer Data Extended Register"]
pub mod txdatax;
#[doc = "TXDATA (w) register accessor: Transmit Buffer Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`]
module"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "Transmit Buffer Data Register"]
pub mod txdata;
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
#[doc = "IEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`]
module"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "PULSECTRL (rw) register accessor: Pulse Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulsectrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulsectrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulsectrl`]
module"]
pub type PULSECTRL = crate::Reg<pulsectrl::PULSECTRL_SPEC>;
#[doc = "Pulse Control Register"]
pub mod pulsectrl;
#[doc = "FREEZE (rw) register accessor: Freeze Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freeze::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freeze::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freeze`]
module"]
pub type FREEZE = crate::Reg<freeze::FREEZE_SPEC>;
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`]
module"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "ROUTE (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`route::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`route::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@route`]
module"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "INPUT (rw) register accessor: LEUART Input Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@input`]
module"]
pub type INPUT = crate::Reg<input::INPUT_SPEC>;
#[doc = "LEUART Input Register"]
pub mod input;
