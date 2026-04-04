#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `BUSFAULT` reader - Bus Fault Response Enable"]
pub type BusfaultR = crate::BitReader;
#[doc = "Field `BUSFAULT` writer - Bus Fault Response Enable"]
pub type BusfaultW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bus Fault Response Enable"]
    #[inline(always)]
    pub fn busfault(&self) -> BusfaultR {
        BusfaultR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bus Fault Response Enable"]
    #[inline(always)]
    pub fn busfault(&mut self) -> BusfaultW<'_, CtrlSpec> {
        BusfaultW::new(self, 0)
    }
}
#[doc = "Memory System Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0x01"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
