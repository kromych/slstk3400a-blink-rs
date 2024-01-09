#[doc = "Register `GAHBCFG` reader"]
pub type R = crate::R<GAHBCFG_SPEC>;
#[doc = "Register `GAHBCFG` writer"]
pub type W = crate::W<GAHBCFG_SPEC>;
#[doc = "Field `GLBLINTRMSK` reader - Global Interrupt Mask"]
pub type GLBLINTRMSK_R = crate::BitReader;
#[doc = "Field `GLBLINTRMSK` writer - Global Interrupt Mask"]
pub type GLBLINTRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBSTLEN` reader - Burst Length/Type"]
pub type HBSTLEN_R = crate::FieldReader<HBSTLEN_A>;
#[doc = "Burst Length/Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HBSTLEN_A {
    #[doc = "0: Single transfer."]
    SINGLE = 0,
    #[doc = "1: Incrementing burst of unspecified length."]
    INCR = 1,
    #[doc = "3: 4-beat incrementing burst."]
    INCR4 = 3,
    #[doc = "5: 8-beat incrementing burst."]
    INCR8 = 5,
    #[doc = "7: 16-beat incrementing burst."]
    INCR16 = 7,
}
impl From<HBSTLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: HBSTLEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HBSTLEN_A {
    type Ux = u8;
}
impl HBSTLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HBSTLEN_A> {
        match self.bits {
            0 => Some(HBSTLEN_A::SINGLE),
            1 => Some(HBSTLEN_A::INCR),
            3 => Some(HBSTLEN_A::INCR4),
            5 => Some(HBSTLEN_A::INCR8),
            7 => Some(HBSTLEN_A::INCR16),
            _ => None,
        }
    }
    #[doc = "Single transfer."]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == HBSTLEN_A::SINGLE
    }
    #[doc = "Incrementing burst of unspecified length."]
    #[inline(always)]
    pub fn is_incr(&self) -> bool {
        *self == HBSTLEN_A::INCR
    }
    #[doc = "4-beat incrementing burst."]
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        *self == HBSTLEN_A::INCR4
    }
    #[doc = "8-beat incrementing burst."]
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        *self == HBSTLEN_A::INCR8
    }
    #[doc = "16-beat incrementing burst."]
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        *self == HBSTLEN_A::INCR16
    }
}
#[doc = "Field `HBSTLEN` writer - Burst Length/Type"]
pub type HBSTLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4, HBSTLEN_A>;
impl<'a, REG> HBSTLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single transfer."]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(HBSTLEN_A::SINGLE)
    }
    #[doc = "Incrementing burst of unspecified length."]
    #[inline(always)]
    pub fn incr(self) -> &'a mut crate::W<REG> {
        self.variant(HBSTLEN_A::INCR)
    }
    #[doc = "4-beat incrementing burst."]
    #[inline(always)]
    pub fn incr4(self) -> &'a mut crate::W<REG> {
        self.variant(HBSTLEN_A::INCR4)
    }
    #[doc = "8-beat incrementing burst."]
    #[inline(always)]
    pub fn incr8(self) -> &'a mut crate::W<REG> {
        self.variant(HBSTLEN_A::INCR8)
    }
    #[doc = "16-beat incrementing burst."]
    #[inline(always)]
    pub fn incr16(self) -> &'a mut crate::W<REG> {
        self.variant(HBSTLEN_A::INCR16)
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPTXFEMPLVL` reader - Non-Periodic TxFIFO Empty Level"]
pub type NPTXFEMPLVL_R = crate::BitReader;
#[doc = "Field `NPTXFEMPLVL` writer - Non-Periodic TxFIFO Empty Level"]
pub type NPTXFEMPLVL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REMMEMSUPP` reader - Remote Memory Support"]
pub type REMMEMSUPP_R = crate::BitReader;
#[doc = "Field `REMMEMSUPP` writer - Remote Memory Support"]
pub type REMMEMSUPP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTIALLDMAWRIT` reader - Notify All DMA Writes"]
pub type NOTIALLDMAWRIT_R = crate::BitReader;
#[doc = "Field `NOTIALLDMAWRIT` writer - Notify All DMA Writes"]
pub type NOTIALLDMAWRIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBSINGLE` reader - AHB Single Support"]
pub type AHBSINGLE_R = crate::BitReader;
#[doc = "Field `AHBSINGLE` writer - AHB Single Support"]
pub type AHBSINGLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Global Interrupt Mask"]
    #[inline(always)]
    pub fn glblintrmsk(&self) -> GLBLINTRMSK_R {
        GLBLINTRMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Burst Length/Type"]
    #[inline(always)]
    pub fn hbstlen(&self) -> HBSTLEN_R {
        HBSTLEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level"]
    #[inline(always)]
    pub fn nptxfemplvl(&self) -> NPTXFEMPLVL_R {
        NPTXFEMPLVL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 21 - Remote Memory Support"]
    #[inline(always)]
    pub fn remmemsupp(&self) -> REMMEMSUPP_R {
        REMMEMSUPP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Notify All DMA Writes"]
    #[inline(always)]
    pub fn notialldmawrit(&self) -> NOTIALLDMAWRIT_R {
        NOTIALLDMAWRIT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - AHB Single Support"]
    #[inline(always)]
    pub fn ahbsingle(&self) -> AHBSINGLE_R {
        AHBSINGLE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn glblintrmsk(&mut self) -> GLBLINTRMSK_W<GAHBCFG_SPEC> {
        GLBLINTRMSK_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - Burst Length/Type"]
    #[inline(always)]
    #[must_use]
    pub fn hbstlen(&mut self) -> HBSTLEN_W<GAHBCFG_SPEC> {
        HBSTLEN_W::new(self, 1)
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<GAHBCFG_SPEC> {
        DMAEN_W::new(self, 5)
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfemplvl(&mut self) -> NPTXFEMPLVL_W<GAHBCFG_SPEC> {
        NPTXFEMPLVL_W::new(self, 7)
    }
    #[doc = "Bit 21 - Remote Memory Support"]
    #[inline(always)]
    #[must_use]
    pub fn remmemsupp(&mut self) -> REMMEMSUPP_W<GAHBCFG_SPEC> {
        REMMEMSUPP_W::new(self, 21)
    }
    #[doc = "Bit 22 - Notify All DMA Writes"]
    #[inline(always)]
    #[must_use]
    pub fn notialldmawrit(&mut self) -> NOTIALLDMAWRIT_W<GAHBCFG_SPEC> {
        NOTIALLDMAWRIT_W::new(self, 22)
    }
    #[doc = "Bit 23 - AHB Single Support"]
    #[inline(always)]
    #[must_use]
    pub fn ahbsingle(&mut self) -> AHBSINGLE_W<GAHBCFG_SPEC> {
        AHBSINGLE_W::new(self, 23)
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
#[doc = "AHB Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAHBCFG_SPEC;
impl crate::RegisterSpec for GAHBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gahbcfg::R`](R) reader structure"]
impl crate::Readable for GAHBCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gahbcfg::W`](W) writer structure"]
impl crate::Writable for GAHBCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GAHBCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
