#[doc = "Register `EM4WUCAUSE` reader"]
pub type R = crate::R<EM4WUCAUSE_SPEC>;
#[doc = "Field `EM4WUCAUSE` reader - EM4 wake-up cause"]
pub type EM4WUCAUSE_R = crate::FieldReader<EM4WUCAUSE_A>;
#[doc = "EM4 wake-up cause\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EM4WUCAUSE_A {
    #[doc = "1: This bit indicates an em4 wake-up request occurred on pin A0"]
    A0 = 1,
    #[doc = "4: This bit indicates an em4 wake-up request occurred on pin C9"]
    C9 = 4,
    #[doc = "8: This bit indicates an em4 wake-up request occurred on pin F1"]
    F1 = 8,
    #[doc = "16: This bit indicates an em4 wake-up request occurred on pin F2"]
    F2 = 16,
    #[doc = "32: This bit indicates an em4 wake-up request occurred on pin E13"]
    E13 = 32,
    #[doc = "64: This bit indicates an em4 wake-up request occurred on pin C4"]
    C4 = 64,
}
impl From<EM4WUCAUSE_A> for u8 {
    #[inline(always)]
    fn from(variant: EM4WUCAUSE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EM4WUCAUSE_A {
    type Ux = u8;
}
impl EM4WUCAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EM4WUCAUSE_A> {
        match self.bits {
            1 => Some(EM4WUCAUSE_A::A0),
            4 => Some(EM4WUCAUSE_A::C9),
            8 => Some(EM4WUCAUSE_A::F1),
            16 => Some(EM4WUCAUSE_A::F2),
            32 => Some(EM4WUCAUSE_A::E13),
            64 => Some(EM4WUCAUSE_A::C4),
            _ => None,
        }
    }
    #[doc = "This bit indicates an em4 wake-up request occurred on pin A0"]
    #[inline(always)]
    pub fn is_a0(&self) -> bool {
        *self == EM4WUCAUSE_A::A0
    }
    #[doc = "This bit indicates an em4 wake-up request occurred on pin C9"]
    #[inline(always)]
    pub fn is_c9(&self) -> bool {
        *self == EM4WUCAUSE_A::C9
    }
    #[doc = "This bit indicates an em4 wake-up request occurred on pin F1"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == EM4WUCAUSE_A::F1
    }
    #[doc = "This bit indicates an em4 wake-up request occurred on pin F2"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == EM4WUCAUSE_A::F2
    }
    #[doc = "This bit indicates an em4 wake-up request occurred on pin E13"]
    #[inline(always)]
    pub fn is_e13(&self) -> bool {
        *self == EM4WUCAUSE_A::E13
    }
    #[doc = "This bit indicates an em4 wake-up request occurred on pin C4"]
    #[inline(always)]
    pub fn is_c4(&self) -> bool {
        *self == EM4WUCAUSE_A::C4
    }
}
impl R {
    #[doc = "Bits 0:6 - EM4 wake-up cause"]
    #[inline(always)]
    pub fn em4wucause(&self) -> EM4WUCAUSE_R {
        EM4WUCAUSE_R::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "EM4 Wake-up Cause Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`em4wucause::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EM4WUCAUSE_SPEC;
impl crate::RegisterSpec for EM4WUCAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em4wucause::R`](R) reader structure"]
impl crate::Readable for EM4WUCAUSE_SPEC {}
#[doc = "`reset()` method sets EM4WUCAUSE to value 0"]
impl crate::Resettable for EM4WUCAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
