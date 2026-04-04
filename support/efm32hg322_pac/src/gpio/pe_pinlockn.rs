#[doc = "Register `PE_PINLOCKN` reader"]
pub type R = crate::R<PePinlocknSpec>;
#[doc = "Register `PE_PINLOCKN` writer"]
pub type W = crate::W<PePinlocknSpec>;
#[doc = "Field `PINLOCKN` reader - Unlocked Pins"]
pub type PinlocknR = crate::FieldReader<u16>;
#[doc = "Field `PINLOCKN` writer - Unlocked Pins"]
pub type PinlocknW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Unlocked Pins"]
    #[inline(always)]
    pub fn pinlockn(&self) -> PinlocknR {
        PinlocknR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Unlocked Pins"]
    #[inline(always)]
    pub fn pinlockn(&mut self) -> PinlocknW<'_, PePinlocknSpec> {
        PinlocknW::new(self, 0)
    }
}
#[doc = "Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_pinlockn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_pinlockn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PePinlocknSpec;
impl crate::RegisterSpec for PePinlocknSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_pinlockn::R`](R) reader structure"]
impl crate::Readable for PePinlocknSpec {}
#[doc = "`write(|w| ..)` method takes [`pe_pinlockn::W`](W) writer structure"]
impl crate::Writable for PePinlocknSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PE_PINLOCKN to value 0xffff"]
impl crate::Resettable for PePinlocknSpec {
    const RESET_VALUE: u32 = 0xffff;
}
