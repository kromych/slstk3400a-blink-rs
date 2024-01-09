#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `WARMUPMODE` reader - Warm-up Mode"]
pub type WARMUPMODE_R = crate::FieldReader<WARMUPMODE_A>;
#[doc = "Warm-up Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WARMUPMODE_A {
    #[doc = "0: ADC is shut down after each conversion"]
    NORMAL = 0,
    #[doc = "1: Bandgap references do not need warm up, but have reduced accuracy."]
    FASTBG = 1,
    #[doc = "2: Reference selected for scan mode is kept warm."]
    KEEPSCANREFWARM = 2,
    #[doc = "3: ADC is kept warmed up and scan reference is kept warm"]
    KEEPADCWARM = 3,
}
impl From<WARMUPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WARMUPMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WARMUPMODE_A {
    type Ux = u8;
}
impl WARMUPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WARMUPMODE_A {
        match self.bits {
            0 => WARMUPMODE_A::NORMAL,
            1 => WARMUPMODE_A::FASTBG,
            2 => WARMUPMODE_A::KEEPSCANREFWARM,
            3 => WARMUPMODE_A::KEEPADCWARM,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC is shut down after each conversion"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == WARMUPMODE_A::NORMAL
    }
    #[doc = "Bandgap references do not need warm up, but have reduced accuracy."]
    #[inline(always)]
    pub fn is_fastbg(&self) -> bool {
        *self == WARMUPMODE_A::FASTBG
    }
    #[doc = "Reference selected for scan mode is kept warm."]
    #[inline(always)]
    pub fn is_keepscanrefwarm(&self) -> bool {
        *self == WARMUPMODE_A::KEEPSCANREFWARM
    }
    #[doc = "ADC is kept warmed up and scan reference is kept warm"]
    #[inline(always)]
    pub fn is_keepadcwarm(&self) -> bool {
        *self == WARMUPMODE_A::KEEPADCWARM
    }
}
#[doc = "Field `WARMUPMODE` writer - Warm-up Mode"]
pub type WARMUPMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WARMUPMODE_A>;
impl<'a, REG> WARMUPMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC is shut down after each conversion"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(WARMUPMODE_A::NORMAL)
    }
    #[doc = "Bandgap references do not need warm up, but have reduced accuracy."]
    #[inline(always)]
    pub fn fastbg(self) -> &'a mut crate::W<REG> {
        self.variant(WARMUPMODE_A::FASTBG)
    }
    #[doc = "Reference selected for scan mode is kept warm."]
    #[inline(always)]
    pub fn keepscanrefwarm(self) -> &'a mut crate::W<REG> {
        self.variant(WARMUPMODE_A::KEEPSCANREFWARM)
    }
    #[doc = "ADC is kept warmed up and scan reference is kept warm"]
    #[inline(always)]
    pub fn keepadcwarm(self) -> &'a mut crate::W<REG> {
        self.variant(WARMUPMODE_A::KEEPADCWARM)
    }
}
#[doc = "Field `TAILGATE` reader - Conversion Tailgating"]
pub type TAILGATE_R = crate::BitReader;
#[doc = "Field `TAILGATE` writer - Conversion Tailgating"]
pub type TAILGATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPFMODE` reader - Low Pass Filter Mode"]
pub type LPFMODE_R = crate::FieldReader<LPFMODE_A>;
#[doc = "Low Pass Filter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPFMODE_A {
    #[doc = "0: No filter or decoupling capacitor"]
    BYPASS = 0,
    #[doc = "1: On chip decoupling capacitor selected"]
    DECAP = 1,
    #[doc = "2: On chip RC filter selected"]
    RCFILT = 2,
}
impl From<LPFMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LPFMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPFMODE_A {
    type Ux = u8;
}
impl LPFMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPFMODE_A> {
        match self.bits {
            0 => Some(LPFMODE_A::BYPASS),
            1 => Some(LPFMODE_A::DECAP),
            2 => Some(LPFMODE_A::RCFILT),
            _ => None,
        }
    }
    #[doc = "No filter or decoupling capacitor"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == LPFMODE_A::BYPASS
    }
    #[doc = "On chip decoupling capacitor selected"]
    #[inline(always)]
    pub fn is_decap(&self) -> bool {
        *self == LPFMODE_A::DECAP
    }
    #[doc = "On chip RC filter selected"]
    #[inline(always)]
    pub fn is_rcfilt(&self) -> bool {
        *self == LPFMODE_A::RCFILT
    }
}
#[doc = "Field `LPFMODE` writer - Low Pass Filter Mode"]
pub type LPFMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LPFMODE_A>;
impl<'a, REG> LPFMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter or decoupling capacitor"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(LPFMODE_A::BYPASS)
    }
    #[doc = "On chip decoupling capacitor selected"]
    #[inline(always)]
    pub fn decap(self) -> &'a mut crate::W<REG> {
        self.variant(LPFMODE_A::DECAP)
    }
    #[doc = "On chip RC filter selected"]
    #[inline(always)]
    pub fn rcfilt(self) -> &'a mut crate::W<REG> {
        self.variant(LPFMODE_A::RCFILT)
    }
}
#[doc = "Field `PRESC` reader - Prescaler Setting"]
pub type PRESC_R = crate::FieldReader<PRESC_A>;
#[doc = "Prescaler Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: `0`"]
    NODIVISION = 0,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC_A {
    type Ux = u8;
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESC_A> {
        match self.bits {
            0 => Some(PRESC_A::NODIVISION),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESC_A::NODIVISION
    }
}
#[doc = "Field `PRESC` writer - Prescaler Setting"]
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 7, PRESC_A>;
impl<'a, REG> PRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::NODIVISION)
    }
}
#[doc = "Field `TIMEBASE` reader - Time Base"]
pub type TIMEBASE_R = crate::FieldReader;
#[doc = "Field `TIMEBASE` writer - Time Base"]
pub type TIMEBASE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `OVSRSEL` reader - Oversample Rate Select"]
pub type OVSRSEL_R = crate::FieldReader<OVSRSEL_A>;
#[doc = "Oversample Rate Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSRSEL_A {
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
impl From<OVSRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSRSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OVSRSEL_A {
    type Ux = u8;
}
impl OVSRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OVSRSEL_A> {
        match self.bits {
            0 => Some(OVSRSEL_A::X2),
            1 => Some(OVSRSEL_A::X4),
            2 => Some(OVSRSEL_A::X8),
            3 => Some(OVSRSEL_A::X16),
            4 => Some(OVSRSEL_A::X32),
            5 => Some(OVSRSEL_A::X64),
            6 => Some(OVSRSEL_A::X128),
            7 => Some(OVSRSEL_A::X256),
            8 => Some(OVSRSEL_A::X512),
            9 => Some(OVSRSEL_A::X1024),
            10 => Some(OVSRSEL_A::X2048),
            11 => Some(OVSRSEL_A::X4096),
            _ => None,
        }
    }
    #[doc = "2 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == OVSRSEL_A::X2
    }
    #[doc = "4 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == OVSRSEL_A::X4
    }
    #[doc = "8 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == OVSRSEL_A::X8
    }
    #[doc = "16 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        *self == OVSRSEL_A::X16
    }
    #[doc = "32 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x32(&self) -> bool {
        *self == OVSRSEL_A::X32
    }
    #[doc = "64 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x64(&self) -> bool {
        *self == OVSRSEL_A::X64
    }
    #[doc = "128 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x128(&self) -> bool {
        *self == OVSRSEL_A::X128
    }
    #[doc = "256 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x256(&self) -> bool {
        *self == OVSRSEL_A::X256
    }
    #[doc = "512 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x512(&self) -> bool {
        *self == OVSRSEL_A::X512
    }
    #[doc = "1024 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x1024(&self) -> bool {
        *self == OVSRSEL_A::X1024
    }
    #[doc = "2048 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x2048(&self) -> bool {
        *self == OVSRSEL_A::X2048
    }
    #[doc = "4096 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x4096(&self) -> bool {
        *self == OVSRSEL_A::X4096
    }
}
#[doc = "Field `OVSRSEL` writer - Oversample Rate Select"]
pub type OVSRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, OVSRSEL_A>;
impl<'a, REG> OVSRSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2 samples for each conversion result"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL_A::X2)
    }
    #[doc = "4 samples for each conversion result"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL_A::X4)
    }
    #[doc = "8 samples for each conversion result"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL_A::X8)
    }
    #[doc = "16 samples for each conversion result"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL_A::X16)
    }
    #[doc = "32 samples for each conversion result"]
    #[inline(always)]
    pub fn x32(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL_A::X32)
    }
    #[doc = "64 samples for each conversion result"]
    #[inline(always)]
    pub fn x64(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL_A::X64)
    }
    #[doc = "128 samples for each conversion result"]
    #[inline(always)]
    pub fn x128(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL_A::X128)
    }
    #[doc = "256 samples for each conversion result"]
    #[inline(always)]
    pub fn x256(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL_A::X256)
    }
    #[doc = "512 samples for each conversion result"]
    #[inline(always)]
    pub fn x512(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL_A::X512)
    }
    #[doc = "1024 samples for each conversion result"]
    #[inline(always)]
    pub fn x1024(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL_A::X1024)
    }
    #[doc = "2048 samples for each conversion result"]
    #[inline(always)]
    pub fn x2048(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL_A::X2048)
    }
    #[doc = "4096 samples for each conversion result"]
    #[inline(always)]
    pub fn x4096(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL_A::X4096)
    }
}
#[doc = "Field `CHCONIDLE` reader - Input channel connected when ADC is IDLE"]
pub type CHCONIDLE_R = crate::BitReader;
#[doc = "Field `CHCONIDLE` writer - Input channel connected when ADC is IDLE"]
pub type CHCONIDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Warm-up Mode"]
    #[inline(always)]
    pub fn warmupmode(&self) -> WARMUPMODE_R {
        WARMUPMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Conversion Tailgating"]
    #[inline(always)]
    pub fn tailgate(&self) -> TAILGATE_R {
        TAILGATE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Low Pass Filter Mode"]
    #[inline(always)]
    pub fn lpfmode(&self) -> LPFMODE_R {
        LPFMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:14 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Time Base"]
    #[inline(always)]
    pub fn timebase(&self) -> TIMEBASE_R {
        TIMEBASE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - Oversample Rate Select"]
    #[inline(always)]
    pub fn ovsrsel(&self) -> OVSRSEL_R {
        OVSRSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Input channel connected when ADC is IDLE"]
    #[inline(always)]
    pub fn chconidle(&self) -> CHCONIDLE_R {
        CHCONIDLE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Warm-up Mode"]
    #[inline(always)]
    #[must_use]
    pub fn warmupmode(&mut self) -> WARMUPMODE_W<CTRL_SPEC> {
        WARMUPMODE_W::new(self, 0)
    }
    #[doc = "Bit 3 - Conversion Tailgating"]
    #[inline(always)]
    #[must_use]
    pub fn tailgate(&mut self) -> TAILGATE_W<CTRL_SPEC> {
        TAILGATE_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Low Pass Filter Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpfmode(&mut self) -> LPFMODE_W<CTRL_SPEC> {
        LPFMODE_W::new(self, 4)
    }
    #[doc = "Bits 8:14 - Prescaler Setting"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<CTRL_SPEC> {
        PRESC_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - Time Base"]
    #[inline(always)]
    #[must_use]
    pub fn timebase(&mut self) -> TIMEBASE_W<CTRL_SPEC> {
        TIMEBASE_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Oversample Rate Select"]
    #[inline(always)]
    #[must_use]
    pub fn ovsrsel(&mut self) -> OVSRSEL_W<CTRL_SPEC> {
        OVSRSEL_W::new(self, 24)
    }
    #[doc = "Bit 28 - Input channel connected when ADC is IDLE"]
    #[inline(always)]
    #[must_use]
    pub fn chconidle(&mut self) -> CHCONIDLE_W<CTRL_SPEC> {
        CHCONIDLE_W::new(self, 28)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x001f_0000"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x001f_0000;
}
