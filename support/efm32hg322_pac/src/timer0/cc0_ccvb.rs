#[doc = "Register `CC0_CCVB` reader"]
pub type R = crate::R<CC0_CCVB_SPEC>;
#[doc = "Register `CC0_CCVB` writer"]
pub type W = crate::W<CC0_CCVB_SPEC>;
#[doc = "Field `CCVB` reader - CC Channel Value Buffer"]
pub type CCVB_R = crate::FieldReader<u16>;
#[doc = "Field `CCVB` writer - CC Channel Value Buffer"]
pub type CCVB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CC Channel Value Buffer"]
    #[inline(always)]
    pub fn ccvb(&self) -> CCVB_R {
        CCVB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CC Channel Value Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ccvb(&mut self) -> CCVB_W<CC0_CCVB_SPEC> {
        CCVB_W::new(self, 0)
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
#[doc = "CC Channel Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc0_ccvb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc0_ccvb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC0_CCVB_SPEC;
impl crate::RegisterSpec for CC0_CCVB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc0_ccvb::R`](R) reader structure"]
impl crate::Readable for CC0_CCVB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cc0_ccvb::W`](W) writer structure"]
impl crate::Writable for CC0_CCVB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC0_CCVB to value 0"]
impl crate::Resettable for CC0_CCVB_SPEC {
    const RESET_VALUE: u32 = 0;
}
