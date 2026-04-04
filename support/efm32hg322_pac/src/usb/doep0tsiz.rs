#[doc = "Register `DOEP0TSIZ` reader"]
pub type R = crate::R<Doep0tsizSpec>;
#[doc = "Register `DOEP0TSIZ` writer"]
pub type W = crate::W<Doep0tsizSpec>;
#[doc = "Field `XFERSIZE` reader - Transfer Size"]
pub type XfersizeR = crate::FieldReader;
#[doc = "Field `XFERSIZE` writer - Transfer Size"]
pub type XfersizeW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PKTCNT` reader - Packet Count"]
pub type PktcntR = crate::BitReader;
#[doc = "Field `PKTCNT` writer - Packet Count"]
pub type PktcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPCNT` reader - SETUP Packet Count"]
pub type SupcntR = crate::FieldReader;
#[doc = "Field `SUPCNT` writer - SETUP Packet Count"]
pub type SupcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XfersizeR {
        XfersizeR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PktcntR {
        PktcntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30 - SETUP Packet Count"]
    #[inline(always)]
    pub fn supcnt(&self) -> SupcntR {
        SupcntR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&mut self) -> XfersizeW<'_, Doep0tsizSpec> {
        XfersizeW::new(self, 0)
    }
    #[doc = "Bit 19 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PktcntW<'_, Doep0tsizSpec> {
        PktcntW::new(self, 19)
    }
    #[doc = "Bits 29:30 - SETUP Packet Count"]
    #[inline(always)]
    pub fn supcnt(&mut self) -> SupcntW<'_, Doep0tsizSpec> {
        SupcntW::new(self, 29)
    }
}
#[doc = "Device OUT Endpoint 0 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0tsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0tsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doep0tsizSpec;
impl crate::RegisterSpec for Doep0tsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doep0tsiz::R`](R) reader structure"]
impl crate::Readable for Doep0tsizSpec {}
#[doc = "`write(|w| ..)` method takes [`doep0tsiz::W`](W) writer structure"]
impl crate::Writable for Doep0tsizSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEP0TSIZ to value 0"]
impl crate::Resettable for Doep0tsizSpec {}
