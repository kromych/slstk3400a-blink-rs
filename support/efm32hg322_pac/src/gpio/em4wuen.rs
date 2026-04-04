#[doc = "Register `EM4WUEN` reader"]
pub type R = crate::R<Em4wuenSpec>;
#[doc = "Register `EM4WUEN` writer"]
pub type W = crate::W<Em4wuenSpec>;
#[doc = "EM4 Wake-up enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Em4wuen {
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
impl From<Em4wuen> for u8 {
    #[inline(always)]
    fn from(variant: Em4wuen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Em4wuen {
    type Ux = u8;
}
impl crate::IsEnum for Em4wuen {}
#[doc = "Field `EM4WUEN` reader - EM4 Wake-up enable"]
pub type Em4wuenR = crate::FieldReader<Em4wuen>;
impl Em4wuenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Em4wuen> {
        match self.bits {
            1 => Some(Em4wuen::A0),
            4 => Some(Em4wuen::C9),
            8 => Some(Em4wuen::F1),
            16 => Some(Em4wuen::F2),
            32 => Some(Em4wuen::E13),
            64 => Some(Em4wuen::C4),
            _ => None,
        }
    }
    #[doc = "Enable em4 wakeup on pin A0"]
    #[inline(always)]
    pub fn is_a0(&self) -> bool {
        *self == Em4wuen::A0
    }
    #[doc = "Enable em4 wakeup on pin C9"]
    #[inline(always)]
    pub fn is_c9(&self) -> bool {
        *self == Em4wuen::C9
    }
    #[doc = "Enable em4 wakeup on pin F1"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == Em4wuen::F1
    }
    #[doc = "Enable em4 wakeup on pin F2"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == Em4wuen::F2
    }
    #[doc = "Enable em4 wakeup on pin E13"]
    #[inline(always)]
    pub fn is_e13(&self) -> bool {
        *self == Em4wuen::E13
    }
    #[doc = "Enable em4 wakeup on pin C4"]
    #[inline(always)]
    pub fn is_c4(&self) -> bool {
        *self == Em4wuen::C4
    }
}
#[doc = "Field `EM4WUEN` writer - EM4 Wake-up enable"]
pub type Em4wuenW<'a, REG> = crate::FieldWriter<'a, REG, 7, Em4wuen>;
impl<'a, REG> Em4wuenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Enable em4 wakeup on pin A0"]
    #[inline(always)]
    pub fn a0(self) -> &'a mut crate::W<REG> {
        self.variant(Em4wuen::A0)
    }
    #[doc = "Enable em4 wakeup on pin C9"]
    #[inline(always)]
    pub fn c9(self) -> &'a mut crate::W<REG> {
        self.variant(Em4wuen::C9)
    }
    #[doc = "Enable em4 wakeup on pin F1"]
    #[inline(always)]
    pub fn f1(self) -> &'a mut crate::W<REG> {
        self.variant(Em4wuen::F1)
    }
    #[doc = "Enable em4 wakeup on pin F2"]
    #[inline(always)]
    pub fn f2(self) -> &'a mut crate::W<REG> {
        self.variant(Em4wuen::F2)
    }
    #[doc = "Enable em4 wakeup on pin E13"]
    #[inline(always)]
    pub fn e13(self) -> &'a mut crate::W<REG> {
        self.variant(Em4wuen::E13)
    }
    #[doc = "Enable em4 wakeup on pin C4"]
    #[inline(always)]
    pub fn c4(self) -> &'a mut crate::W<REG> {
        self.variant(Em4wuen::C4)
    }
}
impl R {
    #[doc = "Bits 0:6 - EM4 Wake-up enable"]
    #[inline(always)]
    pub fn em4wuen(&self) -> Em4wuenR {
        Em4wuenR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - EM4 Wake-up enable"]
    #[inline(always)]
    pub fn em4wuen(&mut self) -> Em4wuenW<'_, Em4wuenSpec> {
        Em4wuenW::new(self, 0)
    }
}
#[doc = "EM4 Wake-up Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`em4wuen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4wuen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Em4wuenSpec;
impl crate::RegisterSpec for Em4wuenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em4wuen::R`](R) reader structure"]
impl crate::Readable for Em4wuenSpec {}
#[doc = "`write(|w| ..)` method takes [`em4wuen::W`](W) writer structure"]
impl crate::Writable for Em4wuenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EM4WUEN to value 0"]
impl crate::Resettable for Em4wuenSpec {}
