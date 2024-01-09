#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pa_ctrl: PA_CTRL,
    pa_model: PA_MODEL,
    pa_modeh: PA_MODEH,
    pa_dout: PA_DOUT,
    pa_doutset: PA_DOUTSET,
    pa_doutclr: PA_DOUTCLR,
    pa_douttgl: PA_DOUTTGL,
    pa_din: PA_DIN,
    pa_pinlockn: PA_PINLOCKN,
    pb_ctrl: PB_CTRL,
    pb_model: PB_MODEL,
    pb_modeh: PB_MODEH,
    pb_dout: PB_DOUT,
    pb_doutset: PB_DOUTSET,
    pb_doutclr: PB_DOUTCLR,
    pb_douttgl: PB_DOUTTGL,
    pb_din: PB_DIN,
    pb_pinlockn: PB_PINLOCKN,
    pc_ctrl: PC_CTRL,
    pc_model: PC_MODEL,
    pc_modeh: PC_MODEH,
    pc_dout: PC_DOUT,
    pc_doutset: PC_DOUTSET,
    pc_doutclr: PC_DOUTCLR,
    pc_douttgl: PC_DOUTTGL,
    pc_din: PC_DIN,
    pc_pinlockn: PC_PINLOCKN,
    pd_ctrl: PD_CTRL,
    pd_model: PD_MODEL,
    pd_modeh: PD_MODEH,
    pd_dout: PD_DOUT,
    pd_doutset: PD_DOUTSET,
    pd_doutclr: PD_DOUTCLR,
    pd_douttgl: PD_DOUTTGL,
    pd_din: PD_DIN,
    pd_pinlockn: PD_PINLOCKN,
    pe_ctrl: PE_CTRL,
    pe_model: PE_MODEL,
    pe_modeh: PE_MODEH,
    pe_dout: PE_DOUT,
    pe_doutset: PE_DOUTSET,
    pe_doutclr: PE_DOUTCLR,
    pe_douttgl: PE_DOUTTGL,
    pe_din: PE_DIN,
    pe_pinlockn: PE_PINLOCKN,
    pf_ctrl: PF_CTRL,
    pf_model: PF_MODEL,
    pf_modeh: PF_MODEH,
    pf_dout: PF_DOUT,
    pf_doutset: PF_DOUTSET,
    pf_doutclr: PF_DOUTCLR,
    pf_douttgl: PF_DOUTTGL,
    pf_din: PF_DIN,
    pf_pinlockn: PF_PINLOCKN,
    _reserved54: [u8; 0x28],
    extipsell: EXTIPSELL,
    extipselh: EXTIPSELH,
    extirise: EXTIRISE,
    extifall: EXTIFALL,
    ien: IEN,
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    route: ROUTE,
    insense: INSENSE,
    lock: LOCK,
    ctrl: CTRL,
    cmd: CMD,
    em4wuen: EM4WUEN,
    em4wupol: EM4WUPOL,
    em4wucause: EM4WUCAUSE,
}
impl RegisterBlock {
    #[doc = "0x00 - Port Control Register"]
    #[inline(always)]
    pub const fn pa_ctrl(&self) -> &PA_CTRL {
        &self.pa_ctrl
    }
    #[doc = "0x04 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pa_model(&self) -> &PA_MODEL {
        &self.pa_model
    }
    #[doc = "0x08 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pa_modeh(&self) -> &PA_MODEH {
        &self.pa_modeh
    }
    #[doc = "0x0c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pa_dout(&self) -> &PA_DOUT {
        &self.pa_dout
    }
    #[doc = "0x10 - Port Data Out Set Register"]
    #[inline(always)]
    pub const fn pa_doutset(&self) -> &PA_DOUTSET {
        &self.pa_doutset
    }
    #[doc = "0x14 - Port Data Out Clear Register"]
    #[inline(always)]
    pub const fn pa_doutclr(&self) -> &PA_DOUTCLR {
        &self.pa_doutclr
    }
    #[doc = "0x18 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pa_douttgl(&self) -> &PA_DOUTTGL {
        &self.pa_douttgl
    }
    #[doc = "0x1c - Port Data In Register"]
    #[inline(always)]
    pub const fn pa_din(&self) -> &PA_DIN {
        &self.pa_din
    }
    #[doc = "0x20 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pa_pinlockn(&self) -> &PA_PINLOCKN {
        &self.pa_pinlockn
    }
    #[doc = "0x24 - Port Control Register"]
    #[inline(always)]
    pub const fn pb_ctrl(&self) -> &PB_CTRL {
        &self.pb_ctrl
    }
    #[doc = "0x28 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pb_model(&self) -> &PB_MODEL {
        &self.pb_model
    }
    #[doc = "0x2c - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pb_modeh(&self) -> &PB_MODEH {
        &self.pb_modeh
    }
    #[doc = "0x30 - Port Data Out Register"]
    #[inline(always)]
    pub const fn pb_dout(&self) -> &PB_DOUT {
        &self.pb_dout
    }
    #[doc = "0x34 - Port Data Out Set Register"]
    #[inline(always)]
    pub const fn pb_doutset(&self) -> &PB_DOUTSET {
        &self.pb_doutset
    }
    #[doc = "0x38 - Port Data Out Clear Register"]
    #[inline(always)]
    pub const fn pb_doutclr(&self) -> &PB_DOUTCLR {
        &self.pb_doutclr
    }
    #[doc = "0x3c - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pb_douttgl(&self) -> &PB_DOUTTGL {
        &self.pb_douttgl
    }
    #[doc = "0x40 - Port Data In Register"]
    #[inline(always)]
    pub const fn pb_din(&self) -> &PB_DIN {
        &self.pb_din
    }
    #[doc = "0x44 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pb_pinlockn(&self) -> &PB_PINLOCKN {
        &self.pb_pinlockn
    }
    #[doc = "0x48 - Port Control Register"]
    #[inline(always)]
    pub const fn pc_ctrl(&self) -> &PC_CTRL {
        &self.pc_ctrl
    }
    #[doc = "0x4c - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pc_model(&self) -> &PC_MODEL {
        &self.pc_model
    }
    #[doc = "0x50 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pc_modeh(&self) -> &PC_MODEH {
        &self.pc_modeh
    }
    #[doc = "0x54 - Port Data Out Register"]
    #[inline(always)]
    pub const fn pc_dout(&self) -> &PC_DOUT {
        &self.pc_dout
    }
    #[doc = "0x58 - Port Data Out Set Register"]
    #[inline(always)]
    pub const fn pc_doutset(&self) -> &PC_DOUTSET {
        &self.pc_doutset
    }
    #[doc = "0x5c - Port Data Out Clear Register"]
    #[inline(always)]
    pub const fn pc_doutclr(&self) -> &PC_DOUTCLR {
        &self.pc_doutclr
    }
    #[doc = "0x60 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pc_douttgl(&self) -> &PC_DOUTTGL {
        &self.pc_douttgl
    }
    #[doc = "0x64 - Port Data In Register"]
    #[inline(always)]
    pub const fn pc_din(&self) -> &PC_DIN {
        &self.pc_din
    }
    #[doc = "0x68 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pc_pinlockn(&self) -> &PC_PINLOCKN {
        &self.pc_pinlockn
    }
    #[doc = "0x6c - Port Control Register"]
    #[inline(always)]
    pub const fn pd_ctrl(&self) -> &PD_CTRL {
        &self.pd_ctrl
    }
    #[doc = "0x70 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pd_model(&self) -> &PD_MODEL {
        &self.pd_model
    }
    #[doc = "0x74 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pd_modeh(&self) -> &PD_MODEH {
        &self.pd_modeh
    }
    #[doc = "0x78 - Port Data Out Register"]
    #[inline(always)]
    pub const fn pd_dout(&self) -> &PD_DOUT {
        &self.pd_dout
    }
    #[doc = "0x7c - Port Data Out Set Register"]
    #[inline(always)]
    pub const fn pd_doutset(&self) -> &PD_DOUTSET {
        &self.pd_doutset
    }
    #[doc = "0x80 - Port Data Out Clear Register"]
    #[inline(always)]
    pub const fn pd_doutclr(&self) -> &PD_DOUTCLR {
        &self.pd_doutclr
    }
    #[doc = "0x84 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pd_douttgl(&self) -> &PD_DOUTTGL {
        &self.pd_douttgl
    }
    #[doc = "0x88 - Port Data In Register"]
    #[inline(always)]
    pub const fn pd_din(&self) -> &PD_DIN {
        &self.pd_din
    }
    #[doc = "0x8c - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pd_pinlockn(&self) -> &PD_PINLOCKN {
        &self.pd_pinlockn
    }
    #[doc = "0x90 - Port Control Register"]
    #[inline(always)]
    pub const fn pe_ctrl(&self) -> &PE_CTRL {
        &self.pe_ctrl
    }
    #[doc = "0x94 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pe_model(&self) -> &PE_MODEL {
        &self.pe_model
    }
    #[doc = "0x98 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pe_modeh(&self) -> &PE_MODEH {
        &self.pe_modeh
    }
    #[doc = "0x9c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pe_dout(&self) -> &PE_DOUT {
        &self.pe_dout
    }
    #[doc = "0xa0 - Port Data Out Set Register"]
    #[inline(always)]
    pub const fn pe_doutset(&self) -> &PE_DOUTSET {
        &self.pe_doutset
    }
    #[doc = "0xa4 - Port Data Out Clear Register"]
    #[inline(always)]
    pub const fn pe_doutclr(&self) -> &PE_DOUTCLR {
        &self.pe_doutclr
    }
    #[doc = "0xa8 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pe_douttgl(&self) -> &PE_DOUTTGL {
        &self.pe_douttgl
    }
    #[doc = "0xac - Port Data In Register"]
    #[inline(always)]
    pub const fn pe_din(&self) -> &PE_DIN {
        &self.pe_din
    }
    #[doc = "0xb0 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pe_pinlockn(&self) -> &PE_PINLOCKN {
        &self.pe_pinlockn
    }
    #[doc = "0xb4 - Port Control Register"]
    #[inline(always)]
    pub const fn pf_ctrl(&self) -> &PF_CTRL {
        &self.pf_ctrl
    }
    #[doc = "0xb8 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pf_model(&self) -> &PF_MODEL {
        &self.pf_model
    }
    #[doc = "0xbc - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pf_modeh(&self) -> &PF_MODEH {
        &self.pf_modeh
    }
    #[doc = "0xc0 - Port Data Out Register"]
    #[inline(always)]
    pub const fn pf_dout(&self) -> &PF_DOUT {
        &self.pf_dout
    }
    #[doc = "0xc4 - Port Data Out Set Register"]
    #[inline(always)]
    pub const fn pf_doutset(&self) -> &PF_DOUTSET {
        &self.pf_doutset
    }
    #[doc = "0xc8 - Port Data Out Clear Register"]
    #[inline(always)]
    pub const fn pf_doutclr(&self) -> &PF_DOUTCLR {
        &self.pf_doutclr
    }
    #[doc = "0xcc - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pf_douttgl(&self) -> &PF_DOUTTGL {
        &self.pf_douttgl
    }
    #[doc = "0xd0 - Port Data In Register"]
    #[inline(always)]
    pub const fn pf_din(&self) -> &PF_DIN {
        &self.pf_din
    }
    #[doc = "0xd4 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pf_pinlockn(&self) -> &PF_PINLOCKN {
        &self.pf_pinlockn
    }
    #[doc = "0x100 - External Interrupt Port Select Low Register"]
    #[inline(always)]
    pub const fn extipsell(&self) -> &EXTIPSELL {
        &self.extipsell
    }
    #[doc = "0x104 - External Interrupt Port Select High Register"]
    #[inline(always)]
    pub const fn extipselh(&self) -> &EXTIPSELH {
        &self.extipselh
    }
    #[doc = "0x108 - External Interrupt Rising Edge Trigger Register"]
    #[inline(always)]
    pub const fn extirise(&self) -> &EXTIRISE {
        &self.extirise
    }
    #[doc = "0x10c - External Interrupt Falling Edge Trigger Register"]
    #[inline(always)]
    pub const fn extifall(&self) -> &EXTIFALL {
        &self.extifall
    }
    #[doc = "0x110 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0x114 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0x118 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0x11c - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
    #[doc = "0x120 - I/O Routing Register"]
    #[inline(always)]
    pub const fn route(&self) -> &ROUTE {
        &self.route
    }
    #[doc = "0x124 - Input Sense Register"]
    #[inline(always)]
    pub const fn insense(&self) -> &INSENSE {
        &self.insense
    }
    #[doc = "0x128 - Configuration Lock Register"]
    #[inline(always)]
    pub const fn lock(&self) -> &LOCK {
        &self.lock
    }
    #[doc = "0x12c - GPIO Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x130 - GPIO Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x134 - EM4 Wake-up Enable Register"]
    #[inline(always)]
    pub const fn em4wuen(&self) -> &EM4WUEN {
        &self.em4wuen
    }
    #[doc = "0x138 - EM4 Wake-up Polarity Register"]
    #[inline(always)]
    pub const fn em4wupol(&self) -> &EM4WUPOL {
        &self.em4wupol
    }
    #[doc = "0x13c - EM4 Wake-up Cause Register"]
    #[inline(always)]
    pub const fn em4wucause(&self) -> &EM4WUCAUSE {
        &self.em4wucause
    }
}
#[doc = "PA_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_ctrl`]
module"]
pub type PA_CTRL = crate::Reg<pa_ctrl::PA_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pa_ctrl;
#[doc = "PA_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_model`]
module"]
pub type PA_MODEL = crate::Reg<pa_model::PA_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pa_model;
#[doc = "PA_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_modeh`]
module"]
pub type PA_MODEH = crate::Reg<pa_modeh::PA_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pa_modeh;
#[doc = "PA_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_dout`]
module"]
pub type PA_DOUT = crate::Reg<pa_dout::PA_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pa_dout;
#[doc = "PA_DOUTSET (w) register accessor: Port Data Out Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_doutset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_doutset`]
module"]
pub type PA_DOUTSET = crate::Reg<pa_doutset::PA_DOUTSET_SPEC>;
#[doc = "Port Data Out Set Register"]
pub mod pa_doutset;
#[doc = "PA_DOUTCLR (w) register accessor: Port Data Out Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_doutclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_doutclr`]
module"]
pub type PA_DOUTCLR = crate::Reg<pa_doutclr::PA_DOUTCLR_SPEC>;
#[doc = "Port Data Out Clear Register"]
pub mod pa_doutclr;
#[doc = "PA_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_douttgl`]
module"]
pub type PA_DOUTTGL = crate::Reg<pa_douttgl::PA_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pa_douttgl;
#[doc = "PA_DIN (r) register accessor: Port Data In Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_din`]
module"]
pub type PA_DIN = crate::Reg<pa_din::PA_DIN_SPEC>;
#[doc = "Port Data In Register"]
pub mod pa_din;
#[doc = "PA_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_pinlockn`]
module"]
pub type PA_PINLOCKN = crate::Reg<pa_pinlockn::PA_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pa_pinlockn;
#[doc = "PB_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_ctrl`]
module"]
pub type PB_CTRL = crate::Reg<pb_ctrl::PB_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pb_ctrl;
#[doc = "PB_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_model`]
module"]
pub type PB_MODEL = crate::Reg<pb_model::PB_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pb_model;
#[doc = "PB_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_modeh`]
module"]
pub type PB_MODEH = crate::Reg<pb_modeh::PB_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pb_modeh;
#[doc = "PB_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_dout`]
module"]
pub type PB_DOUT = crate::Reg<pb_dout::PB_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pb_dout;
#[doc = "PB_DOUTSET (w) register accessor: Port Data Out Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_doutset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_doutset`]
module"]
pub type PB_DOUTSET = crate::Reg<pb_doutset::PB_DOUTSET_SPEC>;
#[doc = "Port Data Out Set Register"]
pub mod pb_doutset;
#[doc = "PB_DOUTCLR (w) register accessor: Port Data Out Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_doutclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_doutclr`]
module"]
pub type PB_DOUTCLR = crate::Reg<pb_doutclr::PB_DOUTCLR_SPEC>;
#[doc = "Port Data Out Clear Register"]
pub mod pb_doutclr;
#[doc = "PB_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_douttgl`]
module"]
pub type PB_DOUTTGL = crate::Reg<pb_douttgl::PB_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pb_douttgl;
#[doc = "PB_DIN (r) register accessor: Port Data In Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_din`]
module"]
pub type PB_DIN = crate::Reg<pb_din::PB_DIN_SPEC>;
#[doc = "Port Data In Register"]
pub mod pb_din;
#[doc = "PB_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_pinlockn`]
module"]
pub type PB_PINLOCKN = crate::Reg<pb_pinlockn::PB_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pb_pinlockn;
#[doc = "PC_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_ctrl`]
module"]
pub type PC_CTRL = crate::Reg<pc_ctrl::PC_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pc_ctrl;
#[doc = "PC_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_model`]
module"]
pub type PC_MODEL = crate::Reg<pc_model::PC_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pc_model;
#[doc = "PC_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_modeh`]
module"]
pub type PC_MODEH = crate::Reg<pc_modeh::PC_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pc_modeh;
#[doc = "PC_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_dout`]
module"]
pub type PC_DOUT = crate::Reg<pc_dout::PC_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pc_dout;
#[doc = "PC_DOUTSET (w) register accessor: Port Data Out Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_doutset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_doutset`]
module"]
pub type PC_DOUTSET = crate::Reg<pc_doutset::PC_DOUTSET_SPEC>;
#[doc = "Port Data Out Set Register"]
pub mod pc_doutset;
#[doc = "PC_DOUTCLR (w) register accessor: Port Data Out Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_doutclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_doutclr`]
module"]
pub type PC_DOUTCLR = crate::Reg<pc_doutclr::PC_DOUTCLR_SPEC>;
#[doc = "Port Data Out Clear Register"]
pub mod pc_doutclr;
#[doc = "PC_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_douttgl`]
module"]
pub type PC_DOUTTGL = crate::Reg<pc_douttgl::PC_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pc_douttgl;
#[doc = "PC_DIN (r) register accessor: Port Data In Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_din`]
module"]
pub type PC_DIN = crate::Reg<pc_din::PC_DIN_SPEC>;
#[doc = "Port Data In Register"]
pub mod pc_din;
#[doc = "PC_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_pinlockn`]
module"]
pub type PC_PINLOCKN = crate::Reg<pc_pinlockn::PC_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pc_pinlockn;
#[doc = "PD_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_ctrl`]
module"]
pub type PD_CTRL = crate::Reg<pd_ctrl::PD_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pd_ctrl;
#[doc = "PD_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_model`]
module"]
pub type PD_MODEL = crate::Reg<pd_model::PD_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pd_model;
#[doc = "PD_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_modeh`]
module"]
pub type PD_MODEH = crate::Reg<pd_modeh::PD_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pd_modeh;
#[doc = "PD_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_dout`]
module"]
pub type PD_DOUT = crate::Reg<pd_dout::PD_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pd_dout;
#[doc = "PD_DOUTSET (w) register accessor: Port Data Out Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_doutset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_doutset`]
module"]
pub type PD_DOUTSET = crate::Reg<pd_doutset::PD_DOUTSET_SPEC>;
#[doc = "Port Data Out Set Register"]
pub mod pd_doutset;
#[doc = "PD_DOUTCLR (w) register accessor: Port Data Out Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_doutclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_doutclr`]
module"]
pub type PD_DOUTCLR = crate::Reg<pd_doutclr::PD_DOUTCLR_SPEC>;
#[doc = "Port Data Out Clear Register"]
pub mod pd_doutclr;
#[doc = "PD_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_douttgl`]
module"]
pub type PD_DOUTTGL = crate::Reg<pd_douttgl::PD_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pd_douttgl;
#[doc = "PD_DIN (r) register accessor: Port Data In Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_din`]
module"]
pub type PD_DIN = crate::Reg<pd_din::PD_DIN_SPEC>;
#[doc = "Port Data In Register"]
pub mod pd_din;
#[doc = "PD_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_pinlockn`]
module"]
pub type PD_PINLOCKN = crate::Reg<pd_pinlockn::PD_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pd_pinlockn;
#[doc = "PE_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_ctrl`]
module"]
pub type PE_CTRL = crate::Reg<pe_ctrl::PE_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pe_ctrl;
#[doc = "PE_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_model`]
module"]
pub type PE_MODEL = crate::Reg<pe_model::PE_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pe_model;
#[doc = "PE_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_modeh`]
module"]
pub type PE_MODEH = crate::Reg<pe_modeh::PE_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pe_modeh;
#[doc = "PE_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_dout`]
module"]
pub type PE_DOUT = crate::Reg<pe_dout::PE_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pe_dout;
#[doc = "PE_DOUTSET (w) register accessor: Port Data Out Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_doutset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_doutset`]
module"]
pub type PE_DOUTSET = crate::Reg<pe_doutset::PE_DOUTSET_SPEC>;
#[doc = "Port Data Out Set Register"]
pub mod pe_doutset;
#[doc = "PE_DOUTCLR (w) register accessor: Port Data Out Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_doutclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_doutclr`]
module"]
pub type PE_DOUTCLR = crate::Reg<pe_doutclr::PE_DOUTCLR_SPEC>;
#[doc = "Port Data Out Clear Register"]
pub mod pe_doutclr;
#[doc = "PE_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_douttgl`]
module"]
pub type PE_DOUTTGL = crate::Reg<pe_douttgl::PE_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pe_douttgl;
#[doc = "PE_DIN (r) register accessor: Port Data In Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_din`]
module"]
pub type PE_DIN = crate::Reg<pe_din::PE_DIN_SPEC>;
#[doc = "Port Data In Register"]
pub mod pe_din;
#[doc = "PE_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_pinlockn`]
module"]
pub type PE_PINLOCKN = crate::Reg<pe_pinlockn::PE_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pe_pinlockn;
#[doc = "PF_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_ctrl`]
module"]
pub type PF_CTRL = crate::Reg<pf_ctrl::PF_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pf_ctrl;
#[doc = "PF_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_model`]
module"]
pub type PF_MODEL = crate::Reg<pf_model::PF_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pf_model;
#[doc = "PF_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_modeh`]
module"]
pub type PF_MODEH = crate::Reg<pf_modeh::PF_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pf_modeh;
#[doc = "PF_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_dout`]
module"]
pub type PF_DOUT = crate::Reg<pf_dout::PF_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pf_dout;
#[doc = "PF_DOUTSET (w) register accessor: Port Data Out Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_doutset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_doutset`]
module"]
pub type PF_DOUTSET = crate::Reg<pf_doutset::PF_DOUTSET_SPEC>;
#[doc = "Port Data Out Set Register"]
pub mod pf_doutset;
#[doc = "PF_DOUTCLR (w) register accessor: Port Data Out Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_doutclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_doutclr`]
module"]
pub type PF_DOUTCLR = crate::Reg<pf_doutclr::PF_DOUTCLR_SPEC>;
#[doc = "Port Data Out Clear Register"]
pub mod pf_doutclr;
#[doc = "PF_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_douttgl`]
module"]
pub type PF_DOUTTGL = crate::Reg<pf_douttgl::PF_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pf_douttgl;
#[doc = "PF_DIN (r) register accessor: Port Data In Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_din`]
module"]
pub type PF_DIN = crate::Reg<pf_din::PF_DIN_SPEC>;
#[doc = "Port Data In Register"]
pub mod pf_din;
#[doc = "PF_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_pinlockn`]
module"]
pub type PF_PINLOCKN = crate::Reg<pf_pinlockn::PF_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pf_pinlockn;
#[doc = "EXTIPSELL (rw) register accessor: External Interrupt Port Select Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extipsell::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extipsell::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipsell`]
module"]
pub type EXTIPSELL = crate::Reg<extipsell::EXTIPSELL_SPEC>;
#[doc = "External Interrupt Port Select Low Register"]
pub mod extipsell;
#[doc = "EXTIPSELH (rw) register accessor: External Interrupt Port Select High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extipselh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extipselh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipselh`]
module"]
pub type EXTIPSELH = crate::Reg<extipselh::EXTIPSELH_SPEC>;
#[doc = "External Interrupt Port Select High Register"]
pub mod extipselh;
#[doc = "EXTIRISE (rw) register accessor: External Interrupt Rising Edge Trigger Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extirise::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extirise::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extirise`]
module"]
pub type EXTIRISE = crate::Reg<extirise::EXTIRISE_SPEC>;
#[doc = "External Interrupt Rising Edge Trigger Register"]
pub mod extirise;
#[doc = "EXTIFALL (rw) register accessor: External Interrupt Falling Edge Trigger Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extifall::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extifall::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extifall`]
module"]
pub type EXTIFALL = crate::Reg<extifall::EXTIFALL_SPEC>;
#[doc = "External Interrupt Falling Edge Trigger Register"]
pub mod extifall;
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
#[doc = "ROUTE (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`route::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`route::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@route`]
module"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "INSENSE (rw) register accessor: Input Sense Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`insense::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`insense::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@insense`]
module"]
pub type INSENSE = crate::Reg<insense::INSENSE_SPEC>;
#[doc = "Input Sense Register"]
pub mod insense;
#[doc = "LOCK (rw) register accessor: Configuration Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "CTRL (rw) register accessor: GPIO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "GPIO Control Register"]
pub mod ctrl;
#[doc = "CMD (w) register accessor: GPIO Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "GPIO Command Register"]
pub mod cmd;
#[doc = "EM4WUEN (rw) register accessor: EM4 Wake-up Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`em4wuen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`em4wuen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4wuen`]
module"]
pub type EM4WUEN = crate::Reg<em4wuen::EM4WUEN_SPEC>;
#[doc = "EM4 Wake-up Enable Register"]
pub mod em4wuen;
#[doc = "EM4WUPOL (rw) register accessor: EM4 Wake-up Polarity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`em4wupol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`em4wupol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4wupol`]
module"]
pub type EM4WUPOL = crate::Reg<em4wupol::EM4WUPOL_SPEC>;
#[doc = "EM4 Wake-up Polarity Register"]
pub mod em4wupol;
#[doc = "EM4WUCAUSE (r) register accessor: EM4 Wake-up Cause Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`em4wucause::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4wucause`]
module"]
pub type EM4WUCAUSE = crate::Reg<em4wucause::EM4WUCAUSE_SPEC>;
#[doc = "EM4 Wake-up Cause Register"]
pub mod em4wucause;
