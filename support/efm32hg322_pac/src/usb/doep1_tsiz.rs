#[doc = "Register `DOEP1_TSIZ` reader"]
pub type R = crate::R<DOEP1_TSIZ_SPEC>;
#[doc = "Register `DOEP1_TSIZ` writer"]
pub type W = crate::W<DOEP1_TSIZ_SPEC>;
#[doc = "Field `XFERSIZE` reader - Transfer Size"]
pub type XFERSIZE_R = crate::FieldReader<u32>;
#[doc = "Field `XFERSIZE` writer - Transfer Size"]
pub type XFERSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - Packet Count"]
pub type PKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Packet Count"]
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RXDPIDSUPCNT` reader - Receive Data PID / SETUP Packet Count"]
pub type RXDPIDSUPCNT_R = crate::FieldReader<RXDPIDSUPCNT_A>;
#[doc = "Receive Data PID / SETUP Packet Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXDPIDSUPCNT_A {
    #[doc = "0: DATA0 PID."]
    DATA0 = 0,
    #[doc = "1: DATA2 PID / 1 Packet."]
    DATA2 = 1,
    #[doc = "2: DATA1 PID / 2 Packets."]
    DATA1 = 2,
    #[doc = "3: MDATA PID / 3 Packets."]
    MDATA = 3,
}
impl From<RXDPIDSUPCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: RXDPIDSUPCNT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXDPIDSUPCNT_A {
    type Ux = u8;
}
impl RXDPIDSUPCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXDPIDSUPCNT_A {
        match self.bits {
            0 => RXDPIDSUPCNT_A::DATA0,
            1 => RXDPIDSUPCNT_A::DATA2,
            2 => RXDPIDSUPCNT_A::DATA1,
            3 => RXDPIDSUPCNT_A::MDATA,
            _ => unreachable!(),
        }
    }
    #[doc = "DATA0 PID."]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == RXDPIDSUPCNT_A::DATA0
    }
    #[doc = "DATA2 PID / 1 Packet."]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == RXDPIDSUPCNT_A::DATA2
    }
    #[doc = "DATA1 PID / 2 Packets."]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == RXDPIDSUPCNT_A::DATA1
    }
    #[doc = "MDATA PID / 3 Packets."]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        *self == RXDPIDSUPCNT_A::MDATA
    }
}
impl R {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Receive Data PID / SETUP Packet Count"]
    #[inline(always)]
    pub fn rxdpidsupcnt(&self) -> RXDPIDSUPCNT_R {
        RXDPIDSUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize(&mut self) -> XFERSIZE_W<DOEP1_TSIZ_SPEC> {
        XFERSIZE_W::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DOEP1_TSIZ_SPEC> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1_tsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1_tsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEP1_TSIZ_SPEC;
impl crate::RegisterSpec for DOEP1_TSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doep1_tsiz::R`](R) reader structure"]
impl crate::Readable for DOEP1_TSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doep1_tsiz::W`](W) writer structure"]
impl crate::Writable for DOEP1_TSIZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEP1_TSIZ to value 0"]
impl crate::Resettable for DOEP1_TSIZ_SPEC {
    const RESET_VALUE: u32 = 0;
}
