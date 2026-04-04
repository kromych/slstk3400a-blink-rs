#[doc = "Register `DTCTRL` reader"]
pub type R = crate::R<DtctrlSpec>;
#[doc = "Register `DTCTRL` writer"]
pub type W = crate::W<DtctrlSpec>;
#[doc = "Field `DTEN` reader - DTI Enable"]
pub type DtenR = crate::BitReader;
#[doc = "Field `DTEN` writer - DTI Enable"]
pub type DtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTDAS` reader - DTI Automatic Start-up Functionality"]
pub type DtdasR = crate::BitReader;
#[doc = "Field `DTDAS` writer - DTI Automatic Start-up Functionality"]
pub type DtdasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIPOL` reader - DTI Inactive Polarity"]
pub type DtipolR = crate::BitReader;
#[doc = "Field `DTIPOL` writer - DTI Inactive Polarity"]
pub type DtipolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTCINV` reader - DTI Complementary Output Invert."]
pub type DtcinvR = crate::BitReader;
#[doc = "Field `DTCINV` writer - DTI Complementary Output Invert."]
pub type DtcinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DTI PRS Source Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtprssel {
    #[doc = "0: PRS Channel 0 selected as input"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    Prsch5 = 5,
}
impl From<Dtprssel> for u8 {
    #[inline(always)]
    fn from(variant: Dtprssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtprssel {
    type Ux = u8;
}
impl crate::IsEnum for Dtprssel {}
#[doc = "Field `DTPRSSEL` reader - DTI PRS Source Channel Select"]
pub type DtprsselR = crate::FieldReader<Dtprssel>;
impl DtprsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dtprssel> {
        match self.bits {
            0 => Some(Dtprssel::Prsch0),
            1 => Some(Dtprssel::Prsch1),
            2 => Some(Dtprssel::Prsch2),
            3 => Some(Dtprssel::Prsch3),
            4 => Some(Dtprssel::Prsch4),
            5 => Some(Dtprssel::Prsch5),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Dtprssel::Prsch0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Dtprssel::Prsch1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Dtprssel::Prsch2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Dtprssel::Prsch3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Dtprssel::Prsch4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Dtprssel::Prsch5
    }
}
#[doc = "Field `DTPRSSEL` writer - DTI PRS Source Channel Select"]
pub type DtprsselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dtprssel>;
impl<'a, REG> DtprsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch5)
    }
}
#[doc = "Field `DTPRSEN` reader - DTI PRS Source Enable"]
pub type DtprsenR = crate::BitReader;
#[doc = "Field `DTPRSEN` writer - DTI PRS Source Enable"]
pub type DtprsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DTI Enable"]
    #[inline(always)]
    pub fn dten(&self) -> DtenR {
        DtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline(always)]
    pub fn dtdas(&self) -> DtdasR {
        DtdasR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DTI Inactive Polarity"]
    #[inline(always)]
    pub fn dtipol(&self) -> DtipolR {
        DtipolR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTI Complementary Output Invert."]
    #[inline(always)]
    pub fn dtcinv(&self) -> DtcinvR {
        DtcinvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - DTI PRS Source Channel Select"]
    #[inline(always)]
    pub fn dtprssel(&self) -> DtprsselR {
        DtprsselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 24 - DTI PRS Source Enable"]
    #[inline(always)]
    pub fn dtprsen(&self) -> DtprsenR {
        DtprsenR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTI Enable"]
    #[inline(always)]
    pub fn dten(&mut self) -> DtenW<'_, DtctrlSpec> {
        DtenW::new(self, 0)
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline(always)]
    pub fn dtdas(&mut self) -> DtdasW<'_, DtctrlSpec> {
        DtdasW::new(self, 1)
    }
    #[doc = "Bit 2 - DTI Inactive Polarity"]
    #[inline(always)]
    pub fn dtipol(&mut self) -> DtipolW<'_, DtctrlSpec> {
        DtipolW::new(self, 2)
    }
    #[doc = "Bit 3 - DTI Complementary Output Invert."]
    #[inline(always)]
    pub fn dtcinv(&mut self) -> DtcinvW<'_, DtctrlSpec> {
        DtcinvW::new(self, 3)
    }
    #[doc = "Bits 4:6 - DTI PRS Source Channel Select"]
    #[inline(always)]
    pub fn dtprssel(&mut self) -> DtprsselW<'_, DtctrlSpec> {
        DtprsselW::new(self, 4)
    }
    #[doc = "Bit 24 - DTI PRS Source Enable"]
    #[inline(always)]
    pub fn dtprsen(&mut self) -> DtprsenW<'_, DtctrlSpec> {
        DtprsenW::new(self, 24)
    }
}
#[doc = "DTI Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtctrlSpec;
impl crate::RegisterSpec for DtctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtctrl::R`](R) reader structure"]
impl crate::Readable for DtctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dtctrl::W`](W) writer structure"]
impl crate::Writable for DtctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTCTRL to value 0"]
impl crate::Resettable for DtctrlSpec {}
