#[doc = "Register `EM4WUEN` reader"]
pub type R = crate::R<EM4WUEN_SPEC>;
#[doc = "Register `EM4WUEN` writer"]
pub type W = crate::W<EM4WUEN_SPEC>;
#[doc = "Field `EM4WUEN` reader - EM4 Wake-up enable"]
pub type EM4WUEN_R = crate::FieldReader<EM4WUEN_A>;
#[doc = "EM4 Wake-up enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EM4WUEN_A {
    #[doc = "1: Enable em4 wakeup on pin A0"]
    A0 = 1,
    #[doc = "4: Enable em4 wakeup on pin C9"]
    C9 = 4,
    #[doc = "8: Enable em4 wakeup on pin F1"]
    F1 = 8,
    #[doc = "16: Enable em4 wakeup on pin F2"]
    F2 = 16,
    #[doc = "32: Enable em4 wakeup on pin E13"]
    E13 = 32,
    #[doc = "64: Enable em4 wakeup on pin C4"]
    C4 = 64,
}
impl From<EM4WUEN_A> for u8 {
    #[inline(always)]
    fn from(variant: EM4WUEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EM4WUEN_A {
    type Ux = u8;
}
impl EM4WUEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EM4WUEN_A> {
        match self.bits {
            1 => Some(EM4WUEN_A::A0),
            4 => Some(EM4WUEN_A::C9),
            8 => Some(EM4WUEN_A::F1),
            16 => Some(EM4WUEN_A::F2),
            32 => Some(EM4WUEN_A::E13),
            64 => Some(EM4WUEN_A::C4),
            _ => None,
        }
    }
    #[doc = "Enable em4 wakeup on pin A0"]
    #[inline(always)]
    pub fn is_a0(&self) -> bool {
        *self == EM4WUEN_A::A0
    }
    #[doc = "Enable em4 wakeup on pin C9"]
    #[inline(always)]
    pub fn is_c9(&self) -> bool {
        *self == EM4WUEN_A::C9
    }
    #[doc = "Enable em4 wakeup on pin F1"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == EM4WUEN_A::F1
    }
    #[doc = "Enable em4 wakeup on pin F2"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == EM4WUEN_A::F2
    }
    #[doc = "Enable em4 wakeup on pin E13"]
    #[inline(always)]
    pub fn is_e13(&self) -> bool {
        *self == EM4WUEN_A::E13
    }
    #[doc = "Enable em4 wakeup on pin C4"]
    #[inline(always)]
    pub fn is_c4(&self) -> bool {
        *self == EM4WUEN_A::C4
    }
}
#[doc = "Field `EM4WUEN` writer - EM4 Wake-up enable"]
pub type EM4WUEN_W<'a, REG> = crate::FieldWriter<'a, REG, 7, EM4WUEN_A>;
impl<'a, REG> EM4WUEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Enable em4 wakeup on pin A0"]
    #[inline(always)]
    pub fn a0(self) -> &'a mut crate::W<REG> {
        self.variant(EM4WUEN_A::A0)
    }
    #[doc = "Enable em4 wakeup on pin C9"]
    #[inline(always)]
    pub fn c9(self) -> &'a mut crate::W<REG> {
        self.variant(EM4WUEN_A::C9)
    }
    #[doc = "Enable em4 wakeup on pin F1"]
    #[inline(always)]
    pub fn f1(self) -> &'a mut crate::W<REG> {
        self.variant(EM4WUEN_A::F1)
    }
    #[doc = "Enable em4 wakeup on pin F2"]
    #[inline(always)]
    pub fn f2(self) -> &'a mut crate::W<REG> {
        self.variant(EM4WUEN_A::F2)
    }
    #[doc = "Enable em4 wakeup on pin E13"]
    #[inline(always)]
    pub fn e13(self) -> &'a mut crate::W<REG> {
        self.variant(EM4WUEN_A::E13)
    }
    #[doc = "Enable em4 wakeup on pin C4"]
    #[inline(always)]
    pub fn c4(self) -> &'a mut crate::W<REG> {
        self.variant(EM4WUEN_A::C4)
    }
}
impl R {
    #[doc = "Bits 0:6 - EM4 Wake-up enable"]
    #[inline(always)]
    pub fn em4wuen(&self) -> EM4WUEN_R {
        EM4WUEN_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - EM4 Wake-up enable"]
    #[inline(always)]
    #[must_use]
    pub fn em4wuen(&mut self) -> EM4WUEN_W<EM4WUEN_SPEC> {
        EM4WUEN_W::new(self, 0)
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
#[doc = "EM4 Wake-up Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`em4wuen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`em4wuen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EM4WUEN_SPEC;
impl crate::RegisterSpec for EM4WUEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em4wuen::R`](R) reader structure"]
impl crate::Readable for EM4WUEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`em4wuen::W`](W) writer structure"]
impl crate::Writable for EM4WUEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EM4WUEN to value 0"]
impl crate::Resettable for EM4WUEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
