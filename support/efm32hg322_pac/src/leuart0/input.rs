#[doc = "Register `INPUT` reader"]
pub type R = crate::R<InputSpec>;
#[doc = "Register `INPUT` writer"]
pub type W = crate::W<InputSpec>;
#[doc = "RX PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxprssel {
    #[doc = "0: PRS Channel 0 selected"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected"]
    Prsch5 = 5,
}
impl From<Rxprssel> for u8 {
    #[inline(always)]
    fn from(variant: Rxprssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxprssel {
    type Ux = u8;
}
impl crate::IsEnum for Rxprssel {}
#[doc = "Field `RXPRSSEL` reader - RX PRS Channel Select"]
pub type RxprsselR = crate::FieldReader<Rxprssel>;
impl RxprsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rxprssel> {
        match self.bits {
            0 => Some(Rxprssel::Prsch0),
            1 => Some(Rxprssel::Prsch1),
            2 => Some(Rxprssel::Prsch2),
            3 => Some(Rxprssel::Prsch3),
            4 => Some(Rxprssel::Prsch4),
            5 => Some(Rxprssel::Prsch5),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Rxprssel::Prsch0
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Rxprssel::Prsch1
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Rxprssel::Prsch2
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Rxprssel::Prsch3
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Rxprssel::Prsch4
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Rxprssel::Prsch5
    }
}
#[doc = "Field `RXPRSSEL` writer - RX PRS Channel Select"]
pub type RxprsselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rxprssel>;
impl<'a, REG> RxprsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch5)
    }
}
#[doc = "Field `RXPRS` reader - PRS RX Enable"]
pub type RxprsR = crate::BitReader;
#[doc = "Field `RXPRS` writer - PRS RX Enable"]
pub type RxprsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - RX PRS Channel Select"]
    #[inline(always)]
    pub fn rxprssel(&self) -> RxprsselR {
        RxprsselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprs(&self) -> RxprsR {
        RxprsR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - RX PRS Channel Select"]
    #[inline(always)]
    pub fn rxprssel(&mut self) -> RxprsselW<'_, InputSpec> {
        RxprsselW::new(self, 0)
    }
    #[doc = "Bit 4 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprs(&mut self) -> RxprsW<'_, InputSpec> {
        RxprsW::new(self, 4)
    }
}
#[doc = "LEUART Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputSpec;
impl crate::RegisterSpec for InputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input::R`](R) reader structure"]
impl crate::Readable for InputSpec {}
#[doc = "`write(|w| ..)` method takes [`input::W`](W) writer structure"]
impl crate::Writable for InputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT to value 0"]
impl crate::Resettable for InputSpec {}
