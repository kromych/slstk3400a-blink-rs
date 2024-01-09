#[doc = "Register `WDATA` reader"]
pub type R = crate::R<WDATA_SPEC>;
#[doc = "Register `WDATA` writer"]
pub type W = crate::W<WDATA_SPEC>;
#[doc = "Field `WDATA` reader - Write Data"]
pub type WDATA_R = crate::FieldReader<u32>;
#[doc = "Field `WDATA` writer - Write Data"]
pub type WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write Data"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write Data"]
    #[inline(always)]
    #[must_use]
    pub fn wdata(&mut self) -> WDATA_W<WDATA_SPEC> {
        WDATA_W::new(self, 0)
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
#[doc = "Write Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDATA_SPEC;
impl crate::RegisterSpec for WDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdata::R`](R) reader structure"]
impl crate::Readable for WDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdata::W`](W) writer structure"]
impl crate::Writable for WDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDATA to value 0"]
impl crate::Resettable for WDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
