#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    hfcoreclkdiv: Hfcoreclkdiv,
    hfperclkdiv: Hfperclkdiv,
    hfrcoctrl: Hfrcoctrl,
    lfrcoctrl: Lfrcoctrl,
    auxhfrcoctrl: Auxhfrcoctrl,
    calctrl: Calctrl,
    calcnt: Calcnt,
    oscencmd: Oscencmd,
    cmd: Cmd,
    lfclksel: Lfclksel,
    status: Status,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    hfcoreclken0: Hfcoreclken0,
    hfperclken0: Hfperclken0,
    _reserved18: [u8; 0x08],
    syncbusy: Syncbusy,
    freeze: Freeze,
    lfaclken0: Lfaclken0,
    _reserved21: [u8; 0x04],
    lfbclken0: Lfbclken0,
    lfcclken0: Lfcclken0,
    lfapresc0: Lfapresc0,
    _reserved24: [u8; 0x04],
    lfbpresc0: Lfbpresc0,
    _reserved25: [u8; 0x04],
    pcntctrl: Pcntctrl,
    _reserved26: [u8; 0x04],
    route: Route,
    lock: Lock,
    _reserved28: [u8; 0x48],
    usbcrctrl: Usbcrctrl,
    ushfrcoctrl: Ushfrcoctrl,
    ushfrcotune: Ushfrcotune,
    ushfrcoconf: Ushfrcoconf,
}
impl RegisterBlock {
    #[doc = "0x00 - CMU Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - High Frequency Core Clock Division Register"]
    #[inline(always)]
    pub const fn hfcoreclkdiv(&self) -> &Hfcoreclkdiv {
        &self.hfcoreclkdiv
    }
    #[doc = "0x08 - High Frequency Peripheral Clock Division Register"]
    #[inline(always)]
    pub const fn hfperclkdiv(&self) -> &Hfperclkdiv {
        &self.hfperclkdiv
    }
    #[doc = "0x0c - HFRCO Control Register"]
    #[inline(always)]
    pub const fn hfrcoctrl(&self) -> &Hfrcoctrl {
        &self.hfrcoctrl
    }
    #[doc = "0x10 - LFRCO Control Register"]
    #[inline(always)]
    pub const fn lfrcoctrl(&self) -> &Lfrcoctrl {
        &self.lfrcoctrl
    }
    #[doc = "0x14 - AUXHFRCO Control Register"]
    #[inline(always)]
    pub const fn auxhfrcoctrl(&self) -> &Auxhfrcoctrl {
        &self.auxhfrcoctrl
    }
    #[doc = "0x18 - Calibration Control Register"]
    #[inline(always)]
    pub const fn calctrl(&self) -> &Calctrl {
        &self.calctrl
    }
    #[doc = "0x1c - Calibration Counter Register"]
    #[inline(always)]
    pub const fn calcnt(&self) -> &Calcnt {
        &self.calcnt
    }
    #[doc = "0x20 - Oscillator Enable/Disable Command Register"]
    #[inline(always)]
    pub const fn oscencmd(&self) -> &Oscencmd {
        &self.oscencmd
    }
    #[doc = "0x24 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x28 - Low Frequency Clock Select Register"]
    #[inline(always)]
    pub const fn lfclksel(&self) -> &Lfclksel {
        &self.lfclksel
    }
    #[doc = "0x2c - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x30 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x34 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x38 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x3c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x40 - High Frequency Core Clock Enable Register 0"]
    #[inline(always)]
    pub const fn hfcoreclken0(&self) -> &Hfcoreclken0 {
        &self.hfcoreclken0
    }
    #[doc = "0x44 - High Frequency Peripheral Clock Enable Register 0"]
    #[inline(always)]
    pub const fn hfperclken0(&self) -> &Hfperclken0 {
        &self.hfperclken0
    }
    #[doc = "0x50 - Synchronization Busy Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x54 - Freeze Register"]
    #[inline(always)]
    pub const fn freeze(&self) -> &Freeze {
        &self.freeze
    }
    #[doc = "0x58 - Low Frequency A Clock Enable Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfaclken0(&self) -> &Lfaclken0 {
        &self.lfaclken0
    }
    #[doc = "0x60 - Low Frequency B Clock Enable Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfbclken0(&self) -> &Lfbclken0 {
        &self.lfbclken0
    }
    #[doc = "0x64 - Low Frequency C Clock Enable Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfcclken0(&self) -> &Lfcclken0 {
        &self.lfcclken0
    }
    #[doc = "0x68 - Low Frequency A Prescaler Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfapresc0(&self) -> &Lfapresc0 {
        &self.lfapresc0
    }
    #[doc = "0x70 - Low Frequency B Prescaler Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfbpresc0(&self) -> &Lfbpresc0 {
        &self.lfbpresc0
    }
    #[doc = "0x78 - PCNT Control Register"]
    #[inline(always)]
    pub const fn pcntctrl(&self) -> &Pcntctrl {
        &self.pcntctrl
    }
    #[doc = "0x80 - I/O Routing Register"]
    #[inline(always)]
    pub const fn route(&self) -> &Route {
        &self.route
    }
    #[doc = "0x84 - Configuration Lock Register"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0xd0 - USB Clock Recovery Control"]
    #[inline(always)]
    pub const fn usbcrctrl(&self) -> &Usbcrctrl {
        &self.usbcrctrl
    }
    #[doc = "0xd4 - USHFRCO Control"]
    #[inline(always)]
    pub const fn ushfrcoctrl(&self) -> &Ushfrcoctrl {
        &self.ushfrcoctrl
    }
    #[doc = "0xd8 - USHFRCO Frequency Tune"]
    #[inline(always)]
    pub const fn ushfrcotune(&self) -> &Ushfrcotune {
        &self.ushfrcotune
    }
    #[doc = "0xdc - USHFRCO Configuration"]
    #[inline(always)]
    pub const fn ushfrcoconf(&self) -> &Ushfrcoconf {
        &self.ushfrcoconf
    }
}
#[doc = "CTRL (rw) register accessor: CMU Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "CMU Control Register"]
pub mod ctrl;
#[doc = "HFCORECLKDIV (rw) register accessor: High Frequency Core Clock Division Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcoreclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcoreclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfcoreclkdiv`] module"]
#[doc(alias = "HFCORECLKDIV")]
pub type Hfcoreclkdiv = crate::Reg<hfcoreclkdiv::HfcoreclkdivSpec>;
#[doc = "High Frequency Core Clock Division Register"]
pub mod hfcoreclkdiv;
#[doc = "HFPERCLKDIV (rw) register accessor: High Frequency Peripheral Clock Division Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfperclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfperclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfperclkdiv`] module"]
#[doc(alias = "HFPERCLKDIV")]
pub type Hfperclkdiv = crate::Reg<hfperclkdiv::HfperclkdivSpec>;
#[doc = "High Frequency Peripheral Clock Division Register"]
pub mod hfperclkdiv;
#[doc = "HFRCOCTRL (rw) register accessor: HFRCO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfrcoctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoctrl`] module"]
#[doc(alias = "HFRCOCTRL")]
pub type Hfrcoctrl = crate::Reg<hfrcoctrl::HfrcoctrlSpec>;
#[doc = "HFRCO Control Register"]
pub mod hfrcoctrl;
#[doc = "LFRCOCTRL (rw) register accessor: LFRCO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfrcoctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfrcoctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfrcoctrl`] module"]
#[doc(alias = "LFRCOCTRL")]
pub type Lfrcoctrl = crate::Reg<lfrcoctrl::LfrcoctrlSpec>;
#[doc = "LFRCO Control Register"]
pub mod lfrcoctrl;
#[doc = "AUXHFRCOCTRL (rw) register accessor: AUXHFRCO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`auxhfrcoctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`auxhfrcoctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auxhfrcoctrl`] module"]
#[doc(alias = "AUXHFRCOCTRL")]
pub type Auxhfrcoctrl = crate::Reg<auxhfrcoctrl::AuxhfrcoctrlSpec>;
#[doc = "AUXHFRCO Control Register"]
pub mod auxhfrcoctrl;
#[doc = "CALCTRL (rw) register accessor: Calibration Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`calctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calctrl`] module"]
#[doc(alias = "CALCTRL")]
pub type Calctrl = crate::Reg<calctrl::CalctrlSpec>;
#[doc = "Calibration Control Register"]
pub mod calctrl;
#[doc = "CALCNT (rw) register accessor: Calibration Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`calcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calcnt`] module"]
#[doc(alias = "CALCNT")]
pub type Calcnt = crate::Reg<calcnt::CalcntSpec>;
#[doc = "Calibration Counter Register"]
pub mod calcnt;
#[doc = "OSCENCMD (w) register accessor: Oscillator Enable/Disable Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oscencmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscencmd`] module"]
#[doc(alias = "OSCENCMD")]
pub type Oscencmd = crate::Reg<oscencmd::OscencmdSpec>;
#[doc = "Oscillator Enable/Disable Command Register"]
pub mod oscencmd;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "LFCLKSEL (rw) register accessor: Low Frequency Clock Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclksel`] module"]
#[doc(alias = "LFCLKSEL")]
pub type Lfclksel = crate::Reg<lfclksel::LfclkselSpec>;
#[doc = "Low Frequency Clock Select Register"]
pub mod lfclksel;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
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
#[doc = "HFCORECLKEN0 (rw) register accessor: High Frequency Core Clock Enable Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcoreclken0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcoreclken0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfcoreclken0`] module"]
#[doc(alias = "HFCORECLKEN0")]
pub type Hfcoreclken0 = crate::Reg<hfcoreclken0::Hfcoreclken0Spec>;
#[doc = "High Frequency Core Clock Enable Register 0"]
pub mod hfcoreclken0;
#[doc = "HFPERCLKEN0 (rw) register accessor: High Frequency Peripheral Clock Enable Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hfperclken0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfperclken0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfperclken0`] module"]
#[doc(alias = "HFPERCLKEN0")]
pub type Hfperclken0 = crate::Reg<hfperclken0::Hfperclken0Spec>;
#[doc = "High Frequency Peripheral Clock Enable Register 0"]
pub mod hfperclken0;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "FREEZE (rw) register accessor: Freeze Register\n\nYou can [`read`](crate::Reg::read) this register and get [`freeze::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freeze::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freeze`] module"]
#[doc(alias = "FREEZE")]
pub type Freeze = crate::Reg<freeze::FreezeSpec>;
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "LFACLKEN0 (rw) register accessor: Low Frequency A Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::Reg::read) this register and get [`lfaclken0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfaclken0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfaclken0`] module"]
#[doc(alias = "LFACLKEN0")]
pub type Lfaclken0 = crate::Reg<lfaclken0::Lfaclken0Spec>;
#[doc = "Low Frequency A Clock Enable Register 0 (Async Reg)"]
pub mod lfaclken0;
#[doc = "LFBCLKEN0 (rw) register accessor: Low Frequency B Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::Reg::read) this register and get [`lfbclken0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfbclken0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfbclken0`] module"]
#[doc(alias = "LFBCLKEN0")]
pub type Lfbclken0 = crate::Reg<lfbclken0::Lfbclken0Spec>;
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)"]
pub mod lfbclken0;
#[doc = "LFCCLKEN0 (rw) register accessor: Low Frequency C Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::Reg::read) this register and get [`lfcclken0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfcclken0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfcclken0`] module"]
#[doc(alias = "LFCCLKEN0")]
pub type Lfcclken0 = crate::Reg<lfcclken0::Lfcclken0Spec>;
#[doc = "Low Frequency C Clock Enable Register 0 (Async Reg)"]
pub mod lfcclken0;
#[doc = "LFAPRESC0 (rw) register accessor: Low Frequency A Prescaler Register 0 (Async Reg)\n\nYou can [`read`](crate::Reg::read) this register and get [`lfapresc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfapresc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfapresc0`] module"]
#[doc(alias = "LFAPRESC0")]
pub type Lfapresc0 = crate::Reg<lfapresc0::Lfapresc0Spec>;
#[doc = "Low Frequency A Prescaler Register 0 (Async Reg)"]
pub mod lfapresc0;
#[doc = "LFBPRESC0 (rw) register accessor: Low Frequency B Prescaler Register 0 (Async Reg)\n\nYou can [`read`](crate::Reg::read) this register and get [`lfbpresc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfbpresc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfbpresc0`] module"]
#[doc(alias = "LFBPRESC0")]
pub type Lfbpresc0 = crate::Reg<lfbpresc0::Lfbpresc0Spec>;
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)"]
pub mod lfbpresc0;
#[doc = "PCNTCTRL (rw) register accessor: PCNT Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcntctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcntctrl`] module"]
#[doc(alias = "PCNTCTRL")]
pub type Pcntctrl = crate::Reg<pcntctrl::PcntctrlSpec>;
#[doc = "PCNT Control Register"]
pub mod pcntctrl;
#[doc = "ROUTE (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@route`] module"]
#[doc(alias = "ROUTE")]
pub type Route = crate::Reg<route::RouteSpec>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "LOCK (rw) register accessor: Configuration Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "USBCRCTRL (rw) register accessor: USB Clock Recovery Control\n\nYou can [`read`](crate::Reg::read) this register and get [`usbcrctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbcrctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbcrctrl`] module"]
#[doc(alias = "USBCRCTRL")]
pub type Usbcrctrl = crate::Reg<usbcrctrl::UsbcrctrlSpec>;
#[doc = "USB Clock Recovery Control"]
pub mod usbcrctrl;
#[doc = "USHFRCOCTRL (rw) register accessor: USHFRCO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ushfrcoctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ushfrcoctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ushfrcoctrl`] module"]
#[doc(alias = "USHFRCOCTRL")]
pub type Ushfrcoctrl = crate::Reg<ushfrcoctrl::UshfrcoctrlSpec>;
#[doc = "USHFRCO Control"]
pub mod ushfrcoctrl;
#[doc = "USHFRCOTUNE (rw) register accessor: USHFRCO Frequency Tune\n\nYou can [`read`](crate::Reg::read) this register and get [`ushfrcotune::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ushfrcotune::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ushfrcotune`] module"]
#[doc(alias = "USHFRCOTUNE")]
pub type Ushfrcotune = crate::Reg<ushfrcotune::UshfrcotuneSpec>;
#[doc = "USHFRCO Frequency Tune"]
pub mod ushfrcotune;
#[doc = "USHFRCOCONF (rw) register accessor: USHFRCO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ushfrcoconf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ushfrcoconf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ushfrcoconf`] module"]
#[doc(alias = "USHFRCOCONF")]
pub type Ushfrcoconf = crate::Reg<ushfrcoconf::UshfrcoconfSpec>;
#[doc = "USHFRCO Configuration"]
pub mod ushfrcoconf;
