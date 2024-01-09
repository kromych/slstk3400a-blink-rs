#[doc = "Register `USHFRCOCTRL` reader"]
pub type R = crate::R<USHFRCOCTRL_SPEC>;
#[doc = "Register `USHFRCOCTRL` writer"]
pub type W = crate::W<USHFRCOCTRL_SPEC>;
#[doc = "Field `TUNING` reader - USHFRCO frequency adjust"]
pub type TUNING_R = crate::FieldReader;
#[doc = "Field `TUNING` writer - USHFRCO frequency adjust"]
pub type TUNING_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DITHEN` reader - USHFRCO dither enable"]
pub type DITHEN_R = crate::BitReader;
#[doc = "Field `DITHEN` writer - USHFRCO dither enable"]
pub type DITHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPEND` reader - USHFRCO suspend"]
pub type SUSPEND_R = crate::BitReader;
#[doc = "Field `SUSPEND` writer - USHFRCO suspend"]
pub type SUSPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - USHFRCO Timeout"]
pub type TIMEOUT_R = crate::FieldReader;
#[doc = "Field `TIMEOUT` writer - USHFRCO Timeout"]
pub type TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:6 - USHFRCO frequency adjust"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - USHFRCO dither enable"]
    #[inline(always)]
    pub fn dithen(&self) -> DITHEN_R {
        DITHEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USHFRCO suspend"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:19 - USHFRCO Timeout"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USHFRCO frequency adjust"]
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TUNING_W<USHFRCOCTRL_SPEC> {
        TUNING_W::new(self, 0)
    }
    #[doc = "Bit 8 - USHFRCO dither enable"]
    #[inline(always)]
    #[must_use]
    pub fn dithen(&mut self) -> DITHEN_W<USHFRCOCTRL_SPEC> {
        DITHEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - USHFRCO suspend"]
    #[inline(always)]
    #[must_use]
    pub fn suspend(&mut self) -> SUSPEND_W<USHFRCOCTRL_SPEC> {
        SUSPEND_W::new(self, 9)
    }
    #[doc = "Bits 12:19 - USHFRCO Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<USHFRCOCTRL_SPEC> {
        TIMEOUT_W::new(self, 12)
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
#[doc = "USHFRCO Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ushfrcoctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ushfrcoctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USHFRCOCTRL_SPEC;
impl crate::RegisterSpec for USHFRCOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ushfrcoctrl::R`](R) reader structure"]
impl crate::Readable for USHFRCOCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ushfrcoctrl::W`](W) writer structure"]
impl crate::Writable for USHFRCOCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USHFRCOCTRL to value 0x000f_f040"]
impl crate::Resettable for USHFRCOCTRL_SPEC {
    const RESET_VALUE: u32 = 0x000f_f040;
}
