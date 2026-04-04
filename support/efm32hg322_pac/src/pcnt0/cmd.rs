#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `LCNTIM` writer - Load CNT Immediately"]
pub type LcntimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTOPBIM` writer - Load TOPB Immediately"]
pub type LtopbimW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Load CNT Immediately"]
    #[inline(always)]
    pub fn lcntim(&mut self) -> LcntimW<'_, CmdSpec> {
        LcntimW::new(self, 0)
    }
    #[doc = "Bit 1 - Load TOPB Immediately"]
    #[inline(always)]
    pub fn ltopbim(&mut self) -> LtopbimW<'_, CmdSpec> {
        LtopbimW::new(self, 1)
    }
}
#[doc = "Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {}
