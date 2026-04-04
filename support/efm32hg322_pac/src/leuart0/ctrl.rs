#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `AUTOTRI` reader - Automatic Transmitter Tristate"]
pub type AutotriR = crate::BitReader;
#[doc = "Field `AUTOTRI` writer - Automatic Transmitter Tristate"]
pub type AutotriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATABITS` reader - Data-Bit Mode"]
pub type DatabitsR = crate::BitReader;
#[doc = "Field `DATABITS` writer - Data-Bit Mode"]
pub type DatabitsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Parity-Bit Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Parity {
    #[doc = "0: Parity bits are not used"]
    None = 0,
    #[doc = "2: Even parity are used. Parity bits are automatically generated and checked by hardware."]
    Even = 2,
    #[doc = "3: Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    Odd = 3,
}
impl From<Parity> for u8 {
    #[inline(always)]
    fn from(variant: Parity) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Parity {
    type Ux = u8;
}
impl crate::IsEnum for Parity {}
#[doc = "Field `PARITY` reader - Parity-Bit Mode"]
pub type ParityR = crate::FieldReader<Parity>;
impl ParityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Parity> {
        match self.bits {
            0 => Some(Parity::None),
            2 => Some(Parity::Even),
            3 => Some(Parity::Odd),
            _ => None,
        }
    }
    #[doc = "Parity bits are not used"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Parity::None
    }
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Parity::Even
    }
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Parity::Odd
    }
}
#[doc = "Field `PARITY` writer - Parity-Bit Mode"]
pub type ParityW<'a, REG> = crate::FieldWriter<'a, REG, 2, Parity>;
impl<'a, REG> ParityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Parity bits are not used"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Parity::None)
    }
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Parity::Even)
    }
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Parity::Odd)
    }
}
#[doc = "Field `STOPBITS` reader - Stop-Bit Mode"]
pub type StopbitsR = crate::BitReader;
#[doc = "Field `STOPBITS` writer - Stop-Bit Mode"]
pub type StopbitsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INV` reader - Invert Input And Output"]
pub type InvR = crate::BitReader;
#[doc = "Field `INV` writer - Invert Input And Output"]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRSDMA` reader - Clear RX DMA On Error"]
pub type ErrsdmaR = crate::BitReader;
#[doc = "Field `ERRSDMA` writer - Clear RX DMA On Error"]
pub type ErrsdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBK` reader - Loopback Enable"]
pub type LoopbkR = crate::BitReader;
#[doc = "Field `LOOPBK` writer - Loopback Enable"]
pub type LoopbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFUBRX` reader - Start-Frame UnBlock RX"]
pub type SfubrxR = crate::BitReader;
#[doc = "Field `SFUBRX` writer - Start-Frame UnBlock RX"]
pub type SfubrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPM` reader - Multi-Processor Mode"]
pub type MpmR = crate::BitReader;
#[doc = "Field `MPM` writer - Multi-Processor Mode"]
pub type MpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAB` reader - Multi-Processor Address-Bit"]
pub type MpabR = crate::BitReader;
#[doc = "Field `MPAB` writer - Multi-Processor Address-Bit"]
pub type MpabW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT8DV` reader - Bit 8 Default Value"]
pub type Bit8dvR = crate::BitReader;
#[doc = "Field `BIT8DV` writer - Bit 8 Default Value"]
pub type Bit8dvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAWU` reader - RX DMA Wakeup"]
pub type RxdmawuR = crate::BitReader;
#[doc = "Field `RXDMAWU` writer - RX DMA Wakeup"]
pub type RxdmawuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAWU` reader - TX DMA Wakeup"]
pub type TxdmawuR = crate::BitReader;
#[doc = "Field `TXDMAWU` writer - TX DMA Wakeup"]
pub type TxdmawuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "TX Delay Transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txdelay {
    #[doc = "0: Frames are transmitted immediately"]
    None = 0,
    #[doc = "1: Transmission of new frames are delayed by a single baud period"]
    Single = 1,
    #[doc = "2: Transmission of new frames are delayed by two baud periods"]
    Double = 2,
    #[doc = "3: Transmission of new frames are delayed by three baud periods"]
    Triple = 3,
}
impl From<Txdelay> for u8 {
    #[inline(always)]
    fn from(variant: Txdelay) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txdelay {
    type Ux = u8;
}
impl crate::IsEnum for Txdelay {}
#[doc = "Field `TXDELAY` reader - TX Delay Transmission"]
pub type TxdelayR = crate::FieldReader<Txdelay>;
impl TxdelayR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdelay {
        match self.bits {
            0 => Txdelay::None,
            1 => Txdelay::Single,
            2 => Txdelay::Double,
            3 => Txdelay::Triple,
            _ => unreachable!(),
        }
    }
    #[doc = "Frames are transmitted immediately"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Txdelay::None
    }
    #[doc = "Transmission of new frames are delayed by a single baud period"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Txdelay::Single
    }
    #[doc = "Transmission of new frames are delayed by two baud periods"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == Txdelay::Double
    }
    #[doc = "Transmission of new frames are delayed by three baud periods"]
    #[inline(always)]
    pub fn is_triple(&self) -> bool {
        *self == Txdelay::Triple
    }
}
#[doc = "Field `TXDELAY` writer - TX Delay Transmission"]
pub type TxdelayW<'a, REG> = crate::FieldWriter<'a, REG, 2, Txdelay, crate::Safe>;
impl<'a, REG> TxdelayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Frames are transmitted immediately"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::None)
    }
    #[doc = "Transmission of new frames are delayed by a single baud period"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::Single)
    }
    #[doc = "Transmission of new frames are delayed by two baud periods"]
    #[inline(always)]
    pub fn double(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::Double)
    }
    #[doc = "Transmission of new frames are delayed by three baud periods"]
    #[inline(always)]
    pub fn triple(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::Triple)
    }
}
impl R {
    #[doc = "Bit 0 - Automatic Transmitter Tristate"]
    #[inline(always)]
    pub fn autotri(&self) -> AutotriR {
        AutotriR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data-Bit Mode"]
    #[inline(always)]
    pub fn databits(&self) -> DatabitsR {
        DatabitsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Parity-Bit Mode"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Stop-Bit Mode"]
    #[inline(always)]
    pub fn stopbits(&self) -> StopbitsR {
        StopbitsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Invert Input And Output"]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear RX DMA On Error"]
    #[inline(always)]
    pub fn errsdma(&self) -> ErrsdmaR {
        ErrsdmaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Loopback Enable"]
    #[inline(always)]
    pub fn loopbk(&self) -> LoopbkR {
        LoopbkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Start-Frame UnBlock RX"]
    #[inline(always)]
    pub fn sfubrx(&self) -> SfubrxR {
        SfubrxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Multi-Processor Mode"]
    #[inline(always)]
    pub fn mpm(&self) -> MpmR {
        MpmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multi-Processor Address-Bit"]
    #[inline(always)]
    pub fn mpab(&self) -> MpabR {
        MpabR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bit 8 Default Value"]
    #[inline(always)]
    pub fn bit8dv(&self) -> Bit8dvR {
        Bit8dvR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RX DMA Wakeup"]
    #[inline(always)]
    pub fn rxdmawu(&self) -> RxdmawuR {
        RxdmawuR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TX DMA Wakeup"]
    #[inline(always)]
    pub fn txdmawu(&self) -> TxdmawuR {
        TxdmawuR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - TX Delay Transmission"]
    #[inline(always)]
    pub fn txdelay(&self) -> TxdelayR {
        TxdelayR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Automatic Transmitter Tristate"]
    #[inline(always)]
    pub fn autotri(&mut self) -> AutotriW<'_, CtrlSpec> {
        AutotriW::new(self, 0)
    }
    #[doc = "Bit 1 - Data-Bit Mode"]
    #[inline(always)]
    pub fn databits(&mut self) -> DatabitsW<'_, CtrlSpec> {
        DatabitsW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Parity-Bit Mode"]
    #[inline(always)]
    pub fn parity(&mut self) -> ParityW<'_, CtrlSpec> {
        ParityW::new(self, 2)
    }
    #[doc = "Bit 4 - Stop-Bit Mode"]
    #[inline(always)]
    pub fn stopbits(&mut self) -> StopbitsW<'_, CtrlSpec> {
        StopbitsW::new(self, 4)
    }
    #[doc = "Bit 5 - Invert Input And Output"]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<'_, CtrlSpec> {
        InvW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear RX DMA On Error"]
    #[inline(always)]
    pub fn errsdma(&mut self) -> ErrsdmaW<'_, CtrlSpec> {
        ErrsdmaW::new(self, 6)
    }
    #[doc = "Bit 7 - Loopback Enable"]
    #[inline(always)]
    pub fn loopbk(&mut self) -> LoopbkW<'_, CtrlSpec> {
        LoopbkW::new(self, 7)
    }
    #[doc = "Bit 8 - Start-Frame UnBlock RX"]
    #[inline(always)]
    pub fn sfubrx(&mut self) -> SfubrxW<'_, CtrlSpec> {
        SfubrxW::new(self, 8)
    }
    #[doc = "Bit 9 - Multi-Processor Mode"]
    #[inline(always)]
    pub fn mpm(&mut self) -> MpmW<'_, CtrlSpec> {
        MpmW::new(self, 9)
    }
    #[doc = "Bit 10 - Multi-Processor Address-Bit"]
    #[inline(always)]
    pub fn mpab(&mut self) -> MpabW<'_, CtrlSpec> {
        MpabW::new(self, 10)
    }
    #[doc = "Bit 11 - Bit 8 Default Value"]
    #[inline(always)]
    pub fn bit8dv(&mut self) -> Bit8dvW<'_, CtrlSpec> {
        Bit8dvW::new(self, 11)
    }
    #[doc = "Bit 12 - RX DMA Wakeup"]
    #[inline(always)]
    pub fn rxdmawu(&mut self) -> RxdmawuW<'_, CtrlSpec> {
        RxdmawuW::new(self, 12)
    }
    #[doc = "Bit 13 - TX DMA Wakeup"]
    #[inline(always)]
    pub fn txdmawu(&mut self) -> TxdmawuW<'_, CtrlSpec> {
        TxdmawuW::new(self, 13)
    }
    #[doc = "Bits 14:15 - TX Delay Transmission"]
    #[inline(always)]
    pub fn txdelay(&mut self) -> TxdelayW<'_, CtrlSpec> {
        TxdelayW::new(self, 14)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
