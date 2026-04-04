#[doc = "Register `DIEP2_TSIZ` reader"]
pub type R = crate::R<Diep2TsizSpec>;
#[doc = "Register `DIEP2_TSIZ` writer"]
pub type W = crate::W<Diep2TsizSpec>;
#[doc = "Field `XFERSIZE` reader - Transfer Size"]
pub type XfersizeR = crate::FieldReader<u32>;
#[doc = "Field `XFERSIZE` writer - Transfer Size"]
pub type XfersizeW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - Packet Count"]
pub type PktcntR = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Packet Count"]
pub type PktcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `MC` reader - Multi Count"]
pub type McR = crate::FieldReader;
#[doc = "Field `MC` writer - Multi Count"]
pub type McW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    #[doc = "Bits 29:30 - Multi Count"]
    #[inline(always)]
    pub fn mc(&self) -> McR {
        McR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&mut self) -> XfersizeW<'_, Diep2TsizSpec> {
        XfersizeW::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PktcntW<'_, Diep2TsizSpec> {
        PktcntW::new(self, 19)
    }
    #[doc = "Bits 29:30 - Multi Count"]
    #[inline(always)]
    pub fn mc(&mut self) -> McW<'_, Diep2TsizSpec> {
        McW::new(self, 29)
    }
}
#[doc = "Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep2_tsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep2_tsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diep2TsizSpec;
impl crate::RegisterSpec for Diep2TsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep2_tsiz::R`](R) reader structure"]
impl crate::Readable for Diep2TsizSpec {}
#[doc = "`write(|w| ..)` method takes [`diep2_tsiz::W`](W) writer structure"]
impl crate::Writable for Diep2TsizSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEP2_TSIZ to value 0"]
impl crate::Resettable for Diep2TsizSpec {}
