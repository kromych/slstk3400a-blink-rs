#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pa_ctrl: PaCtrl,
    pa_model: PaModel,
    pa_modeh: PaModeh,
    pa_dout: PaDout,
    pa_doutset: PaDoutset,
    pa_doutclr: PaDoutclr,
    pa_douttgl: PaDouttgl,
    pa_din: PaDin,
    pa_pinlockn: PaPinlockn,
    pb_ctrl: PbCtrl,
    pb_model: PbModel,
    pb_modeh: PbModeh,
    pb_dout: PbDout,
    pb_doutset: PbDoutset,
    pb_doutclr: PbDoutclr,
    pb_douttgl: PbDouttgl,
    pb_din: PbDin,
    pb_pinlockn: PbPinlockn,
    pc_ctrl: PcCtrl,
    pc_model: PcModel,
    pc_modeh: PcModeh,
    pc_dout: PcDout,
    pc_doutset: PcDoutset,
    pc_doutclr: PcDoutclr,
    pc_douttgl: PcDouttgl,
    pc_din: PcDin,
    pc_pinlockn: PcPinlockn,
    pd_ctrl: PdCtrl,
    pd_model: PdModel,
    pd_modeh: PdModeh,
    pd_dout: PdDout,
    pd_doutset: PdDoutset,
    pd_doutclr: PdDoutclr,
    pd_douttgl: PdDouttgl,
    pd_din: PdDin,
    pd_pinlockn: PdPinlockn,
    pe_ctrl: PeCtrl,
    pe_model: PeModel,
    pe_modeh: PeModeh,
    pe_dout: PeDout,
    pe_doutset: PeDoutset,
    pe_doutclr: PeDoutclr,
    pe_douttgl: PeDouttgl,
    pe_din: PeDin,
    pe_pinlockn: PePinlockn,
    pf_ctrl: PfCtrl,
    pf_model: PfModel,
    pf_modeh: PfModeh,
    pf_dout: PfDout,
    pf_doutset: PfDoutset,
    pf_doutclr: PfDoutclr,
    pf_douttgl: PfDouttgl,
    pf_din: PfDin,
    pf_pinlockn: PfPinlockn,
    _reserved54: [u8; 0x28],
    extipsell: Extipsell,
    extipselh: Extipselh,
    extirise: Extirise,
    extifall: Extifall,
    ien: Ien,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    route: Route,
    insense: Insense,
    lock: Lock,
    ctrl: Ctrl,
    cmd: Cmd,
    em4wuen: Em4wuen,
    em4wupol: Em4wupol,
    em4wucause: Em4wucause,
}
impl RegisterBlock {
    #[doc = "0x00 - Port Control Register"]
    #[inline(always)]
    pub const fn pa_ctrl(&self) -> &PaCtrl {
        &self.pa_ctrl
    }
    #[doc = "0x04 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pa_model(&self) -> &PaModel {
        &self.pa_model
    }
    #[doc = "0x08 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pa_modeh(&self) -> &PaModeh {
        &self.pa_modeh
    }
    #[doc = "0x0c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pa_dout(&self) -> &PaDout {
        &self.pa_dout
    }
    #[doc = "0x10 - Port Data Out Set Register"]
    #[inline(always)]
    pub const fn pa_doutset(&self) -> &PaDoutset {
        &self.pa_doutset
    }
    #[doc = "0x14 - Port Data Out Clear Register"]
    #[inline(always)]
    pub const fn pa_doutclr(&self) -> &PaDoutclr {
        &self.pa_doutclr
    }
    #[doc = "0x18 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pa_douttgl(&self) -> &PaDouttgl {
        &self.pa_douttgl
    }
    #[doc = "0x1c - Port Data In Register"]
    #[inline(always)]
    pub const fn pa_din(&self) -> &PaDin {
        &self.pa_din
    }
    #[doc = "0x20 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pa_pinlockn(&self) -> &PaPinlockn {
        &self.pa_pinlockn
    }
    #[doc = "0x24 - Port Control Register"]
    #[inline(always)]
    pub const fn pb_ctrl(&self) -> &PbCtrl {
        &self.pb_ctrl
    }
    #[doc = "0x28 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pb_model(&self) -> &PbModel {
        &self.pb_model
    }
    #[doc = "0x2c - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pb_modeh(&self) -> &PbModeh {
        &self.pb_modeh
    }
    #[doc = "0x30 - Port Data Out Register"]
    #[inline(always)]
    pub const fn pb_dout(&self) -> &PbDout {
        &self.pb_dout
    }
    #[doc = "0x34 - Port Data Out Set Register"]
    #[inline(always)]
    pub const fn pb_doutset(&self) -> &PbDoutset {
        &self.pb_doutset
    }
    #[doc = "0x38 - Port Data Out Clear Register"]
    #[inline(always)]
    pub const fn pb_doutclr(&self) -> &PbDoutclr {
        &self.pb_doutclr
    }
    #[doc = "0x3c - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pb_douttgl(&self) -> &PbDouttgl {
        &self.pb_douttgl
    }
    #[doc = "0x40 - Port Data In Register"]
    #[inline(always)]
    pub const fn pb_din(&self) -> &PbDin {
        &self.pb_din
    }
    #[doc = "0x44 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pb_pinlockn(&self) -> &PbPinlockn {
        &self.pb_pinlockn
    }
    #[doc = "0x48 - Port Control Register"]
    #[inline(always)]
    pub const fn pc_ctrl(&self) -> &PcCtrl {
        &self.pc_ctrl
    }
    #[doc = "0x4c - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pc_model(&self) -> &PcModel {
        &self.pc_model
    }
    #[doc = "0x50 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pc_modeh(&self) -> &PcModeh {
        &self.pc_modeh
    }
    #[doc = "0x54 - Port Data Out Register"]
    #[inline(always)]
    pub const fn pc_dout(&self) -> &PcDout {
        &self.pc_dout
    }
    #[doc = "0x58 - Port Data Out Set Register"]
    #[inline(always)]
    pub const fn pc_doutset(&self) -> &PcDoutset {
        &self.pc_doutset
    }
    #[doc = "0x5c - Port Data Out Clear Register"]
    #[inline(always)]
    pub const fn pc_doutclr(&self) -> &PcDoutclr {
        &self.pc_doutclr
    }
    #[doc = "0x60 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pc_douttgl(&self) -> &PcDouttgl {
        &self.pc_douttgl
    }
    #[doc = "0x64 - Port Data In Register"]
    #[inline(always)]
    pub const fn pc_din(&self) -> &PcDin {
        &self.pc_din
    }
    #[doc = "0x68 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pc_pinlockn(&self) -> &PcPinlockn {
        &self.pc_pinlockn
    }
    #[doc = "0x6c - Port Control Register"]
    #[inline(always)]
    pub const fn pd_ctrl(&self) -> &PdCtrl {
        &self.pd_ctrl
    }
    #[doc = "0x70 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pd_model(&self) -> &PdModel {
        &self.pd_model
    }
    #[doc = "0x74 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pd_modeh(&self) -> &PdModeh {
        &self.pd_modeh
    }
    #[doc = "0x78 - Port Data Out Register"]
    #[inline(always)]
    pub const fn pd_dout(&self) -> &PdDout {
        &self.pd_dout
    }
    #[doc = "0x7c - Port Data Out Set Register"]
    #[inline(always)]
    pub const fn pd_doutset(&self) -> &PdDoutset {
        &self.pd_doutset
    }
    #[doc = "0x80 - Port Data Out Clear Register"]
    #[inline(always)]
    pub const fn pd_doutclr(&self) -> &PdDoutclr {
        &self.pd_doutclr
    }
    #[doc = "0x84 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pd_douttgl(&self) -> &PdDouttgl {
        &self.pd_douttgl
    }
    #[doc = "0x88 - Port Data In Register"]
    #[inline(always)]
    pub const fn pd_din(&self) -> &PdDin {
        &self.pd_din
    }
    #[doc = "0x8c - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pd_pinlockn(&self) -> &PdPinlockn {
        &self.pd_pinlockn
    }
    #[doc = "0x90 - Port Control Register"]
    #[inline(always)]
    pub const fn pe_ctrl(&self) -> &PeCtrl {
        &self.pe_ctrl
    }
    #[doc = "0x94 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pe_model(&self) -> &PeModel {
        &self.pe_model
    }
    #[doc = "0x98 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pe_modeh(&self) -> &PeModeh {
        &self.pe_modeh
    }
    #[doc = "0x9c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pe_dout(&self) -> &PeDout {
        &self.pe_dout
    }
    #[doc = "0xa0 - Port Data Out Set Register"]
    #[inline(always)]
    pub const fn pe_doutset(&self) -> &PeDoutset {
        &self.pe_doutset
    }
    #[doc = "0xa4 - Port Data Out Clear Register"]
    #[inline(always)]
    pub const fn pe_doutclr(&self) -> &PeDoutclr {
        &self.pe_doutclr
    }
    #[doc = "0xa8 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pe_douttgl(&self) -> &PeDouttgl {
        &self.pe_douttgl
    }
    #[doc = "0xac - Port Data In Register"]
    #[inline(always)]
    pub const fn pe_din(&self) -> &PeDin {
        &self.pe_din
    }
    #[doc = "0xb0 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pe_pinlockn(&self) -> &PePinlockn {
        &self.pe_pinlockn
    }
    #[doc = "0xb4 - Port Control Register"]
    #[inline(always)]
    pub const fn pf_ctrl(&self) -> &PfCtrl {
        &self.pf_ctrl
    }
    #[doc = "0xb8 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pf_model(&self) -> &PfModel {
        &self.pf_model
    }
    #[doc = "0xbc - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pf_modeh(&self) -> &PfModeh {
        &self.pf_modeh
    }
    #[doc = "0xc0 - Port Data Out Register"]
    #[inline(always)]
    pub const fn pf_dout(&self) -> &PfDout {
        &self.pf_dout
    }
    #[doc = "0xc4 - Port Data Out Set Register"]
    #[inline(always)]
    pub const fn pf_doutset(&self) -> &PfDoutset {
        &self.pf_doutset
    }
    #[doc = "0xc8 - Port Data Out Clear Register"]
    #[inline(always)]
    pub const fn pf_doutclr(&self) -> &PfDoutclr {
        &self.pf_doutclr
    }
    #[doc = "0xcc - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pf_douttgl(&self) -> &PfDouttgl {
        &self.pf_douttgl
    }
    #[doc = "0xd0 - Port Data In Register"]
    #[inline(always)]
    pub const fn pf_din(&self) -> &PfDin {
        &self.pf_din
    }
    #[doc = "0xd4 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pf_pinlockn(&self) -> &PfPinlockn {
        &self.pf_pinlockn
    }
    #[doc = "0x100 - External Interrupt Port Select Low Register"]
    #[inline(always)]
    pub const fn extipsell(&self) -> &Extipsell {
        &self.extipsell
    }
    #[doc = "0x104 - External Interrupt Port Select High Register"]
    #[inline(always)]
    pub const fn extipselh(&self) -> &Extipselh {
        &self.extipselh
    }
    #[doc = "0x108 - External Interrupt Rising Edge Trigger Register"]
    #[inline(always)]
    pub const fn extirise(&self) -> &Extirise {
        &self.extirise
    }
    #[doc = "0x10c - External Interrupt Falling Edge Trigger Register"]
    #[inline(always)]
    pub const fn extifall(&self) -> &Extifall {
        &self.extifall
    }
    #[doc = "0x110 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x114 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x118 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x11c - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x120 - I/O Routing Register"]
    #[inline(always)]
    pub const fn route(&self) -> &Route {
        &self.route
    }
    #[doc = "0x124 - Input Sense Register"]
    #[inline(always)]
    pub const fn insense(&self) -> &Insense {
        &self.insense
    }
    #[doc = "0x128 - Configuration Lock Register"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x12c - GPIO Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x130 - GPIO Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x134 - EM4 Wake-up Enable Register"]
    #[inline(always)]
    pub const fn em4wuen(&self) -> &Em4wuen {
        &self.em4wuen
    }
    #[doc = "0x138 - EM4 Wake-up Polarity Register"]
    #[inline(always)]
    pub const fn em4wupol(&self) -> &Em4wupol {
        &self.em4wupol
    }
    #[doc = "0x13c - EM4 Wake-up Cause Register"]
    #[inline(always)]
    pub const fn em4wucause(&self) -> &Em4wucause {
        &self.em4wucause
    }
}
#[doc = "PA_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_ctrl`] module"]
#[doc(alias = "PA_CTRL")]
pub type PaCtrl = crate::Reg<pa_ctrl::PaCtrlSpec>;
#[doc = "Port Control Register"]
pub mod pa_ctrl;
#[doc = "PA_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_model`] module"]
#[doc(alias = "PA_MODEL")]
pub type PaModel = crate::Reg<pa_model::PaModelSpec>;
#[doc = "Port Pin Mode Low Register"]
pub mod pa_model;
#[doc = "PA_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_modeh`] module"]
#[doc(alias = "PA_MODEH")]
pub type PaModeh = crate::Reg<pa_modeh::PaModehSpec>;
#[doc = "Port Pin Mode High Register"]
pub mod pa_modeh;
#[doc = "PA_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_dout`] module"]
#[doc(alias = "PA_DOUT")]
pub type PaDout = crate::Reg<pa_dout::PaDoutSpec>;
#[doc = "Port Data Out Register"]
pub mod pa_dout;
#[doc = "PA_DOUTSET (w) register accessor: Port Data Out Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_doutset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_doutset`] module"]
#[doc(alias = "PA_DOUTSET")]
pub type PaDoutset = crate::Reg<pa_doutset::PaDoutsetSpec>;
#[doc = "Port Data Out Set Register"]
pub mod pa_doutset;
#[doc = "PA_DOUTCLR (w) register accessor: Port Data Out Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_doutclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_doutclr`] module"]
#[doc(alias = "PA_DOUTCLR")]
pub type PaDoutclr = crate::Reg<pa_doutclr::PaDoutclrSpec>;
#[doc = "Port Data Out Clear Register"]
pub mod pa_doutclr;
#[doc = "PA_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_douttgl`] module"]
#[doc(alias = "PA_DOUTTGL")]
pub type PaDouttgl = crate::Reg<pa_douttgl::PaDouttglSpec>;
#[doc = "Port Data Out Toggle Register"]
pub mod pa_douttgl;
#[doc = "PA_DIN (r) register accessor: Port Data In Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_din`] module"]
#[doc(alias = "PA_DIN")]
pub type PaDin = crate::Reg<pa_din::PaDinSpec>;
#[doc = "Port Data In Register"]
pub mod pa_din;
#[doc = "PA_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_pinlockn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_pinlockn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_pinlockn`] module"]
#[doc(alias = "PA_PINLOCKN")]
pub type PaPinlockn = crate::Reg<pa_pinlockn::PaPinlocknSpec>;
#[doc = "Port Unlocked Pins Register"]
pub mod pa_pinlockn;
#[doc = "PB_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_ctrl`] module"]
#[doc(alias = "PB_CTRL")]
pub type PbCtrl = crate::Reg<pb_ctrl::PbCtrlSpec>;
#[doc = "Port Control Register"]
pub mod pb_ctrl;
#[doc = "PB_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_model`] module"]
#[doc(alias = "PB_MODEL")]
pub type PbModel = crate::Reg<pb_model::PbModelSpec>;
#[doc = "Port Pin Mode Low Register"]
pub mod pb_model;
#[doc = "PB_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_modeh`] module"]
#[doc(alias = "PB_MODEH")]
pub type PbModeh = crate::Reg<pb_modeh::PbModehSpec>;
#[doc = "Port Pin Mode High Register"]
pub mod pb_modeh;
#[doc = "PB_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_dout`] module"]
#[doc(alias = "PB_DOUT")]
pub type PbDout = crate::Reg<pb_dout::PbDoutSpec>;
#[doc = "Port Data Out Register"]
pub mod pb_dout;
#[doc = "PB_DOUTSET (w) register accessor: Port Data Out Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_doutset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_doutset`] module"]
#[doc(alias = "PB_DOUTSET")]
pub type PbDoutset = crate::Reg<pb_doutset::PbDoutsetSpec>;
#[doc = "Port Data Out Set Register"]
pub mod pb_doutset;
#[doc = "PB_DOUTCLR (w) register accessor: Port Data Out Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_doutclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_doutclr`] module"]
#[doc(alias = "PB_DOUTCLR")]
pub type PbDoutclr = crate::Reg<pb_doutclr::PbDoutclrSpec>;
#[doc = "Port Data Out Clear Register"]
pub mod pb_doutclr;
#[doc = "PB_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_douttgl`] module"]
#[doc(alias = "PB_DOUTTGL")]
pub type PbDouttgl = crate::Reg<pb_douttgl::PbDouttglSpec>;
#[doc = "Port Data Out Toggle Register"]
pub mod pb_douttgl;
#[doc = "PB_DIN (r) register accessor: Port Data In Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_din`] module"]
#[doc(alias = "PB_DIN")]
pub type PbDin = crate::Reg<pb_din::PbDinSpec>;
#[doc = "Port Data In Register"]
pub mod pb_din;
#[doc = "PB_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_pinlockn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_pinlockn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_pinlockn`] module"]
#[doc(alias = "PB_PINLOCKN")]
pub type PbPinlockn = crate::Reg<pb_pinlockn::PbPinlocknSpec>;
#[doc = "Port Unlocked Pins Register"]
pub mod pb_pinlockn;
#[doc = "PC_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_ctrl`] module"]
#[doc(alias = "PC_CTRL")]
pub type PcCtrl = crate::Reg<pc_ctrl::PcCtrlSpec>;
#[doc = "Port Control Register"]
pub mod pc_ctrl;
#[doc = "PC_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_model`] module"]
#[doc(alias = "PC_MODEL")]
pub type PcModel = crate::Reg<pc_model::PcModelSpec>;
#[doc = "Port Pin Mode Low Register"]
pub mod pc_model;
#[doc = "PC_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_modeh`] module"]
#[doc(alias = "PC_MODEH")]
pub type PcModeh = crate::Reg<pc_modeh::PcModehSpec>;
#[doc = "Port Pin Mode High Register"]
pub mod pc_modeh;
#[doc = "PC_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_dout`] module"]
#[doc(alias = "PC_DOUT")]
pub type PcDout = crate::Reg<pc_dout::PcDoutSpec>;
#[doc = "Port Data Out Register"]
pub mod pc_dout;
#[doc = "PC_DOUTSET (w) register accessor: Port Data Out Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_doutset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_doutset`] module"]
#[doc(alias = "PC_DOUTSET")]
pub type PcDoutset = crate::Reg<pc_doutset::PcDoutsetSpec>;
#[doc = "Port Data Out Set Register"]
pub mod pc_doutset;
#[doc = "PC_DOUTCLR (w) register accessor: Port Data Out Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_doutclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_doutclr`] module"]
#[doc(alias = "PC_DOUTCLR")]
pub type PcDoutclr = crate::Reg<pc_doutclr::PcDoutclrSpec>;
#[doc = "Port Data Out Clear Register"]
pub mod pc_doutclr;
#[doc = "PC_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_douttgl`] module"]
#[doc(alias = "PC_DOUTTGL")]
pub type PcDouttgl = crate::Reg<pc_douttgl::PcDouttglSpec>;
#[doc = "Port Data Out Toggle Register"]
pub mod pc_douttgl;
#[doc = "PC_DIN (r) register accessor: Port Data In Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_din`] module"]
#[doc(alias = "PC_DIN")]
pub type PcDin = crate::Reg<pc_din::PcDinSpec>;
#[doc = "Port Data In Register"]
pub mod pc_din;
#[doc = "PC_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_pinlockn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_pinlockn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_pinlockn`] module"]
#[doc(alias = "PC_PINLOCKN")]
pub type PcPinlockn = crate::Reg<pc_pinlockn::PcPinlocknSpec>;
#[doc = "Port Unlocked Pins Register"]
pub mod pc_pinlockn;
#[doc = "PD_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_ctrl`] module"]
#[doc(alias = "PD_CTRL")]
pub type PdCtrl = crate::Reg<pd_ctrl::PdCtrlSpec>;
#[doc = "Port Control Register"]
pub mod pd_ctrl;
#[doc = "PD_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_model`] module"]
#[doc(alias = "PD_MODEL")]
pub type PdModel = crate::Reg<pd_model::PdModelSpec>;
#[doc = "Port Pin Mode Low Register"]
pub mod pd_model;
#[doc = "PD_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_modeh`] module"]
#[doc(alias = "PD_MODEH")]
pub type PdModeh = crate::Reg<pd_modeh::PdModehSpec>;
#[doc = "Port Pin Mode High Register"]
pub mod pd_modeh;
#[doc = "PD_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_dout`] module"]
#[doc(alias = "PD_DOUT")]
pub type PdDout = crate::Reg<pd_dout::PdDoutSpec>;
#[doc = "Port Data Out Register"]
pub mod pd_dout;
#[doc = "PD_DOUTSET (w) register accessor: Port Data Out Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_doutset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_doutset`] module"]
#[doc(alias = "PD_DOUTSET")]
pub type PdDoutset = crate::Reg<pd_doutset::PdDoutsetSpec>;
#[doc = "Port Data Out Set Register"]
pub mod pd_doutset;
#[doc = "PD_DOUTCLR (w) register accessor: Port Data Out Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_doutclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_doutclr`] module"]
#[doc(alias = "PD_DOUTCLR")]
pub type PdDoutclr = crate::Reg<pd_doutclr::PdDoutclrSpec>;
#[doc = "Port Data Out Clear Register"]
pub mod pd_doutclr;
#[doc = "PD_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_douttgl`] module"]
#[doc(alias = "PD_DOUTTGL")]
pub type PdDouttgl = crate::Reg<pd_douttgl::PdDouttglSpec>;
#[doc = "Port Data Out Toggle Register"]
pub mod pd_douttgl;
#[doc = "PD_DIN (r) register accessor: Port Data In Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_din`] module"]
#[doc(alias = "PD_DIN")]
pub type PdDin = crate::Reg<pd_din::PdDinSpec>;
#[doc = "Port Data In Register"]
pub mod pd_din;
#[doc = "PD_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_pinlockn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_pinlockn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_pinlockn`] module"]
#[doc(alias = "PD_PINLOCKN")]
pub type PdPinlockn = crate::Reg<pd_pinlockn::PdPinlocknSpec>;
#[doc = "Port Unlocked Pins Register"]
pub mod pd_pinlockn;
#[doc = "PE_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_ctrl`] module"]
#[doc(alias = "PE_CTRL")]
pub type PeCtrl = crate::Reg<pe_ctrl::PeCtrlSpec>;
#[doc = "Port Control Register"]
pub mod pe_ctrl;
#[doc = "PE_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_model`] module"]
#[doc(alias = "PE_MODEL")]
pub type PeModel = crate::Reg<pe_model::PeModelSpec>;
#[doc = "Port Pin Mode Low Register"]
pub mod pe_model;
#[doc = "PE_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_modeh`] module"]
#[doc(alias = "PE_MODEH")]
pub type PeModeh = crate::Reg<pe_modeh::PeModehSpec>;
#[doc = "Port Pin Mode High Register"]
pub mod pe_modeh;
#[doc = "PE_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_dout`] module"]
#[doc(alias = "PE_DOUT")]
pub type PeDout = crate::Reg<pe_dout::PeDoutSpec>;
#[doc = "Port Data Out Register"]
pub mod pe_dout;
#[doc = "PE_DOUTSET (w) register accessor: Port Data Out Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_doutset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_doutset`] module"]
#[doc(alias = "PE_DOUTSET")]
pub type PeDoutset = crate::Reg<pe_doutset::PeDoutsetSpec>;
#[doc = "Port Data Out Set Register"]
pub mod pe_doutset;
#[doc = "PE_DOUTCLR (w) register accessor: Port Data Out Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_doutclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_doutclr`] module"]
#[doc(alias = "PE_DOUTCLR")]
pub type PeDoutclr = crate::Reg<pe_doutclr::PeDoutclrSpec>;
#[doc = "Port Data Out Clear Register"]
pub mod pe_doutclr;
#[doc = "PE_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_douttgl`] module"]
#[doc(alias = "PE_DOUTTGL")]
pub type PeDouttgl = crate::Reg<pe_douttgl::PeDouttglSpec>;
#[doc = "Port Data Out Toggle Register"]
pub mod pe_douttgl;
#[doc = "PE_DIN (r) register accessor: Port Data In Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_din`] module"]
#[doc(alias = "PE_DIN")]
pub type PeDin = crate::Reg<pe_din::PeDinSpec>;
#[doc = "Port Data In Register"]
pub mod pe_din;
#[doc = "PE_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_pinlockn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_pinlockn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_pinlockn`] module"]
#[doc(alias = "PE_PINLOCKN")]
pub type PePinlockn = crate::Reg<pe_pinlockn::PePinlocknSpec>;
#[doc = "Port Unlocked Pins Register"]
pub mod pe_pinlockn;
#[doc = "PF_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_ctrl`] module"]
#[doc(alias = "PF_CTRL")]
pub type PfCtrl = crate::Reg<pf_ctrl::PfCtrlSpec>;
#[doc = "Port Control Register"]
pub mod pf_ctrl;
#[doc = "PF_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_model`] module"]
#[doc(alias = "PF_MODEL")]
pub type PfModel = crate::Reg<pf_model::PfModelSpec>;
#[doc = "Port Pin Mode Low Register"]
pub mod pf_model;
#[doc = "PF_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_modeh`] module"]
#[doc(alias = "PF_MODEH")]
pub type PfModeh = crate::Reg<pf_modeh::PfModehSpec>;
#[doc = "Port Pin Mode High Register"]
pub mod pf_modeh;
#[doc = "PF_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_dout`] module"]
#[doc(alias = "PF_DOUT")]
pub type PfDout = crate::Reg<pf_dout::PfDoutSpec>;
#[doc = "Port Data Out Register"]
pub mod pf_dout;
#[doc = "PF_DOUTSET (w) register accessor: Port Data Out Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_doutset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_doutset`] module"]
#[doc(alias = "PF_DOUTSET")]
pub type PfDoutset = crate::Reg<pf_doutset::PfDoutsetSpec>;
#[doc = "Port Data Out Set Register"]
pub mod pf_doutset;
#[doc = "PF_DOUTCLR (w) register accessor: Port Data Out Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_doutclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_doutclr`] module"]
#[doc(alias = "PF_DOUTCLR")]
pub type PfDoutclr = crate::Reg<pf_doutclr::PfDoutclrSpec>;
#[doc = "Port Data Out Clear Register"]
pub mod pf_doutclr;
#[doc = "PF_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_douttgl`] module"]
#[doc(alias = "PF_DOUTTGL")]
pub type PfDouttgl = crate::Reg<pf_douttgl::PfDouttglSpec>;
#[doc = "Port Data Out Toggle Register"]
pub mod pf_douttgl;
#[doc = "PF_DIN (r) register accessor: Port Data In Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_din`] module"]
#[doc(alias = "PF_DIN")]
pub type PfDin = crate::Reg<pf_din::PfDinSpec>;
#[doc = "Port Data In Register"]
pub mod pf_din;
#[doc = "PF_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_pinlockn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_pinlockn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_pinlockn`] module"]
#[doc(alias = "PF_PINLOCKN")]
pub type PfPinlockn = crate::Reg<pf_pinlockn::PfPinlocknSpec>;
#[doc = "Port Unlocked Pins Register"]
pub mod pf_pinlockn;
#[doc = "EXTIPSELL (rw) register accessor: External Interrupt Port Select Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extipsell::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipsell::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipsell`] module"]
#[doc(alias = "EXTIPSELL")]
pub type Extipsell = crate::Reg<extipsell::ExtipsellSpec>;
#[doc = "External Interrupt Port Select Low Register"]
pub mod extipsell;
#[doc = "EXTIPSELH (rw) register accessor: External Interrupt Port Select High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extipselh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipselh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipselh`] module"]
#[doc(alias = "EXTIPSELH")]
pub type Extipselh = crate::Reg<extipselh::ExtipselhSpec>;
#[doc = "External Interrupt Port Select High Register"]
pub mod extipselh;
#[doc = "EXTIRISE (rw) register accessor: External Interrupt Rising Edge Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extirise::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extirise::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extirise`] module"]
#[doc(alias = "EXTIRISE")]
pub type Extirise = crate::Reg<extirise::ExtiriseSpec>;
#[doc = "External Interrupt Rising Edge Trigger Register"]
pub mod extirise;
#[doc = "EXTIFALL (rw) register accessor: External Interrupt Falling Edge Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extifall::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extifall::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extifall`] module"]
#[doc(alias = "EXTIFALL")]
pub type Extifall = crate::Reg<extifall::ExtifallSpec>;
#[doc = "External Interrupt Falling Edge Trigger Register"]
pub mod extifall;
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
#[doc = "ROUTE (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@route`] module"]
#[doc(alias = "ROUTE")]
pub type Route = crate::Reg<route::RouteSpec>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "INSENSE (rw) register accessor: Input Sense Register\n\nYou can [`read`](crate::Reg::read) this register and get [`insense::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`insense::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@insense`] module"]
#[doc(alias = "INSENSE")]
pub type Insense = crate::Reg<insense::InsenseSpec>;
#[doc = "Input Sense Register"]
pub mod insense;
#[doc = "LOCK (rw) register accessor: Configuration Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "CTRL (rw) register accessor: GPIO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "GPIO Control Register"]
pub mod ctrl;
#[doc = "CMD (w) register accessor: GPIO Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "GPIO Command Register"]
pub mod cmd;
#[doc = "EM4WUEN (rw) register accessor: EM4 Wake-up Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`em4wuen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4wuen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4wuen`] module"]
#[doc(alias = "EM4WUEN")]
pub type Em4wuen = crate::Reg<em4wuen::Em4wuenSpec>;
#[doc = "EM4 Wake-up Enable Register"]
pub mod em4wuen;
#[doc = "EM4WUPOL (rw) register accessor: EM4 Wake-up Polarity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`em4wupol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4wupol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4wupol`] module"]
#[doc(alias = "EM4WUPOL")]
pub type Em4wupol = crate::Reg<em4wupol::Em4wupolSpec>;
#[doc = "EM4 Wake-up Polarity Register"]
pub mod em4wupol;
#[doc = "EM4WUCAUSE (r) register accessor: EM4 Wake-up Cause Register\n\nYou can [`read`](crate::Reg::read) this register and get [`em4wucause::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4wucause`] module"]
#[doc(alias = "EM4WUCAUSE")]
pub type Em4wucause = crate::Reg<em4wucause::Em4wucauseSpec>;
#[doc = "EM4 Wake-up Cause Register"]
pub mod em4wucause;
