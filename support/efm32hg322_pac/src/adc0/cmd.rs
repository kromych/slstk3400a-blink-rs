#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `SINGLESTART` writer - Single Conversion Start"]
pub type SinglestartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLESTOP` writer - Single Conversion Stop"]
pub type SinglestopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANSTART` writer - Scan Sequence Start"]
pub type ScanstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANSTOP` writer - Scan Sequence Stop"]
pub type ScanstopW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Single Conversion Start"]
    #[inline(always)]
    pub fn singlestart(&mut self) -> SinglestartW<'_, CmdSpec> {
        SinglestartW::new(self, 0)
    }
    #[doc = "Bit 1 - Single Conversion Stop"]
    #[inline(always)]
    pub fn singlestop(&mut self) -> SinglestopW<'_, CmdSpec> {
        SinglestopW::new(self, 1)
    }
    #[doc = "Bit 2 - Scan Sequence Start"]
    #[inline(always)]
    pub fn scanstart(&mut self) -> ScanstartW<'_, CmdSpec> {
        ScanstartW::new(self, 2)
    }
    #[doc = "Bit 3 - Scan Sequence Stop"]
    #[inline(always)]
    pub fn scanstop(&mut self) -> ScanstopW<'_, CmdSpec> {
        ScanstopW::new(self, 3)
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
