#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `SYNC` reader - USART Synchronous Mode"]
pub type SyncR = crate::BitReader;
#[doc = "Field `SYNC` writer - USART Synchronous Mode"]
pub type SyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBK` reader - Loopback Enable"]
pub type LoopbkR = crate::BitReader;
#[doc = "Field `LOOPBK` writer - Loopback Enable"]
pub type LoopbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCEN` reader - Collision Check Enable"]
pub type CcenR = crate::BitReader;
#[doc = "Field `CCEN` writer - Collision Check Enable"]
pub type CcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPM` reader - Multi-Processor Mode"]
pub type MpmR = crate::BitReader;
#[doc = "Field `MPM` writer - Multi-Processor Mode"]
pub type MpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAB` reader - Multi-Processor Address-Bit"]
pub type MpabR = crate::BitReader;
#[doc = "Field `MPAB` writer - Multi-Processor Address-Bit"]
pub type MpabW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Oversampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ovs {
    #[doc = "0: Regular UART mode with 16X oversampling in asynchronous mode"]
    X16 = 0,
    #[doc = "1: Double speed with 8X oversampling in asynchronous mode"]
    X8 = 1,
    #[doc = "2: 6X oversampling in asynchronous mode"]
    X6 = 2,
    #[doc = "3: Quadruple speed with 4X oversampling in asynchronous mode"]
    X4 = 3,
}
impl From<Ovs> for u8 {
    #[inline(always)]
    fn from(variant: Ovs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ovs {
    type Ux = u8;
}
impl crate::IsEnum for Ovs {}
#[doc = "Field `OVS` reader - Oversampling"]
pub type OvsR = crate::FieldReader<Ovs>;
impl OvsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovs {
        match self.bits {
            0 => Ovs::X16,
            1 => Ovs::X8,
            2 => Ovs::X6,
            3 => Ovs::X4,
            _ => unreachable!(),
        }
    }
    #[doc = "Regular UART mode with 16X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        *self == Ovs::X16
    }
    #[doc = "Double speed with 8X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == Ovs::X8
    }
    #[doc = "6X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn is_x6(&self) -> bool {
        *self == Ovs::X6
    }
    #[doc = "Quadruple speed with 4X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == Ovs::X4
    }
}
#[doc = "Field `OVS` writer - Oversampling"]
pub type OvsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ovs, crate::Safe>;
impl<'a, REG> OvsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Regular UART mode with 16X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut crate::W<REG> {
        self.variant(Ovs::X16)
    }
    #[doc = "Double speed with 8X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut crate::W<REG> {
        self.variant(Ovs::X8)
    }
    #[doc = "6X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x6(self) -> &'a mut crate::W<REG> {
        self.variant(Ovs::X6)
    }
    #[doc = "Quadruple speed with 4X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut crate::W<REG> {
        self.variant(Ovs::X4)
    }
}
#[doc = "Field `CLKPOL` reader - Clock Polarity"]
pub type ClkpolR = crate::BitReader;
#[doc = "Field `CLKPOL` writer - Clock Polarity"]
pub type ClkpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKPHA` reader - Clock Edge For Setup/Sample"]
pub type ClkphaR = crate::BitReader;
#[doc = "Field `CLKPHA` writer - Clock Edge For Setup/Sample"]
pub type ClkphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSBF` reader - Most Significant Bit First"]
pub type MsbfR = crate::BitReader;
#[doc = "Field `MSBF` writer - Most Significant Bit First"]
pub type MsbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSMA` reader - Action On Slave-Select In Master Mode"]
pub type CsmaR = crate::BitReader;
#[doc = "Field `CSMA` writer - Action On Slave-Select In Master Mode"]
pub type CsmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBIL` reader - TX Buffer Interrupt Level"]
pub type TxbilR = crate::BitReader;
#[doc = "Field `TXBIL` writer - TX Buffer Interrupt Level"]
pub type TxbilW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINV` reader - Receiver Input Invert"]
pub type RxinvR = crate::BitReader;
#[doc = "Field `RXINV` writer - Receiver Input Invert"]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXINV` reader - Transmitter output Invert"]
pub type TxinvR = crate::BitReader;
#[doc = "Field `TXINV` writer - Transmitter output Invert"]
pub type TxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSINV` reader - Chip Select Invert"]
pub type CsinvR = crate::BitReader;
#[doc = "Field `CSINV` writer - Chip Select Invert"]
pub type CsinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCS` reader - Automatic Chip Select"]
pub type AutocsR = crate::BitReader;
#[doc = "Field `AUTOCS` writer - Automatic Chip Select"]
pub type AutocsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOTRI` reader - Automatic TX Tristate"]
pub type AutotriR = crate::BitReader;
#[doc = "Field `AUTOTRI` writer - Automatic TX Tristate"]
pub type AutotriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCMODE` reader - SmartCard Mode"]
pub type ScmodeR = crate::BitReader;
#[doc = "Field `SCMODE` writer - SmartCard Mode"]
pub type ScmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCRETRANS` reader - SmartCard Retransmit"]
pub type ScretransR = crate::BitReader;
#[doc = "Field `SCRETRANS` writer - SmartCard Retransmit"]
pub type ScretransW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SKIPPERRF` reader - Skip Parity Error Frames"]
pub type SkipperrfR = crate::BitReader;
#[doc = "Field `SKIPPERRF` writer - Skip Parity Error Frames"]
pub type SkipperrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT8DV` reader - Bit 8 Default Value"]
pub type Bit8dvR = crate::BitReader;
#[doc = "Field `BIT8DV` writer - Bit 8 Default Value"]
pub type Bit8dvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRSDMA` reader - Halt DMA On Error"]
pub type ErrsdmaR = crate::BitReader;
#[doc = "Field `ERRSDMA` writer - Halt DMA On Error"]
pub type ErrsdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRSRX` reader - Disable RX On Error"]
pub type ErrsrxR = crate::BitReader;
#[doc = "Field `ERRSRX` writer - Disable RX On Error"]
pub type ErrsrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRSTX` reader - Disable TX On Error"]
pub type ErrstxR = crate::BitReader;
#[doc = "Field `ERRSTX` writer - Disable TX On Error"]
pub type ErrstxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSEARLY` reader - Synchronous Slave Setup Early"]
pub type SssearlyR = crate::BitReader;
#[doc = "Field `SSSEARLY` writer - Synchronous Slave Setup Early"]
pub type SssearlyW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `BYTESWAP` reader - Byteswap In Double Accesses"]
pub type ByteswapR = crate::BitReader;
#[doc = "Field `BYTESWAP` writer - Byteswap In Double Accesses"]
pub type ByteswapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOTX` reader - Always Transmit When RX Not Full"]
pub type AutotxR = crate::BitReader;
#[doc = "Field `AUTOTX` writer - Always Transmit When RX Not Full"]
pub type AutotxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MVDIS` reader - Majority Vote Disable"]
pub type MvdisR = crate::BitReader;
#[doc = "Field `MVDIS` writer - Majority Vote Disable"]
pub type MvdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMSDELAY` reader - Synchronous Master Sample Delay"]
pub type SmsdelayR = crate::BitReader;
#[doc = "Field `SMSDELAY` writer - Synchronous Master Sample Delay"]
pub type SmsdelayW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USART Synchronous Mode"]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Loopback Enable"]
    #[inline(always)]
    pub fn loopbk(&self) -> LoopbkR {
        LoopbkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Collision Check Enable"]
    #[inline(always)]
    pub fn ccen(&self) -> CcenR {
        CcenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Multi-Processor Mode"]
    #[inline(always)]
    pub fn mpm(&self) -> MpmR {
        MpmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Multi-Processor Address-Bit"]
    #[inline(always)]
    pub fn mpab(&self) -> MpabR {
        MpabR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Oversampling"]
    #[inline(always)]
    pub fn ovs(&self) -> OvsR {
        OvsR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - Clock Polarity"]
    #[inline(always)]
    pub fn clkpol(&self) -> ClkpolR {
        ClkpolR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock Edge For Setup/Sample"]
    #[inline(always)]
    pub fn clkpha(&self) -> ClkphaR {
        ClkphaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&self) -> MsbfR {
        MsbfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Action On Slave-Select In Master Mode"]
    #[inline(always)]
    pub fn csma(&self) -> CsmaR {
        CsmaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TX Buffer Interrupt Level"]
    #[inline(always)]
    pub fn txbil(&self) -> TxbilR {
        TxbilR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receiver Input Invert"]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmitter output Invert"]
    #[inline(always)]
    pub fn txinv(&self) -> TxinvR {
        TxinvR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Chip Select Invert"]
    #[inline(always)]
    pub fn csinv(&self) -> CsinvR {
        CsinvR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Automatic Chip Select"]
    #[inline(always)]
    pub fn autocs(&self) -> AutocsR {
        AutocsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Automatic TX Tristate"]
    #[inline(always)]
    pub fn autotri(&self) -> AutotriR {
        AutotriR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SmartCard Mode"]
    #[inline(always)]
    pub fn scmode(&self) -> ScmodeR {
        ScmodeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SmartCard Retransmit"]
    #[inline(always)]
    pub fn scretrans(&self) -> ScretransR {
        ScretransR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Skip Parity Error Frames"]
    #[inline(always)]
    pub fn skipperrf(&self) -> SkipperrfR {
        SkipperrfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit 8 Default Value"]
    #[inline(always)]
    pub fn bit8dv(&self) -> Bit8dvR {
        Bit8dvR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Halt DMA On Error"]
    #[inline(always)]
    pub fn errsdma(&self) -> ErrsdmaR {
        ErrsdmaR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Disable RX On Error"]
    #[inline(always)]
    pub fn errsrx(&self) -> ErrsrxR {
        ErrsrxR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable TX On Error"]
    #[inline(always)]
    pub fn errstx(&self) -> ErrstxR {
        ErrstxR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Synchronous Slave Setup Early"]
    #[inline(always)]
    pub fn sssearly(&self) -> SssearlyR {
        SssearlyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - TX Delay Transmission"]
    #[inline(always)]
    pub fn txdelay(&self) -> TxdelayR {
        TxdelayR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - Byteswap In Double Accesses"]
    #[inline(always)]
    pub fn byteswap(&self) -> ByteswapR {
        ByteswapR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Always Transmit When RX Not Full"]
    #[inline(always)]
    pub fn autotx(&self) -> AutotxR {
        AutotxR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Majority Vote Disable"]
    #[inline(always)]
    pub fn mvdis(&self) -> MvdisR {
        MvdisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Synchronous Master Sample Delay"]
    #[inline(always)]
    pub fn smsdelay(&self) -> SmsdelayR {
        SmsdelayR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USART Synchronous Mode"]
    #[inline(always)]
    pub fn sync(&mut self) -> SyncW<'_, CtrlSpec> {
        SyncW::new(self, 0)
    }
    #[doc = "Bit 1 - Loopback Enable"]
    #[inline(always)]
    pub fn loopbk(&mut self) -> LoopbkW<'_, CtrlSpec> {
        LoopbkW::new(self, 1)
    }
    #[doc = "Bit 2 - Collision Check Enable"]
    #[inline(always)]
    pub fn ccen(&mut self) -> CcenW<'_, CtrlSpec> {
        CcenW::new(self, 2)
    }
    #[doc = "Bit 3 - Multi-Processor Mode"]
    #[inline(always)]
    pub fn mpm(&mut self) -> MpmW<'_, CtrlSpec> {
        MpmW::new(self, 3)
    }
    #[doc = "Bit 4 - Multi-Processor Address-Bit"]
    #[inline(always)]
    pub fn mpab(&mut self) -> MpabW<'_, CtrlSpec> {
        MpabW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Oversampling"]
    #[inline(always)]
    pub fn ovs(&mut self) -> OvsW<'_, CtrlSpec> {
        OvsW::new(self, 5)
    }
    #[doc = "Bit 8 - Clock Polarity"]
    #[inline(always)]
    pub fn clkpol(&mut self) -> ClkpolW<'_, CtrlSpec> {
        ClkpolW::new(self, 8)
    }
    #[doc = "Bit 9 - Clock Edge For Setup/Sample"]
    #[inline(always)]
    pub fn clkpha(&mut self) -> ClkphaW<'_, CtrlSpec> {
        ClkphaW::new(self, 9)
    }
    #[doc = "Bit 10 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&mut self) -> MsbfW<'_, CtrlSpec> {
        MsbfW::new(self, 10)
    }
    #[doc = "Bit 11 - Action On Slave-Select In Master Mode"]
    #[inline(always)]
    pub fn csma(&mut self) -> CsmaW<'_, CtrlSpec> {
        CsmaW::new(self, 11)
    }
    #[doc = "Bit 12 - TX Buffer Interrupt Level"]
    #[inline(always)]
    pub fn txbil(&mut self) -> TxbilW<'_, CtrlSpec> {
        TxbilW::new(self, 12)
    }
    #[doc = "Bit 13 - Receiver Input Invert"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RxinvW<'_, CtrlSpec> {
        RxinvW::new(self, 13)
    }
    #[doc = "Bit 14 - Transmitter output Invert"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TxinvW<'_, CtrlSpec> {
        TxinvW::new(self, 14)
    }
    #[doc = "Bit 15 - Chip Select Invert"]
    #[inline(always)]
    pub fn csinv(&mut self) -> CsinvW<'_, CtrlSpec> {
        CsinvW::new(self, 15)
    }
    #[doc = "Bit 16 - Automatic Chip Select"]
    #[inline(always)]
    pub fn autocs(&mut self) -> AutocsW<'_, CtrlSpec> {
        AutocsW::new(self, 16)
    }
    #[doc = "Bit 17 - Automatic TX Tristate"]
    #[inline(always)]
    pub fn autotri(&mut self) -> AutotriW<'_, CtrlSpec> {
        AutotriW::new(self, 17)
    }
    #[doc = "Bit 18 - SmartCard Mode"]
    #[inline(always)]
    pub fn scmode(&mut self) -> ScmodeW<'_, CtrlSpec> {
        ScmodeW::new(self, 18)
    }
    #[doc = "Bit 19 - SmartCard Retransmit"]
    #[inline(always)]
    pub fn scretrans(&mut self) -> ScretransW<'_, CtrlSpec> {
        ScretransW::new(self, 19)
    }
    #[doc = "Bit 20 - Skip Parity Error Frames"]
    #[inline(always)]
    pub fn skipperrf(&mut self) -> SkipperrfW<'_, CtrlSpec> {
        SkipperrfW::new(self, 20)
    }
    #[doc = "Bit 21 - Bit 8 Default Value"]
    #[inline(always)]
    pub fn bit8dv(&mut self) -> Bit8dvW<'_, CtrlSpec> {
        Bit8dvW::new(self, 21)
    }
    #[doc = "Bit 22 - Halt DMA On Error"]
    #[inline(always)]
    pub fn errsdma(&mut self) -> ErrsdmaW<'_, CtrlSpec> {
        ErrsdmaW::new(self, 22)
    }
    #[doc = "Bit 23 - Disable RX On Error"]
    #[inline(always)]
    pub fn errsrx(&mut self) -> ErrsrxW<'_, CtrlSpec> {
        ErrsrxW::new(self, 23)
    }
    #[doc = "Bit 24 - Disable TX On Error"]
    #[inline(always)]
    pub fn errstx(&mut self) -> ErrstxW<'_, CtrlSpec> {
        ErrstxW::new(self, 24)
    }
    #[doc = "Bit 25 - Synchronous Slave Setup Early"]
    #[inline(always)]
    pub fn sssearly(&mut self) -> SssearlyW<'_, CtrlSpec> {
        SssearlyW::new(self, 25)
    }
    #[doc = "Bits 26:27 - TX Delay Transmission"]
    #[inline(always)]
    pub fn txdelay(&mut self) -> TxdelayW<'_, CtrlSpec> {
        TxdelayW::new(self, 26)
    }
    #[doc = "Bit 28 - Byteswap In Double Accesses"]
    #[inline(always)]
    pub fn byteswap(&mut self) -> ByteswapW<'_, CtrlSpec> {
        ByteswapW::new(self, 28)
    }
    #[doc = "Bit 29 - Always Transmit When RX Not Full"]
    #[inline(always)]
    pub fn autotx(&mut self) -> AutotxW<'_, CtrlSpec> {
        AutotxW::new(self, 29)
    }
    #[doc = "Bit 30 - Majority Vote Disable"]
    #[inline(always)]
    pub fn mvdis(&mut self) -> MvdisW<'_, CtrlSpec> {
        MvdisW::new(self, 30)
    }
    #[doc = "Bit 31 - Synchronous Master Sample Delay"]
    #[inline(always)]
    pub fn smsdelay(&mut self) -> SmsdelayW<'_, CtrlSpec> {
        SmsdelayW::new(self, 31)
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
