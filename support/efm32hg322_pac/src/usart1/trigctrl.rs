#[doc = "Register `TRIGCTRL` reader"]
pub type R = crate::R<TrigctrlSpec>;
#[doc = "Register `TRIGCTRL` writer"]
pub type W = crate::W<TrigctrlSpec>;
#[doc = "Trigger PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsel {
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
impl From<Tsel> for u8 {
    #[inline(always)]
    fn from(variant: Tsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsel {
    type Ux = u8;
}
impl crate::IsEnum for Tsel {}
#[doc = "Field `TSEL` reader - Trigger PRS Channel Select"]
pub type TselR = crate::FieldReader<Tsel>;
impl TselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tsel> {
        match self.bits {
            0 => Some(Tsel::Prsch0),
            1 => Some(Tsel::Prsch1),
            2 => Some(Tsel::Prsch2),
            3 => Some(Tsel::Prsch3),
            4 => Some(Tsel::Prsch4),
            5 => Some(Tsel::Prsch5),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Tsel::Prsch0
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Tsel::Prsch1
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Tsel::Prsch2
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Tsel::Prsch3
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Tsel::Prsch4
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Tsel::Prsch5
    }
}
#[doc = "Field `TSEL` writer - Trigger PRS Channel Select"]
pub type TselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tsel>;
impl<'a, REG> TselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Prsch5)
    }
}
#[doc = "Field `RXTEN` reader - Receive Trigger Enable"]
pub type RxtenR = crate::BitReader;
#[doc = "Field `RXTEN` writer - Receive Trigger Enable"]
pub type RxtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTEN` reader - Transmit Trigger Enable"]
pub type TxtenR = crate::BitReader;
#[doc = "Field `TXTEN` writer - Transmit Trigger Enable"]
pub type TxtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOTXTEN` reader - AUTOTX Trigger Enable"]
pub type AutotxtenR = crate::BitReader;
#[doc = "Field `AUTOTXTEN` writer - AUTOTX Trigger Enable"]
pub type AutotxtenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Trigger PRS Channel Select"]
    #[inline(always)]
    pub fn tsel(&self) -> TselR {
        TselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Receive Trigger Enable"]
    #[inline(always)]
    pub fn rxten(&self) -> RxtenR {
        RxtenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Trigger Enable"]
    #[inline(always)]
    pub fn txten(&self) -> TxtenR {
        TxtenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AUTOTX Trigger Enable"]
    #[inline(always)]
    pub fn autotxten(&self) -> AutotxtenR {
        AutotxtenR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Trigger PRS Channel Select"]
    #[inline(always)]
    pub fn tsel(&mut self) -> TselW<'_, TrigctrlSpec> {
        TselW::new(self, 0)
    }
    #[doc = "Bit 4 - Receive Trigger Enable"]
    #[inline(always)]
    pub fn rxten(&mut self) -> RxtenW<'_, TrigctrlSpec> {
        RxtenW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit Trigger Enable"]
    #[inline(always)]
    pub fn txten(&mut self) -> TxtenW<'_, TrigctrlSpec> {
        TxtenW::new(self, 5)
    }
    #[doc = "Bit 6 - AUTOTX Trigger Enable"]
    #[inline(always)]
    pub fn autotxten(&mut self) -> AutotxtenW<'_, TrigctrlSpec> {
        AutotxtenW::new(self, 6)
    }
}
#[doc = "USART Trigger Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`trigctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrigctrlSpec;
impl crate::RegisterSpec for TrigctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigctrl::R`](R) reader structure"]
impl crate::Readable for TrigctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`trigctrl::W`](W) writer structure"]
impl crate::Writable for TrigctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRIGCTRL to value 0"]
impl crate::Resettable for TrigctrlSpec {}
