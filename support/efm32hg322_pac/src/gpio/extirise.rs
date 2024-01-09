#[doc = "Register `EXTIRISE` reader"]
pub type R = crate::R<EXTIRISE_SPEC>;
#[doc = "Register `EXTIRISE` writer"]
pub type W = crate::W<EXTIRISE_SPEC>;
#[doc = "Field `EXTIRISE` reader - External Interrupt n Rising Edge Trigger Enable"]
pub type EXTIRISE_R = crate::FieldReader<u16>;
#[doc = "Field `EXTIRISE` writer - External Interrupt n Rising Edge Trigger Enable"]
pub type EXTIRISE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - External Interrupt n Rising Edge Trigger Enable"]
    #[inline(always)]
    pub fn extirise(&self) -> EXTIRISE_R {
        EXTIRISE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - External Interrupt n Rising Edge Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extirise(&mut self) -> EXTIRISE_W<EXTIRISE_SPEC> {
        EXTIRISE_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "External Interrupt Rising Edge Trigger Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extirise::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extirise::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTIRISE_SPEC;
impl crate::RegisterSpec for EXTIRISE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extirise::R`](R) reader structure"]
impl crate::Readable for EXTIRISE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extirise::W`](W) writer structure"]
impl crate::Writable for EXTIRISE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTIRISE to value 0"]
impl crate::Resettable for EXTIRISE_SPEC {
    const RESET_VALUE: u32 = 0;
}
