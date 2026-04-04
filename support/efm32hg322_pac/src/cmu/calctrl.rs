#[doc = "Register `CALCTRL` reader"]
pub type R = crate::R<CalctrlSpec>;
#[doc = "Register `CALCTRL` writer"]
pub type W = crate::W<CalctrlSpec>;
#[doc = "Calibration Up-counter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Upsel {
    #[doc = "0: Select HFXO as up-counter."]
    Hfxo = 0,
    #[doc = "1: Select LFXO as up-counter."]
    Lfxo = 1,
    #[doc = "2: Select HFRCO as up-counter."]
    Hfrco = 2,
    #[doc = "3: Select LFRCO as up-counter."]
    Lfrco = 3,
    #[doc = "4: Select AUXHFRCO as up-counter."]
    Auxhfrco = 4,
    #[doc = "5: Select USHFRCO as up-counter."]
    Ushfrco = 5,
}
impl From<Upsel> for u8 {
    #[inline(always)]
    fn from(variant: Upsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Upsel {
    type Ux = u8;
}
impl crate::IsEnum for Upsel {}
#[doc = "Field `UPSEL` reader - Calibration Up-counter Select"]
pub type UpselR = crate::FieldReader<Upsel>;
impl UpselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Upsel> {
        match self.bits {
            0 => Some(Upsel::Hfxo),
            1 => Some(Upsel::Lfxo),
            2 => Some(Upsel::Hfrco),
            3 => Some(Upsel::Lfrco),
            4 => Some(Upsel::Auxhfrco),
            5 => Some(Upsel::Ushfrco),
            _ => None,
        }
    }
    #[doc = "Select HFXO as up-counter."]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Upsel::Hfxo
    }
    #[doc = "Select LFXO as up-counter."]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Upsel::Lfxo
    }
    #[doc = "Select HFRCO as up-counter."]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == Upsel::Hfrco
    }
    #[doc = "Select LFRCO as up-counter."]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Upsel::Lfrco
    }
    #[doc = "Select AUXHFRCO as up-counter."]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == Upsel::Auxhfrco
    }
    #[doc = "Select USHFRCO as up-counter."]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == Upsel::Ushfrco
    }
}
#[doc = "Field `UPSEL` writer - Calibration Up-counter Select"]
pub type UpselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Upsel>;
impl<'a, REG> UpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select HFXO as up-counter."]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Hfxo)
    }
    #[doc = "Select LFXO as up-counter."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Lfxo)
    }
    #[doc = "Select HFRCO as up-counter."]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Hfrco)
    }
    #[doc = "Select LFRCO as up-counter."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Lfrco)
    }
    #[doc = "Select AUXHFRCO as up-counter."]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Auxhfrco)
    }
    #[doc = "Select USHFRCO as up-counter."]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Ushfrco)
    }
}
#[doc = "Calibration Down-counter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Downsel {
    #[doc = "0: Select HFCLK for down-counter."]
    Hfclk = 0,
    #[doc = "1: Select HFXO for down-counter."]
    Hfxo = 1,
    #[doc = "2: Select LFXO for down-counter."]
    Lfxo = 2,
    #[doc = "3: Select HFRCO for down-counter."]
    Hfrco = 3,
    #[doc = "4: Select LFRCO for down-counter."]
    Lfrco = 4,
    #[doc = "5: Select AUXHFRCO for down-counter."]
    Auxhfrco = 5,
    #[doc = "6: Select USHFRCO for down-counter."]
    Ushfrco = 6,
}
impl From<Downsel> for u8 {
    #[inline(always)]
    fn from(variant: Downsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Downsel {
    type Ux = u8;
}
impl crate::IsEnum for Downsel {}
#[doc = "Field `DOWNSEL` reader - Calibration Down-counter Select"]
pub type DownselR = crate::FieldReader<Downsel>;
impl DownselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Downsel> {
        match self.bits {
            0 => Some(Downsel::Hfclk),
            1 => Some(Downsel::Hfxo),
            2 => Some(Downsel::Lfxo),
            3 => Some(Downsel::Hfrco),
            4 => Some(Downsel::Lfrco),
            5 => Some(Downsel::Auxhfrco),
            6 => Some(Downsel::Ushfrco),
            _ => None,
        }
    }
    #[doc = "Select HFCLK for down-counter."]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == Downsel::Hfclk
    }
    #[doc = "Select HFXO for down-counter."]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Downsel::Hfxo
    }
    #[doc = "Select LFXO for down-counter."]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Downsel::Lfxo
    }
    #[doc = "Select HFRCO for down-counter."]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == Downsel::Hfrco
    }
    #[doc = "Select LFRCO for down-counter."]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Downsel::Lfrco
    }
    #[doc = "Select AUXHFRCO for down-counter."]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == Downsel::Auxhfrco
    }
    #[doc = "Select USHFRCO for down-counter."]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == Downsel::Ushfrco
    }
}
#[doc = "Field `DOWNSEL` writer - Calibration Down-counter Select"]
pub type DownselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Downsel>;
impl<'a, REG> DownselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select HFCLK for down-counter."]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Hfclk)
    }
    #[doc = "Select HFXO for down-counter."]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Hfxo)
    }
    #[doc = "Select LFXO for down-counter."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Lfxo)
    }
    #[doc = "Select HFRCO for down-counter."]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Hfrco)
    }
    #[doc = "Select LFRCO for down-counter."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Lfrco)
    }
    #[doc = "Select AUXHFRCO for down-counter."]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Auxhfrco)
    }
    #[doc = "Select USHFRCO for down-counter."]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Ushfrco)
    }
}
#[doc = "Field `CONT` reader - Continuous Calibration"]
pub type ContR = crate::BitReader;
#[doc = "Field `CONT` writer - Continuous Calibration"]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Calibration Up-counter Select"]
    #[inline(always)]
    pub fn upsel(&self) -> UpselR {
        UpselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Calibration Down-counter Select"]
    #[inline(always)]
    pub fn downsel(&self) -> DownselR {
        DownselR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Continuous Calibration"]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Calibration Up-counter Select"]
    #[inline(always)]
    pub fn upsel(&mut self) -> UpselW<'_, CalctrlSpec> {
        UpselW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Calibration Down-counter Select"]
    #[inline(always)]
    pub fn downsel(&mut self) -> DownselW<'_, CalctrlSpec> {
        DownselW::new(self, 3)
    }
    #[doc = "Bit 6 - Continuous Calibration"]
    #[inline(always)]
    pub fn cont(&mut self) -> ContW<'_, CalctrlSpec> {
        ContW::new(self, 6)
    }
}
#[doc = "Calibration Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`calctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalctrlSpec;
impl crate::RegisterSpec for CalctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calctrl::R`](R) reader structure"]
impl crate::Readable for CalctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`calctrl::W`](W) writer structure"]
impl crate::Writable for CalctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALCTRL to value 0"]
impl crate::Resettable for CalctrlSpec {}
