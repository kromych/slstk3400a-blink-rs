#[doc = "Register `IRCTRL` reader"]
pub type R = crate::R<IrctrlSpec>;
#[doc = "Register `IRCTRL` writer"]
pub type W = crate::W<IrctrlSpec>;
#[doc = "Field `IREN` reader - Enable IrDA Module"]
pub type IrenR = crate::BitReader;
#[doc = "Field `IREN` writer - Enable IrDA Module"]
pub type IrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "IrDA TX Pulse Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpw {
    #[doc = "0: IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    One = 0,
    #[doc = "1: IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    Two = 1,
    #[doc = "2: IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    Three = 2,
    #[doc = "3: IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    Four = 3,
}
impl From<Irpw> for u8 {
    #[inline(always)]
    fn from(variant: Irpw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpw {
    type Ux = u8;
}
impl crate::IsEnum for Irpw {}
#[doc = "Field `IRPW` reader - IrDA TX Pulse Width"]
pub type IrpwR = crate::FieldReader<Irpw>;
impl IrpwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpw {
        match self.bits {
            0 => Irpw::One,
            1 => Irpw::Two,
            2 => Irpw::Three,
            3 => Irpw::Four,
            _ => unreachable!(),
        }
    }
    #[doc = "IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Irpw::One
    }
    #[doc = "IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Irpw::Two
    }
    #[doc = "IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Irpw::Three
    }
    #[doc = "IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Irpw::Four
    }
}
#[doc = "Field `IRPW` writer - IrDA TX Pulse Width"]
pub type IrpwW<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpw, crate::Safe>;
impl<'a, REG> IrpwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Irpw::One)
    }
    #[doc = "IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Irpw::Two)
    }
    #[doc = "IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(Irpw::Three)
    }
    #[doc = "IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(Irpw::Four)
    }
}
#[doc = "Field `IRFILT` reader - IrDA RX Filter"]
pub type IrfiltR = crate::BitReader;
#[doc = "Field `IRFILT` writer - IrDA RX Filter"]
pub type IrfiltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "IrDA PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irprssel {
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
impl From<Irprssel> for u8 {
    #[inline(always)]
    fn from(variant: Irprssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irprssel {
    type Ux = u8;
}
impl crate::IsEnum for Irprssel {}
#[doc = "Field `IRPRSSEL` reader - IrDA PRS Channel Select"]
pub type IrprsselR = crate::FieldReader<Irprssel>;
impl IrprsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Irprssel> {
        match self.bits {
            0 => Some(Irprssel::Prsch0),
            1 => Some(Irprssel::Prsch1),
            2 => Some(Irprssel::Prsch2),
            3 => Some(Irprssel::Prsch3),
            4 => Some(Irprssel::Prsch4),
            5 => Some(Irprssel::Prsch5),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Irprssel::Prsch0
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Irprssel::Prsch1
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Irprssel::Prsch2
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Irprssel::Prsch3
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Irprssel::Prsch4
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Irprssel::Prsch5
    }
}
#[doc = "Field `IRPRSSEL` writer - IrDA PRS Channel Select"]
pub type IrprsselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Irprssel>;
impl<'a, REG> IrprsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Irprssel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Irprssel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Irprssel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Irprssel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Irprssel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Irprssel::Prsch5)
    }
}
#[doc = "Field `IRPRSEN` reader - IrDA PRS Channel Enable"]
pub type IrprsenR = crate::BitReader;
#[doc = "Field `IRPRSEN` writer - IrDA PRS Channel Enable"]
pub type IrprsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline(always)]
    pub fn iren(&self) -> IrenR {
        IrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline(always)]
    pub fn irpw(&self) -> IrpwR {
        IrpwR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline(always)]
    pub fn irfilt(&self) -> IrfiltR {
        IrfiltR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - IrDA PRS Channel Select"]
    #[inline(always)]
    pub fn irprssel(&self) -> IrprsselR {
        IrprsselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - IrDA PRS Channel Enable"]
    #[inline(always)]
    pub fn irprsen(&self) -> IrprsenR {
        IrprsenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline(always)]
    pub fn iren(&mut self) -> IrenW<'_, IrctrlSpec> {
        IrenW::new(self, 0)
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline(always)]
    pub fn irpw(&mut self) -> IrpwW<'_, IrctrlSpec> {
        IrpwW::new(self, 1)
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline(always)]
    pub fn irfilt(&mut self) -> IrfiltW<'_, IrctrlSpec> {
        IrfiltW::new(self, 3)
    }
    #[doc = "Bits 4:6 - IrDA PRS Channel Select"]
    #[inline(always)]
    pub fn irprssel(&mut self) -> IrprsselW<'_, IrctrlSpec> {
        IrprsselW::new(self, 4)
    }
    #[doc = "Bit 7 - IrDA PRS Channel Enable"]
    #[inline(always)]
    pub fn irprsen(&mut self) -> IrprsenW<'_, IrctrlSpec> {
        IrprsenW::new(self, 7)
    }
}
#[doc = "IrDA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrctrlSpec;
impl crate::RegisterSpec for IrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irctrl::R`](R) reader structure"]
impl crate::Readable for IrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`irctrl::W`](W) writer structure"]
impl crate::Writable for IrctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRCTRL to value 0"]
impl crate::Resettable for IrctrlSpec {}
