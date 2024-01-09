#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    hfcoreclkdiv: HFCORECLKDIV,
    hfperclkdiv: HFPERCLKDIV,
    hfrcoctrl: HFRCOCTRL,
    lfrcoctrl: LFRCOCTRL,
    auxhfrcoctrl: AUXHFRCOCTRL,
    calctrl: CALCTRL,
    calcnt: CALCNT,
    oscencmd: OSCENCMD,
    cmd: CMD,
    lfclksel: LFCLKSEL,
    status: STATUS,
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    ien: IEN,
    hfcoreclken0: HFCORECLKEN0,
    hfperclken0: HFPERCLKEN0,
    _reserved18: [u8; 0x08],
    syncbusy: SYNCBUSY,
    freeze: FREEZE,
    lfaclken0: LFACLKEN0,
    _reserved21: [u8; 0x04],
    lfbclken0: LFBCLKEN0,
    lfcclken0: LFCCLKEN0,
    lfapresc0: LFAPRESC0,
    _reserved24: [u8; 0x04],
    lfbpresc0: LFBPRESC0,
    _reserved25: [u8; 0x04],
    pcntctrl: PCNTCTRL,
    _reserved26: [u8; 0x04],
    route: ROUTE,
    lock: LOCK,
    _reserved28: [u8; 0x48],
    usbcrctrl: USBCRCTRL,
    ushfrcoctrl: USHFRCOCTRL,
    ushfrcotune: USHFRCOTUNE,
    ushfrcoconf: USHFRCOCONF,
}
impl RegisterBlock {
    #[doc = "0x00 - CMU Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - High Frequency Core Clock Division Register"]
    #[inline(always)]
    pub const fn hfcoreclkdiv(&self) -> &HFCORECLKDIV {
        &self.hfcoreclkdiv
    }
    #[doc = "0x08 - High Frequency Peripheral Clock Division Register"]
    #[inline(always)]
    pub const fn hfperclkdiv(&self) -> &HFPERCLKDIV {
        &self.hfperclkdiv
    }
    #[doc = "0x0c - HFRCO Control Register"]
    #[inline(always)]
    pub const fn hfrcoctrl(&self) -> &HFRCOCTRL {
        &self.hfrcoctrl
    }
    #[doc = "0x10 - LFRCO Control Register"]
    #[inline(always)]
    pub const fn lfrcoctrl(&self) -> &LFRCOCTRL {
        &self.lfrcoctrl
    }
    #[doc = "0x14 - AUXHFRCO Control Register"]
    #[inline(always)]
    pub const fn auxhfrcoctrl(&self) -> &AUXHFRCOCTRL {
        &self.auxhfrcoctrl
    }
    #[doc = "0x18 - Calibration Control Register"]
    #[inline(always)]
    pub const fn calctrl(&self) -> &CALCTRL {
        &self.calctrl
    }
    #[doc = "0x1c - Calibration Counter Register"]
    #[inline(always)]
    pub const fn calcnt(&self) -> &CALCNT {
        &self.calcnt
    }
    #[doc = "0x20 - Oscillator Enable/Disable Command Register"]
    #[inline(always)]
    pub const fn oscencmd(&self) -> &OSCENCMD {
        &self.oscencmd
    }
    #[doc = "0x24 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x28 - Low Frequency Clock Select Register"]
    #[inline(always)]
    pub const fn lfclksel(&self) -> &LFCLKSEL {
        &self.lfclksel
    }
    #[doc = "0x2c - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x30 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0x34 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0x38 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
    #[doc = "0x3c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0x40 - High Frequency Core Clock Enable Register 0"]
    #[inline(always)]
    pub const fn hfcoreclken0(&self) -> &HFCORECLKEN0 {
        &self.hfcoreclken0
    }
    #[doc = "0x44 - High Frequency Peripheral Clock Enable Register 0"]
    #[inline(always)]
    pub const fn hfperclken0(&self) -> &HFPERCLKEN0 {
        &self.hfperclken0
    }
    #[doc = "0x50 - Synchronization Busy Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &SYNCBUSY {
        &self.syncbusy
    }
    #[doc = "0x54 - Freeze Register"]
    #[inline(always)]
    pub const fn freeze(&self) -> &FREEZE {
        &self.freeze
    }
    #[doc = "0x58 - Low Frequency A Clock Enable Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfaclken0(&self) -> &LFACLKEN0 {
        &self.lfaclken0
    }
    #[doc = "0x60 - Low Frequency B Clock Enable Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfbclken0(&self) -> &LFBCLKEN0 {
        &self.lfbclken0
    }
    #[doc = "0x64 - Low Frequency C Clock Enable Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfcclken0(&self) -> &LFCCLKEN0 {
        &self.lfcclken0
    }
    #[doc = "0x68 - Low Frequency A Prescaler Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfapresc0(&self) -> &LFAPRESC0 {
        &self.lfapresc0
    }
    #[doc = "0x70 - Low Frequency B Prescaler Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfbpresc0(&self) -> &LFBPRESC0 {
        &self.lfbpresc0
    }
    #[doc = "0x78 - PCNT Control Register"]
    #[inline(always)]
    pub const fn pcntctrl(&self) -> &PCNTCTRL {
        &self.pcntctrl
    }
    #[doc = "0x80 - I/O Routing Register"]
    #[inline(always)]
    pub const fn route(&self) -> &ROUTE {
        &self.route
    }
    #[doc = "0x84 - Configuration Lock Register"]
    #[inline(always)]
    pub const fn lock(&self) -> &LOCK {
        &self.lock
    }
    #[doc = "0xd0 - USB Clock Recovery Control"]
    #[inline(always)]
    pub const fn usbcrctrl(&self) -> &USBCRCTRL {
        &self.usbcrctrl
    }
    #[doc = "0xd4 - USHFRCO Control"]
    #[inline(always)]
    pub const fn ushfrcoctrl(&self) -> &USHFRCOCTRL {
        &self.ushfrcoctrl
    }
    #[doc = "0xd8 - USHFRCO Frequency Tune"]
    #[inline(always)]
    pub const fn ushfrcotune(&self) -> &USHFRCOTUNE {
        &self.ushfrcotune
    }
    #[doc = "0xdc - USHFRCO Configuration"]
    #[inline(always)]
    pub const fn ushfrcoconf(&self) -> &USHFRCOCONF {
        &self.ushfrcoconf
    }
}
#[doc = "CTRL (rw) register accessor: CMU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "CMU Control Register"]
pub mod ctrl;
#[doc = "HFCORECLKDIV (rw) register accessor: High Frequency Core Clock Division Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfcoreclkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfcoreclkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfcoreclkdiv`]
module"]
pub type HFCORECLKDIV = crate::Reg<hfcoreclkdiv::HFCORECLKDIV_SPEC>;
#[doc = "High Frequency Core Clock Division Register"]
pub mod hfcoreclkdiv;
#[doc = "HFPERCLKDIV (rw) register accessor: High Frequency Peripheral Clock Division Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfperclkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfperclkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfperclkdiv`]
module"]
pub type HFPERCLKDIV = crate::Reg<hfperclkdiv::HFPERCLKDIV_SPEC>;
#[doc = "High Frequency Peripheral Clock Division Register"]
pub mod hfperclkdiv;
#[doc = "HFRCOCTRL (rw) register accessor: HFRCO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfrcoctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfrcoctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoctrl`]
module"]
pub type HFRCOCTRL = crate::Reg<hfrcoctrl::HFRCOCTRL_SPEC>;
#[doc = "HFRCO Control Register"]
pub mod hfrcoctrl;
#[doc = "LFRCOCTRL (rw) register accessor: LFRCO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfrcoctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfrcoctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfrcoctrl`]
module"]
pub type LFRCOCTRL = crate::Reg<lfrcoctrl::LFRCOCTRL_SPEC>;
#[doc = "LFRCO Control Register"]
pub mod lfrcoctrl;
#[doc = "AUXHFRCOCTRL (rw) register accessor: AUXHFRCO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxhfrcoctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxhfrcoctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auxhfrcoctrl`]
module"]
pub type AUXHFRCOCTRL = crate::Reg<auxhfrcoctrl::AUXHFRCOCTRL_SPEC>;
#[doc = "AUXHFRCO Control Register"]
pub mod auxhfrcoctrl;
#[doc = "CALCTRL (rw) register accessor: Calibration Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calctrl`]
module"]
pub type CALCTRL = crate::Reg<calctrl::CALCTRL_SPEC>;
#[doc = "Calibration Control Register"]
pub mod calctrl;
#[doc = "CALCNT (rw) register accessor: Calibration Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calcnt`]
module"]
pub type CALCNT = crate::Reg<calcnt::CALCNT_SPEC>;
#[doc = "Calibration Counter Register"]
pub mod calcnt;
#[doc = "OSCENCMD (w) register accessor: Oscillator Enable/Disable Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscencmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscencmd`]
module"]
pub type OSCENCMD = crate::Reg<oscencmd::OSCENCMD_SPEC>;
#[doc = "Oscillator Enable/Disable Command Register"]
pub mod oscencmd;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "LFCLKSEL (rw) register accessor: Low Frequency Clock Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfclksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfclksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclksel`]
module"]
pub type LFCLKSEL = crate::Reg<lfclksel::LFCLKSEL_SPEC>;
#[doc = "Low Frequency Clock Select Register"]
pub mod lfclksel;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
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
#[doc = "HFCORECLKEN0 (rw) register accessor: High Frequency Core Clock Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfcoreclken0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfcoreclken0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfcoreclken0`]
module"]
pub type HFCORECLKEN0 = crate::Reg<hfcoreclken0::HFCORECLKEN0_SPEC>;
#[doc = "High Frequency Core Clock Enable Register 0"]
pub mod hfcoreclken0;
#[doc = "HFPERCLKEN0 (rw) register accessor: High Frequency Peripheral Clock Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfperclken0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfperclken0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfperclken0`]
module"]
pub type HFPERCLKEN0 = crate::Reg<hfperclken0::HFPERCLKEN0_SPEC>;
#[doc = "High Frequency Peripheral Clock Enable Register 0"]
pub mod hfperclken0;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`]
module"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "FREEZE (rw) register accessor: Freeze Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freeze::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freeze::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freeze`]
module"]
pub type FREEZE = crate::Reg<freeze::FREEZE_SPEC>;
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "LFACLKEN0 (rw) register accessor: Low Frequency A Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfaclken0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfaclken0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfaclken0`]
module"]
pub type LFACLKEN0 = crate::Reg<lfaclken0::LFACLKEN0_SPEC>;
#[doc = "Low Frequency A Clock Enable Register 0 (Async Reg)"]
pub mod lfaclken0;
#[doc = "LFBCLKEN0 (rw) register accessor: Low Frequency B Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfbclken0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfbclken0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfbclken0`]
module"]
pub type LFBCLKEN0 = crate::Reg<lfbclken0::LFBCLKEN0_SPEC>;
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)"]
pub mod lfbclken0;
#[doc = "LFCCLKEN0 (rw) register accessor: Low Frequency C Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfcclken0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfcclken0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfcclken0`]
module"]
pub type LFCCLKEN0 = crate::Reg<lfcclken0::LFCCLKEN0_SPEC>;
#[doc = "Low Frequency C Clock Enable Register 0 (Async Reg)"]
pub mod lfcclken0;
#[doc = "LFAPRESC0 (rw) register accessor: Low Frequency A Prescaler Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfapresc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfapresc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfapresc0`]
module"]
pub type LFAPRESC0 = crate::Reg<lfapresc0::LFAPRESC0_SPEC>;
#[doc = "Low Frequency A Prescaler Register 0 (Async Reg)"]
pub mod lfapresc0;
#[doc = "LFBPRESC0 (rw) register accessor: Low Frequency B Prescaler Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfbpresc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfbpresc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfbpresc0`]
module"]
pub type LFBPRESC0 = crate::Reg<lfbpresc0::LFBPRESC0_SPEC>;
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)"]
pub mod lfbpresc0;
#[doc = "PCNTCTRL (rw) register accessor: PCNT Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcntctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcntctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcntctrl`]
module"]
pub type PCNTCTRL = crate::Reg<pcntctrl::PCNTCTRL_SPEC>;
#[doc = "PCNT Control Register"]
pub mod pcntctrl;
#[doc = "ROUTE (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`route::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`route::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@route`]
module"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "LOCK (rw) register accessor: Configuration Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "USBCRCTRL (rw) register accessor: USB Clock Recovery Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbcrctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbcrctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbcrctrl`]
module"]
pub type USBCRCTRL = crate::Reg<usbcrctrl::USBCRCTRL_SPEC>;
#[doc = "USB Clock Recovery Control"]
pub mod usbcrctrl;
#[doc = "USHFRCOCTRL (rw) register accessor: USHFRCO Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ushfrcoctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ushfrcoctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ushfrcoctrl`]
module"]
pub type USHFRCOCTRL = crate::Reg<ushfrcoctrl::USHFRCOCTRL_SPEC>;
#[doc = "USHFRCO Control"]
pub mod ushfrcoctrl;
#[doc = "USHFRCOTUNE (rw) register accessor: USHFRCO Frequency Tune\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ushfrcotune::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ushfrcotune::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ushfrcotune`]
module"]
pub type USHFRCOTUNE = crate::Reg<ushfrcotune::USHFRCOTUNE_SPEC>;
#[doc = "USHFRCO Frequency Tune"]
pub mod ushfrcotune;
#[doc = "USHFRCOCONF (rw) register accessor: USHFRCO Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ushfrcoconf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ushfrcoconf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ushfrcoconf`]
module"]
pub type USHFRCOCONF = crate::Reg<ushfrcoconf::USHFRCOCONF_SPEC>;
#[doc = "USHFRCO Configuration"]
pub mod ushfrcoconf;
