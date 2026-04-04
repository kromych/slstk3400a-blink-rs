#[doc = "Register `EM4WUPOL` reader"]
pub type R = crate::R<Em4wupolSpec>;
#[doc = "Register `EM4WUPOL` writer"]
pub type W = crate::W<Em4wupolSpec>;
#[doc = "EM4 Wake-up Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Em4wupol {
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
impl From<Em4wupol> for u8 {
    #[inline(always)]
    fn from(variant: Em4wupol) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Em4wupol {
    type Ux = u8;
}
impl crate::IsEnum for Em4wupol {}
#[doc = "Field `EM4WUPOL` reader - EM4 Wake-up Polarity"]
pub type Em4wupolR = crate::FieldReader<Em4wupol>;
impl Em4wupolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Em4wupol> {
        match self.bits {
            1 => Some(Em4wupol::A0),
            4 => Some(Em4wupol::C9),
            8 => Some(Em4wupol::F1),
            16 => Some(Em4wupol::F2),
            32 => Some(Em4wupol::E13),
            64 => Some(Em4wupol::C4),
            _ => None,
        }
    }
    #[doc = "Determines polarity on pin A0"]
    #[inline(always)]
    pub fn is_a0(&self) -> bool {
        *self == Em4wupol::A0
    }
    #[doc = "Determines polarity on pin C9"]
    #[inline(always)]
    pub fn is_c9(&self) -> bool {
        *self == Em4wupol::C9
    }
    #[doc = "Determines polarity on pin F1"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == Em4wupol::F1
    }
    #[doc = "Determines polarity on pin F2"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == Em4wupol::F2
    }
    #[doc = "Determines polarity on pin E13"]
    #[inline(always)]
    pub fn is_e13(&self) -> bool {
        *self == Em4wupol::E13
    }
    #[doc = "Determines polarity on pin C4"]
    #[inline(always)]
    pub fn is_c4(&self) -> bool {
        *self == Em4wupol::C4
    }
}
#[doc = "Field `EM4WUPOL` writer - EM4 Wake-up Polarity"]
pub type Em4wupolW<'a, REG> = crate::FieldWriter<'a, REG, 7, Em4wupol>;
impl<'a, REG> Em4wupolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Determines polarity on pin A0"]
    #[inline(always)]
    pub fn a0(self) -> &'a mut crate::W<REG> {
        self.variant(Em4wupol::A0)
    }
    #[doc = "Determines polarity on pin C9"]
    #[inline(always)]
    pub fn c9(self) -> &'a mut crate::W<REG> {
        self.variant(Em4wupol::C9)
    }
    #[doc = "Determines polarity on pin F1"]
    #[inline(always)]
    pub fn f1(self) -> &'a mut crate::W<REG> {
        self.variant(Em4wupol::F1)
    }
    #[doc = "Determines polarity on pin F2"]
    #[inline(always)]
    pub fn f2(self) -> &'a mut crate::W<REG> {
        self.variant(Em4wupol::F2)
    }
    #[doc = "Determines polarity on pin E13"]
    #[inline(always)]
    pub fn e13(self) -> &'a mut crate::W<REG> {
        self.variant(Em4wupol::E13)
    }
    #[doc = "Determines polarity on pin C4"]
    #[inline(always)]
    pub fn c4(self) -> &'a mut crate::W<REG> {
        self.variant(Em4wupol::C4)
    }
}
impl R {
    #[doc = "Bits 0:6 - EM4 Wake-up Polarity"]
    #[inline(always)]
    pub fn em4wupol(&self) -> Em4wupolR {
        Em4wupolR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - EM4 Wake-up Polarity"]
    #[inline(always)]
    pub fn em4wupol(&mut self) -> Em4wupolW<'_, Em4wupolSpec> {
        Em4wupolW::new(self, 0)
    }
}
#[doc = "EM4 Wake-up Polarity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`em4wupol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4wupol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Em4wupolSpec;
impl crate::RegisterSpec for Em4wupolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em4wupol::R`](R) reader structure"]
impl crate::Readable for Em4wupolSpec {}
#[doc = "`write(|w| ..)` method takes [`em4wupol::W`](W) writer structure"]
impl crate::Writable for Em4wupolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EM4WUPOL to value 0"]
impl crate::Resettable for Em4wupolSpec {}
