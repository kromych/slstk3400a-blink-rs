#[doc = "Register `LFBPRESC0` reader"]
pub type R = crate::R<Lfbpresc0Spec>;
#[doc = "Register `LFBPRESC0` writer"]
pub type W = crate::W<Lfbpresc0Spec>;
#[doc = "Low Energy UART 0 Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Leuart0 {
    #[doc = "0: LFBCLKLEUART0 = LFBCLK"]
    Div1 = 0,
    #[doc = "1: LFBCLKLEUART0 = LFBCLK/2"]
    Div2 = 1,
    #[doc = "2: LFBCLKLEUART0 = LFBCLK/4"]
    Div4 = 2,
    #[doc = "3: LFBCLKLEUART0 = LFBCLK/8"]
    Div8 = 3,
}
impl From<Leuart0> for u8 {
    #[inline(always)]
    fn from(variant: Leuart0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Leuart0 {
    type Ux = u8;
}
impl crate::IsEnum for Leuart0 {}
#[doc = "Field `LEUART0` reader - Low Energy UART 0 Prescaler"]
pub type Leuart0R = crate::FieldReader<Leuart0>;
impl Leuart0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leuart0 {
        match self.bits {
            0 => Leuart0::Div1,
            1 => Leuart0::Div2,
            2 => Leuart0::Div4,
            3 => Leuart0::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Leuart0::Div1
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Leuart0::Div2
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Leuart0::Div4
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Leuart0::Div8
    }
}
#[doc = "Field `LEUART0` writer - Low Energy UART 0 Prescaler"]
pub type Leuart0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Leuart0, crate::Safe>;
impl<'a, REG> Leuart0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFBCLKLEUART0 = LFBCLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Leuart0::Div1)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Leuart0::Div2)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Leuart0::Div4)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Leuart0::Div8)
    }
}
impl R {
    #[doc = "Bits 0:1 - Low Energy UART 0 Prescaler"]
    #[inline(always)]
    pub fn leuart0(&self) -> Leuart0R {
        Leuart0R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low Energy UART 0 Prescaler"]
    #[inline(always)]
    pub fn leuart0(&mut self) -> Leuart0W<'_, Lfbpresc0Spec> {
        Leuart0W::new(self, 0)
    }
}
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)\n\nYou can [`read`](crate::Reg::read) this register and get [`lfbpresc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfbpresc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lfbpresc0Spec;
impl crate::RegisterSpec for Lfbpresc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfbpresc0::R`](R) reader structure"]
impl crate::Readable for Lfbpresc0Spec {}
#[doc = "`write(|w| ..)` method takes [`lfbpresc0::W`](W) writer structure"]
impl crate::Writable for Lfbpresc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFBPRESC0 to value 0"]
impl crate::Resettable for Lfbpresc0Spec {}
