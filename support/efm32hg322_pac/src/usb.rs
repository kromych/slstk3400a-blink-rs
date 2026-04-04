#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    status: Status,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    route: Route,
    _reserved7: [u8; 0x0003_bfec],
    gahbcfg: Gahbcfg,
    gusbcfg: Gusbcfg,
    grstctl: Grstctl,
    gintsts: Gintsts,
    gintmsk: Gintmsk,
    grxstsr: Grxstsr,
    grxstsp: Grxstsp,
    grxfsiz: Grxfsiz,
    gnptxfsiz: Gnptxfsiz,
    _reserved16: [u8; 0x30],
    gdfifocfg: Gdfifocfg,
    _reserved17: [u8; 0xa4],
    dieptxf1: Dieptxf1,
    dieptxf2: Dieptxf2,
    dieptxf3: Dieptxf3,
    _reserved20: [u8; 0x06f0],
    dcfg: Dcfg,
    dctl: Dctl,
    dsts: Dsts,
    _reserved23: [u8; 0x04],
    diepmsk: Diepmsk,
    doepmsk: Doepmsk,
    daint: Daint,
    daintmsk: Daintmsk,
    _reserved27: [u8; 0x14],
    diepempmsk: Diepempmsk,
    _reserved28: [u8; 0xc8],
    diep0ctl: Diep0ctl,
    _reserved29: [u8; 0x04],
    diep0int: Diep0int,
    _reserved30: [u8; 0x04],
    diep0tsiz: Diep0tsiz,
    diep0dmaaddr: Diep0dmaaddr,
    diep0txfsts: Diep0txfsts,
    _reserved33: [u8; 0x04],
    diep0_ctl: Diep0Ctl,
    _reserved34: [u8; 0x04],
    diep0_int: Diep0Int,
    _reserved35: [u8; 0x04],
    diep0_tsiz: Diep0Tsiz,
    diep0_dmaaddr: Diep0Dmaaddr,
    diep0_txfsts: Diep0Txfsts,
    _reserved38: [u8; 0x04],
    diep1_ctl: Diep1Ctl,
    _reserved39: [u8; 0x04],
    diep1_int: Diep1Int,
    _reserved40: [u8; 0x04],
    diep1_tsiz: Diep1Tsiz,
    diep1_dmaaddr: Diep1Dmaaddr,
    diep1_txfsts: Diep1Txfsts,
    _reserved43: [u8; 0x04],
    diep2_ctl: Diep2Ctl,
    _reserved44: [u8; 0x04],
    diep2_int: Diep2Int,
    _reserved45: [u8; 0x04],
    diep2_tsiz: Diep2Tsiz,
    diep2_dmaaddr: Diep2Dmaaddr,
    diep2_txfsts: Diep2Txfsts,
    _reserved48: [u8; 0x0184],
    doep0ctl: Doep0ctl,
    _reserved49: [u8; 0x04],
    doep0int: Doep0int,
    _reserved50: [u8; 0x04],
    doep0tsiz: Doep0tsiz,
    doep0dmaaddr: Doep0dmaaddr,
    _reserved52: [u8; 0x08],
    doep0_ctl: Doep0Ctl,
    _reserved53: [u8; 0x04],
    doep0_int: Doep0Int,
    _reserved54: [u8; 0x04],
    doep0_tsiz: Doep0Tsiz,
    doep0_dmaaddr: Doep0Dmaaddr,
    _reserved56: [u8; 0x08],
    doep1_ctl: Doep1Ctl,
    _reserved57: [u8; 0x04],
    doep1_int: Doep1Int,
    _reserved58: [u8; 0x04],
    doep1_tsiz: Doep1Tsiz,
    doep1_dmaaddr: Doep1Dmaaddr,
    _reserved60: [u8; 0x08],
    doep2_ctl: Doep2Ctl,
    _reserved61: [u8; 0x04],
    doep2_int: Doep2Int,
    _reserved62: [u8; 0x04],
    doep2_tsiz: Doep2Tsiz,
    doep2_dmaaddr: Doep2Dmaaddr,
    _reserved64: [u8; 0x0288],
    pcgcctl: Pcgcctl,
}
impl RegisterBlock {
    #[doc = "0x00 - System Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - System Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x0c - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x10 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x14 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x18 - I/O Routing Register"]
    #[inline(always)]
    pub const fn route(&self) -> &Route {
        &self.route
    }
    #[doc = "0x3c008 - AHB Configuration Register"]
    #[inline(always)]
    pub const fn gahbcfg(&self) -> &Gahbcfg {
        &self.gahbcfg
    }
    #[doc = "0x3c00c - USB Configuration Register"]
    #[inline(always)]
    pub const fn gusbcfg(&self) -> &Gusbcfg {
        &self.gusbcfg
    }
    #[doc = "0x3c010 - Reset Register"]
    #[inline(always)]
    pub const fn grstctl(&self) -> &Grstctl {
        &self.grstctl
    }
    #[doc = "0x3c014 - Interrupt Register"]
    #[inline(always)]
    pub const fn gintsts(&self) -> &Gintsts {
        &self.gintsts
    }
    #[doc = "0x3c018 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn gintmsk(&self) -> &Gintmsk {
        &self.gintmsk
    }
    #[doc = "0x3c01c - Receive Status Debug Read Register"]
    #[inline(always)]
    pub const fn grxstsr(&self) -> &Grxstsr {
        &self.grxstsr
    }
    #[doc = "0x3c020 - Receive Status Read and Pop Register"]
    #[inline(always)]
    pub const fn grxstsp(&self) -> &Grxstsp {
        &self.grxstsp
    }
    #[doc = "0x3c024 - Receive FIFO Size Register"]
    #[inline(always)]
    pub const fn grxfsiz(&self) -> &Grxfsiz {
        &self.grxfsiz
    }
    #[doc = "0x3c028 - Non-periodic Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn gnptxfsiz(&self) -> &Gnptxfsiz {
        &self.gnptxfsiz
    }
    #[doc = "0x3c05c - Global DFIFO Configuration Register"]
    #[inline(always)]
    pub const fn gdfifocfg(&self) -> &Gdfifocfg {
        &self.gdfifocfg
    }
    #[doc = "0x3c104 - Device IN Endpoint Transmit FIFO 1 Size Register"]
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &Dieptxf1 {
        &self.dieptxf1
    }
    #[doc = "0x3c108 - Device IN Endpoint Transmit FIFO 2 Size Register"]
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &Dieptxf2 {
        &self.dieptxf2
    }
    #[doc = "0x3c10c - Device IN Endpoint Transmit FIFO 3 Size Register"]
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &Dieptxf3 {
        &self.dieptxf3
    }
    #[doc = "0x3c800 - Device Configuration Register"]
    #[inline(always)]
    pub const fn dcfg(&self) -> &Dcfg {
        &self.dcfg
    }
    #[doc = "0x3c804 - Device Control Register"]
    #[inline(always)]
    pub const fn dctl(&self) -> &Dctl {
        &self.dctl
    }
    #[doc = "0x3c808 - Device Status Register"]
    #[inline(always)]
    pub const fn dsts(&self) -> &Dsts {
        &self.dsts
    }
    #[doc = "0x3c810 - Device IN Endpoint Common Interrupt Mask Register"]
    #[inline(always)]
    pub const fn diepmsk(&self) -> &Diepmsk {
        &self.diepmsk
    }
    #[doc = "0x3c814 - Device OUT Endpoint Common Interrupt Mask Register"]
    #[inline(always)]
    pub const fn doepmsk(&self) -> &Doepmsk {
        &self.doepmsk
    }
    #[doc = "0x3c818 - Device All Endpoints Interrupt Register"]
    #[inline(always)]
    pub const fn daint(&self) -> &Daint {
        &self.daint
    }
    #[doc = "0x3c81c - Device All Endpoints Interrupt Mask Register"]
    #[inline(always)]
    pub const fn daintmsk(&self) -> &Daintmsk {
        &self.daintmsk
    }
    #[doc = "0x3c834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &Diepempmsk {
        &self.diepempmsk
    }
    #[doc = "0x3c900 - Device IN Endpoint 0 Control Register"]
    #[inline(always)]
    pub const fn diep0ctl(&self) -> &Diep0ctl {
        &self.diep0ctl
    }
    #[doc = "0x3c908 - Device IN Endpoint 0 Interrupt Register"]
    #[inline(always)]
    pub const fn diep0int(&self) -> &Diep0int {
        &self.diep0int
    }
    #[doc = "0x3c910 - Device IN Endpoint 0 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep0tsiz(&self) -> &Diep0tsiz {
        &self.diep0tsiz
    }
    #[doc = "0x3c914 - Device IN Endpoint 0 DMA Address Register"]
    #[inline(always)]
    pub const fn diep0dmaaddr(&self) -> &Diep0dmaaddr {
        &self.diep0dmaaddr
    }
    #[doc = "0x3c918 - Device IN Endpoint 0 Transmit FIFO Status Register"]
    #[inline(always)]
    pub const fn diep0txfsts(&self) -> &Diep0txfsts {
        &self.diep0txfsts
    }
    #[doc = "0x3c920 - Device IN Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn diep0_ctl(&self) -> &Diep0Ctl {
        &self.diep0_ctl
    }
    #[doc = "0x3c928 - Device IN Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn diep0_int(&self) -> &Diep0Int {
        &self.diep0_int
    }
    #[doc = "0x3c930 - Device IN Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep0_tsiz(&self) -> &Diep0Tsiz {
        &self.diep0_tsiz
    }
    #[doc = "0x3c934 - Device IN Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn diep0_dmaaddr(&self) -> &Diep0Dmaaddr {
        &self.diep0_dmaaddr
    }
    #[doc = "0x3c938 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    #[inline(always)]
    pub const fn diep0_txfsts(&self) -> &Diep0Txfsts {
        &self.diep0_txfsts
    }
    #[doc = "0x3c940 - Device IN Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn diep1_ctl(&self) -> &Diep1Ctl {
        &self.diep1_ctl
    }
    #[doc = "0x3c948 - Device IN Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn diep1_int(&self) -> &Diep1Int {
        &self.diep1_int
    }
    #[doc = "0x3c950 - Device IN Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep1_tsiz(&self) -> &Diep1Tsiz {
        &self.diep1_tsiz
    }
    #[doc = "0x3c954 - Device IN Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn diep1_dmaaddr(&self) -> &Diep1Dmaaddr {
        &self.diep1_dmaaddr
    }
    #[doc = "0x3c958 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    #[inline(always)]
    pub const fn diep1_txfsts(&self) -> &Diep1Txfsts {
        &self.diep1_txfsts
    }
    #[doc = "0x3c960 - Device IN Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn diep2_ctl(&self) -> &Diep2Ctl {
        &self.diep2_ctl
    }
    #[doc = "0x3c968 - Device IN Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn diep2_int(&self) -> &Diep2Int {
        &self.diep2_int
    }
    #[doc = "0x3c970 - Device IN Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep2_tsiz(&self) -> &Diep2Tsiz {
        &self.diep2_tsiz
    }
    #[doc = "0x3c974 - Device IN Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn diep2_dmaaddr(&self) -> &Diep2Dmaaddr {
        &self.diep2_dmaaddr
    }
    #[doc = "0x3c978 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    #[inline(always)]
    pub const fn diep2_txfsts(&self) -> &Diep2Txfsts {
        &self.diep2_txfsts
    }
    #[doc = "0x3cb00 - Device OUT Endpoint 0 Control Register"]
    #[inline(always)]
    pub const fn doep0ctl(&self) -> &Doep0ctl {
        &self.doep0ctl
    }
    #[doc = "0x3cb08 - Device OUT Endpoint 0 Interrupt Register"]
    #[inline(always)]
    pub const fn doep0int(&self) -> &Doep0int {
        &self.doep0int
    }
    #[doc = "0x3cb10 - Device OUT Endpoint 0 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep0tsiz(&self) -> &Doep0tsiz {
        &self.doep0tsiz
    }
    #[doc = "0x3cb14 - Device OUT Endpoint 0 DMA Address Register"]
    #[inline(always)]
    pub const fn doep0dmaaddr(&self) -> &Doep0dmaaddr {
        &self.doep0dmaaddr
    }
    #[doc = "0x3cb20 - Device OUT Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn doep0_ctl(&self) -> &Doep0Ctl {
        &self.doep0_ctl
    }
    #[doc = "0x3cb28 - Device OUT Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn doep0_int(&self) -> &Doep0Int {
        &self.doep0_int
    }
    #[doc = "0x3cb30 - Device OUT Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep0_tsiz(&self) -> &Doep0Tsiz {
        &self.doep0_tsiz
    }
    #[doc = "0x3cb34 - Device OUT Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn doep0_dmaaddr(&self) -> &Doep0Dmaaddr {
        &self.doep0_dmaaddr
    }
    #[doc = "0x3cb40 - Device OUT Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn doep1_ctl(&self) -> &Doep1Ctl {
        &self.doep1_ctl
    }
    #[doc = "0x3cb48 - Device OUT Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn doep1_int(&self) -> &Doep1Int {
        &self.doep1_int
    }
    #[doc = "0x3cb50 - Device OUT Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep1_tsiz(&self) -> &Doep1Tsiz {
        &self.doep1_tsiz
    }
    #[doc = "0x3cb54 - Device OUT Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn doep1_dmaaddr(&self) -> &Doep1Dmaaddr {
        &self.doep1_dmaaddr
    }
    #[doc = "0x3cb60 - Device OUT Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn doep2_ctl(&self) -> &Doep2Ctl {
        &self.doep2_ctl
    }
    #[doc = "0x3cb68 - Device OUT Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn doep2_int(&self) -> &Doep2Int {
        &self.doep2_int
    }
    #[doc = "0x3cb70 - Device OUT Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep2_tsiz(&self) -> &Doep2Tsiz {
        &self.doep2_tsiz
    }
    #[doc = "0x3cb74 - Device OUT Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn doep2_dmaaddr(&self) -> &Doep2Dmaaddr {
        &self.doep2_dmaaddr
    }
    #[doc = "0x3ce00 - Power and Clock Gating Control Register"]
    #[inline(always)]
    pub const fn pcgcctl(&self) -> &Pcgcctl {
        &self.pcgcctl
    }
}
#[doc = "CTRL (rw) register accessor: System Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "System Control Register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: System Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "System Status Register"]
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
#[doc = "ROUTE (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@route`] module"]
#[doc(alias = "ROUTE")]
pub type Route = crate::Reg<route::RouteSpec>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "GAHBCFG (rw) register accessor: AHB Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gahbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gahbcfg`] module"]
#[doc(alias = "GAHBCFG")]
pub type Gahbcfg = crate::Reg<gahbcfg::GahbcfgSpec>;
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: USB Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gusbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gusbcfg`] module"]
#[doc(alias = "GUSBCFG")]
pub type Gusbcfg = crate::Reg<gusbcfg::GusbcfgSpec>;
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`grstctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstctl`] module"]
#[doc(alias = "GRSTCTL")]
pub type Grstctl = crate::Reg<grstctl::GrstctlSpec>;
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gintsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintsts`] module"]
#[doc(alias = "GINTSTS")]
pub type Gintsts = crate::Reg<gintsts::GintstsSpec>;
#[doc = "Interrupt Register"]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintmsk`] module"]
#[doc(alias = "GINTMSK")]
pub type Gintmsk = crate::Reg<gintmsk::GintmskSpec>;
#[doc = "Interrupt Mask Register"]
pub mod gintmsk;
#[doc = "GRXSTSR (r) register accessor: Receive Status Debug Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr`] module"]
#[doc(alias = "GRXSTSR")]
pub type Grxstsr = crate::Reg<grxstsr::GrxstsrSpec>;
#[doc = "Receive Status Debug Read Register"]
pub mod grxstsr;
#[doc = "GRXSTSP (r) register accessor: Receive Status Read and Pop Register\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp`] module"]
#[doc(alias = "GRXSTSP")]
pub type Grxstsp = crate::Reg<grxstsp::GrxstspSpec>;
#[doc = "Receive Status Read and Pop Register"]
pub mod grxstsp;
#[doc = "GRXFSIZ (rw) register accessor: Receive FIFO Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`grxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxfsiz`] module"]
#[doc(alias = "GRXFSIZ")]
pub type Grxfsiz = crate::Reg<grxfsiz::GrxfsizSpec>;
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ (rw) register accessor: Non-periodic Transmit FIFO Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gnptxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gnptxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxfsiz`] module"]
#[doc(alias = "GNPTXFSIZ")]
pub type Gnptxfsiz = crate::Reg<gnptxfsiz::GnptxfsizSpec>;
#[doc = "Non-periodic Transmit FIFO Size Register"]
pub mod gnptxfsiz;
#[doc = "GDFIFOCFG (rw) register accessor: Global DFIFO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdfifocfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdfifocfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdfifocfg`] module"]
#[doc(alias = "GDFIFOCFG")]
pub type Gdfifocfg = crate::Reg<gdfifocfg::GdfifocfgSpec>;
#[doc = "Global DFIFO Configuration Register"]
pub mod gdfifocfg;
#[doc = "DIEPTXF1 (rw) register accessor: Device IN Endpoint Transmit FIFO 1 Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf1`] module"]
#[doc(alias = "DIEPTXF1")]
pub type Dieptxf1 = crate::Reg<dieptxf1::Dieptxf1Spec>;
#[doc = "Device IN Endpoint Transmit FIFO 1 Size Register"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: Device IN Endpoint Transmit FIFO 2 Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf2`] module"]
#[doc(alias = "DIEPTXF2")]
pub type Dieptxf2 = crate::Reg<dieptxf2::Dieptxf2Spec>;
#[doc = "Device IN Endpoint Transmit FIFO 2 Size Register"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: Device IN Endpoint Transmit FIFO 3 Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf3`] module"]
#[doc(alias = "DIEPTXF3")]
pub type Dieptxf3 = crate::Reg<dieptxf3::Dieptxf3Spec>;
#[doc = "Device IN Endpoint Transmit FIFO 3 Size Register"]
pub mod dieptxf3;
#[doc = "DCFG (rw) register accessor: Device Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`] module"]
#[doc(alias = "DCFG")]
pub type Dcfg = crate::Reg<dcfg::DcfgSpec>;
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: Device Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`] module"]
#[doc(alias = "DCTL")]
pub type Dctl = crate::Reg<dctl::DctlSpec>;
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: Device Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsts`] module"]
#[doc(alias = "DSTS")]
pub type Dsts = crate::Reg<dsts::DstsSpec>;
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: Device IN Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepmsk`] module"]
#[doc(alias = "DIEPMSK")]
pub type Diepmsk = crate::Reg<diepmsk::DiepmskSpec>;
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: Device OUT Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepmsk`] module"]
#[doc(alias = "DOEPMSK")]
pub type Doepmsk = crate::Reg<doepmsk::DoepmskSpec>;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: Device All Endpoints Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`daint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daint`] module"]
#[doc(alias = "DAINT")]
pub type Daint = crate::Reg<daint::DaintSpec>;
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: Device All Endpoints Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`daintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daintmsk`] module"]
#[doc(alias = "DAINTMSK")]
pub type Daintmsk = crate::Reg<daintmsk::DaintmskSpec>;
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "DIEPEMPMSK (rw) register accessor: Device IN Endpoint FIFO Empty Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepempmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepempmsk`] module"]
#[doc(alias = "DIEPEMPMSK")]
pub type Diepempmsk = crate::Reg<diepempmsk::DiepempmskSpec>;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "DIEP0CTL (rw) register accessor: Device IN Endpoint 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0ctl`] module"]
#[doc(alias = "DIEP0CTL")]
pub type Diep0ctl = crate::Reg<diep0ctl::Diep0ctlSpec>;
#[doc = "Device IN Endpoint 0 Control Register"]
pub mod diep0ctl;
#[doc = "DIEP0INT (rw) register accessor: Device IN Endpoint 0 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0int`] module"]
#[doc(alias = "DIEP0INT")]
pub type Diep0int = crate::Reg<diep0int::Diep0intSpec>;
#[doc = "Device IN Endpoint 0 Interrupt Register"]
pub mod diep0int;
#[doc = "DIEP0TSIZ (rw) register accessor: Device IN Endpoint 0 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0tsiz`] module"]
#[doc(alias = "DIEP0TSIZ")]
pub type Diep0tsiz = crate::Reg<diep0tsiz::Diep0tsizSpec>;
#[doc = "Device IN Endpoint 0 Transfer Size Register"]
pub mod diep0tsiz;
#[doc = "DIEP0DMAADDR (rw) register accessor: Device IN Endpoint 0 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0dmaaddr`] module"]
#[doc(alias = "DIEP0DMAADDR")]
pub type Diep0dmaaddr = crate::Reg<diep0dmaaddr::Diep0dmaaddrSpec>;
#[doc = "Device IN Endpoint 0 DMA Address Register"]
pub mod diep0dmaaddr;
#[doc = "DIEP0TXFSTS (r) register accessor: Device IN Endpoint 0 Transmit FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0txfsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0txfsts`] module"]
#[doc(alias = "DIEP0TXFSTS")]
pub type Diep0txfsts = crate::Reg<diep0txfsts::Diep0txfstsSpec>;
#[doc = "Device IN Endpoint 0 Transmit FIFO Status Register"]
pub mod diep0txfsts;
#[doc = "DIEP0_CTL (rw) register accessor: Device IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_ctl`] module"]
#[doc(alias = "DIEP0_CTL")]
pub type Diep0Ctl = crate::Reg<diep0_ctl::Diep0CtlSpec>;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep0_ctl;
#[doc = "DIEP0_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_int`] module"]
#[doc(alias = "DIEP0_INT")]
pub type Diep0Int = crate::Reg<diep0_int::Diep0IntSpec>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep0_int;
#[doc = "DIEP0_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_tsiz`] module"]
#[doc(alias = "DIEP0_TSIZ")]
pub type Diep0Tsiz = crate::Reg<diep0_tsiz::Diep0TsizSpec>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep0_tsiz;
#[doc = "DIEP0_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_dmaaddr`] module"]
#[doc(alias = "DIEP0_DMAADDR")]
pub type Diep0Dmaaddr = crate::Reg<diep0_dmaaddr::Diep0DmaaddrSpec>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep0_dmaaddr;
#[doc = "DIEP0_TXFSTS (r) register accessor: Device IN Endpoint x+1 Transmit FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0_txfsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_txfsts`] module"]
#[doc(alias = "DIEP0_TXFSTS")]
pub type Diep0Txfsts = crate::Reg<diep0_txfsts::Diep0TxfstsSpec>;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep0_txfsts;
#[doc = "DIEP1_CTL (rw) register accessor: Device IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep1_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep1_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_ctl`] module"]
#[doc(alias = "DIEP1_CTL")]
pub type Diep1Ctl = crate::Reg<diep1_ctl::Diep1CtlSpec>;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep1_ctl;
#[doc = "DIEP1_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep1_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep1_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_int`] module"]
#[doc(alias = "DIEP1_INT")]
pub type Diep1Int = crate::Reg<diep1_int::Diep1IntSpec>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep1_int;
#[doc = "DIEP1_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep1_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep1_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_tsiz`] module"]
#[doc(alias = "DIEP1_TSIZ")]
pub type Diep1Tsiz = crate::Reg<diep1_tsiz::Diep1TsizSpec>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep1_tsiz;
#[doc = "DIEP1_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep1_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep1_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_dmaaddr`] module"]
#[doc(alias = "DIEP1_DMAADDR")]
pub type Diep1Dmaaddr = crate::Reg<diep1_dmaaddr::Diep1DmaaddrSpec>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep1_dmaaddr;
#[doc = "DIEP1_TXFSTS (r) register accessor: Device IN Endpoint x+1 Transmit FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep1_txfsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_txfsts`] module"]
#[doc(alias = "DIEP1_TXFSTS")]
pub type Diep1Txfsts = crate::Reg<diep1_txfsts::Diep1TxfstsSpec>;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep1_txfsts;
#[doc = "DIEP2_CTL (rw) register accessor: Device IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep2_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep2_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_ctl`] module"]
#[doc(alias = "DIEP2_CTL")]
pub type Diep2Ctl = crate::Reg<diep2_ctl::Diep2CtlSpec>;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep2_ctl;
#[doc = "DIEP2_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep2_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep2_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_int`] module"]
#[doc(alias = "DIEP2_INT")]
pub type Diep2Int = crate::Reg<diep2_int::Diep2IntSpec>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep2_int;
#[doc = "DIEP2_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep2_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep2_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_tsiz`] module"]
#[doc(alias = "DIEP2_TSIZ")]
pub type Diep2Tsiz = crate::Reg<diep2_tsiz::Diep2TsizSpec>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep2_tsiz;
#[doc = "DIEP2_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep2_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep2_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_dmaaddr`] module"]
#[doc(alias = "DIEP2_DMAADDR")]
pub type Diep2Dmaaddr = crate::Reg<diep2_dmaaddr::Diep2DmaaddrSpec>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep2_dmaaddr;
#[doc = "DIEP2_TXFSTS (r) register accessor: Device IN Endpoint x+1 Transmit FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep2_txfsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_txfsts`] module"]
#[doc(alias = "DIEP2_TXFSTS")]
pub type Diep2Txfsts = crate::Reg<diep2_txfsts::Diep2TxfstsSpec>;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep2_txfsts;
#[doc = "DOEP0CTL (rw) register accessor: Device OUT Endpoint 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0ctl`] module"]
#[doc(alias = "DOEP0CTL")]
pub type Doep0ctl = crate::Reg<doep0ctl::Doep0ctlSpec>;
#[doc = "Device OUT Endpoint 0 Control Register"]
pub mod doep0ctl;
#[doc = "DOEP0INT (rw) register accessor: Device OUT Endpoint 0 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0int`] module"]
#[doc(alias = "DOEP0INT")]
pub type Doep0int = crate::Reg<doep0int::Doep0intSpec>;
#[doc = "Device OUT Endpoint 0 Interrupt Register"]
pub mod doep0int;
#[doc = "DOEP0TSIZ (rw) register accessor: Device OUT Endpoint 0 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0tsiz`] module"]
#[doc(alias = "DOEP0TSIZ")]
pub type Doep0tsiz = crate::Reg<doep0tsiz::Doep0tsizSpec>;
#[doc = "Device OUT Endpoint 0 Transfer Size Register"]
pub mod doep0tsiz;
#[doc = "DOEP0DMAADDR (rw) register accessor: Device OUT Endpoint 0 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0dmaaddr`] module"]
#[doc(alias = "DOEP0DMAADDR")]
pub type Doep0dmaaddr = crate::Reg<doep0dmaaddr::Doep0dmaaddrSpec>;
#[doc = "Device OUT Endpoint 0 DMA Address Register"]
pub mod doep0dmaaddr;
#[doc = "DOEP0_CTL (rw) register accessor: Device OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0_ctl`] module"]
#[doc(alias = "DOEP0_CTL")]
pub type Doep0Ctl = crate::Reg<doep0_ctl::Doep0CtlSpec>;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep0_ctl;
#[doc = "DOEP0_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0_int`] module"]
#[doc(alias = "DOEP0_INT")]
pub type Doep0Int = crate::Reg<doep0_int::Doep0IntSpec>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep0_int;
#[doc = "DOEP0_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0_tsiz`] module"]
#[doc(alias = "DOEP0_TSIZ")]
pub type Doep0Tsiz = crate::Reg<doep0_tsiz::Doep0TsizSpec>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep0_tsiz;
#[doc = "DOEP0_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0_dmaaddr`] module"]
#[doc(alias = "DOEP0_DMAADDR")]
pub type Doep0Dmaaddr = crate::Reg<doep0_dmaaddr::Doep0DmaaddrSpec>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep0_dmaaddr;
#[doc = "DOEP1_CTL (rw) register accessor: Device OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep1_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep1_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1_ctl`] module"]
#[doc(alias = "DOEP1_CTL")]
pub type Doep1Ctl = crate::Reg<doep1_ctl::Doep1CtlSpec>;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep1_ctl;
#[doc = "DOEP1_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep1_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep1_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1_int`] module"]
#[doc(alias = "DOEP1_INT")]
pub type Doep1Int = crate::Reg<doep1_int::Doep1IntSpec>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep1_int;
#[doc = "DOEP1_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep1_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep1_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1_tsiz`] module"]
#[doc(alias = "DOEP1_TSIZ")]
pub type Doep1Tsiz = crate::Reg<doep1_tsiz::Doep1TsizSpec>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep1_tsiz;
#[doc = "DOEP1_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep1_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep1_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1_dmaaddr`] module"]
#[doc(alias = "DOEP1_DMAADDR")]
pub type Doep1Dmaaddr = crate::Reg<doep1_dmaaddr::Doep1DmaaddrSpec>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep1_dmaaddr;
#[doc = "DOEP2_CTL (rw) register accessor: Device OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep2_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep2_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2_ctl`] module"]
#[doc(alias = "DOEP2_CTL")]
pub type Doep2Ctl = crate::Reg<doep2_ctl::Doep2CtlSpec>;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep2_ctl;
#[doc = "DOEP2_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep2_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep2_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2_int`] module"]
#[doc(alias = "DOEP2_INT")]
pub type Doep2Int = crate::Reg<doep2_int::Doep2IntSpec>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep2_int;
#[doc = "DOEP2_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep2_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep2_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2_tsiz`] module"]
#[doc(alias = "DOEP2_TSIZ")]
pub type Doep2Tsiz = crate::Reg<doep2_tsiz::Doep2TsizSpec>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep2_tsiz;
#[doc = "DOEP2_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep2_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep2_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2_dmaaddr`] module"]
#[doc(alias = "DOEP2_DMAADDR")]
pub type Doep2Dmaaddr = crate::Reg<doep2_dmaaddr::Doep2DmaaddrSpec>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep2_dmaaddr;
#[doc = "PCGCCTL (rw) register accessor: Power and Clock Gating Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcgcctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcgcctl`] module"]
#[doc(alias = "PCGCCTL")]
pub type Pcgcctl = crate::Reg<pcgcctl::PcgcctlSpec>;
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;
