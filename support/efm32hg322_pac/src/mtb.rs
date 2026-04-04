#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    position: Position,
    master: Master,
    flow: Flow,
    base: Base,
}
impl RegisterBlock {
    #[doc = "0x00 - MTB Trace Position Register."]
    #[inline(always)]
    pub const fn position(&self) -> &Position {
        &self.position
    }
    #[doc = "0x04 - MTB Trace Control Register"]
    #[inline(always)]
    pub const fn master(&self) -> &Master {
        &self.master
    }
    #[doc = "0x08 - MTB Trace Flow Register"]
    #[inline(always)]
    pub const fn flow(&self) -> &Flow {
        &self.flow
    }
    #[doc = "0x0c - MTB Trace Base Register"]
    #[inline(always)]
    pub const fn base(&self) -> &Base {
        &self.base
    }
}
#[doc = "POSITION (rw) register accessor: MTB Trace Position Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`position::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`position::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@position`] module"]
#[doc(alias = "POSITION")]
pub type Position = crate::Reg<position::PositionSpec>;
#[doc = "MTB Trace Position Register."]
pub mod position;
#[doc = "MASTER (rw) register accessor: MTB Trace Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`master::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`master::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@master`] module"]
#[doc(alias = "MASTER")]
pub type Master = crate::Reg<master::MasterSpec>;
#[doc = "MTB Trace Control Register"]
pub mod master;
#[doc = "FLOW (rw) register accessor: MTB Trace Flow Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flow`] module"]
#[doc(alias = "FLOW")]
pub type Flow = crate::Reg<flow::FlowSpec>;
#[doc = "MTB Trace Flow Register"]
pub mod flow;
#[doc = "BASE (rw) register accessor: MTB Trace Base Register\n\nYou can [`read`](crate::Reg::read) this register and get [`base::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`base::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base`] module"]
#[doc(alias = "BASE")]
pub type Base = crate::Reg<base::BaseSpec>;
#[doc = "MTB Trace Base Register"]
pub mod base;
