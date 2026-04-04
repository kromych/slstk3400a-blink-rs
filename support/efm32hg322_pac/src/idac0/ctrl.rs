#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EN` reader - Current DAC Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Current DAC Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURSINK` reader - Current Sink Enable"]
pub type CursinkR = crate::BitReader;
#[doc = "Field `CURSINK` writer - Current Sink Enable"]
pub type CursinkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINOUTTRANS` reader - Minimum Output Transition Enable"]
pub type MinouttransR = crate::BitReader;
#[doc = "Field `MINOUTTRANS` writer - Minimum Output Transition Enable"]
pub type MinouttransW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEN` reader - Output Enable"]
pub type OutenR = crate::BitReader;
#[doc = "Field `OUTEN` writer - Output Enable"]
pub type OutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTMODE` reader - Output Modes"]
pub type OutmodeR = crate::BitReader;
#[doc = "Field `OUTMODE` writer - Output Modes"]
pub type OutmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTENPRS` reader - PRS Controlled Output Enable"]
pub type OutenprsR = crate::BitReader;
#[doc = "Field `OUTENPRS` writer - PRS Controlled Output Enable"]
pub type OutenprsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "IDAC Output PRS channnel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prssel {
    #[doc = "0: PRS Channel 0 selected."]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected."]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected."]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected."]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected."]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected."]
    Prsch5 = 5,
}
impl From<Prssel> for u8 {
    #[inline(always)]
    fn from(variant: Prssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prssel {
    type Ux = u8;
}
impl crate::IsEnum for Prssel {}
#[doc = "Field `PRSSEL` reader - IDAC Output PRS channnel Select"]
pub type PrsselR = crate::FieldReader<Prssel>;
impl PrsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prssel> {
        match self.bits {
            0 => Some(Prssel::Prsch0),
            1 => Some(Prssel::Prsch1),
            2 => Some(Prssel::Prsch2),
            3 => Some(Prssel::Prsch3),
            4 => Some(Prssel::Prsch4),
            5 => Some(Prssel::Prsch5),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Prssel::Prsch0
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Prssel::Prsch1
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Prssel::Prsch2
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Prssel::Prsch3
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Prssel::Prsch4
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Prssel::Prsch5
    }
}
#[doc = "Field `PRSSEL` writer - IDAC Output PRS channnel Select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Prssel>;
impl<'a, REG> PrsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch5)
    }
}
impl R {
    #[doc = "Bit 0 - Current DAC Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Current Sink Enable"]
    #[inline(always)]
    pub fn cursink(&self) -> CursinkR {
        CursinkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Minimum Output Transition Enable"]
    #[inline(always)]
    pub fn minouttrans(&self) -> MinouttransR {
        MinouttransR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Enable"]
    #[inline(always)]
    pub fn outen(&self) -> OutenR {
        OutenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output Modes"]
    #[inline(always)]
    pub fn outmode(&self) -> OutmodeR {
        OutmodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 18 - PRS Controlled Output Enable"]
    #[inline(always)]
    pub fn outenprs(&self) -> OutenprsR {
        OutenprsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:22 - IDAC Output PRS channnel Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Current DAC Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Current Sink Enable"]
    #[inline(always)]
    pub fn cursink(&mut self) -> CursinkW<'_, CtrlSpec> {
        CursinkW::new(self, 1)
    }
    #[doc = "Bit 2 - Minimum Output Transition Enable"]
    #[inline(always)]
    pub fn minouttrans(&mut self) -> MinouttransW<'_, CtrlSpec> {
        MinouttransW::new(self, 2)
    }
    #[doc = "Bit 3 - Output Enable"]
    #[inline(always)]
    pub fn outen(&mut self) -> OutenW<'_, CtrlSpec> {
        OutenW::new(self, 3)
    }
    #[doc = "Bit 4 - Output Modes"]
    #[inline(always)]
    pub fn outmode(&mut self) -> OutmodeW<'_, CtrlSpec> {
        OutmodeW::new(self, 4)
    }
    #[doc = "Bit 18 - PRS Controlled Output Enable"]
    #[inline(always)]
    pub fn outenprs(&mut self) -> OutenprsW<'_, CtrlSpec> {
        OutenprsW::new(self, 18)
    }
    #[doc = "Bits 20:22 - IDAC Output PRS channnel Select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<'_, CtrlSpec> {
        PrsselW::new(self, 20)
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
