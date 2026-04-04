#[doc = "Register `DIEP0TSIZ` reader"]
pub type R = crate::R<Diep0tsizSpec>;
#[doc = "Register `DIEP0TSIZ` writer"]
pub type W = crate::W<Diep0tsizSpec>;
#[doc = "Field `XFERSIZE` reader - Transfer Size"]
pub type XfersizeR = crate::FieldReader;
#[doc = "Field `XFERSIZE` writer - Transfer Size"]
pub type XfersizeW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PKTCNT` reader - Packet Count"]
pub type PktcntR = crate::FieldReader;
#[doc = "Field `PKTCNT` writer - Packet Count"]
pub type PktcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XfersizeR {
        XfersizeR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PktcntR {
        PktcntR::new(((self.bits >> 19) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&mut self) -> XfersizeW<'_, Diep0tsizSpec> {
        XfersizeW::new(self, 0)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PktcntW<'_, Diep0tsizSpec> {
        PktcntW::new(self, 19)
    }
}
#[doc = "Device IN Endpoint 0 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0tsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0tsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diep0tsizSpec;
impl crate::RegisterSpec for Diep0tsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep0tsiz::R`](R) reader structure"]
impl crate::Readable for Diep0tsizSpec {}
#[doc = "`write(|w| ..)` method takes [`diep0tsiz::W`](W) writer structure"]
impl crate::Writable for Diep0tsizSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEP0TSIZ to value 0"]
impl crate::Resettable for Diep0tsizSpec {}
