#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    cmd: Cmd,
    status: Status,
    clkdiv: Clkdiv,
    startframe: Startframe,
    sigframe: Sigframe,
    rxdatax: Rxdatax,
    rxdata: Rxdata,
    rxdataxp: Rxdataxp,
    txdatax: Txdatax,
    txdata: Txdata,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    pulsectrl: Pulsectrl,
    freeze: Freeze,
    syncbusy: Syncbusy,
    _reserved18: [u8; 0x0c],
    route: Route,
    _reserved19: [u8; 0x54],
    input: Input,
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
    #[doc = "0x0c - Clock Control Register"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x10 - Start Frame Register"]
    #[inline(always)]
    pub const fn startframe(&self) -> &Startframe {
        &self.startframe
    }
    #[doc = "0x14 - Signal Frame Register"]
    #[inline(always)]
    pub const fn sigframe(&self) -> &Sigframe {
        &self.sigframe
    }
    #[doc = "0x18 - Receive Buffer Data Extended Register"]
    #[inline(always)]
    pub const fn rxdatax(&self) -> &Rxdatax {
        &self.rxdatax
    }
    #[doc = "0x1c - Receive Buffer Data Register"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x20 - Receive Buffer Data Extended Peek Register"]
    #[inline(always)]
    pub const fn rxdataxp(&self) -> &Rxdataxp {
        &self.rxdataxp
    }
    #[doc = "0x24 - Transmit Buffer Data Extended Register"]
    #[inline(always)]
    pub const fn txdatax(&self) -> &Txdatax {
        &self.txdatax
    }
    #[doc = "0x28 - Transmit Buffer Data Register"]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
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
    #[doc = "0x3c - Pulse Control Register"]
    #[inline(always)]
    pub const fn pulsectrl(&self) -> &Pulsectrl {
        &self.pulsectrl
    }
    #[doc = "0x40 - Freeze Register"]
    #[inline(always)]
    pub const fn freeze(&self) -> &Freeze {
        &self.freeze
    }
    #[doc = "0x44 - Synchronization Busy Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x54 - I/O Routing Register"]
    #[inline(always)]
    pub const fn route(&self) -> &Route {
        &self.route
    }
    #[doc = "0xac - LEUART Input Register"]
    #[inline(always)]
    pub const fn input(&self) -> &Input {
        &self.input
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
#[doc = "CLKDIV (rw) register accessor: Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`] module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "Clock Control Register"]
pub mod clkdiv;
#[doc = "STARTFRAME (rw) register accessor: Start Frame Register\n\nYou can [`read`](crate::Reg::read) this register and get [`startframe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`startframe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@startframe`] module"]
#[doc(alias = "STARTFRAME")]
pub type Startframe = crate::Reg<startframe::StartframeSpec>;
#[doc = "Start Frame Register"]
pub mod startframe;
#[doc = "SIGFRAME (rw) register accessor: Signal Frame Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sigframe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigframe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigframe`] module"]
#[doc(alias = "SIGFRAME")]
pub type Sigframe = crate::Reg<sigframe::SigframeSpec>;
#[doc = "Signal Frame Register"]
pub mod sigframe;
#[doc = "RXDATAX (r) register accessor: Receive Buffer Data Extended Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdatax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@rxdatax`] module"]
#[doc(alias = "RXDATAX")]
pub type Rxdatax = crate::Reg<rxdatax::RxdataxSpec>;
#[doc = "Receive Buffer Data Extended Register"]
pub mod rxdatax;
#[doc = "RXDATA (r) register accessor: Receive Buffer Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@rxdata`] module"]
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RxdataSpec>;
#[doc = "Receive Buffer Data Register"]
pub mod rxdata;
#[doc = "RXDATAXP (r) register accessor: Receive Buffer Data Extended Peek Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdataxp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdataxp`] module"]
#[doc(alias = "RXDATAXP")]
pub type Rxdataxp = crate::Reg<rxdataxp::RxdataxpSpec>;
#[doc = "Receive Buffer Data Extended Peek Register"]
pub mod rxdataxp;
#[doc = "TXDATAX (w) register accessor: Transmit Buffer Data Extended Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdatax::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdatax`] module"]
#[doc(alias = "TXDATAX")]
pub type Txdatax = crate::Reg<txdatax::TxdataxSpec>;
#[doc = "Transmit Buffer Data Extended Register"]
pub mod txdatax;
#[doc = "TXDATA (w) register accessor: Transmit Buffer Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`] module"]
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TxdataSpec>;
#[doc = "Transmit Buffer Data Register"]
pub mod txdata;
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
#[doc = "PULSECTRL (rw) register accessor: Pulse Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pulsectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pulsectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulsectrl`] module"]
#[doc(alias = "PULSECTRL")]
pub type Pulsectrl = crate::Reg<pulsectrl::PulsectrlSpec>;
#[doc = "Pulse Control Register"]
pub mod pulsectrl;
#[doc = "FREEZE (rw) register accessor: Freeze Register\n\nYou can [`read`](crate::Reg::read) this register and get [`freeze::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freeze::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freeze`] module"]
#[doc(alias = "FREEZE")]
pub type Freeze = crate::Reg<freeze::FreezeSpec>;
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "ROUTE (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@route`] module"]
#[doc(alias = "ROUTE")]
pub type Route = crate::Reg<route::RouteSpec>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "INPUT (rw) register accessor: LEUART Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@input`] module"]
#[doc(alias = "INPUT")]
pub type Input = crate::Reg<input::InputSpec>;
#[doc = "LEUART Input Register"]
pub mod input;
