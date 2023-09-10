#[doc = "Register `DTOGEN` reader"]
pub type R = crate::R<DTOGEN_SPEC>;
#[doc = "Register `DTOGEN` writer"]
pub type W = crate::W<DTOGEN_SPEC>;
#[doc = "Field `DTOGCC0EN` reader - DTI CC0 Output Generation Enable"]
pub type DTOGCC0EN_R = crate::BitReader;
#[doc = "Field `DTOGCC0EN` writer - DTI CC0 Output Generation Enable"]
pub type DTOGCC0EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTOGCC1EN` reader - DTI CC1 Output Generation Enable"]
pub type DTOGCC1EN_R = crate::BitReader;
#[doc = "Field `DTOGCC1EN` writer - DTI CC1 Output Generation Enable"]
pub type DTOGCC1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTOGCC2EN` reader - DTI CC2 Output Generation Enable"]
pub type DTOGCC2EN_R = crate::BitReader;
#[doc = "Field `DTOGCC2EN` writer - DTI CC2 Output Generation Enable"]
pub type DTOGCC2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTOGCDTI0EN` reader - DTI CDTI0 Output Generation Enable"]
pub type DTOGCDTI0EN_R = crate::BitReader;
#[doc = "Field `DTOGCDTI0EN` writer - DTI CDTI0 Output Generation Enable"]
pub type DTOGCDTI0EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTOGCDTI1EN` reader - DTI CDTI1 Output Generation Enable"]
pub type DTOGCDTI1EN_R = crate::BitReader;
#[doc = "Field `DTOGCDTI1EN` writer - DTI CDTI1 Output Generation Enable"]
pub type DTOGCDTI1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTOGCDTI2EN` reader - DTI CDTI2 Output Generation Enable"]
pub type DTOGCDTI2EN_R = crate::BitReader;
#[doc = "Field `DTOGCDTI2EN` writer - DTI CDTI2 Output Generation Enable"]
pub type DTOGCDTI2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DTI CC0 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc0en(&self) -> DTOGCC0EN_R {
        DTOGCC0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTI CC1 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc1en(&self) -> DTOGCC1EN_R {
        DTOGCC1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DTI CC2 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc2en(&self) -> DTOGCC2EN_R {
        DTOGCC2EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTI CDTI0 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti0en(&self) -> DTOGCDTI0EN_R {
        DTOGCDTI0EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DTI CDTI1 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti1en(&self) -> DTOGCDTI1EN_R {
        DTOGCDTI1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DTI CDTI2 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti2en(&self) -> DTOGCDTI2EN_R {
        DTOGCDTI2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTI CC0 Output Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtogcc0en(&mut self) -> DTOGCC0EN_W<DTOGEN_SPEC, 0> {
        DTOGCC0EN_W::new(self)
    }
    #[doc = "Bit 1 - DTI CC1 Output Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtogcc1en(&mut self) -> DTOGCC1EN_W<DTOGEN_SPEC, 1> {
        DTOGCC1EN_W::new(self)
    }
    #[doc = "Bit 2 - DTI CC2 Output Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtogcc2en(&mut self) -> DTOGCC2EN_W<DTOGEN_SPEC, 2> {
        DTOGCC2EN_W::new(self)
    }
    #[doc = "Bit 3 - DTI CDTI0 Output Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtogcdti0en(&mut self) -> DTOGCDTI0EN_W<DTOGEN_SPEC, 3> {
        DTOGCDTI0EN_W::new(self)
    }
    #[doc = "Bit 4 - DTI CDTI1 Output Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtogcdti1en(&mut self) -> DTOGCDTI1EN_W<DTOGEN_SPEC, 4> {
        DTOGCDTI1EN_W::new(self)
    }
    #[doc = "Bit 5 - DTI CDTI2 Output Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtogcdti2en(&mut self) -> DTOGCDTI2EN_W<DTOGEN_SPEC, 5> {
        DTOGCDTI2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DTI Output Generation Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtogen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtogen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTOGEN_SPEC;
impl crate::RegisterSpec for DTOGEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtogen::R`](R) reader structure"]
impl crate::Readable for DTOGEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dtogen::W`](W) writer structure"]
impl crate::Writable for DTOGEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTOGEN to value 0"]
impl crate::Resettable for DTOGEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
