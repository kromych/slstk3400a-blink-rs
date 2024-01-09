#[doc = "Register `READCTRL` reader"]
pub type R = crate::R<READCTRL_SPEC>;
#[doc = "Register `READCTRL` writer"]
pub type W = crate::W<READCTRL_SPEC>;
#[doc = "Field `MODE` reader - Read Mode"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "Read Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Zero wait-states inserted in fetch or read transfers."]
    WS0 = 0,
    #[doc = "1: One wait-state inserted for each fetch or read transfer. This mode is required for a core frequency above 16 MHz."]
    WS1 = 1,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::WS0),
            1 => Some(MODE_A::WS1),
            _ => None,
        }
    }
    #[doc = "Zero wait-states inserted in fetch or read transfers."]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == MODE_A::WS0
    }
    #[doc = "One wait-state inserted for each fetch or read transfer. This mode is required for a core frequency above 16 MHz."]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == MODE_A::WS1
    }
}
#[doc = "Field `MODE` writer - Read Mode"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Zero wait-states inserted in fetch or read transfers."]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::WS0)
    }
    #[doc = "One wait-state inserted for each fetch or read transfer. This mode is required for a core frequency above 16 MHz."]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::WS1)
    }
}
#[doc = "Field `IFCDIS` reader - Internal Flash Cache Disable"]
pub type IFCDIS_R = crate::BitReader;
#[doc = "Field `IFCDIS` writer - Internal Flash Cache Disable"]
pub type IFCDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIDIS` reader - Automatic Invalidate Disable"]
pub type AIDIS_R = crate::BitReader;
#[doc = "Field `AIDIS` writer - Automatic Invalidate Disable"]
pub type AIDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMCEN` reader - RAM Cache Enable"]
pub type RAMCEN_R = crate::BitReader;
#[doc = "Field `RAMCEN` writer - RAM Cache Enable"]
pub type RAMCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Read Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Internal Flash Cache Disable"]
    #[inline(always)]
    pub fn ifcdis(&self) -> IFCDIS_R {
        IFCDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic Invalidate Disable"]
    #[inline(always)]
    pub fn aidis(&self) -> AIDIS_R {
        AIDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - RAM Cache Enable"]
    #[inline(always)]
    pub fn ramcen(&self) -> RAMCEN_R {
        RAMCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Read Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<READCTRL_SPEC> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bit 3 - Internal Flash Cache Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ifcdis(&mut self) -> IFCDIS_W<READCTRL_SPEC> {
        IFCDIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Automatic Invalidate Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aidis(&mut self) -> AIDIS_W<READCTRL_SPEC> {
        AIDIS_W::new(self, 4)
    }
    #[doc = "Bit 7 - RAM Cache Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ramcen(&mut self) -> RAMCEN_W<READCTRL_SPEC> {
        RAMCEN_W::new(self, 7)
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
#[doc = "Read Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`readctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`readctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct READCTRL_SPEC;
impl crate::RegisterSpec for READCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`readctrl::R`](R) reader structure"]
impl crate::Readable for READCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`readctrl::W`](W) writer structure"]
impl crate::Writable for READCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets READCTRL to value 0x01"]
impl crate::Resettable for READCTRL_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
