#[doc = "Register `DUTYCONFIG` reader"]
pub type R = crate::R<DutyconfigSpec>;
#[doc = "Register `DUTYCONFIG` writer"]
pub type W = crate::W<DutyconfigSpec>;
#[doc = "Field `DUTYCYCLEEN` reader - Duty Cycle Enable."]
pub type DutycycleenR = crate::BitReader;
#[doc = "Field `DUTYCYCLEEN` writer - Duty Cycle Enable."]
pub type DutycycleenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM2DUTYCYCLEDIS` reader - EM2/EM3 Duty Cycle Disable."]
pub type Em2dutycycledisR = crate::BitReader;
#[doc = "Field `EM2DUTYCYCLEDIS` writer - EM2/EM3 Duty Cycle Disable."]
pub type Em2dutycycledisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Duty Cycle Enable."]
    #[inline(always)]
    pub fn dutycycleen(&self) -> DutycycleenR {
        DutycycleenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EM2/EM3 Duty Cycle Disable."]
    #[inline(always)]
    pub fn em2dutycycledis(&self) -> Em2dutycycledisR {
        Em2dutycycledisR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Duty Cycle Enable."]
    #[inline(always)]
    pub fn dutycycleen(&mut self) -> DutycycleenW<'_, DutyconfigSpec> {
        DutycycleenW::new(self, 0)
    }
    #[doc = "Bit 1 - EM2/EM3 Duty Cycle Disable."]
    #[inline(always)]
    pub fn em2dutycycledis(&mut self) -> Em2dutycycledisW<'_, DutyconfigSpec> {
        Em2dutycycledisW::new(self, 1)
    }
}
#[doc = "Duty Cycle Configauration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dutyconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dutyconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DutyconfigSpec;
impl crate::RegisterSpec for DutyconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dutyconfig::R`](R) reader structure"]
impl crate::Readable for DutyconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`dutyconfig::W`](W) writer structure"]
impl crate::Writable for DutyconfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DUTYCONFIG to value 0"]
impl crate::Resettable for DutyconfigSpec {}
