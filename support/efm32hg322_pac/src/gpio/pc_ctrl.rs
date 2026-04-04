#[doc = "Register `PC_CTRL` reader"]
pub type R = crate::R<PcCtrlSpec>;
#[doc = "Register `PC_CTRL` writer"]
pub type W = crate::W<PcCtrlSpec>;
#[doc = "Drive Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Drivemode {
    #[doc = "0: 6 mA drive current"]
    Standard = 0,
    #[doc = "1: 0.1 mA drive current"]
    Lowest = 1,
    #[doc = "2: 20 mA drive current"]
    High = 2,
    #[doc = "3: 1 mA drive current"]
    Low = 3,
}
impl From<Drivemode> for u8 {
    #[inline(always)]
    fn from(variant: Drivemode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Drivemode {
    type Ux = u8;
}
impl crate::IsEnum for Drivemode {}
#[doc = "Field `DRIVEMODE` reader - Drive Mode Select"]
pub type DrivemodeR = crate::FieldReader<Drivemode>;
impl DrivemodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drivemode {
        match self.bits {
            0 => Drivemode::Standard,
            1 => Drivemode::Lowest,
            2 => Drivemode::High,
            3 => Drivemode::Low,
            _ => unreachable!(),
        }
    }
    #[doc = "6 mA drive current"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Drivemode::Standard
    }
    #[doc = "0.1 mA drive current"]
    #[inline(always)]
    pub fn is_lowest(&self) -> bool {
        *self == Drivemode::Lowest
    }
    #[doc = "20 mA drive current"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Drivemode::High
    }
    #[doc = "1 mA drive current"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Drivemode::Low
    }
}
#[doc = "Field `DRIVEMODE` writer - Drive Mode Select"]
pub type DrivemodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Drivemode, crate::Safe>;
impl<'a, REG> DrivemodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "6 mA drive current"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(Drivemode::Standard)
    }
    #[doc = "0.1 mA drive current"]
    #[inline(always)]
    pub fn lowest(self) -> &'a mut crate::W<REG> {
        self.variant(Drivemode::Lowest)
    }
    #[doc = "20 mA drive current"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Drivemode::High)
    }
    #[doc = "1 mA drive current"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Drivemode::Low)
    }
}
impl R {
    #[doc = "Bits 0:1 - Drive Mode Select"]
    #[inline(always)]
    pub fn drivemode(&self) -> DrivemodeR {
        DrivemodeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Drive Mode Select"]
    #[inline(always)]
    pub fn drivemode(&mut self) -> DrivemodeW<'_, PcCtrlSpec> {
        DrivemodeW::new(self, 0)
    }
}
#[doc = "Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcCtrlSpec;
impl crate::RegisterSpec for PcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc_ctrl::R`](R) reader structure"]
impl crate::Readable for PcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pc_ctrl::W`](W) writer structure"]
impl crate::Writable for PcCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PC_CTRL to value 0"]
impl crate::Resettable for PcCtrlSpec {}
