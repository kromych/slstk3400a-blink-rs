#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Warm-up Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Warmupmode {
    #[doc = "0: ADC is shut down after each conversion"]
    Normal = 0,
    #[doc = "1: Bandgap references do not need warm up, but have reduced accuracy."]
    Fastbg = 1,
    #[doc = "2: Reference selected for scan mode is kept warm."]
    Keepscanrefwarm = 2,
    #[doc = "3: ADC is kept warmed up and scan reference is kept warm"]
    Keepadcwarm = 3,
}
impl From<Warmupmode> for u8 {
    #[inline(always)]
    fn from(variant: Warmupmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Warmupmode {
    type Ux = u8;
}
impl crate::IsEnum for Warmupmode {}
#[doc = "Field `WARMUPMODE` reader - Warm-up Mode"]
pub type WarmupmodeR = crate::FieldReader<Warmupmode>;
impl WarmupmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Warmupmode {
        match self.bits {
            0 => Warmupmode::Normal,
            1 => Warmupmode::Fastbg,
            2 => Warmupmode::Keepscanrefwarm,
            3 => Warmupmode::Keepadcwarm,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC is shut down after each conversion"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Warmupmode::Normal
    }
    #[doc = "Bandgap references do not need warm up, but have reduced accuracy."]
    #[inline(always)]
    pub fn is_fastbg(&self) -> bool {
        *self == Warmupmode::Fastbg
    }
    #[doc = "Reference selected for scan mode is kept warm."]
    #[inline(always)]
    pub fn is_keepscanrefwarm(&self) -> bool {
        *self == Warmupmode::Keepscanrefwarm
    }
    #[doc = "ADC is kept warmed up and scan reference is kept warm"]
    #[inline(always)]
    pub fn is_keepadcwarm(&self) -> bool {
        *self == Warmupmode::Keepadcwarm
    }
}
#[doc = "Field `WARMUPMODE` writer - Warm-up Mode"]
pub type WarmupmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Warmupmode, crate::Safe>;
impl<'a, REG> WarmupmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC is shut down after each conversion"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Warmupmode::Normal)
    }
    #[doc = "Bandgap references do not need warm up, but have reduced accuracy."]
    #[inline(always)]
    pub fn fastbg(self) -> &'a mut crate::W<REG> {
        self.variant(Warmupmode::Fastbg)
    }
    #[doc = "Reference selected for scan mode is kept warm."]
    #[inline(always)]
    pub fn keepscanrefwarm(self) -> &'a mut crate::W<REG> {
        self.variant(Warmupmode::Keepscanrefwarm)
    }
    #[doc = "ADC is kept warmed up and scan reference is kept warm"]
    #[inline(always)]
    pub fn keepadcwarm(self) -> &'a mut crate::W<REG> {
        self.variant(Warmupmode::Keepadcwarm)
    }
}
#[doc = "Field `TAILGATE` reader - Conversion Tailgating"]
pub type TailgateR = crate::BitReader;
#[doc = "Field `TAILGATE` writer - Conversion Tailgating"]
pub type TailgateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Low Pass Filter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpfmode {
    #[doc = "0: No filter or decoupling capacitor"]
    Bypass = 0,
    #[doc = "1: On chip decoupling capacitor selected"]
    Decap = 1,
    #[doc = "2: On chip RC filter selected"]
    Rcfilt = 2,
}
impl From<Lpfmode> for u8 {
    #[inline(always)]
    fn from(variant: Lpfmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpfmode {
    type Ux = u8;
}
impl crate::IsEnum for Lpfmode {}
#[doc = "Field `LPFMODE` reader - Low Pass Filter Mode"]
pub type LpfmodeR = crate::FieldReader<Lpfmode>;
impl LpfmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lpfmode> {
        match self.bits {
            0 => Some(Lpfmode::Bypass),
            1 => Some(Lpfmode::Decap),
            2 => Some(Lpfmode::Rcfilt),
            _ => None,
        }
    }
    #[doc = "No filter or decoupling capacitor"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Lpfmode::Bypass
    }
    #[doc = "On chip decoupling capacitor selected"]
    #[inline(always)]
    pub fn is_decap(&self) -> bool {
        *self == Lpfmode::Decap
    }
    #[doc = "On chip RC filter selected"]
    #[inline(always)]
    pub fn is_rcfilt(&self) -> bool {
        *self == Lpfmode::Rcfilt
    }
}
#[doc = "Field `LPFMODE` writer - Low Pass Filter Mode"]
pub type LpfmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lpfmode>;
impl<'a, REG> LpfmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter or decoupling capacitor"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Lpfmode::Bypass)
    }
    #[doc = "On chip decoupling capacitor selected"]
    #[inline(always)]
    pub fn decap(self) -> &'a mut crate::W<REG> {
        self.variant(Lpfmode::Decap)
    }
    #[doc = "On chip RC filter selected"]
    #[inline(always)]
    pub fn rcfilt(self) -> &'a mut crate::W<REG> {
        self.variant(Lpfmode::Rcfilt)
    }
}
#[doc = "Prescaler Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Presc {
    #[doc = "0: `0`"]
    Nodivision = 0,
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
            0 => Some(Presc::Nodivision),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == Presc::Nodivision
    }
}
#[doc = "Field `PRESC` writer - Prescaler Setting"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 7, Presc>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Nodivision)
    }
}
#[doc = "Field `TIMEBASE` reader - Time Base"]
pub type TimebaseR = crate::FieldReader;
#[doc = "Field `TIMEBASE` writer - Time Base"]
pub type TimebaseW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Oversample Rate Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ovsrsel {
    #[doc = "0: 2 samples for each conversion result"]
    X2 = 0,
    #[doc = "1: 4 samples for each conversion result"]
    X4 = 1,
    #[doc = "2: 8 samples for each conversion result"]
    X8 = 2,
    #[doc = "3: 16 samples for each conversion result"]
    X16 = 3,
    #[doc = "4: 32 samples for each conversion result"]
    X32 = 4,
    #[doc = "5: 64 samples for each conversion result"]
    X64 = 5,
    #[doc = "6: 128 samples for each conversion result"]
    X128 = 6,
    #[doc = "7: 256 samples for each conversion result"]
    X256 = 7,
    #[doc = "8: 512 samples for each conversion result"]
    X512 = 8,
    #[doc = "9: 1024 samples for each conversion result"]
    X1024 = 9,
    #[doc = "10: 2048 samples for each conversion result"]
    X2048 = 10,
    #[doc = "11: 4096 samples for each conversion result"]
    X4096 = 11,
}
impl From<Ovsrsel> for u8 {
    #[inline(always)]
    fn from(variant: Ovsrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ovsrsel {
    type Ux = u8;
}
impl crate::IsEnum for Ovsrsel {}
#[doc = "Field `OVSRSEL` reader - Oversample Rate Select"]
pub type OvsrselR = crate::FieldReader<Ovsrsel>;
impl OvsrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ovsrsel> {
        match self.bits {
            0 => Some(Ovsrsel::X2),
            1 => Some(Ovsrsel::X4),
            2 => Some(Ovsrsel::X8),
            3 => Some(Ovsrsel::X16),
            4 => Some(Ovsrsel::X32),
            5 => Some(Ovsrsel::X64),
            6 => Some(Ovsrsel::X128),
            7 => Some(Ovsrsel::X256),
            8 => Some(Ovsrsel::X512),
            9 => Some(Ovsrsel::X1024),
            10 => Some(Ovsrsel::X2048),
            11 => Some(Ovsrsel::X4096),
            _ => None,
        }
    }
    #[doc = "2 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == Ovsrsel::X2
    }
    #[doc = "4 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == Ovsrsel::X4
    }
    #[doc = "8 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == Ovsrsel::X8
    }
    #[doc = "16 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        *self == Ovsrsel::X16
    }
    #[doc = "32 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x32(&self) -> bool {
        *self == Ovsrsel::X32
    }
    #[doc = "64 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x64(&self) -> bool {
        *self == Ovsrsel::X64
    }
    #[doc = "128 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x128(&self) -> bool {
        *self == Ovsrsel::X128
    }
    #[doc = "256 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x256(&self) -> bool {
        *self == Ovsrsel::X256
    }
    #[doc = "512 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x512(&self) -> bool {
        *self == Ovsrsel::X512
    }
    #[doc = "1024 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x1024(&self) -> bool {
        *self == Ovsrsel::X1024
    }
    #[doc = "2048 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x2048(&self) -> bool {
        *self == Ovsrsel::X2048
    }
    #[doc = "4096 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x4096(&self) -> bool {
        *self == Ovsrsel::X4096
    }
}
#[doc = "Field `OVSRSEL` writer - Oversample Rate Select"]
pub type OvsrselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ovsrsel>;
impl<'a, REG> OvsrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2 samples for each conversion result"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsrsel::X2)
    }
    #[doc = "4 samples for each conversion result"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsrsel::X4)
    }
    #[doc = "8 samples for each conversion result"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsrsel::X8)
    }
    #[doc = "16 samples for each conversion result"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsrsel::X16)
    }
    #[doc = "32 samples for each conversion result"]
    #[inline(always)]
    pub fn x32(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsrsel::X32)
    }
    #[doc = "64 samples for each conversion result"]
    #[inline(always)]
    pub fn x64(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsrsel::X64)
    }
    #[doc = "128 samples for each conversion result"]
    #[inline(always)]
    pub fn x128(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsrsel::X128)
    }
    #[doc = "256 samples for each conversion result"]
    #[inline(always)]
    pub fn x256(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsrsel::X256)
    }
    #[doc = "512 samples for each conversion result"]
    #[inline(always)]
    pub fn x512(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsrsel::X512)
    }
    #[doc = "1024 samples for each conversion result"]
    #[inline(always)]
    pub fn x1024(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsrsel::X1024)
    }
    #[doc = "2048 samples for each conversion result"]
    #[inline(always)]
    pub fn x2048(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsrsel::X2048)
    }
    #[doc = "4096 samples for each conversion result"]
    #[inline(always)]
    pub fn x4096(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsrsel::X4096)
    }
}
#[doc = "Field `CHCONIDLE` reader - Input channel connected when ADC is IDLE"]
pub type ChconidleR = crate::BitReader;
#[doc = "Field `CHCONIDLE` writer - Input channel connected when ADC is IDLE"]
pub type ChconidleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Warm-up Mode"]
    #[inline(always)]
    pub fn warmupmode(&self) -> WarmupmodeR {
        WarmupmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Conversion Tailgating"]
    #[inline(always)]
    pub fn tailgate(&self) -> TailgateR {
        TailgateR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Low Pass Filter Mode"]
    #[inline(always)]
    pub fn lpfmode(&self) -> LpfmodeR {
        LpfmodeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:14 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Time Base"]
    #[inline(always)]
    pub fn timebase(&self) -> TimebaseR {
        TimebaseR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - Oversample Rate Select"]
    #[inline(always)]
    pub fn ovsrsel(&self) -> OvsrselR {
        OvsrselR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Input channel connected when ADC is IDLE"]
    #[inline(always)]
    pub fn chconidle(&self) -> ChconidleR {
        ChconidleR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Warm-up Mode"]
    #[inline(always)]
    pub fn warmupmode(&mut self) -> WarmupmodeW<'_, CtrlSpec> {
        WarmupmodeW::new(self, 0)
    }
    #[doc = "Bit 3 - Conversion Tailgating"]
    #[inline(always)]
    pub fn tailgate(&mut self) -> TailgateW<'_, CtrlSpec> {
        TailgateW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Low Pass Filter Mode"]
    #[inline(always)]
    pub fn lpfmode(&mut self) -> LpfmodeW<'_, CtrlSpec> {
        LpfmodeW::new(self, 4)
    }
    #[doc = "Bits 8:14 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<'_, CtrlSpec> {
        PrescW::new(self, 8)
    }
    #[doc = "Bits 16:22 - Time Base"]
    #[inline(always)]
    pub fn timebase(&mut self) -> TimebaseW<'_, CtrlSpec> {
        TimebaseW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Oversample Rate Select"]
    #[inline(always)]
    pub fn ovsrsel(&mut self) -> OvsrselW<'_, CtrlSpec> {
        OvsrselW::new(self, 24)
    }
    #[doc = "Bit 28 - Input channel connected when ADC is IDLE"]
    #[inline(always)]
    pub fn chconidle(&mut self) -> ChconidleW<'_, CtrlSpec> {
        ChconidleW::new(self, 28)
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
#[doc = "`reset()` method sets CTRL to value 0x001f_0000"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x001f_0000;
}
