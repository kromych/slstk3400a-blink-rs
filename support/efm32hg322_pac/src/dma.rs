#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    status: Status,
    config: Config,
    ctrlbase: Ctrlbase,
    altctrlbase: Altctrlbase,
    chwaitstatus: Chwaitstatus,
    chswreq: Chswreq,
    chusebursts: Chusebursts,
    chuseburstc: Chuseburstc,
    chreqmasks: Chreqmasks,
    chreqmaskc: Chreqmaskc,
    chens: Chens,
    chenc: Chenc,
    chalts: Chalts,
    chaltc: Chaltc,
    chpris: Chpris,
    chpric: Chpric,
    _reserved16: [u8; 0x0c],
    errorc: Errorc,
    _reserved17: [u8; 0x0dc0],
    chreqstatus: Chreqstatus,
    _reserved18: [u8; 0x04],
    chsreqstatus: Chsreqstatus,
    _reserved19: [u8; 0x01e4],
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    _reserved23: [u8; 0xf0],
    ch0_ctrl: Ch0Ctrl,
    ch1_ctrl: Ch1Ctrl,
    ch2_ctrl: Ch2Ctrl,
    ch3_ctrl: Ch3Ctrl,
    ch4_ctrl: Ch4Ctrl,
    ch5_ctrl: Ch5Ctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Status Registers"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x04 - DMA Configuration Register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x08 - Channel Control Data Base Pointer Register"]
    #[inline(always)]
    pub const fn ctrlbase(&self) -> &Ctrlbase {
        &self.ctrlbase
    }
    #[doc = "0x0c - Channel Alternate Control Data Base Pointer Register"]
    #[inline(always)]
    pub const fn altctrlbase(&self) -> &Altctrlbase {
        &self.altctrlbase
    }
    #[doc = "0x10 - Channel Wait on Request Status Register"]
    #[inline(always)]
    pub const fn chwaitstatus(&self) -> &Chwaitstatus {
        &self.chwaitstatus
    }
    #[doc = "0x14 - Channel Software Request Register"]
    #[inline(always)]
    pub const fn chswreq(&self) -> &Chswreq {
        &self.chswreq
    }
    #[doc = "0x18 - Channel Useburst Set Register"]
    #[inline(always)]
    pub const fn chusebursts(&self) -> &Chusebursts {
        &self.chusebursts
    }
    #[doc = "0x1c - Channel Useburst Clear Register"]
    #[inline(always)]
    pub const fn chuseburstc(&self) -> &Chuseburstc {
        &self.chuseburstc
    }
    #[doc = "0x20 - Channel Request Mask Set Register"]
    #[inline(always)]
    pub const fn chreqmasks(&self) -> &Chreqmasks {
        &self.chreqmasks
    }
    #[doc = "0x24 - Channel Request Mask Clear Register"]
    #[inline(always)]
    pub const fn chreqmaskc(&self) -> &Chreqmaskc {
        &self.chreqmaskc
    }
    #[doc = "0x28 - Channel Enable Set Register"]
    #[inline(always)]
    pub const fn chens(&self) -> &Chens {
        &self.chens
    }
    #[doc = "0x2c - Channel Enable Clear Register"]
    #[inline(always)]
    pub const fn chenc(&self) -> &Chenc {
        &self.chenc
    }
    #[doc = "0x30 - Channel Alternate Set Register"]
    #[inline(always)]
    pub const fn chalts(&self) -> &Chalts {
        &self.chalts
    }
    #[doc = "0x34 - Channel Alternate Clear Register"]
    #[inline(always)]
    pub const fn chaltc(&self) -> &Chaltc {
        &self.chaltc
    }
    #[doc = "0x38 - Channel Priority Set Register"]
    #[inline(always)]
    pub const fn chpris(&self) -> &Chpris {
        &self.chpris
    }
    #[doc = "0x3c - Channel Priority Clear Register"]
    #[inline(always)]
    pub const fn chpric(&self) -> &Chpric {
        &self.chpric
    }
    #[doc = "0x4c - Bus Error Clear Register"]
    #[inline(always)]
    pub const fn errorc(&self) -> &Errorc {
        &self.errorc
    }
    #[doc = "0xe10 - Channel Request Status"]
    #[inline(always)]
    pub const fn chreqstatus(&self) -> &Chreqstatus {
        &self.chreqstatus
    }
    #[doc = "0xe18 - Channel Single Request Status"]
    #[inline(always)]
    pub const fn chsreqstatus(&self) -> &Chsreqstatus {
        &self.chsreqstatus
    }
    #[doc = "0x1000 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x1004 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x1008 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x100c - Interrupt Enable register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x1100 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch0_ctrl(&self) -> &Ch0Ctrl {
        &self.ch0_ctrl
    }
    #[doc = "0x1104 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch1_ctrl(&self) -> &Ch1Ctrl {
        &self.ch1_ctrl
    }
    #[doc = "0x1108 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch2_ctrl(&self) -> &Ch2Ctrl {
        &self.ch2_ctrl
    }
    #[doc = "0x110c - Channel Control Register"]
    #[inline(always)]
    pub const fn ch3_ctrl(&self) -> &Ch3Ctrl {
        &self.ch3_ctrl
    }
    #[doc = "0x1110 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch4_ctrl(&self) -> &Ch4Ctrl {
        &self.ch4_ctrl
    }
    #[doc = "0x1114 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch5_ctrl(&self) -> &Ch5Ctrl {
        &self.ch5_ctrl
    }
}
#[doc = "STATUS (r) register accessor: DMA Status Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "DMA Status Registers"]
pub mod status;
#[doc = "CONFIG (w) register accessor: DMA Configuration Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "DMA Configuration Register"]
pub mod config;
#[doc = "CTRLBASE (rw) register accessor: Channel Control Data Base Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlbase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlbase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlbase`] module"]
#[doc(alias = "CTRLBASE")]
pub type Ctrlbase = crate::Reg<ctrlbase::CtrlbaseSpec>;
#[doc = "Channel Control Data Base Pointer Register"]
pub mod ctrlbase;
#[doc = "ALTCTRLBASE (r) register accessor: Channel Alternate Control Data Base Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`altctrlbase::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@altctrlbase`] module"]
#[doc(alias = "ALTCTRLBASE")]
pub type Altctrlbase = crate::Reg<altctrlbase::AltctrlbaseSpec>;
#[doc = "Channel Alternate Control Data Base Pointer Register"]
pub mod altctrlbase;
#[doc = "CHWAITSTATUS (r) register accessor: Channel Wait on Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chwaitstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chwaitstatus`] module"]
#[doc(alias = "CHWAITSTATUS")]
pub type Chwaitstatus = crate::Reg<chwaitstatus::ChwaitstatusSpec>;
#[doc = "Channel Wait on Request Status Register"]
pub mod chwaitstatus;
#[doc = "CHSWREQ (w) register accessor: Channel Software Request Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chswreq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chswreq`] module"]
#[doc(alias = "CHSWREQ")]
pub type Chswreq = crate::Reg<chswreq::ChswreqSpec>;
#[doc = "Channel Software Request Register"]
pub mod chswreq;
#[doc = "CHUSEBURSTS (rw) register accessor: Channel Useburst Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chusebursts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chusebursts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chusebursts`] module"]
#[doc(alias = "CHUSEBURSTS")]
pub type Chusebursts = crate::Reg<chusebursts::ChuseburstsSpec>;
#[doc = "Channel Useburst Set Register"]
pub mod chusebursts;
#[doc = "CHUSEBURSTC (w) register accessor: Channel Useburst Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chuseburstc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chuseburstc`] module"]
#[doc(alias = "CHUSEBURSTC")]
pub type Chuseburstc = crate::Reg<chuseburstc::ChuseburstcSpec>;
#[doc = "Channel Useburst Clear Register"]
pub mod chuseburstc;
#[doc = "CHREQMASKS (w) register accessor: Channel Request Mask Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chreqmasks::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chreqmasks`] module"]
#[doc(alias = "CHREQMASKS")]
pub type Chreqmasks = crate::Reg<chreqmasks::ChreqmasksSpec>;
#[doc = "Channel Request Mask Set Register"]
pub mod chreqmasks;
#[doc = "CHREQMASKC (w) register accessor: Channel Request Mask Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chreqmaskc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chreqmaskc`] module"]
#[doc(alias = "CHREQMASKC")]
pub type Chreqmaskc = crate::Reg<chreqmaskc::ChreqmaskcSpec>;
#[doc = "Channel Request Mask Clear Register"]
pub mod chreqmaskc;
#[doc = "CHENS (w) register accessor: Channel Enable Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chens::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chens`] module"]
#[doc(alias = "CHENS")]
pub type Chens = crate::Reg<chens::ChensSpec>;
#[doc = "Channel Enable Set Register"]
pub mod chens;
#[doc = "CHENC (w) register accessor: Channel Enable Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chenc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chenc`] module"]
#[doc(alias = "CHENC")]
pub type Chenc = crate::Reg<chenc::ChencSpec>;
#[doc = "Channel Enable Clear Register"]
pub mod chenc;
#[doc = "CHALTS (w) register accessor: Channel Alternate Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chalts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chalts`] module"]
#[doc(alias = "CHALTS")]
pub type Chalts = crate::Reg<chalts::ChaltsSpec>;
#[doc = "Channel Alternate Set Register"]
pub mod chalts;
#[doc = "CHALTC (w) register accessor: Channel Alternate Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chaltc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chaltc`] module"]
#[doc(alias = "CHALTC")]
pub type Chaltc = crate::Reg<chaltc::ChaltcSpec>;
#[doc = "Channel Alternate Clear Register"]
pub mod chaltc;
#[doc = "CHPRIS (w) register accessor: Channel Priority Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chpris::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chpris`] module"]
#[doc(alias = "CHPRIS")]
pub type Chpris = crate::Reg<chpris::ChprisSpec>;
#[doc = "Channel Priority Set Register"]
pub mod chpris;
#[doc = "CHPRIC (w) register accessor: Channel Priority Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chpric::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chpric`] module"]
#[doc(alias = "CHPRIC")]
pub type Chpric = crate::Reg<chpric::ChpricSpec>;
#[doc = "Channel Priority Clear Register"]
pub mod chpric;
#[doc = "ERRORC (rw) register accessor: Bus Error Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errorc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errorc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errorc`] module"]
#[doc(alias = "ERRORC")]
pub type Errorc = crate::Reg<errorc::ErrorcSpec>;
#[doc = "Bus Error Clear Register"]
pub mod errorc;
#[doc = "CHREQSTATUS (r) register accessor: Channel Request Status\n\nYou can [`read`](crate::Reg::read) this register and get [`chreqstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chreqstatus`] module"]
#[doc(alias = "CHREQSTATUS")]
pub type Chreqstatus = crate::Reg<chreqstatus::ChreqstatusSpec>;
#[doc = "Channel Request Status"]
pub mod chreqstatus;
#[doc = "CHSREQSTATUS (r) register accessor: Channel Single Request Status\n\nYou can [`read`](crate::Reg::read) this register and get [`chsreqstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsreqstatus`] module"]
#[doc(alias = "CHSREQSTATUS")]
pub type Chsreqstatus = crate::Reg<chsreqstatus::ChsreqstatusSpec>;
#[doc = "Channel Single Request Status"]
pub mod chsreqstatus;
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
#[doc = "IEN (rw) register accessor: Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "Interrupt Enable register"]
pub mod ien;
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
