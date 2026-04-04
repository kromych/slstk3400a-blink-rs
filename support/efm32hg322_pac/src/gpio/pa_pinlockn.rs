#[doc = "Register `PA_PINLOCKN` reader"]
pub type R = crate::R<PaPinlocknSpec>;
#[doc = "Register `PA_PINLOCKN` writer"]
pub type W = crate::W<PaPinlocknSpec>;
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
    pub fn pinlockn(&mut self) -> PinlocknW<'_, PaPinlocknSpec> {
        PinlocknW::new(self, 0)
    }
}
#[doc = "Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_pinlockn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_pinlockn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaPinlocknSpec;
impl crate::RegisterSpec for PaPinlocknSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_pinlockn::R`](R) reader structure"]
impl crate::Readable for PaPinlocknSpec {}
#[doc = "`write(|w| ..)` method takes [`pa_pinlockn::W`](W) writer structure"]
impl crate::Writable for PaPinlocknSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PA_PINLOCKN to value 0xffff"]
impl crate::Resettable for PaPinlocknSpec {
    const RESET_VALUE: u32 = 0xffff;
}
