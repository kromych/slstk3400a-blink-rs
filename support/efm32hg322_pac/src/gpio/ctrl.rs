#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EM4RET` reader - Enable EM4 retention"]
pub type Em4retR = crate::BitReader;
#[doc = "Field `EM4RET` writer - Enable EM4 retention"]
pub type Em4retW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable EM4 retention"]
    #[inline(always)]
    pub fn em4ret(&self) -> Em4retR {
        Em4retR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable EM4 retention"]
    #[inline(always)]
    pub fn em4ret(&mut self) -> Em4retW<'_, CtrlSpec> {
        Em4retW::new(self, 0)
    }
}
#[doc = "GPIO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
