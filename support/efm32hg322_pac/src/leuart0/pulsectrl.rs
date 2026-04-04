#[doc = "Register `PULSECTRL` reader"]
pub type R = crate::R<PulsectrlSpec>;
#[doc = "Register `PULSECTRL` writer"]
pub type W = crate::W<PulsectrlSpec>;
#[doc = "Field `PULSEW` reader - Pulse Width"]
pub type PulsewR = crate::FieldReader;
#[doc = "Field `PULSEW` writer - Pulse Width"]
pub type PulsewW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PULSEEN` reader - Pulse Generator/Extender Enable"]
pub type PulseenR = crate::BitReader;
#[doc = "Field `PULSEEN` writer - Pulse Generator/Extender Enable"]
pub type PulseenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PULSEFILT` reader - Pulse Filter"]
pub type PulsefiltR = crate::BitReader;
#[doc = "Field `PULSEFILT` writer - Pulse Filter"]
pub type PulsefiltW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Pulse Width"]
    #[inline(always)]
    pub fn pulsew(&self) -> PulsewR {
        PulsewR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Pulse Generator/Extender Enable"]
    #[inline(always)]
    pub fn pulseen(&self) -> PulseenR {
        PulseenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pulse Filter"]
    #[inline(always)]
    pub fn pulsefilt(&self) -> PulsefiltR {
        PulsefiltR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pulse Width"]
    #[inline(always)]
    pub fn pulsew(&mut self) -> PulsewW<'_, PulsectrlSpec> {
        PulsewW::new(self, 0)
    }
    #[doc = "Bit 4 - Pulse Generator/Extender Enable"]
    #[inline(always)]
    pub fn pulseen(&mut self) -> PulseenW<'_, PulsectrlSpec> {
        PulseenW::new(self, 4)
    }
    #[doc = "Bit 5 - Pulse Filter"]
    #[inline(always)]
    pub fn pulsefilt(&mut self) -> PulsefiltW<'_, PulsectrlSpec> {
        PulsefiltW::new(self, 5)
    }
}
#[doc = "Pulse Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pulsectrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pulsectrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PulsectrlSpec;
impl crate::RegisterSpec for PulsectrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pulsectrl::R`](R) reader structure"]
impl crate::Readable for PulsectrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pulsectrl::W`](W) writer structure"]
impl crate::Writable for PulsectrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PULSECTRL to value 0"]
impl crate::Resettable for PulsectrlSpec {}
