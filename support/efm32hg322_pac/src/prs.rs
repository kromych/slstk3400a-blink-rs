#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    swpulse: Swpulse,
    swlevel: Swlevel,
    route: Route,
    _reserved3: [u8; 0x04],
    ch0_ctrl: Ch0Ctrl,
    ch1_ctrl: Ch1Ctrl,
    ch2_ctrl: Ch2Ctrl,
    ch3_ctrl: Ch3Ctrl,
    ch4_ctrl: Ch4Ctrl,
    ch5_ctrl: Ch5Ctrl,
    _reserved9: [u8; 0x18],
    tracectrl: Tracectrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Software Pulse Register"]
    #[inline(always)]
    pub const fn swpulse(&self) -> &Swpulse {
        &self.swpulse
    }
    #[doc = "0x04 - Software Level Register"]
    #[inline(always)]
    pub const fn swlevel(&self) -> &Swlevel {
        &self.swlevel
    }
    #[doc = "0x08 - I/O Routing Register"]
    #[inline(always)]
    pub const fn route(&self) -> &Route {
        &self.route
    }
    #[doc = "0x10 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch0_ctrl(&self) -> &Ch0Ctrl {
        &self.ch0_ctrl
    }
    #[doc = "0x14 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch1_ctrl(&self) -> &Ch1Ctrl {
        &self.ch1_ctrl
    }
    #[doc = "0x18 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch2_ctrl(&self) -> &Ch2Ctrl {
        &self.ch2_ctrl
    }
    #[doc = "0x1c - Channel Control Register"]
    #[inline(always)]
    pub const fn ch3_ctrl(&self) -> &Ch3Ctrl {
        &self.ch3_ctrl
    }
    #[doc = "0x20 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch4_ctrl(&self) -> &Ch4Ctrl {
        &self.ch4_ctrl
    }
    #[doc = "0x24 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch5_ctrl(&self) -> &Ch5Ctrl {
        &self.ch5_ctrl
    }
    #[doc = "0x40 - MTB Trace Control Register"]
    #[inline(always)]
    pub const fn tracectrl(&self) -> &Tracectrl {
        &self.tracectrl
    }
}
#[doc = "SWPULSE (w) register accessor: Software Pulse Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpulse::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swpulse`] module"]
#[doc(alias = "SWPULSE")]
pub type Swpulse = crate::Reg<swpulse::SwpulseSpec>;
#[doc = "Software Pulse Register"]
pub mod swpulse;
#[doc = "SWLEVEL (rw) register accessor: Software Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`swlevel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swlevel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swlevel`] module"]
#[doc(alias = "SWLEVEL")]
pub type Swlevel = crate::Reg<swlevel::SwlevelSpec>;
#[doc = "Software Level Register"]
pub mod swlevel;
#[doc = "ROUTE (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@route`] module"]
#[doc(alias = "ROUTE")]
pub type Route = crate::Reg<route::RouteSpec>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "CH0_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_ctrl`] module"]
#[doc(alias = "CH0_CTRL")]
pub type Ch0Ctrl = crate::Reg<ch0_ctrl::Ch0CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch0_ctrl;
#[doc = "CH1_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_ctrl`] module"]
#[doc(alias = "CH1_CTRL")]
pub type Ch1Ctrl = crate::Reg<ch1_ctrl::Ch1CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch1_ctrl;
#[doc = "CH2_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_ctrl`] module"]
#[doc(alias = "CH2_CTRL")]
pub type Ch2Ctrl = crate::Reg<ch2_ctrl::Ch2CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch2_ctrl;
#[doc = "CH3_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_ctrl`] module"]
#[doc(alias = "CH3_CTRL")]
pub type Ch3Ctrl = crate::Reg<ch3_ctrl::Ch3CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch3_ctrl;
#[doc = "CH4_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_ctrl`] module"]
#[doc(alias = "CH4_CTRL")]
pub type Ch4Ctrl = crate::Reg<ch4_ctrl::Ch4CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch4_ctrl;
#[doc = "CH5_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_ctrl`] module"]
#[doc(alias = "CH5_CTRL")]
pub type Ch5Ctrl = crate::Reg<ch5_ctrl::Ch5CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch5_ctrl;
#[doc = "TRACECTRL (rw) register accessor: MTB Trace Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tracectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tracectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tracectrl`] module"]
#[doc(alias = "TRACECTRL")]
pub type Tracectrl = crate::Reg<tracectrl::TracectrlSpec>;
#[doc = "MTB Trace Control Register"]
pub mod tracectrl;
