#[doc = "Register `AUXCTRL` reader"]
pub type R = crate::R<AuxctrlSpec>;
#[doc = "Register `AUXCTRL` writer"]
pub type W = crate::W<AuxctrlSpec>;
#[doc = "Field `HRCCLR` reader - Hard Reset Cause Clear"]
pub type HrcclrR = crate::BitReader;
#[doc = "Field `HRCCLR` writer - Hard Reset Cause Clear"]
pub type HrcclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Hard Reset Cause Clear"]
    #[inline(always)]
    pub fn hrcclr(&self) -> HrcclrR {
        HrcclrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hard Reset Cause Clear"]
    #[inline(always)]
    pub fn hrcclr(&mut self) -> HrcclrW<'_, AuxctrlSpec> {
        HrcclrW::new(self, 0)
    }
}
#[doc = "Auxiliary Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`auxctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`auxctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxctrlSpec;
impl crate::RegisterSpec for AuxctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auxctrl::R`](R) reader structure"]
impl crate::Readable for AuxctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`auxctrl::W`](W) writer structure"]
impl crate::Writable for AuxctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUXCTRL to value 0"]
impl crate::Resettable for AuxctrlSpec {}
