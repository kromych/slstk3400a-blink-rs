#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    position: POSITION,
    master: MASTER,
    flow: FLOW,
    base: BASE,
}
impl RegisterBlock {
    #[doc = "0x00 - MTB Trace Position Register."]
    #[inline(always)]
    pub const fn position(&self) -> &POSITION {
        &self.position
    }
    #[doc = "0x04 - MTB Trace Control Register"]
    #[inline(always)]
    pub const fn master(&self) -> &MASTER {
        &self.master
    }
    #[doc = "0x08 - MTB Trace Flow Register"]
    #[inline(always)]
    pub const fn flow(&self) -> &FLOW {
        &self.flow
    }
    #[doc = "0x0c - MTB Trace Base Register"]
    #[inline(always)]
    pub const fn base(&self) -> &BASE {
        &self.base
    }
}
#[doc = "POSITION (rw) register accessor: MTB Trace Position Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`position::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`position::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@position`]
module"]
pub type POSITION = crate::Reg<position::POSITION_SPEC>;
#[doc = "MTB Trace Position Register."]
pub mod position;
#[doc = "MASTER (rw) register accessor: MTB Trace Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`master::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`master::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@master`]
module"]
pub type MASTER = crate::Reg<master::MASTER_SPEC>;
#[doc = "MTB Trace Control Register"]
pub mod master;
#[doc = "FLOW (rw) register accessor: MTB Trace Flow Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flow::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flow::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flow`]
module"]
pub type FLOW = crate::Reg<flow::FLOW_SPEC>;
#[doc = "MTB Trace Flow Register"]
pub mod flow;
#[doc = "BASE (rw) register accessor: MTB Trace Base Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base`]
module"]
pub type BASE = crate::Reg<base::BASE_SPEC>;
#[doc = "MTB Trace Base Register"]
pub mod base;
