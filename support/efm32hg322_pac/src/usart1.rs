#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    frame: FRAME,
    trigctrl: TRIGCTRL,
    cmd: CMD,
    status: STATUS,
    clkdiv: CLKDIV,
    rxdatax: RXDATAX,
    rxdata: RXDATA,
    rxdoublex: RXDOUBLEX,
    rxdouble: RXDOUBLE,
    rxdataxp: RXDATAXP,
    rxdoublexp: RXDOUBLEXP,
    txdatax: TXDATAX,
    txdata: TXDATA,
    txdoublex: TXDOUBLEX,
    txdouble: TXDOUBLE,
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    ien: IEN,
    irctrl: IRCTRL,
    route: ROUTE,
    input: INPUT,
    i2sctrl: I2SCTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - USART Frame Format Register"]
    #[inline(always)]
    pub const fn frame(&self) -> &FRAME {
        &self.frame
    }
    #[doc = "0x08 - USART Trigger Control register"]
    #[inline(always)]
    pub const fn trigctrl(&self) -> &TRIGCTRL {
        &self.trigctrl
    }
    #[doc = "0x0c - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x10 - USART Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x14 - Clock Control Register"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &CLKDIV {
        &self.clkdiv
    }
    #[doc = "0x18 - RX Buffer Data Extended Register"]
    #[inline(always)]
    pub const fn rxdatax(&self) -> &RXDATAX {
        &self.rxdatax
    }
    #[doc = "0x1c - RX Buffer Data Register"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &RXDATA {
        &self.rxdata
    }
    #[doc = "0x20 - RX Buffer Double Data Extended Register"]
    #[inline(always)]
    pub const fn rxdoublex(&self) -> &RXDOUBLEX {
        &self.rxdoublex
    }
    #[doc = "0x24 - RX FIFO Double Data Register"]
    #[inline(always)]
    pub const fn rxdouble(&self) -> &RXDOUBLE {
        &self.rxdouble
    }
    #[doc = "0x28 - RX Buffer Data Extended Peek Register"]
    #[inline(always)]
    pub const fn rxdataxp(&self) -> &RXDATAXP {
        &self.rxdataxp
    }
    #[doc = "0x2c - RX Buffer Double Data Extended Peek Register"]
    #[inline(always)]
    pub const fn rxdoublexp(&self) -> &RXDOUBLEXP {
        &self.rxdoublexp
    }
    #[doc = "0x30 - TX Buffer Data Extended Register"]
    #[inline(always)]
    pub const fn txdatax(&self) -> &TXDATAX {
        &self.txdatax
    }
    #[doc = "0x34 - TX Buffer Data Register"]
    #[inline(always)]
    pub const fn txdata(&self) -> &TXDATA {
        &self.txdata
    }
    #[doc = "0x38 - TX Buffer Double Data Extended Register"]
    #[inline(always)]
    pub const fn txdoublex(&self) -> &TXDOUBLEX {
        &self.txdoublex
    }
    #[doc = "0x3c - TX Buffer Double Data Register"]
    #[inline(always)]
    pub const fn txdouble(&self) -> &TXDOUBLE {
        &self.txdouble
    }
    #[doc = "0x40 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0x44 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0x48 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
    #[doc = "0x4c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0x50 - IrDA Control Register"]
    #[inline(always)]
    pub const fn irctrl(&self) -> &IRCTRL {
        &self.irctrl
    }
    #[doc = "0x54 - I/O Routing Register"]
    #[inline(always)]
    pub const fn route(&self) -> &ROUTE {
        &self.route
    }
    #[doc = "0x58 - USART Input Register"]
    #[inline(always)]
    pub const fn input(&self) -> &INPUT {
        &self.input
    }
    #[doc = "0x5c - I2S Control Register"]
    #[inline(always)]
    pub const fn i2sctrl(&self) -> &I2SCTRL {
        &self.i2sctrl
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "FRAME (rw) register accessor: USART Frame Format Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frame::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frame::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame`]
module"]
pub type FRAME = crate::Reg<frame::FRAME_SPEC>;
#[doc = "USART Frame Format Register"]
pub mod frame;
#[doc = "TRIGCTRL (rw) register accessor: USART Trigger Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trigctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigctrl`]
module"]
pub type TRIGCTRL = crate::Reg<trigctrl::TRIGCTRL_SPEC>;
#[doc = "USART Trigger Control register"]
pub mod trigctrl;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: USART Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "USART Status Register"]
pub mod status;
#[doc = "CLKDIV (rw) register accessor: Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`]
module"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock Control Register"]
pub mod clkdiv;
#[doc = "RXDATAX (r) register accessor: RX Buffer Data Extended Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdatax::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdatax`]
module"]
pub type RXDATAX = crate::Reg<rxdatax::RXDATAX_SPEC>;
#[doc = "RX Buffer Data Extended Register"]
pub mod rxdatax;
#[doc = "RXDATA (r) register accessor: RX Buffer Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`]
module"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "RX Buffer Data Register"]
pub mod rxdata;
#[doc = "RXDOUBLEX (r) register accessor: RX Buffer Double Data Extended Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdoublex::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdoublex`]
module"]
pub type RXDOUBLEX = crate::Reg<rxdoublex::RXDOUBLEX_SPEC>;
#[doc = "RX Buffer Double Data Extended Register"]
pub mod rxdoublex;
#[doc = "RXDOUBLE (r) register accessor: RX FIFO Double Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdouble::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdouble`]
module"]
pub type RXDOUBLE = crate::Reg<rxdouble::RXDOUBLE_SPEC>;
#[doc = "RX FIFO Double Data Register"]
pub mod rxdouble;
#[doc = "RXDATAXP (r) register accessor: RX Buffer Data Extended Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdataxp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdataxp`]
module"]
pub type RXDATAXP = crate::Reg<rxdataxp::RXDATAXP_SPEC>;
#[doc = "RX Buffer Data Extended Peek Register"]
pub mod rxdataxp;
#[doc = "RXDOUBLEXP (r) register accessor: RX Buffer Double Data Extended Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdoublexp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdoublexp`]
module"]
pub type RXDOUBLEXP = crate::Reg<rxdoublexp::RXDOUBLEXP_SPEC>;
#[doc = "RX Buffer Double Data Extended Peek Register"]
pub mod rxdoublexp;
#[doc = "TXDATAX (w) register accessor: TX Buffer Data Extended Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdatax::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdatax`]
module"]
pub type TXDATAX = crate::Reg<txdatax::TXDATAX_SPEC>;
#[doc = "TX Buffer Data Extended Register"]
pub mod txdatax;
#[doc = "TXDATA (w) register accessor: TX Buffer Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`]
module"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "TX Buffer Data Register"]
pub mod txdata;
#[doc = "TXDOUBLEX (w) register accessor: TX Buffer Double Data Extended Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdoublex::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdoublex`]
module"]
pub type TXDOUBLEX = crate::Reg<txdoublex::TXDOUBLEX_SPEC>;
#[doc = "TX Buffer Double Data Extended Register"]
pub mod txdoublex;
#[doc = "TXDOUBLE (w) register accessor: TX Buffer Double Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdouble::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdouble`]
module"]
pub type TXDOUBLE = crate::Reg<txdouble::TXDOUBLE_SPEC>;
#[doc = "TX Buffer Double Data Register"]
pub mod txdouble;
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
#[doc = "IRCTRL (rw) register accessor: IrDA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irctrl`]
module"]
pub type IRCTRL = crate::Reg<irctrl::IRCTRL_SPEC>;
#[doc = "IrDA Control Register"]
pub mod irctrl;
#[doc = "ROUTE (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`route::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`route::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@route`]
module"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "INPUT (rw) register accessor: USART Input Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@input`]
module"]
pub type INPUT = crate::Reg<input::INPUT_SPEC>;
#[doc = "USART Input Register"]
pub mod input;
#[doc = "I2SCTRL (rw) register accessor: I2S Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sctrl`]
module"]
pub type I2SCTRL = crate::Reg<i2sctrl::I2SCTRL_SPEC>;
#[doc = "I2S Control Register"]
pub mod i2sctrl;
