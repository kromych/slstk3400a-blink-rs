#[doc = "Register `IRCTRL` reader"]
pub type R = crate::R<IRCTRL_SPEC>;
#[doc = "Register `IRCTRL` writer"]
pub type W = crate::W<IRCTRL_SPEC>;
#[doc = "Field `IREN` reader - Enable IrDA Module"]
pub type IREN_R = crate::BitReader;
#[doc = "Field `IREN` writer - Enable IrDA Module"]
pub type IREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRPW` reader - IrDA TX Pulse Width"]
pub type IRPW_R = crate::FieldReader<IRPW_A>;
#[doc = "IrDA TX Pulse Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IRPW_A {
    #[doc = "0: IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    ONE = 0,
    #[doc = "1: IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    TWO = 1,
    #[doc = "2: IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    THREE = 2,
    #[doc = "3: IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    FOUR = 3,
}
impl From<IRPW_A> for u8 {
    #[inline(always)]
    fn from(variant: IRPW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IRPW_A {
    type Ux = u8;
}
impl IRPW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IRPW_A {
        match self.bits {
            0 => IRPW_A::ONE,
            1 => IRPW_A::TWO,
            2 => IRPW_A::THREE,
            3 => IRPW_A::FOUR,
            _ => unreachable!(),
        }
    }
    #[doc = "IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == IRPW_A::ONE
    }
    #[doc = "IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == IRPW_A::TWO
    }
    #[doc = "IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == IRPW_A::THREE
    }
    #[doc = "IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == IRPW_A::FOUR
    }
}
#[doc = "Field `IRPW` writer - IrDA TX Pulse Width"]
pub type IRPW_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, IRPW_A>;
impl<'a, REG> IRPW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(IRPW_A::ONE)
    }
    #[doc = "IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(IRPW_A::TWO)
    }
    #[doc = "IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(IRPW_A::THREE)
    }
    #[doc = "IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(IRPW_A::FOUR)
    }
}
#[doc = "Field `IRFILT` reader - IrDA RX Filter"]
pub type IRFILT_R = crate::BitReader;
#[doc = "Field `IRFILT` writer - IrDA RX Filter"]
pub type IRFILT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRPRSSEL` reader - IrDA PRS Channel Select"]
pub type IRPRSSEL_R = crate::FieldReader<IRPRSSEL_A>;
#[doc = "IrDA PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IRPRSSEL_A {
    #[doc = "0: PRS Channel 0 selected"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected"]
    PRSCH5 = 5,
}
impl From<IRPRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IRPRSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IRPRSSEL_A {
    type Ux = u8;
}
impl IRPRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IRPRSSEL_A> {
        match self.bits {
            0 => Some(IRPRSSEL_A::PRSCH0),
            1 => Some(IRPRSSEL_A::PRSCH1),
            2 => Some(IRPRSSEL_A::PRSCH2),
            3 => Some(IRPRSSEL_A::PRSCH3),
            4 => Some(IRPRSSEL_A::PRSCH4),
            5 => Some(IRPRSSEL_A::PRSCH5),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH0
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH1
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH2
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH3
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH4
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH5
    }
}
#[doc = "Field `IRPRSSEL` writer - IrDA PRS Channel Select"]
pub type IRPRSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, IRPRSSEL_A>;
impl<'a, REG> IRPRSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(IRPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(IRPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(IRPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(IRPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(IRPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(IRPRSSEL_A::PRSCH5)
    }
}
#[doc = "Field `IRPRSEN` reader - IrDA PRS Channel Enable"]
pub type IRPRSEN_R = crate::BitReader;
#[doc = "Field `IRPRSEN` writer - IrDA PRS Channel Enable"]
pub type IRPRSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline(always)]
    pub fn irpw(&self) -> IRPW_R {
        IRPW_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline(always)]
    pub fn irfilt(&self) -> IRFILT_R {
        IRFILT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - IrDA PRS Channel Select"]
    #[inline(always)]
    pub fn irprssel(&self) -> IRPRSSEL_R {
        IRPRSSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - IrDA PRS Channel Enable"]
    #[inline(always)]
    pub fn irprsen(&self) -> IRPRSEN_R {
        IRPRSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<IRCTRL_SPEC> {
        IREN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline(always)]
    #[must_use]
    pub fn irpw(&mut self) -> IRPW_W<IRCTRL_SPEC> {
        IRPW_W::new(self, 1)
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline(always)]
    #[must_use]
    pub fn irfilt(&mut self) -> IRFILT_W<IRCTRL_SPEC> {
        IRFILT_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - IrDA PRS Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn irprssel(&mut self) -> IRPRSSEL_W<IRCTRL_SPEC> {
        IRPRSSEL_W::new(self, 4)
    }
    #[doc = "Bit 7 - IrDA PRS Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irprsen(&mut self) -> IRPRSEN_W<IRCTRL_SPEC> {
        IRPRSEN_W::new(self, 7)
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
#[doc = "IrDA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRCTRL_SPEC;
impl crate::RegisterSpec for IRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irctrl::R`](R) reader structure"]
impl crate::Readable for IRCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irctrl::W`](W) writer structure"]
impl crate::Writable for IRCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRCTRL to value 0"]
impl crate::Resettable for IRCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
