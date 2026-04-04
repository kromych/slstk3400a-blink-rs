#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    curprog: Curprog,
    cal: Cal,
    dutyconfig: Dutyconfig,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Current Programming Register"]
    #[inline(always)]
    pub const fn curprog(&self) -> &Curprog {
        &self.curprog
    }
    #[doc = "0x08 - Calibration Register"]
    #[inline(always)]
    pub const fn cal(&self) -> &Cal {
        &self.cal
    }
    #[doc = "0x0c - Duty Cycle Configauration Register"]
    #[inline(always)]
    pub const fn dutyconfig(&self) -> &Dutyconfig {
        &self.dutyconfig
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CURPROG (rw) register accessor: Current Programming Register\n\nYou can [`read`](crate::Reg::read) this register and get [`curprog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`curprog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@curprog`] module"]
#[doc(alias = "CURPROG")]
pub type Curprog = crate::Reg<curprog::CurprogSpec>;
#[doc = "Current Programming Register"]
pub mod curprog;
#[doc = "CAL (rw) register accessor: Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal`] module"]
#[doc(alias = "CAL")]
pub type Cal = crate::Reg<cal::CalSpec>;
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "DUTYCONFIG (rw) register accessor: Duty Cycle Configauration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dutyconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dutyconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dutyconfig`] module"]
#[doc(alias = "DUTYCONFIG")]
pub type Dutyconfig = crate::Reg<dutyconfig::DutyconfigSpec>;
#[doc = "Duty Cycle Configauration Register"]
pub mod dutyconfig;
