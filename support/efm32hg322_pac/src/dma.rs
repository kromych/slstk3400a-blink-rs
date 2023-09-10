#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Status Registers"]
    pub status: STATUS,
    #[doc = "0x04 - DMA Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x08 - Channel Control Data Base Pointer Register"]
    pub ctrlbase: CTRLBASE,
    #[doc = "0x0c - Channel Alternate Control Data Base Pointer Register"]
    pub altctrlbase: ALTCTRLBASE,
    #[doc = "0x10 - Channel Wait on Request Status Register"]
    pub chwaitstatus: CHWAITSTATUS,
    #[doc = "0x14 - Channel Software Request Register"]
    pub chswreq: CHSWREQ,
    #[doc = "0x18 - Channel Useburst Set Register"]
    pub chusebursts: CHUSEBURSTS,
    #[doc = "0x1c - Channel Useburst Clear Register"]
    pub chuseburstc: CHUSEBURSTC,
    #[doc = "0x20 - Channel Request Mask Set Register"]
    pub chreqmasks: CHREQMASKS,
    #[doc = "0x24 - Channel Request Mask Clear Register"]
    pub chreqmaskc: CHREQMASKC,
    #[doc = "0x28 - Channel Enable Set Register"]
    pub chens: CHENS,
    #[doc = "0x2c - Channel Enable Clear Register"]
    pub chenc: CHENC,
    #[doc = "0x30 - Channel Alternate Set Register"]
    pub chalts: CHALTS,
    #[doc = "0x34 - Channel Alternate Clear Register"]
    pub chaltc: CHALTC,
    #[doc = "0x38 - Channel Priority Set Register"]
    pub chpris: CHPRIS,
    #[doc = "0x3c - Channel Priority Clear Register"]
    pub chpric: CHPRIC,
    _reserved16: [u8; 0x0c],
    #[doc = "0x4c - Bus Error Clear Register"]
    pub errorc: ERRORC,
    _reserved17: [u8; 0x0dc0],
    #[doc = "0xe10 - Channel Request Status"]
    pub chreqstatus: CHREQSTATUS,
    _reserved18: [u8; 0x04],
    #[doc = "0xe18 - Channel Single Request Status"]
    pub chsreqstatus: CHSREQSTATUS,
    _reserved19: [u8; 0x01e4],
    #[doc = "0x1000 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x1004 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x1008 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x100c - Interrupt Enable register"]
    pub ien: IEN,
    _reserved23: [u8; 0xf0],
    #[doc = "0x1100 - Channel Control Register"]
    pub ch0_ctrl: CH0_CTRL,
    #[doc = "0x1104 - Channel Control Register"]
    pub ch1_ctrl: CH1_CTRL,
    #[doc = "0x1108 - Channel Control Register"]
    pub ch2_ctrl: CH2_CTRL,
    #[doc = "0x110c - Channel Control Register"]
    pub ch3_ctrl: CH3_CTRL,
    #[doc = "0x1110 - Channel Control Register"]
    pub ch4_ctrl: CH4_CTRL,
    #[doc = "0x1114 - Channel Control Register"]
    pub ch5_ctrl: CH5_CTRL,
}
#[doc = "STATUS (r) register accessor: DMA Status Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "DMA Status Registers"]
pub mod status;
#[doc = "CONFIG (w) register accessor: DMA Configuration Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`config`] module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "DMA Configuration Register"]
pub mod config;
#[doc = "CTRLBASE (rw) register accessor: Channel Control Data Base Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlbase::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlbase::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrlbase`] module"]
pub type CTRLBASE = crate::Reg<ctrlbase::CTRLBASE_SPEC>;
#[doc = "Channel Control Data Base Pointer Register"]
pub mod ctrlbase;
#[doc = "ALTCTRLBASE (r) register accessor: Channel Alternate Control Data Base Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`altctrlbase::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`altctrlbase`] module"]
pub type ALTCTRLBASE = crate::Reg<altctrlbase::ALTCTRLBASE_SPEC>;
#[doc = "Channel Alternate Control Data Base Pointer Register"]
pub mod altctrlbase;
#[doc = "CHWAITSTATUS (r) register accessor: Channel Wait on Request Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chwaitstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chwaitstatus`] module"]
pub type CHWAITSTATUS = crate::Reg<chwaitstatus::CHWAITSTATUS_SPEC>;
#[doc = "Channel Wait on Request Status Register"]
pub mod chwaitstatus;
#[doc = "CHSWREQ (w) register accessor: Channel Software Request Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chswreq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chswreq`] module"]
pub type CHSWREQ = crate::Reg<chswreq::CHSWREQ_SPEC>;
#[doc = "Channel Software Request Register"]
pub mod chswreq;
#[doc = "CHUSEBURSTS (rw) register accessor: Channel Useburst Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chusebursts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chusebursts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chusebursts`] module"]
pub type CHUSEBURSTS = crate::Reg<chusebursts::CHUSEBURSTS_SPEC>;
#[doc = "Channel Useburst Set Register"]
pub mod chusebursts;
#[doc = "CHUSEBURSTC (w) register accessor: Channel Useburst Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chuseburstc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chuseburstc`] module"]
pub type CHUSEBURSTC = crate::Reg<chuseburstc::CHUSEBURSTC_SPEC>;
#[doc = "Channel Useburst Clear Register"]
pub mod chuseburstc;
#[doc = "CHREQMASKS (w) register accessor: Channel Request Mask Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chreqmasks::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chreqmasks`] module"]
pub type CHREQMASKS = crate::Reg<chreqmasks::CHREQMASKS_SPEC>;
#[doc = "Channel Request Mask Set Register"]
pub mod chreqmasks;
#[doc = "CHREQMASKC (w) register accessor: Channel Request Mask Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chreqmaskc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chreqmaskc`] module"]
pub type CHREQMASKC = crate::Reg<chreqmaskc::CHREQMASKC_SPEC>;
#[doc = "Channel Request Mask Clear Register"]
pub mod chreqmaskc;
#[doc = "CHENS (w) register accessor: Channel Enable Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chens::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chens`] module"]
pub type CHENS = crate::Reg<chens::CHENS_SPEC>;
#[doc = "Channel Enable Set Register"]
pub mod chens;
#[doc = "CHENC (w) register accessor: Channel Enable Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chenc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chenc`] module"]
pub type CHENC = crate::Reg<chenc::CHENC_SPEC>;
#[doc = "Channel Enable Clear Register"]
pub mod chenc;
#[doc = "CHALTS (w) register accessor: Channel Alternate Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chalts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chalts`] module"]
pub type CHALTS = crate::Reg<chalts::CHALTS_SPEC>;
#[doc = "Channel Alternate Set Register"]
pub mod chalts;
#[doc = "CHALTC (w) register accessor: Channel Alternate Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chaltc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chaltc`] module"]
pub type CHALTC = crate::Reg<chaltc::CHALTC_SPEC>;
#[doc = "Channel Alternate Clear Register"]
pub mod chaltc;
#[doc = "CHPRIS (w) register accessor: Channel Priority Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpris::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chpris`] module"]
pub type CHPRIS = crate::Reg<chpris::CHPRIS_SPEC>;
#[doc = "Channel Priority Set Register"]
pub mod chpris;
#[doc = "CHPRIC (w) register accessor: Channel Priority Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpric::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chpric`] module"]
pub type CHPRIC = crate::Reg<chpric::CHPRIC_SPEC>;
#[doc = "Channel Priority Clear Register"]
pub mod chpric;
#[doc = "ERRORC (rw) register accessor: Bus Error Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errorc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`errorc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`errorc`] module"]
pub type ERRORC = crate::Reg<errorc::ERRORC_SPEC>;
#[doc = "Bus Error Clear Register"]
pub mod errorc;
#[doc = "CHREQSTATUS (r) register accessor: Channel Request Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chreqstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chreqstatus`] module"]
pub type CHREQSTATUS = crate::Reg<chreqstatus::CHREQSTATUS_SPEC>;
#[doc = "Channel Request Status"]
pub mod chreqstatus;
#[doc = "CHSREQSTATUS (r) register accessor: Channel Single Request Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chsreqstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chsreqstatus`] module"]
pub type CHSREQSTATUS = crate::Reg<chsreqstatus::CHSREQSTATUS_SPEC>;
#[doc = "Channel Single Request Status"]
pub mod chsreqstatus;
#[doc = "IF (r) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`if_`] module"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS (w) register accessor: Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ifs`] module"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ifc`] module"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN (rw) register accessor: Interrupt Enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ien`] module"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable register"]
pub mod ien;
#[doc = "CH0_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch0_ctrl`] module"]
pub type CH0_CTRL = crate::Reg<ch0_ctrl::CH0_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch0_ctrl;
#[doc = "CH1_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch1_ctrl`] module"]
pub type CH1_CTRL = crate::Reg<ch1_ctrl::CH1_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch1_ctrl;
#[doc = "CH2_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch2_ctrl`] module"]
pub type CH2_CTRL = crate::Reg<ch2_ctrl::CH2_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch2_ctrl;
#[doc = "CH3_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch3_ctrl`] module"]
pub type CH3_CTRL = crate::Reg<ch3_ctrl::CH3_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch3_ctrl;
#[doc = "CH4_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch4_ctrl`] module"]
pub type CH4_CTRL = crate::Reg<ch4_ctrl::CH4_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch4_ctrl;
#[doc = "CH5_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch5_ctrl`] module"]
pub type CH5_CTRL = crate::Reg<ch5_ctrl::CH5_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch5_ctrl;
