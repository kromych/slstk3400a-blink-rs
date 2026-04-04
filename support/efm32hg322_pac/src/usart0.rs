#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    frame: Frame,
    trigctrl: Trigctrl,
    cmd: Cmd,
    status: Status,
    clkdiv: Clkdiv,
    rxdatax: Rxdatax,
    rxdata: Rxdata,
    rxdoublex: Rxdoublex,
    rxdouble: Rxdouble,
    rxdataxp: Rxdataxp,
    rxdoublexp: Rxdoublexp,
    txdatax: Txdatax,
    txdata: Txdata,
    txdoublex: Txdoublex,
    txdouble: Txdouble,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    irctrl: Irctrl,
    route: Route,
    input: Input,
    i2sctrl: I2sctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - USART Frame Format Register"]
    #[inline(always)]
    pub const fn frame(&self) -> &Frame {
        &self.frame
    }
    #[doc = "0x08 - USART Trigger Control register"]
    #[inline(always)]
    pub const fn trigctrl(&self) -> &Trigctrl {
        &self.trigctrl
    }
    #[doc = "0x0c - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x10 - USART Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x14 - Clock Control Register"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x18 - RX Buffer Data Extended Register"]
    #[inline(always)]
    pub const fn rxdatax(&self) -> &Rxdatax {
        &self.rxdatax
    }
    #[doc = "0x1c - RX Buffer Data Register"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x20 - RX Buffer Double Data Extended Register"]
    #[inline(always)]
    pub const fn rxdoublex(&self) -> &Rxdoublex {
        &self.rxdoublex
    }
    #[doc = "0x24 - RX FIFO Double Data Register"]
    #[inline(always)]
    pub const fn rxdouble(&self) -> &Rxdouble {
        &self.rxdouble
    }
    #[doc = "0x28 - RX Buffer Data Extended Peek Register"]
    #[inline(always)]
    pub const fn rxdataxp(&self) -> &Rxdataxp {
        &self.rxdataxp
    }
    #[doc = "0x2c - RX Buffer Double Data Extended Peek Register"]
    #[inline(always)]
    pub const fn rxdoublexp(&self) -> &Rxdoublexp {
        &self.rxdoublexp
    }
    #[doc = "0x30 - TX Buffer Data Extended Register"]
    #[inline(always)]
    pub const fn txdatax(&self) -> &Txdatax {
        &self.txdatax
    }
    #[doc = "0x34 - TX Buffer Data Register"]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x38 - TX Buffer Double Data Extended Register"]
    #[inline(always)]
    pub const fn txdoublex(&self) -> &Txdoublex {
        &self.txdoublex
    }
    #[doc = "0x3c - TX Buffer Double Data Register"]
    #[inline(always)]
    pub const fn txdouble(&self) -> &Txdouble {
        &self.txdouble
    }
    #[doc = "0x40 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x44 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x48 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x4c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x50 - IrDA Control Register"]
    #[inline(always)]
    pub const fn irctrl(&self) -> &Irctrl {
        &self.irctrl
    }
    #[doc = "0x54 - I/O Routing Register"]
    #[inline(always)]
    pub const fn route(&self) -> &Route {
        &self.route
    }
    #[doc = "0x58 - USART Input Register"]
    #[inline(always)]
    pub const fn input(&self) -> &Input {
        &self.input
    }
    #[doc = "0x5c - I2S Control Register"]
    #[inline(always)]
    pub const fn i2sctrl(&self) -> &I2sctrl {
        &self.i2sctrl
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "FRAME (rw) register accessor: USART Frame Format Register\n\nYou can [`read`](crate::Reg::read) this register and get [`frame::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame`] module"]
#[doc(alias = "FRAME")]
pub type Frame = crate::Reg<frame::FrameSpec>;
#[doc = "USART Frame Format Register"]
pub mod frame;
#[doc = "TRIGCTRL (rw) register accessor: USART Trigger Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`trigctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigctrl`] module"]
#[doc(alias = "TRIGCTRL")]
pub type Trigctrl = crate::Reg<trigctrl::TrigctrlSpec>;
#[doc = "USART Trigger Control register"]
pub mod trigctrl;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: USART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "USART Status Register"]
pub mod status;
#[doc = "CLKDIV (rw) register accessor: Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`] module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "Clock Control Register"]
pub mod clkdiv;
#[doc = "RXDATAX (r) register accessor: RX Buffer Data Extended Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdatax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@rxdatax`] module"]
#[doc(alias = "RXDATAX")]
pub type Rxdatax = crate::Reg<rxdatax::RxdataxSpec>;
#[doc = "RX Buffer Data Extended Register"]
pub mod rxdatax;
#[doc = "RXDATA (r) register accessor: RX Buffer Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@rxdata`] module"]
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RxdataSpec>;
#[doc = "RX Buffer Data Register"]
pub mod rxdata;
#[doc = "RXDOUBLEX (r) register accessor: RX Buffer Double Data Extended Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdoublex::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@rxdoublex`] module"]
#[doc(alias = "RXDOUBLEX")]
pub type Rxdoublex = crate::Reg<rxdoublex::RxdoublexSpec>;
#[doc = "RX Buffer Double Data Extended Register"]
pub mod rxdoublex;
#[doc = "RXDOUBLE (r) register accessor: RX FIFO Double Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdouble::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@rxdouble`] module"]
#[doc(alias = "RXDOUBLE")]
pub type Rxdouble = crate::Reg<rxdouble::RxdoubleSpec>;
#[doc = "RX FIFO Double Data Register"]
pub mod rxdouble;
#[doc = "RXDATAXP (r) register accessor: RX Buffer Data Extended Peek Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdataxp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdataxp`] module"]
#[doc(alias = "RXDATAXP")]
pub type Rxdataxp = crate::Reg<rxdataxp::RxdataxpSpec>;
#[doc = "RX Buffer Data Extended Peek Register"]
pub mod rxdataxp;
#[doc = "RXDOUBLEXP (r) register accessor: RX Buffer Double Data Extended Peek Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdoublexp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdoublexp`] module"]
#[doc(alias = "RXDOUBLEXP")]
pub type Rxdoublexp = crate::Reg<rxdoublexp::RxdoublexpSpec>;
#[doc = "RX Buffer Double Data Extended Peek Register"]
pub mod rxdoublexp;
#[doc = "TXDATAX (w) register accessor: TX Buffer Data Extended Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdatax::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdatax`] module"]
#[doc(alias = "TXDATAX")]
pub type Txdatax = crate::Reg<txdatax::TxdataxSpec>;
#[doc = "TX Buffer Data Extended Register"]
pub mod txdatax;
#[doc = "TXDATA (w) register accessor: TX Buffer Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`] module"]
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TxdataSpec>;
#[doc = "TX Buffer Data Register"]
pub mod txdata;
#[doc = "TXDOUBLEX (w) register accessor: TX Buffer Double Data Extended Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdoublex::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdoublex`] module"]
#[doc(alias = "TXDOUBLEX")]
pub type Txdoublex = crate::Reg<txdoublex::TxdoublexSpec>;
#[doc = "TX Buffer Double Data Extended Register"]
pub mod txdoublex;
#[doc = "TXDOUBLE (w) register accessor: TX Buffer Double Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdouble::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdouble`] module"]
#[doc(alias = "TXDOUBLE")]
pub type Txdouble = crate::Reg<txdouble::TxdoubleSpec>;
#[doc = "TX Buffer Double Data Register"]
pub mod txdouble;
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
#[doc = "IRCTRL (rw) register accessor: IrDA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irctrl`] module"]
#[doc(alias = "IRCTRL")]
pub type Irctrl = crate::Reg<irctrl::IrctrlSpec>;
#[doc = "IrDA Control Register"]
pub mod irctrl;
#[doc = "ROUTE (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@route`] module"]
#[doc(alias = "ROUTE")]
pub type Route = crate::Reg<route::RouteSpec>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "INPUT (rw) register accessor: USART Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@input`] module"]
#[doc(alias = "INPUT")]
pub type Input = crate::Reg<input::InputSpec>;
#[doc = "USART Input Register"]
pub mod input;
#[doc = "I2SCTRL (rw) register accessor: I2S Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sctrl`] module"]
#[doc(alias = "I2SCTRL")]
pub type I2sctrl = crate::Reg<i2sctrl::I2sctrlSpec>;
#[doc = "I2S Control Register"]
pub mod i2sctrl;
