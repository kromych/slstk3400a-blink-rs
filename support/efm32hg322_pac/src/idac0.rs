#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    curprog: CURPROG,
    cal: CAL,
    dutyconfig: DUTYCONFIG,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Current Programming Register"]
    #[inline(always)]
    pub const fn curprog(&self) -> &CURPROG {
        &self.curprog
    }
    #[doc = "0x08 - Calibration Register"]
    #[inline(always)]
    pub const fn cal(&self) -> &CAL {
        &self.cal
    }
    #[doc = "0x0c - Duty Cycle Configauration Register"]
    #[inline(always)]
    pub const fn dutyconfig(&self) -> &DUTYCONFIG {
        &self.dutyconfig
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CURPROG (rw) register accessor: Current Programming Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`curprog::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`curprog::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@curprog`]
module"]
pub type CURPROG = crate::Reg<curprog::CURPROG_SPEC>;
#[doc = "Current Programming Register"]
pub mod curprog;
#[doc = "CAL (rw) register accessor: Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal`]
module"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "DUTYCONFIG (rw) register accessor: Duty Cycle Configauration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dutyconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dutyconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dutyconfig`]
module"]
pub type DUTYCONFIG = crate::Reg<dutyconfig::DUTYCONFIG_SPEC>;
#[doc = "Duty Cycle Configauration Register"]
pub mod dutyconfig;
