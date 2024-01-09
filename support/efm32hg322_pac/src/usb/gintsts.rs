#[doc = "Register `GINTSTS` reader"]
pub type R = crate::R<GINTSTS_SPEC>;
#[doc = "Register `GINTSTS` writer"]
pub type W = crate::W<GINTSTS_SPEC>;
#[doc = "Field `CURMOD` reader - Current Mode of Operation"]
pub type CURMOD_R = crate::BitReader;
#[doc = "Field `SOF` reader - Start of Frame"]
pub type SOF_R = crate::BitReader;
#[doc = "Field `SOF` writer - Start of Frame"]
pub type SOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLVL` reader - RxFIFO Non-Empty"]
pub type RXFLVL_R = crate::BitReader;
#[doc = "Field `GINNAKEFF` reader - Global IN Non-periodic NAK Effective"]
pub type GINNAKEFF_R = crate::BitReader;
#[doc = "Field `GOUTNAKEFF` reader - Global OUT NAK Effective"]
pub type GOUTNAKEFF_R = crate::BitReader;
#[doc = "Field `ERLYSUSP` reader - Early Suspend"]
pub type ERLYSUSP_R = crate::BitReader;
#[doc = "Field `ERLYSUSP` writer - Early Suspend"]
pub type ERLYSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSP` reader - USB Suspend"]
pub type USBSUSP_R = crate::BitReader;
#[doc = "Field `USBSUSP` writer - USB Suspend"]
pub type USBSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRST` reader - USB Reset"]
pub type USBRST_R = crate::BitReader;
#[doc = "Field `USBRST` writer - USB Reset"]
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDONE` reader - Enumeration Done"]
pub type ENUMDONE_R = crate::BitReader;
#[doc = "Field `ENUMDONE` writer - Enumeration Done"]
pub type ENUMDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOOUTDROP` reader - Isochronous OUT Packet Dropped Interrupt"]
pub type ISOOUTDROP_R = crate::BitReader;
#[doc = "Field `ISOOUTDROP` writer - Isochronous OUT Packet Dropped Interrupt"]
pub type ISOOUTDROP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPF` reader - End of Periodic Frame Interrupt"]
pub type EOPF_R = crate::BitReader;
#[doc = "Field `EOPF` writer - End of Periodic Frame Interrupt"]
pub type EOPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPINT` reader - IN Endpoints Interrupt"]
pub type IEPINT_R = crate::BitReader;
#[doc = "Field `OEPINT` reader - OUT Endpoints Interrupt"]
pub type OEPINT_R = crate::BitReader;
#[doc = "Field `INCOMPISOIN` reader - Incomplete Isochronous IN Transfer"]
pub type INCOMPISOIN_R = crate::BitReader;
#[doc = "Field `INCOMPISOIN` writer - Incomplete Isochronous IN Transfer"]
pub type INCOMPISOIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCOMPLP` reader - Incomplete Periodic Transfer"]
pub type INCOMPLP_R = crate::BitReader;
#[doc = "Field `INCOMPLP` writer - Incomplete Periodic Transfer"]
pub type INCOMPLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETSUSP` reader - Data Fetch Suspended"]
pub type FETSUSP_R = crate::BitReader;
#[doc = "Field `FETSUSP` writer - Data Fetch Suspended"]
pub type FETSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETDET` reader - Reset detected Interrupt"]
pub type RESETDET_R = crate::BitReader;
#[doc = "Field `RESETDET` writer - Reset detected Interrupt"]
pub type RESETDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPINT` reader - Resume/Remote Wakeup Detected Interrupt"]
pub type WKUPINT_R = crate::BitReader;
#[doc = "Field `WKUPINT` writer - Resume/Remote Wakeup Detected Interrupt"]
pub type WKUPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Current Mode of Operation"]
    #[inline(always)]
    pub fn curmod(&self) -> CURMOD_R {
        CURMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO Non-Empty"]
    #[inline(always)]
    pub fn rxflvl(&self) -> RXFLVL_R {
        RXFLVL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Global IN Non-periodic NAK Effective"]
    #[inline(always)]
    pub fn ginnakeff(&self) -> GINNAKEFF_R {
        GINNAKEFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective"]
    #[inline(always)]
    pub fn goutnakeff(&self) -> GOUTNAKEFF_R {
        GOUTNAKEFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Suspend"]
    #[inline(always)]
    pub fn erlysusp(&self) -> ERLYSUSP_R {
        ERLYSUSP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB Suspend"]
    #[inline(always)]
    pub fn usbsusp(&self) -> USBSUSP_R {
        USBSUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB Reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration Done"]
    #[inline(always)]
    pub fn enumdone(&self) -> ENUMDONE_R {
        ENUMDONE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt"]
    #[inline(always)]
    pub fn isooutdrop(&self) -> ISOOUTDROP_R {
        ISOOUTDROP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt"]
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer"]
    #[inline(always)]
    pub fn incompisoin(&self) -> INCOMPISOIN_R {
        INCOMPISOIN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer"]
    #[inline(always)]
    pub fn incomplp(&self) -> INCOMPLP_R {
        INCOMPLP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data Fetch Suspended"]
    #[inline(always)]
    pub fn fetsusp(&self) -> FETSUSP_R {
        FETSUSP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset detected Interrupt"]
    #[inline(always)]
    pub fn resetdet(&self) -> RESETDET_R {
        RESETDET_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt"]
    #[inline(always)]
    pub fn wkupint(&self) -> WKUPINT_R {
        WKUPINT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<GINTSTS_SPEC> {
        SOF_W::new(self, 3)
    }
    #[doc = "Bit 10 - Early Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn erlysusp(&mut self) -> ERLYSUSP_W<GINTSTS_SPEC> {
        ERLYSUSP_W::new(self, 10)
    }
    #[doc = "Bit 11 - USB Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn usbsusp(&mut self) -> USBSUSP_W<GINTSTS_SPEC> {
        USBSUSP_W::new(self, 11)
    }
    #[doc = "Bit 12 - USB Reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<GINTSTS_SPEC> {
        USBRST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enumeration Done"]
    #[inline(always)]
    #[must_use]
    pub fn enumdone(&mut self) -> ENUMDONE_W<GINTSTS_SPEC> {
        ENUMDONE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isooutdrop(&mut self) -> ISOOUTDROP_W<GINTSTS_SPEC> {
        ISOOUTDROP_W::new(self, 14)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eopf(&mut self) -> EOPF_W<GINTSTS_SPEC> {
        EOPF_W::new(self, 15)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn incompisoin(&mut self) -> INCOMPISOIN_W<GINTSTS_SPEC> {
        INCOMPISOIN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn incomplp(&mut self) -> INCOMPLP_W<GINTSTS_SPEC> {
        INCOMPLP_W::new(self, 21)
    }
    #[doc = "Bit 22 - Data Fetch Suspended"]
    #[inline(always)]
    #[must_use]
    pub fn fetsusp(&mut self) -> FETSUSP_W<GINTSTS_SPEC> {
        FETSUSP_W::new(self, 22)
    }
    #[doc = "Bit 23 - Reset detected Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn resetdet(&mut self) -> RESETDET_W<GINTSTS_SPEC> {
        RESETDET_W::new(self, 23)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wkupint(&mut self) -> WKUPINT_W<GINTSTS_SPEC> {
        WKUPINT_W::new(self, 31)
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
#[doc = "Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GINTSTS_SPEC;
impl crate::RegisterSpec for GINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintsts::R`](R) reader structure"]
impl crate::Readable for GINTSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gintsts::W`](W) writer structure"]
impl crate::Writable for GINTSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GINTSTS to value 0"]
impl crate::Resettable for GINTSTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
