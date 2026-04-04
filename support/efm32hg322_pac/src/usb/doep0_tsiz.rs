#[doc = "Register `DOEP0_TSIZ` reader"]
pub type R = crate::R<Doep0TsizSpec>;
#[doc = "Register `DOEP0_TSIZ` writer"]
pub type W = crate::W<Doep0TsizSpec>;
#[doc = "Field `XFERSIZE` reader - Transfer Size"]
pub type XfersizeR = crate::FieldReader<u32>;
#[doc = "Field `XFERSIZE` writer - Transfer Size"]
pub type XfersizeW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - Packet Count"]
pub type PktcntR = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Packet Count"]
pub type PktcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Receive Data PID / SETUP Packet Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxdpidsupcnt {
    #[doc = "0: DATA0 PID."]
    Data0 = 0,
    #[doc = "1: DATA2 PID / 1 Packet."]
    Data2 = 1,
    #[doc = "2: DATA1 PID / 2 Packets."]
    Data1 = 2,
    #[doc = "3: MDATA PID / 3 Packets."]
    Mdata = 3,
}
impl From<Rxdpidsupcnt> for u8 {
    #[inline(always)]
    fn from(variant: Rxdpidsupcnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxdpidsupcnt {
    type Ux = u8;
}
impl crate::IsEnum for Rxdpidsupcnt {}
#[doc = "Field `RXDPIDSUPCNT` reader - Receive Data PID / SETUP Packet Count"]
pub type RxdpidsupcntR = crate::FieldReader<Rxdpidsupcnt>;
impl RxdpidsupcntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdpidsupcnt {
        match self.bits {
            0 => Rxdpidsupcnt::Data0,
            1 => Rxdpidsupcnt::Data2,
            2 => Rxdpidsupcnt::Data1,
            3 => Rxdpidsupcnt::Mdata,
            _ => unreachable!(),
        }
    }
    #[doc = "DATA0 PID."]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == Rxdpidsupcnt::Data0
    }
    #[doc = "DATA2 PID / 1 Packet."]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == Rxdpidsupcnt::Data2
    }
    #[doc = "DATA1 PID / 2 Packets."]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == Rxdpidsupcnt::Data1
    }
    #[doc = "MDATA PID / 3 Packets."]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        *self == Rxdpidsupcnt::Mdata
    }
}
impl R {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XfersizeR {
        XfersizeR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PktcntR {
        PktcntR::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Receive Data PID / SETUP Packet Count"]
    #[inline(always)]
    pub fn rxdpidsupcnt(&self) -> RxdpidsupcntR {
        RxdpidsupcntR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&mut self) -> XfersizeW<'_, Doep0TsizSpec> {
        XfersizeW::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PktcntW<'_, Doep0TsizSpec> {
        PktcntW::new(self, 19)
    }
}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0_tsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0_tsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doep0TsizSpec;
impl crate::RegisterSpec for Doep0TsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doep0_tsiz::R`](R) reader structure"]
impl crate::Readable for Doep0TsizSpec {}
#[doc = "`write(|w| ..)` method takes [`doep0_tsiz::W`](W) writer structure"]
impl crate::Writable for Doep0TsizSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEP0_TSIZ to value 0"]
impl crate::Resettable for Doep0TsizSpec {}
