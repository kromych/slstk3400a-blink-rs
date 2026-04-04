#[doc = "Register `EM4WUCAUSE` reader"]
pub type R = crate::R<Em4wucauseSpec>;
#[doc = "EM4 wake-up cause\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Em4wucause {
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
impl From<Em4wucause> for u8 {
    #[inline(always)]
    fn from(variant: Em4wucause) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Em4wucause {
    type Ux = u8;
}
impl crate::IsEnum for Em4wucause {}
#[doc = "Field `EM4WUCAUSE` reader - EM4 wake-up cause"]
pub type Em4wucauseR = crate::FieldReader<Em4wucause>;
impl Em4wucauseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Em4wucause> {
        match self.bits {
            1 => Some(Em4wucause::A0),
            4 => Some(Em4wucause::C9),
            8 => Some(Em4wucause::F1),
            16 => Some(Em4wucause::F2),
            32 => Some(Em4wucause::E13),
            64 => Some(Em4wucause::C4),
            _ => None,
        }
    }
    #[doc = "This bit indicates an em4 wake-up request occurred on pin A0"]
    #[inline(always)]
    pub fn is_a0(&self) -> bool {
        *self == Em4wucause::A0
    }
    #[doc = "This bit indicates an em4 wake-up request occurred on pin C9"]
    #[inline(always)]
    pub fn is_c9(&self) -> bool {
        *self == Em4wucause::C9
    }
    #[doc = "This bit indicates an em4 wake-up request occurred on pin F1"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == Em4wucause::F1
    }
    #[doc = "This bit indicates an em4 wake-up request occurred on pin F2"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == Em4wucause::F2
    }
    #[doc = "This bit indicates an em4 wake-up request occurred on pin E13"]
    #[inline(always)]
    pub fn is_e13(&self) -> bool {
        *self == Em4wucause::E13
    }
    #[doc = "This bit indicates an em4 wake-up request occurred on pin C4"]
    #[inline(always)]
    pub fn is_c4(&self) -> bool {
        *self == Em4wucause::C4
    }
}
impl R {
    #[doc = "Bits 0:6 - EM4 wake-up cause"]
    #[inline(always)]
    pub fn em4wucause(&self) -> Em4wucauseR {
        Em4wucauseR::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "EM4 Wake-up Cause Register\n\nYou can [`read`](crate::Reg::read) this register and get [`em4wucause::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Em4wucauseSpec;
impl crate::RegisterSpec for Em4wucauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em4wucause::R`](R) reader structure"]
impl crate::Readable for Em4wucauseSpec {}
#[doc = "`reset()` method sets EM4WUCAUSE to value 0"]
impl crate::Resettable for Em4wucauseSpec {}
