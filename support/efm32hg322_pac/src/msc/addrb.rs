#[doc = "Register `ADDRB` reader"]
pub type R = crate::R<ADDRB_SPEC>;
#[doc = "Register `ADDRB` writer"]
pub type W = crate::W<ADDRB_SPEC>;
#[doc = "Field `ADDRB` reader - Page Erase or Write Address Buffer"]
pub type ADDRB_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRB` writer - Page Erase or Write Address Buffer"]
pub type ADDRB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Page Erase or Write Address Buffer"]
    #[inline(always)]
    pub fn addrb(&self) -> ADDRB_R {
        ADDRB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Page Erase or Write Address Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn addrb(&mut self) -> ADDRB_W<ADDRB_SPEC, 0> {
        ADDRB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Page Erase/Write Address Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDRB_SPEC;
impl crate::RegisterSpec for ADDRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addrb::R`](R) reader structure"]
impl crate::Readable for ADDRB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addrb::W`](W) writer structure"]
impl crate::Writable for ADDRB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDRB to value 0"]
impl crate::Resettable for ADDRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
