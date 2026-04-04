#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EN` reader - RTC Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - RTC Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DebugrunR = crate::BitReader;
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DebugrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP0TOP` reader - Compare Channel 0 is Top Value"]
pub type Comp0topR = crate::BitReader;
#[doc = "Field `COMP0TOP` writer - Compare Channel 0 is Top Value"]
pub type Comp0topW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RTC Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DebugrunR {
        DebugrunR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare Channel 0 is Top Value"]
    #[inline(always)]
    pub fn comp0top(&self) -> Comp0topR {
        Comp0topR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DebugrunW<'_, CtrlSpec> {
        DebugrunW::new(self, 1)
    }
    #[doc = "Bit 2 - Compare Channel 0 is Top Value"]
    #[inline(always)]
    pub fn comp0top(&mut self) -> Comp0topW<'_, CtrlSpec> {
        Comp0topW::new(self, 2)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
