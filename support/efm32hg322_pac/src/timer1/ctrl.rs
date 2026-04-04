#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Up-count mode"]
    Up = 0,
    #[doc = "1: Down-count mode"]
    Down = 1,
    #[doc = "2: Up/down-count mode"]
    Updown = 2,
    #[doc = "3: Quadrature decoder mode"]
    Qdec = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Timer Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::Up,
            1 => Mode::Down,
            2 => Mode::Updown,
            3 => Mode::Qdec,
            _ => unreachable!(),
        }
    }
    #[doc = "Up-count mode"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Mode::Up
    }
    #[doc = "Down-count mode"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Mode::Down
    }
    #[doc = "Up/down-count mode"]
    #[inline(always)]
    pub fn is_updown(&self) -> bool {
        *self == Mode::Updown
    }
    #[doc = "Quadrature decoder mode"]
    #[inline(always)]
    pub fn is_qdec(&self) -> bool {
        *self == Mode::Qdec
    }
}
#[doc = "Field `MODE` writer - Timer Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Up-count mode"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Up)
    }
    #[doc = "Down-count mode"]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Down)
    }
    #[doc = "Up/down-count mode"]
    #[inline(always)]
    pub fn updown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Updown)
    }
    #[doc = "Quadrature decoder mode"]
    #[inline(always)]
    pub fn qdec(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Qdec)
    }
}
#[doc = "Field `SYNC` reader - Timer Start/Stop/Reload Synchronization"]
pub type SyncR = crate::BitReader;
#[doc = "Field `SYNC` writer - Timer Start/Stop/Reload Synchronization"]
pub type SyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSMEN` reader - One-shot Mode Enable"]
pub type OsmenR = crate::BitReader;
#[doc = "Field `OSMEN` writer - One-shot Mode Enable"]
pub type OsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QDM` reader - Quadrature Decoder Mode Selection"]
pub type QdmR = crate::BitReader;
#[doc = "Field `QDM` writer - Quadrature Decoder Mode Selection"]
pub type QdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DebugrunR = crate::BitReader;
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DebugrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACLRACT` reader - DMA Request Clear on Active"]
pub type DmaclractR = crate::BitReader;
#[doc = "Field `DMACLRACT` writer - DMA Request Clear on Active"]
pub type DmaclractW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timer Rising Input Edge Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Risea {
    #[doc = "0: No action"]
    None = 0,
    #[doc = "1: Start counter without reload"]
    Start = 1,
    #[doc = "2: Stop counter without reload"]
    Stop = 2,
    #[doc = "3: Reload and start counter"]
    Reloadstart = 3,
}
impl From<Risea> for u8 {
    #[inline(always)]
    fn from(variant: Risea) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Risea {
    type Ux = u8;
}
impl crate::IsEnum for Risea {}
#[doc = "Field `RISEA` reader - Timer Rising Input Edge Action"]
pub type RiseaR = crate::FieldReader<Risea>;
impl RiseaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Risea {
        match self.bits {
            0 => Risea::None,
            1 => Risea::Start,
            2 => Risea::Stop,
            3 => Risea::Reloadstart,
            _ => unreachable!(),
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Risea::None
    }
    #[doc = "Start counter without reload"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Risea::Start
    }
    #[doc = "Stop counter without reload"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Risea::Stop
    }
    #[doc = "Reload and start counter"]
    #[inline(always)]
    pub fn is_reloadstart(&self) -> bool {
        *self == Risea::Reloadstart
    }
}
#[doc = "Field `RISEA` writer - Timer Rising Input Edge Action"]
pub type RiseaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Risea, crate::Safe>;
impl<'a, REG> RiseaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Risea::None)
    }
    #[doc = "Start counter without reload"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Risea::Start)
    }
    #[doc = "Stop counter without reload"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Risea::Stop)
    }
    #[doc = "Reload and start counter"]
    #[inline(always)]
    pub fn reloadstart(self) -> &'a mut crate::W<REG> {
        self.variant(Risea::Reloadstart)
    }
}
#[doc = "Timer Falling Input Edge Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Falla {
    #[doc = "0: No action"]
    None = 0,
    #[doc = "1: Start counter without reload"]
    Start = 1,
    #[doc = "2: Stop counter without reload"]
    Stop = 2,
    #[doc = "3: Reload and start counter"]
    Reloadstart = 3,
}
impl From<Falla> for u8 {
    #[inline(always)]
    fn from(variant: Falla) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Falla {
    type Ux = u8;
}
impl crate::IsEnum for Falla {}
#[doc = "Field `FALLA` reader - Timer Falling Input Edge Action"]
pub type FallaR = crate::FieldReader<Falla>;
impl FallaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Falla {
        match self.bits {
            0 => Falla::None,
            1 => Falla::Start,
            2 => Falla::Stop,
            3 => Falla::Reloadstart,
            _ => unreachable!(),
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Falla::None
    }
    #[doc = "Start counter without reload"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Falla::Start
    }
    #[doc = "Stop counter without reload"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Falla::Stop
    }
    #[doc = "Reload and start counter"]
    #[inline(always)]
    pub fn is_reloadstart(&self) -> bool {
        *self == Falla::Reloadstart
    }
}
#[doc = "Field `FALLA` writer - Timer Falling Input Edge Action"]
pub type FallaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Falla, crate::Safe>;
impl<'a, REG> FallaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Falla::None)
    }
    #[doc = "Start counter without reload"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Falla::Start)
    }
    #[doc = "Stop counter without reload"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Falla::Stop)
    }
    #[doc = "Reload and start counter"]
    #[inline(always)]
    pub fn reloadstart(self) -> &'a mut crate::W<REG> {
        self.variant(Falla::Reloadstart)
    }
}
#[doc = "Field `X2CNT` reader - 2x Count Mode"]
pub type X2cntR = crate::BitReader;
#[doc = "Field `X2CNT` writer - 2x Count Mode"]
pub type X2cntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: Prescaled HFPERCLK"]
    Preschfperclk = 0,
    #[doc = "1: Compare/Capture Channel 1 Input"]
    Cc1 = 1,
    #[doc = "2: Timer is clocked by underflow(down-count) or overflow(up-count) in the lower numbered neighbor Timer"]
    Timerouf = 2,
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clksel {
    type Ux = u8;
}
impl crate::IsEnum for Clksel {}
#[doc = "Field `CLKSEL` reader - Clock Source Select"]
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clksel> {
        match self.bits {
            0 => Some(Clksel::Preschfperclk),
            1 => Some(Clksel::Cc1),
            2 => Some(Clksel::Timerouf),
            _ => None,
        }
    }
    #[doc = "Prescaled HFPERCLK"]
    #[inline(always)]
    pub fn is_preschfperclk(&self) -> bool {
        *self == Clksel::Preschfperclk
    }
    #[doc = "Compare/Capture Channel 1 Input"]
    #[inline(always)]
    pub fn is_cc1(&self) -> bool {
        *self == Clksel::Cc1
    }
    #[doc = "Timer is clocked by underflow(down-count) or overflow(up-count) in the lower numbered neighbor Timer"]
    #[inline(always)]
    pub fn is_timerouf(&self) -> bool {
        *self == Clksel::Timerouf
    }
}
#[doc = "Field `CLKSEL` writer - Clock Source Select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaled HFPERCLK"]
    #[inline(always)]
    pub fn preschfperclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Preschfperclk)
    }
    #[doc = "Compare/Capture Channel 1 Input"]
    #[inline(always)]
    pub fn cc1(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Cc1)
    }
    #[doc = "Timer is clocked by underflow(down-count) or overflow(up-count) in the lower numbered neighbor Timer"]
    #[inline(always)]
    pub fn timerouf(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Timerouf)
    }
}
#[doc = "Prescaler Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Presc {
    #[doc = "0: The HFPERCLK is undivided"]
    Div1 = 0,
    #[doc = "1: The HFPERCLK is divided by 2"]
    Div2 = 1,
    #[doc = "2: The HFPERCLK is divided by 4"]
    Div4 = 2,
    #[doc = "3: The HFPERCLK is divided by 8"]
    Div8 = 3,
    #[doc = "4: The HFPERCLK is divided by 16"]
    Div16 = 4,
    #[doc = "5: The HFPERCLK is divided by 32"]
    Div32 = 5,
    #[doc = "6: The HFPERCLK is divided by 64"]
    Div64 = 6,
    #[doc = "7: The HFPERCLK is divided by 128"]
    Div128 = 7,
    #[doc = "8: The HFPERCLK is divided by 256"]
    Div256 = 8,
    #[doc = "9: The HFPERCLK is divided by 512"]
    Div512 = 9,
    #[doc = "10: The HFPERCLK is divided by 1024"]
    Div1024 = 10,
}
impl From<Presc> for u8 {
    #[inline(always)]
    fn from(variant: Presc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Presc {
    type Ux = u8;
}
impl crate::IsEnum for Presc {}
#[doc = "Field `PRESC` reader - Prescaler Setting"]
pub type PrescR = crate::FieldReader<Presc>;
impl PrescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Presc> {
        match self.bits {
            0 => Some(Presc::Div1),
            1 => Some(Presc::Div2),
            2 => Some(Presc::Div4),
            3 => Some(Presc::Div8),
            4 => Some(Presc::Div16),
            5 => Some(Presc::Div32),
            6 => Some(Presc::Div64),
            7 => Some(Presc::Div128),
            8 => Some(Presc::Div256),
            9 => Some(Presc::Div512),
            10 => Some(Presc::Div1024),
            _ => None,
        }
    }
    #[doc = "The HFPERCLK is undivided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Presc::Div1
    }
    #[doc = "The HFPERCLK is divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Presc::Div2
    }
    #[doc = "The HFPERCLK is divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Presc::Div4
    }
    #[doc = "The HFPERCLK is divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Presc::Div8
    }
    #[doc = "The HFPERCLK is divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Presc::Div16
    }
    #[doc = "The HFPERCLK is divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Presc::Div32
    }
    #[doc = "The HFPERCLK is divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Presc::Div64
    }
    #[doc = "The HFPERCLK is divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Presc::Div128
    }
    #[doc = "The HFPERCLK is divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Presc::Div256
    }
    #[doc = "The HFPERCLK is divided by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == Presc::Div512
    }
    #[doc = "The HFPERCLK is divided by 1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == Presc::Div1024
    }
}
#[doc = "Field `PRESC` writer - Prescaler Setting"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 4, Presc>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The HFPERCLK is undivided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div1)
    }
    #[doc = "The HFPERCLK is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div2)
    }
    #[doc = "The HFPERCLK is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div4)
    }
    #[doc = "The HFPERCLK is divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div8)
    }
    #[doc = "The HFPERCLK is divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div16)
    }
    #[doc = "The HFPERCLK is divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div32)
    }
    #[doc = "The HFPERCLK is divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div64)
    }
    #[doc = "The HFPERCLK is divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div128)
    }
    #[doc = "The HFPERCLK is divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div256)
    }
    #[doc = "The HFPERCLK is divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div512)
    }
    #[doc = "The HFPERCLK is divided by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div1024)
    }
}
#[doc = "Field `ATI` reader - Always Track Inputs"]
pub type AtiR = crate::BitReader;
#[doc = "Field `ATI` writer - Always Track Inputs"]
pub type AtiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSSCOIST` reader - Reload-Start Sets Compare Output initial State"]
pub type RsscoistR = crate::BitReader;
#[doc = "Field `RSSCOIST` writer - Reload-Start Sets Compare Output initial State"]
pub type RsscoistW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Timer Start/Stop/Reload Synchronization"]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - One-shot Mode Enable"]
    #[inline(always)]
    pub fn osmen(&self) -> OsmenR {
        OsmenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Quadrature Decoder Mode Selection"]
    #[inline(always)]
    pub fn qdm(&self) -> QdmR {
        QdmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DebugrunR {
        DebugrunR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Request Clear on Active"]
    #[inline(always)]
    pub fn dmaclract(&self) -> DmaclractR {
        DmaclractR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Timer Rising Input Edge Action"]
    #[inline(always)]
    pub fn risea(&self) -> RiseaR {
        RiseaR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Timer Falling Input Edge Action"]
    #[inline(always)]
    pub fn falla(&self) -> FallaR {
        FallaR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 13 - 2x Count Mode"]
    #[inline(always)]
    pub fn x2cnt(&self) -> X2cntR {
        X2cntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Clock Source Select"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Always Track Inputs"]
    #[inline(always)]
    pub fn ati(&self) -> AtiR {
        AtiR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reload-Start Sets Compare Output initial State"]
    #[inline(always)]
    pub fn rsscoist(&self) -> RsscoistR {
        RsscoistR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CtrlSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 3 - Timer Start/Stop/Reload Synchronization"]
    #[inline(always)]
    pub fn sync(&mut self) -> SyncW<'_, CtrlSpec> {
        SyncW::new(self, 3)
    }
    #[doc = "Bit 4 - One-shot Mode Enable"]
    #[inline(always)]
    pub fn osmen(&mut self) -> OsmenW<'_, CtrlSpec> {
        OsmenW::new(self, 4)
    }
    #[doc = "Bit 5 - Quadrature Decoder Mode Selection"]
    #[inline(always)]
    pub fn qdm(&mut self) -> QdmW<'_, CtrlSpec> {
        QdmW::new(self, 5)
    }
    #[doc = "Bit 6 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DebugrunW<'_, CtrlSpec> {
        DebugrunW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA Request Clear on Active"]
    #[inline(always)]
    pub fn dmaclract(&mut self) -> DmaclractW<'_, CtrlSpec> {
        DmaclractW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Timer Rising Input Edge Action"]
    #[inline(always)]
    pub fn risea(&mut self) -> RiseaW<'_, CtrlSpec> {
        RiseaW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Timer Falling Input Edge Action"]
    #[inline(always)]
    pub fn falla(&mut self) -> FallaW<'_, CtrlSpec> {
        FallaW::new(self, 10)
    }
    #[doc = "Bit 13 - 2x Count Mode"]
    #[inline(always)]
    pub fn x2cnt(&mut self) -> X2cntW<'_, CtrlSpec> {
        X2cntW::new(self, 13)
    }
    #[doc = "Bits 16:17 - Clock Source Select"]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<'_, CtrlSpec> {
        ClkselW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<'_, CtrlSpec> {
        PrescW::new(self, 24)
    }
    #[doc = "Bit 28 - Always Track Inputs"]
    #[inline(always)]
    pub fn ati(&mut self) -> AtiW<'_, CtrlSpec> {
        AtiW::new(self, 28)
    }
    #[doc = "Bit 29 - Reload-Start Sets Compare Output initial State"]
    #[inline(always)]
    pub fn rsscoist(&mut self) -> RsscoistW<'_, CtrlSpec> {
        RsscoistW::new(self, 29)
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
