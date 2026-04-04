#[doc = "Register `INSENSE` reader"]
pub type R = crate::R<InsenseSpec>;
#[doc = "Register `INSENSE` writer"]
pub type W = crate::W<InsenseSpec>;
#[doc = "Field `INT` reader - Interrupt Sense Enable"]
pub type IntR = crate::BitReader;
#[doc = "Field `INT` writer - Interrupt Sense Enable"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRS` reader - PRS Sense Enable"]
pub type PrsR = crate::BitReader;
#[doc = "Field `PRS` writer - PRS Sense Enable"]
pub type PrsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt Sense Enable"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PRS Sense Enable"]
    #[inline(always)]
    pub fn prs(&self) -> PrsR {
        PrsR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Sense Enable"]
    #[inline(always)]
    pub fn int(&mut self) -> IntW<'_, InsenseSpec> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 1 - PRS Sense Enable"]
    #[inline(always)]
    pub fn prs(&mut self) -> PrsW<'_, InsenseSpec> {
        PrsW::new(self, 1)
    }
}
#[doc = "Input Sense Register\n\nYou can [`read`](crate::Reg::read) this register and get [`insense::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`insense::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InsenseSpec;
impl crate::RegisterSpec for InsenseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`insense::R`](R) reader structure"]
impl crate::Readable for InsenseSpec {}
#[doc = "`write(|w| ..)` method takes [`insense::W`](W) writer structure"]
impl crate::Writable for InsenseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INSENSE to value 0x03"]
impl crate::Resettable for InsenseSpec {
    const RESET_VALUE: u32 = 0x03;
}
