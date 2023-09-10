#[doc = "Register `EM4WUPOL` reader"]
pub type R = crate::R<EM4WUPOL_SPEC>;
#[doc = "Register `EM4WUPOL` writer"]
pub type W = crate::W<EM4WUPOL_SPEC>;
#[doc = "Field `EM4WUPOL` reader - EM4 Wake-up Polarity"]
pub type EM4WUPOL_R = crate::FieldReader<EM4WUPOL_A>;
#[doc = "EM4 Wake-up Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EM4WUPOL_A {
    #[doc = "1: Determines polarity on pin A0"]
    A0 = 1,
    #[doc = "4: Determines polarity on pin C9"]
    C9 = 4,
    #[doc = "8: Determines polarity on pin F1"]
    F1 = 8,
    #[doc = "16: Determines polarity on pin F2"]
    F2 = 16,
    #[doc = "32: Determines polarity on pin E13"]
    E13 = 32,
    #[doc = "64: Determines polarity on pin C4"]
    C4 = 64,
}
impl From<EM4WUPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: EM4WUPOL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EM4WUPOL_A {
    type Ux = u8;
}
impl EM4WUPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EM4WUPOL_A> {
        match self.bits {
            1 => Some(EM4WUPOL_A::A0),
            4 => Some(EM4WUPOL_A::C9),
            8 => Some(EM4WUPOL_A::F1),
            16 => Some(EM4WUPOL_A::F2),
            32 => Some(EM4WUPOL_A::E13),
            64 => Some(EM4WUPOL_A::C4),
            _ => None,
        }
    }
    #[doc = "Determines polarity on pin A0"]
    #[inline(always)]
    pub fn is_a0(&self) -> bool {
        *self == EM4WUPOL_A::A0
    }
    #[doc = "Determines polarity on pin C9"]
    #[inline(always)]
    pub fn is_c9(&self) -> bool {
        *self == EM4WUPOL_A::C9
    }
    #[doc = "Determines polarity on pin F1"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == EM4WUPOL_A::F1
    }
    #[doc = "Determines polarity on pin F2"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == EM4WUPOL_A::F2
    }
    #[doc = "Determines polarity on pin E13"]
    #[inline(always)]
    pub fn is_e13(&self) -> bool {
        *self == EM4WUPOL_A::E13
    }
    #[doc = "Determines polarity on pin C4"]
    #[inline(always)]
    pub fn is_c4(&self) -> bool {
        *self == EM4WUPOL_A::C4
    }
}
#[doc = "Field `EM4WUPOL` writer - EM4 Wake-up Polarity"]
pub type EM4WUPOL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O, EM4WUPOL_A>;
impl<'a, REG, const O: u8> EM4WUPOL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Determines polarity on pin A0"]
    #[inline(always)]
    pub fn a0(self) -> &'a mut crate::W<REG> {
        self.variant(EM4WUPOL_A::A0)
    }
    #[doc = "Determines polarity on pin C9"]
    #[inline(always)]
    pub fn c9(self) -> &'a mut crate::W<REG> {
        self.variant(EM4WUPOL_A::C9)
    }
    #[doc = "Determines polarity on pin F1"]
    #[inline(always)]
    pub fn f1(self) -> &'a mut crate::W<REG> {
        self.variant(EM4WUPOL_A::F1)
    }
    #[doc = "Determines polarity on pin F2"]
    #[inline(always)]
    pub fn f2(self) -> &'a mut crate::W<REG> {
        self.variant(EM4WUPOL_A::F2)
    }
    #[doc = "Determines polarity on pin E13"]
    #[inline(always)]
    pub fn e13(self) -> &'a mut crate::W<REG> {
        self.variant(EM4WUPOL_A::E13)
    }
    #[doc = "Determines polarity on pin C4"]
    #[inline(always)]
    pub fn c4(self) -> &'a mut crate::W<REG> {
        self.variant(EM4WUPOL_A::C4)
    }
}
impl R {
    #[doc = "Bits 0:6 - EM4 Wake-up Polarity"]
    #[inline(always)]
    pub fn em4wupol(&self) -> EM4WUPOL_R {
        EM4WUPOL_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - EM4 Wake-up Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn em4wupol(&mut self) -> EM4WUPOL_W<EM4WUPOL_SPEC, 0> {
        EM4WUPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "EM4 Wake-up Polarity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`em4wupol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`em4wupol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EM4WUPOL_SPEC;
impl crate::RegisterSpec for EM4WUPOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em4wupol::R`](R) reader structure"]
impl crate::Readable for EM4WUPOL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`em4wupol::W`](W) writer structure"]
impl crate::Writable for EM4WUPOL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EM4WUPOL to value 0"]
impl crate::Resettable for EM4WUPOL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
