#[doc = "Register `GRXSTSP` reader"]
pub type R = crate::R<GrxstspSpec>;
#[doc = "Field `CHEPNUM` reader - Channel Number host only / Endpoint Number"]
pub type ChepnumR = crate::FieldReader;
#[doc = "Field `BCNT` reader - Byte Count (host or device)"]
pub type BcntR = crate::FieldReader<u16>;
#[doc = "Data PID (host or device)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dpid {
    #[doc = "0: DATA0 PID."]
    Data0 = 0,
    #[doc = "1: DATA1 PID."]
    Data1 = 1,
    #[doc = "2: DATA2 PID."]
    Data2 = 2,
    #[doc = "3: MDATA PID."]
    Mdata = 3,
}
impl From<Dpid> for u8 {
    #[inline(always)]
    fn from(variant: Dpid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dpid {
    type Ux = u8;
}
impl crate::IsEnum for Dpid {}
#[doc = "Field `DPID` reader - Data PID (host or device)"]
pub type DpidR = crate::FieldReader<Dpid>;
impl DpidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpid {
        match self.bits {
            0 => Dpid::Data0,
            1 => Dpid::Data1,
            2 => Dpid::Data2,
            3 => Dpid::Mdata,
            _ => unreachable!(),
        }
    }
    #[doc = "DATA0 PID."]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == Dpid::Data0
    }
    #[doc = "DATA1 PID."]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == Dpid::Data1
    }
    #[doc = "DATA2 PID."]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == Dpid::Data2
    }
    #[doc = "MDATA PID."]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        *self == Dpid::Mdata
    }
}
#[doc = "Packet Status (host or device)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pktsts {
    #[doc = "1: Device mode: Global OUT NAK (triggers an interrupt)."]
    Goutnak = 1,
    #[doc = "2: Host mode: IN data packet received. Device mode: OUT data packet received."]
    Pktrcv = 2,
    #[doc = "3: Host mode: IN transfer completed (triggers an interrupt). Device mode: OUT transfer completed (triggers an interrupt)."]
    Xfercompl = 3,
    #[doc = "4: Device mode: SETUP transaction completed (triggers an interrupt)."]
    Setupcompl = 4,
    #[doc = "5: Host mode: Data toggle error (triggers an interrupt)."]
    Tglerr = 5,
    #[doc = "6: Device mode: SETUP data packet received."]
    Setuprcv = 6,
    #[doc = "7: Host mode: Channel halted (triggers an interrupt)."]
    Chlt = 7,
}
impl From<Pktsts> for u8 {
    #[inline(always)]
    fn from(variant: Pktsts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pktsts {
    type Ux = u8;
}
impl crate::IsEnum for Pktsts {}
#[doc = "Field `PKTSTS` reader - Packet Status (host or device)"]
pub type PktstsR = crate::FieldReader<Pktsts>;
impl PktstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pktsts> {
        match self.bits {
            1 => Some(Pktsts::Goutnak),
            2 => Some(Pktsts::Pktrcv),
            3 => Some(Pktsts::Xfercompl),
            4 => Some(Pktsts::Setupcompl),
            5 => Some(Pktsts::Tglerr),
            6 => Some(Pktsts::Setuprcv),
            7 => Some(Pktsts::Chlt),
            _ => None,
        }
    }
    #[doc = "Device mode: Global OUT NAK (triggers an interrupt)."]
    #[inline(always)]
    pub fn is_goutnak(&self) -> bool {
        *self == Pktsts::Goutnak
    }
    #[doc = "Host mode: IN data packet received. Device mode: OUT data packet received."]
    #[inline(always)]
    pub fn is_pktrcv(&self) -> bool {
        *self == Pktsts::Pktrcv
    }
    #[doc = "Host mode: IN transfer completed (triggers an interrupt). Device mode: OUT transfer completed (triggers an interrupt)."]
    #[inline(always)]
    pub fn is_xfercompl(&self) -> bool {
        *self == Pktsts::Xfercompl
    }
    #[doc = "Device mode: SETUP transaction completed (triggers an interrupt)."]
    #[inline(always)]
    pub fn is_setupcompl(&self) -> bool {
        *self == Pktsts::Setupcompl
    }
    #[doc = "Host mode: Data toggle error (triggers an interrupt)."]
    #[inline(always)]
    pub fn is_tglerr(&self) -> bool {
        *self == Pktsts::Tglerr
    }
    #[doc = "Device mode: SETUP data packet received."]
    #[inline(always)]
    pub fn is_setuprcv(&self) -> bool {
        *self == Pktsts::Setuprcv
    }
    #[doc = "Host mode: Channel halted (triggers an interrupt)."]
    #[inline(always)]
    pub fn is_chlt(&self) -> bool {
        *self == Pktsts::Chlt
    }
}
#[doc = "Field `FN` reader - Frame Number"]
pub type FnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Channel Number host only / Endpoint Number"]
    #[inline(always)]
    pub fn chepnum(&self) -> ChepnumR {
        ChepnumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte Count (host or device)"]
    #[inline(always)]
    pub fn bcnt(&self) -> BcntR {
        BcntR::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID (host or device)"]
    #[inline(always)]
    pub fn dpid(&self) -> DpidR {
        DpidR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - Packet Status (host or device)"]
    #[inline(always)]
    pub fn pktsts(&self) -> PktstsR {
        PktstsR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:24 - Frame Number"]
    #[inline(always)]
    pub fn fn_(&self) -> FnR {
        FnR::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
#[doc = "Receive Status Read and Pop Register\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrxstspSpec;
impl crate::RegisterSpec for GrxstspSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxstsp::R`](R) reader structure"]
impl crate::Readable for GrxstspSpec {}
#[doc = "`reset()` method sets GRXSTSP to value 0"]
impl crate::Resettable for GrxstspSpec {}
