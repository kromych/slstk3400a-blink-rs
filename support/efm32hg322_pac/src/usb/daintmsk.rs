#[doc = "Register `DAINTMSK` reader"]
pub type R = crate::R<DAINTMSK_SPEC>;
#[doc = "Register `DAINTMSK` writer"]
pub type W = crate::W<DAINTMSK_SPEC>;
#[doc = "Field `INEPMSK0` reader - IN Endpoint 0 Interrupt mask Bit"]
pub type INEPMSK0_R = crate::BitReader;
#[doc = "Field `INEPMSK0` writer - IN Endpoint 0 Interrupt mask Bit"]
pub type INEPMSK0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INEPMSK1` reader - IN Endpoint 1 Interrupt mask Bit"]
pub type INEPMSK1_R = crate::BitReader;
#[doc = "Field `INEPMSK1` writer - IN Endpoint 1 Interrupt mask Bit"]
pub type INEPMSK1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INEPMSK2` reader - IN Endpoint 2 Interrupt mask Bit"]
pub type INEPMSK2_R = crate::BitReader;
#[doc = "Field `INEPMSK2` writer - IN Endpoint 2 Interrupt mask Bit"]
pub type INEPMSK2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INEPMSK3` reader - IN Endpoint 3 Interrupt mask Bit"]
pub type INEPMSK3_R = crate::BitReader;
#[doc = "Field `INEPMSK3` writer - IN Endpoint 3 Interrupt mask Bit"]
pub type INEPMSK3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUTEPMSK0` reader - OUT Endpoint 0 Interrupt mask Bit"]
pub type OUTEPMSK0_R = crate::BitReader;
#[doc = "Field `OUTEPMSK0` writer - OUT Endpoint 0 Interrupt mask Bit"]
pub type OUTEPMSK0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUTEPMSK1` reader - OUT Endpoint 1 Interrupt mask Bit"]
pub type OUTEPMSK1_R = crate::BitReader;
#[doc = "Field `OUTEPMSK1` writer - OUT Endpoint 1 Interrupt mask Bit"]
pub type OUTEPMSK1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUTEPMSK2` reader - OUT Endpoint 2 Interrupt mask Bit"]
pub type OUTEPMSK2_R = crate::BitReader;
#[doc = "Field `OUTEPMSK2` writer - OUT Endpoint 2 Interrupt mask Bit"]
pub type OUTEPMSK2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUTEPMSK3` reader - OUT Endpoint 3 Interrupt mask Bit"]
pub type OUTEPMSK3_R = crate::BitReader;
#[doc = "Field `OUTEPMSK3` writer - OUT Endpoint 3 Interrupt mask Bit"]
pub type OUTEPMSK3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - IN Endpoint 0 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk0(&self) -> INEPMSK0_R {
        INEPMSK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IN Endpoint 1 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk1(&self) -> INEPMSK1_R {
        INEPMSK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IN Endpoint 2 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk2(&self) -> INEPMSK2_R {
        INEPMSK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IN Endpoint 3 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk3(&self) -> INEPMSK3_R {
        INEPMSK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - OUT Endpoint 0 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk0(&self) -> OUTEPMSK0_R {
        OUTEPMSK0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk1(&self) -> OUTEPMSK1_R {
        OUTEPMSK1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OUT Endpoint 2 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk2(&self) -> OUTEPMSK2_R {
        OUTEPMSK2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoint 3 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk3(&self) -> OUTEPMSK3_R {
        OUTEPMSK3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IN Endpoint 0 Interrupt mask Bit"]
    #[inline(always)]
    #[must_use]
    pub fn inepmsk0(&mut self) -> INEPMSK0_W<DAINTMSK_SPEC, 0> {
        INEPMSK0_W::new(self)
    }
    #[doc = "Bit 1 - IN Endpoint 1 Interrupt mask Bit"]
    #[inline(always)]
    #[must_use]
    pub fn inepmsk1(&mut self) -> INEPMSK1_W<DAINTMSK_SPEC, 1> {
        INEPMSK1_W::new(self)
    }
    #[doc = "Bit 2 - IN Endpoint 2 Interrupt mask Bit"]
    #[inline(always)]
    #[must_use]
    pub fn inepmsk2(&mut self) -> INEPMSK2_W<DAINTMSK_SPEC, 2> {
        INEPMSK2_W::new(self)
    }
    #[doc = "Bit 3 - IN Endpoint 3 Interrupt mask Bit"]
    #[inline(always)]
    #[must_use]
    pub fn inepmsk3(&mut self) -> INEPMSK3_W<DAINTMSK_SPEC, 3> {
        INEPMSK3_W::new(self)
    }
    #[doc = "Bit 16 - OUT Endpoint 0 Interrupt mask Bit"]
    #[inline(always)]
    #[must_use]
    pub fn outepmsk0(&mut self) -> OUTEPMSK0_W<DAINTMSK_SPEC, 16> {
        OUTEPMSK0_W::new(self)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 Interrupt mask Bit"]
    #[inline(always)]
    #[must_use]
    pub fn outepmsk1(&mut self) -> OUTEPMSK1_W<DAINTMSK_SPEC, 17> {
        OUTEPMSK1_W::new(self)
    }
    #[doc = "Bit 18 - OUT Endpoint 2 Interrupt mask Bit"]
    #[inline(always)]
    #[must_use]
    pub fn outepmsk2(&mut self) -> OUTEPMSK2_W<DAINTMSK_SPEC, 18> {
        OUTEPMSK2_W::new(self)
    }
    #[doc = "Bit 19 - OUT Endpoint 3 Interrupt mask Bit"]
    #[inline(always)]
    #[must_use]
    pub fn outepmsk3(&mut self) -> OUTEPMSK3_W<DAINTMSK_SPEC, 19> {
        OUTEPMSK3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device All Endpoints Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAINTMSK_SPEC;
impl crate::RegisterSpec for DAINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daintmsk::R`](R) reader structure"]
impl crate::Readable for DAINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`daintmsk::W`](W) writer structure"]
impl crate::Writable for DAINTMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAINTMSK to value 0"]
impl crate::Resettable for DAINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
