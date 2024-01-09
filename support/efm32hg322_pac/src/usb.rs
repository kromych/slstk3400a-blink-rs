#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    status: STATUS,
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    ien: IEN,
    route: ROUTE,
    _reserved7: [u8; 0x0003_bfec],
    gahbcfg: GAHBCFG,
    gusbcfg: GUSBCFG,
    grstctl: GRSTCTL,
    gintsts: GINTSTS,
    gintmsk: GINTMSK,
    grxstsr: GRXSTSR,
    grxstsp: GRXSTSP,
    grxfsiz: GRXFSIZ,
    gnptxfsiz: GNPTXFSIZ,
    _reserved16: [u8; 0x30],
    gdfifocfg: GDFIFOCFG,
    _reserved17: [u8; 0xa4],
    dieptxf1: DIEPTXF1,
    dieptxf2: DIEPTXF2,
    dieptxf3: DIEPTXF3,
    _reserved20: [u8; 0x06f0],
    dcfg: DCFG,
    dctl: DCTL,
    dsts: DSTS,
    _reserved23: [u8; 0x04],
    diepmsk: DIEPMSK,
    doepmsk: DOEPMSK,
    daint: DAINT,
    daintmsk: DAINTMSK,
    _reserved27: [u8; 0x14],
    diepempmsk: DIEPEMPMSK,
    _reserved28: [u8; 0xc8],
    diep0ctl: DIEP0CTL,
    _reserved29: [u8; 0x04],
    diep0int: DIEP0INT,
    _reserved30: [u8; 0x04],
    diep0tsiz: DIEP0TSIZ,
    diep0dmaaddr: DIEP0DMAADDR,
    diep0txfsts: DIEP0TXFSTS,
    _reserved33: [u8; 0x04],
    diep0_ctl: DIEP0_CTL,
    _reserved34: [u8; 0x04],
    diep0_int: DIEP0_INT,
    _reserved35: [u8; 0x04],
    diep0_tsiz: DIEP0_TSIZ,
    diep0_dmaaddr: DIEP0_DMAADDR,
    diep0_txfsts: DIEP0_TXFSTS,
    _reserved38: [u8; 0x04],
    diep1_ctl: DIEP1_CTL,
    _reserved39: [u8; 0x04],
    diep1_int: DIEP1_INT,
    _reserved40: [u8; 0x04],
    diep1_tsiz: DIEP1_TSIZ,
    diep1_dmaaddr: DIEP1_DMAADDR,
    diep1_txfsts: DIEP1_TXFSTS,
    _reserved43: [u8; 0x04],
    diep2_ctl: DIEP2_CTL,
    _reserved44: [u8; 0x04],
    diep2_int: DIEP2_INT,
    _reserved45: [u8; 0x04],
    diep2_tsiz: DIEP2_TSIZ,
    diep2_dmaaddr: DIEP2_DMAADDR,
    diep2_txfsts: DIEP2_TXFSTS,
    _reserved48: [u8; 0x0184],
    doep0ctl: DOEP0CTL,
    _reserved49: [u8; 0x04],
    doep0int: DOEP0INT,
    _reserved50: [u8; 0x04],
    doep0tsiz: DOEP0TSIZ,
    doep0dmaaddr: DOEP0DMAADDR,
    _reserved52: [u8; 0x08],
    doep0_ctl: DOEP0_CTL,
    _reserved53: [u8; 0x04],
    doep0_int: DOEP0_INT,
    _reserved54: [u8; 0x04],
    doep0_tsiz: DOEP0_TSIZ,
    doep0_dmaaddr: DOEP0_DMAADDR,
    _reserved56: [u8; 0x08],
    doep1_ctl: DOEP1_CTL,
    _reserved57: [u8; 0x04],
    doep1_int: DOEP1_INT,
    _reserved58: [u8; 0x04],
    doep1_tsiz: DOEP1_TSIZ,
    doep1_dmaaddr: DOEP1_DMAADDR,
    _reserved60: [u8; 0x08],
    doep2_ctl: DOEP2_CTL,
    _reserved61: [u8; 0x04],
    doep2_int: DOEP2_INT,
    _reserved62: [u8; 0x04],
    doep2_tsiz: DOEP2_TSIZ,
    doep2_dmaaddr: DOEP2_DMAADDR,
    _reserved64: [u8; 0x0288],
    pcgcctl: PCGCCTL,
}
impl RegisterBlock {
    #[doc = "0x00 - System Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - System Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x08 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0x0c - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0x10 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
    #[doc = "0x14 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0x18 - I/O Routing Register"]
    #[inline(always)]
    pub const fn route(&self) -> &ROUTE {
        &self.route
    }
    #[doc = "0x3c008 - AHB Configuration Register"]
    #[inline(always)]
    pub const fn gahbcfg(&self) -> &GAHBCFG {
        &self.gahbcfg
    }
    #[doc = "0x3c00c - USB Configuration Register"]
    #[inline(always)]
    pub const fn gusbcfg(&self) -> &GUSBCFG {
        &self.gusbcfg
    }
    #[doc = "0x3c010 - Reset Register"]
    #[inline(always)]
    pub const fn grstctl(&self) -> &GRSTCTL {
        &self.grstctl
    }
    #[doc = "0x3c014 - Interrupt Register"]
    #[inline(always)]
    pub const fn gintsts(&self) -> &GINTSTS {
        &self.gintsts
    }
    #[doc = "0x3c018 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn gintmsk(&self) -> &GINTMSK {
        &self.gintmsk
    }
    #[doc = "0x3c01c - Receive Status Debug Read Register"]
    #[inline(always)]
    pub const fn grxstsr(&self) -> &GRXSTSR {
        &self.grxstsr
    }
    #[doc = "0x3c020 - Receive Status Read and Pop Register"]
    #[inline(always)]
    pub const fn grxstsp(&self) -> &GRXSTSP {
        &self.grxstsp
    }
    #[doc = "0x3c024 - Receive FIFO Size Register"]
    #[inline(always)]
    pub const fn grxfsiz(&self) -> &GRXFSIZ {
        &self.grxfsiz
    }
    #[doc = "0x3c028 - Non-periodic Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn gnptxfsiz(&self) -> &GNPTXFSIZ {
        &self.gnptxfsiz
    }
    #[doc = "0x3c05c - Global DFIFO Configuration Register"]
    #[inline(always)]
    pub const fn gdfifocfg(&self) -> &GDFIFOCFG {
        &self.gdfifocfg
    }
    #[doc = "0x3c104 - Device IN Endpoint Transmit FIFO 1 Size Register"]
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &DIEPTXF1 {
        &self.dieptxf1
    }
    #[doc = "0x3c108 - Device IN Endpoint Transmit FIFO 2 Size Register"]
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &DIEPTXF2 {
        &self.dieptxf2
    }
    #[doc = "0x3c10c - Device IN Endpoint Transmit FIFO 3 Size Register"]
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &DIEPTXF3 {
        &self.dieptxf3
    }
    #[doc = "0x3c800 - Device Configuration Register"]
    #[inline(always)]
    pub const fn dcfg(&self) -> &DCFG {
        &self.dcfg
    }
    #[doc = "0x3c804 - Device Control Register"]
    #[inline(always)]
    pub const fn dctl(&self) -> &DCTL {
        &self.dctl
    }
    #[doc = "0x3c808 - Device Status Register"]
    #[inline(always)]
    pub const fn dsts(&self) -> &DSTS {
        &self.dsts
    }
    #[doc = "0x3c810 - Device IN Endpoint Common Interrupt Mask Register"]
    #[inline(always)]
    pub const fn diepmsk(&self) -> &DIEPMSK {
        &self.diepmsk
    }
    #[doc = "0x3c814 - Device OUT Endpoint Common Interrupt Mask Register"]
    #[inline(always)]
    pub const fn doepmsk(&self) -> &DOEPMSK {
        &self.doepmsk
    }
    #[doc = "0x3c818 - Device All Endpoints Interrupt Register"]
    #[inline(always)]
    pub const fn daint(&self) -> &DAINT {
        &self.daint
    }
    #[doc = "0x3c81c - Device All Endpoints Interrupt Mask Register"]
    #[inline(always)]
    pub const fn daintmsk(&self) -> &DAINTMSK {
        &self.daintmsk
    }
    #[doc = "0x3c834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &DIEPEMPMSK {
        &self.diepempmsk
    }
    #[doc = "0x3c900 - Device IN Endpoint 0 Control Register"]
    #[inline(always)]
    pub const fn diep0ctl(&self) -> &DIEP0CTL {
        &self.diep0ctl
    }
    #[doc = "0x3c908 - Device IN Endpoint 0 Interrupt Register"]
    #[inline(always)]
    pub const fn diep0int(&self) -> &DIEP0INT {
        &self.diep0int
    }
    #[doc = "0x3c910 - Device IN Endpoint 0 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep0tsiz(&self) -> &DIEP0TSIZ {
        &self.diep0tsiz
    }
    #[doc = "0x3c914 - Device IN Endpoint 0 DMA Address Register"]
    #[inline(always)]
    pub const fn diep0dmaaddr(&self) -> &DIEP0DMAADDR {
        &self.diep0dmaaddr
    }
    #[doc = "0x3c918 - Device IN Endpoint 0 Transmit FIFO Status Register"]
    #[inline(always)]
    pub const fn diep0txfsts(&self) -> &DIEP0TXFSTS {
        &self.diep0txfsts
    }
    #[doc = "0x3c920 - Device IN Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn diep0_ctl(&self) -> &DIEP0_CTL {
        &self.diep0_ctl
    }
    #[doc = "0x3c928 - Device IN Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn diep0_int(&self) -> &DIEP0_INT {
        &self.diep0_int
    }
    #[doc = "0x3c930 - Device IN Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep0_tsiz(&self) -> &DIEP0_TSIZ {
        &self.diep0_tsiz
    }
    #[doc = "0x3c934 - Device IN Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn diep0_dmaaddr(&self) -> &DIEP0_DMAADDR {
        &self.diep0_dmaaddr
    }
    #[doc = "0x3c938 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    #[inline(always)]
    pub const fn diep0_txfsts(&self) -> &DIEP0_TXFSTS {
        &self.diep0_txfsts
    }
    #[doc = "0x3c940 - Device IN Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn diep1_ctl(&self) -> &DIEP1_CTL {
        &self.diep1_ctl
    }
    #[doc = "0x3c948 - Device IN Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn diep1_int(&self) -> &DIEP1_INT {
        &self.diep1_int
    }
    #[doc = "0x3c950 - Device IN Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep1_tsiz(&self) -> &DIEP1_TSIZ {
        &self.diep1_tsiz
    }
    #[doc = "0x3c954 - Device IN Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn diep1_dmaaddr(&self) -> &DIEP1_DMAADDR {
        &self.diep1_dmaaddr
    }
    #[doc = "0x3c958 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    #[inline(always)]
    pub const fn diep1_txfsts(&self) -> &DIEP1_TXFSTS {
        &self.diep1_txfsts
    }
    #[doc = "0x3c960 - Device IN Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn diep2_ctl(&self) -> &DIEP2_CTL {
        &self.diep2_ctl
    }
    #[doc = "0x3c968 - Device IN Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn diep2_int(&self) -> &DIEP2_INT {
        &self.diep2_int
    }
    #[doc = "0x3c970 - Device IN Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep2_tsiz(&self) -> &DIEP2_TSIZ {
        &self.diep2_tsiz
    }
    #[doc = "0x3c974 - Device IN Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn diep2_dmaaddr(&self) -> &DIEP2_DMAADDR {
        &self.diep2_dmaaddr
    }
    #[doc = "0x3c978 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    #[inline(always)]
    pub const fn diep2_txfsts(&self) -> &DIEP2_TXFSTS {
        &self.diep2_txfsts
    }
    #[doc = "0x3cb00 - Device OUT Endpoint 0 Control Register"]
    #[inline(always)]
    pub const fn doep0ctl(&self) -> &DOEP0CTL {
        &self.doep0ctl
    }
    #[doc = "0x3cb08 - Device OUT Endpoint 0 Interrupt Register"]
    #[inline(always)]
    pub const fn doep0int(&self) -> &DOEP0INT {
        &self.doep0int
    }
    #[doc = "0x3cb10 - Device OUT Endpoint 0 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep0tsiz(&self) -> &DOEP0TSIZ {
        &self.doep0tsiz
    }
    #[doc = "0x3cb14 - Device OUT Endpoint 0 DMA Address Register"]
    #[inline(always)]
    pub const fn doep0dmaaddr(&self) -> &DOEP0DMAADDR {
        &self.doep0dmaaddr
    }
    #[doc = "0x3cb20 - Device OUT Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn doep0_ctl(&self) -> &DOEP0_CTL {
        &self.doep0_ctl
    }
    #[doc = "0x3cb28 - Device OUT Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn doep0_int(&self) -> &DOEP0_INT {
        &self.doep0_int
    }
    #[doc = "0x3cb30 - Device OUT Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep0_tsiz(&self) -> &DOEP0_TSIZ {
        &self.doep0_tsiz
    }
    #[doc = "0x3cb34 - Device OUT Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn doep0_dmaaddr(&self) -> &DOEP0_DMAADDR {
        &self.doep0_dmaaddr
    }
    #[doc = "0x3cb40 - Device OUT Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn doep1_ctl(&self) -> &DOEP1_CTL {
        &self.doep1_ctl
    }
    #[doc = "0x3cb48 - Device OUT Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn doep1_int(&self) -> &DOEP1_INT {
        &self.doep1_int
    }
    #[doc = "0x3cb50 - Device OUT Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep1_tsiz(&self) -> &DOEP1_TSIZ {
        &self.doep1_tsiz
    }
    #[doc = "0x3cb54 - Device OUT Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn doep1_dmaaddr(&self) -> &DOEP1_DMAADDR {
        &self.doep1_dmaaddr
    }
    #[doc = "0x3cb60 - Device OUT Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn doep2_ctl(&self) -> &DOEP2_CTL {
        &self.doep2_ctl
    }
    #[doc = "0x3cb68 - Device OUT Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn doep2_int(&self) -> &DOEP2_INT {
        &self.doep2_int
    }
    #[doc = "0x3cb70 - Device OUT Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep2_tsiz(&self) -> &DOEP2_TSIZ {
        &self.doep2_tsiz
    }
    #[doc = "0x3cb74 - Device OUT Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn doep2_dmaaddr(&self) -> &DOEP2_DMAADDR {
        &self.doep2_dmaaddr
    }
    #[doc = "0x3ce00 - Power and Clock Gating Control Register"]
    #[inline(always)]
    pub const fn pcgcctl(&self) -> &PCGCCTL {
        &self.pcgcctl
    }
}
#[doc = "CTRL (rw) register accessor: System Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "System Control Register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: System Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "System Status Register"]
pub mod status;
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
#[doc = "ROUTE (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`route::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`route::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@route`]
module"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "GAHBCFG (rw) register accessor: AHB Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gahbcfg`]
module"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: USB Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gusbcfg`]
module"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstctl`]
module"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintsts`]
module"]
pub type GINTSTS = crate::Reg<gintsts::GINTSTS_SPEC>;
#[doc = "Interrupt Register"]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintmsk`]
module"]
pub type GINTMSK = crate::Reg<gintmsk::GINTMSK_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod gintmsk;
#[doc = "GRXSTSR (r) register accessor: Receive Status Debug Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr`]
module"]
pub type GRXSTSR = crate::Reg<grxstsr::GRXSTSR_SPEC>;
#[doc = "Receive Status Debug Read Register"]
pub mod grxstsr;
#[doc = "GRXSTSP (r) register accessor: Receive Status Read and Pop Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp`]
module"]
pub type GRXSTSP = crate::Reg<grxstsp::GRXSTSP_SPEC>;
#[doc = "Receive Status Read and Pop Register"]
pub mod grxstsp;
#[doc = "GRXFSIZ (rw) register accessor: Receive FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxfsiz`]
module"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ (rw) register accessor: Non-periodic Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxfsiz`]
module"]
pub type GNPTXFSIZ = crate::Reg<gnptxfsiz::GNPTXFSIZ_SPEC>;
#[doc = "Non-periodic Transmit FIFO Size Register"]
pub mod gnptxfsiz;
#[doc = "GDFIFOCFG (rw) register accessor: Global DFIFO Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdfifocfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdfifocfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdfifocfg`]
module"]
pub type GDFIFOCFG = crate::Reg<gdfifocfg::GDFIFOCFG_SPEC>;
#[doc = "Global DFIFO Configuration Register"]
pub mod gdfifocfg;
#[doc = "DIEPTXF1 (rw) register accessor: Device IN Endpoint Transmit FIFO 1 Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf1`]
module"]
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO 1 Size Register"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: Device IN Endpoint Transmit FIFO 2 Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf2`]
module"]
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO 2 Size Register"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: Device IN Endpoint Transmit FIFO 3 Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf3`]
module"]
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO 3 Size Register"]
pub mod dieptxf3;
#[doc = "DCFG (rw) register accessor: Device Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`]
module"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: Device Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`]
module"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: Device Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsts`]
module"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: Device IN Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepmsk`]
module"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: Device OUT Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepmsk`]
module"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: Device All Endpoints Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daint`]
module"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: Device All Endpoints Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daintmsk`]
module"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "DIEPEMPMSK (rw) register accessor: Device IN Endpoint FIFO Empty Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepempmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepempmsk`]
module"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "DIEP0CTL (rw) register accessor: Device IN Endpoint 0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0ctl`]
module"]
pub type DIEP0CTL = crate::Reg<diep0ctl::DIEP0CTL_SPEC>;
#[doc = "Device IN Endpoint 0 Control Register"]
pub mod diep0ctl;
#[doc = "DIEP0INT (rw) register accessor: Device IN Endpoint 0 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0int`]
module"]
pub type DIEP0INT = crate::Reg<diep0int::DIEP0INT_SPEC>;
#[doc = "Device IN Endpoint 0 Interrupt Register"]
pub mod diep0int;
#[doc = "DIEP0TSIZ (rw) register accessor: Device IN Endpoint 0 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0tsiz`]
module"]
pub type DIEP0TSIZ = crate::Reg<diep0tsiz::DIEP0TSIZ_SPEC>;
#[doc = "Device IN Endpoint 0 Transfer Size Register"]
pub mod diep0tsiz;
#[doc = "DIEP0DMAADDR (rw) register accessor: Device IN Endpoint 0 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0dmaaddr`]
module"]
pub type DIEP0DMAADDR = crate::Reg<diep0dmaaddr::DIEP0DMAADDR_SPEC>;
#[doc = "Device IN Endpoint 0 DMA Address Register"]
pub mod diep0dmaaddr;
#[doc = "DIEP0TXFSTS (r) register accessor: Device IN Endpoint 0 Transmit FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0txfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0txfsts`]
module"]
pub type DIEP0TXFSTS = crate::Reg<diep0txfsts::DIEP0TXFSTS_SPEC>;
#[doc = "Device IN Endpoint 0 Transmit FIFO Status Register"]
pub mod diep0txfsts;
#[doc = "DIEP0_CTL (rw) register accessor: Device IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_ctl`]
module"]
pub type DIEP0_CTL = crate::Reg<diep0_ctl::DIEP0_CTL_SPEC>;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep0_ctl;
#[doc = "DIEP0_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_int`]
module"]
pub type DIEP0_INT = crate::Reg<diep0_int::DIEP0_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep0_int;
#[doc = "DIEP0_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_tsiz`]
module"]
pub type DIEP0_TSIZ = crate::Reg<diep0_tsiz::DIEP0_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep0_tsiz;
#[doc = "DIEP0_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_dmaaddr`]
module"]
pub type DIEP0_DMAADDR = crate::Reg<diep0_dmaaddr::DIEP0_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep0_dmaaddr;
#[doc = "DIEP0_TXFSTS (r) register accessor: Device IN Endpoint x+1 Transmit FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0_txfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_txfsts`]
module"]
pub type DIEP0_TXFSTS = crate::Reg<diep0_txfsts::DIEP0_TXFSTS_SPEC>;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep0_txfsts;
#[doc = "DIEP1_CTL (rw) register accessor: Device IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_ctl`]
module"]
pub type DIEP1_CTL = crate::Reg<diep1_ctl::DIEP1_CTL_SPEC>;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep1_ctl;
#[doc = "DIEP1_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_int`]
module"]
pub type DIEP1_INT = crate::Reg<diep1_int::DIEP1_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep1_int;
#[doc = "DIEP1_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_tsiz`]
module"]
pub type DIEP1_TSIZ = crate::Reg<diep1_tsiz::DIEP1_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep1_tsiz;
#[doc = "DIEP1_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_dmaaddr`]
module"]
pub type DIEP1_DMAADDR = crate::Reg<diep1_dmaaddr::DIEP1_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep1_dmaaddr;
#[doc = "DIEP1_TXFSTS (r) register accessor: Device IN Endpoint x+1 Transmit FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1_txfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_txfsts`]
module"]
pub type DIEP1_TXFSTS = crate::Reg<diep1_txfsts::DIEP1_TXFSTS_SPEC>;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep1_txfsts;
#[doc = "DIEP2_CTL (rw) register accessor: Device IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_ctl`]
module"]
pub type DIEP2_CTL = crate::Reg<diep2_ctl::DIEP2_CTL_SPEC>;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep2_ctl;
#[doc = "DIEP2_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_int`]
module"]
pub type DIEP2_INT = crate::Reg<diep2_int::DIEP2_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep2_int;
#[doc = "DIEP2_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_tsiz`]
module"]
pub type DIEP2_TSIZ = crate::Reg<diep2_tsiz::DIEP2_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep2_tsiz;
#[doc = "DIEP2_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_dmaaddr`]
module"]
pub type DIEP2_DMAADDR = crate::Reg<diep2_dmaaddr::DIEP2_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep2_dmaaddr;
#[doc = "DIEP2_TXFSTS (r) register accessor: Device IN Endpoint x+1 Transmit FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2_txfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_txfsts`]
module"]
pub type DIEP2_TXFSTS = crate::Reg<diep2_txfsts::DIEP2_TXFSTS_SPEC>;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep2_txfsts;
#[doc = "DOEP0CTL (rw) register accessor: Device OUT Endpoint 0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0ctl`]
module"]
pub type DOEP0CTL = crate::Reg<doep0ctl::DOEP0CTL_SPEC>;
#[doc = "Device OUT Endpoint 0 Control Register"]
pub mod doep0ctl;
#[doc = "DOEP0INT (rw) register accessor: Device OUT Endpoint 0 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0int`]
module"]
pub type DOEP0INT = crate::Reg<doep0int::DOEP0INT_SPEC>;
#[doc = "Device OUT Endpoint 0 Interrupt Register"]
pub mod doep0int;
#[doc = "DOEP0TSIZ (rw) register accessor: Device OUT Endpoint 0 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0tsiz`]
module"]
pub type DOEP0TSIZ = crate::Reg<doep0tsiz::DOEP0TSIZ_SPEC>;
#[doc = "Device OUT Endpoint 0 Transfer Size Register"]
pub mod doep0tsiz;
#[doc = "DOEP0DMAADDR (rw) register accessor: Device OUT Endpoint 0 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0dmaaddr`]
module"]
pub type DOEP0DMAADDR = crate::Reg<doep0dmaaddr::DOEP0DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint 0 DMA Address Register"]
pub mod doep0dmaaddr;
#[doc = "DOEP0_CTL (rw) register accessor: Device OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0_ctl`]
module"]
pub type DOEP0_CTL = crate::Reg<doep0_ctl::DOEP0_CTL_SPEC>;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep0_ctl;
#[doc = "DOEP0_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0_int`]
module"]
pub type DOEP0_INT = crate::Reg<doep0_int::DOEP0_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep0_int;
#[doc = "DOEP0_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0_tsiz`]
module"]
pub type DOEP0_TSIZ = crate::Reg<doep0_tsiz::DOEP0_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep0_tsiz;
#[doc = "DOEP0_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0_dmaaddr`]
module"]
pub type DOEP0_DMAADDR = crate::Reg<doep0_dmaaddr::DOEP0_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep0_dmaaddr;
#[doc = "DOEP1_CTL (rw) register accessor: Device OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1_ctl`]
module"]
pub type DOEP1_CTL = crate::Reg<doep1_ctl::DOEP1_CTL_SPEC>;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep1_ctl;
#[doc = "DOEP1_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1_int`]
module"]
pub type DOEP1_INT = crate::Reg<doep1_int::DOEP1_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep1_int;
#[doc = "DOEP1_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1_tsiz`]
module"]
pub type DOEP1_TSIZ = crate::Reg<doep1_tsiz::DOEP1_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep1_tsiz;
#[doc = "DOEP1_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1_dmaaddr`]
module"]
pub type DOEP1_DMAADDR = crate::Reg<doep1_dmaaddr::DOEP1_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep1_dmaaddr;
#[doc = "DOEP2_CTL (rw) register accessor: Device OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2_ctl`]
module"]
pub type DOEP2_CTL = crate::Reg<doep2_ctl::DOEP2_CTL_SPEC>;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep2_ctl;
#[doc = "DOEP2_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2_int`]
module"]
pub type DOEP2_INT = crate::Reg<doep2_int::DOEP2_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep2_int;
#[doc = "DOEP2_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2_tsiz`]
module"]
pub type DOEP2_TSIZ = crate::Reg<doep2_tsiz::DOEP2_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep2_tsiz;
#[doc = "DOEP2_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2_dmaaddr`]
module"]
pub type DOEP2_DMAADDR = crate::Reg<doep2_dmaaddr::DOEP2_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep2_dmaaddr;
#[doc = "PCGCCTL (rw) register accessor: Power and Clock Gating Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcgcctl`]
module"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;
