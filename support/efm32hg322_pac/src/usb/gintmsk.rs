#[doc = "Register `GINTMSK` reader"]
pub type R = crate::R<GINTMSK_SPEC>;
#[doc = "Register `GINTMSK` writer"]
pub type W = crate::W<GINTMSK_SPEC>;
#[doc = "Field `MODEMISMSK` reader - Mode Mismatch Interrupt Mask"]
pub type MODEMISMSK_R = crate::BitReader;
#[doc = "Field `MODEMISMSK` writer - Mode Mismatch Interrupt Mask"]
pub type MODEMISMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFMSK` reader - Start of Frame Mask"]
pub type SOFMSK_R = crate::BitReader;
#[doc = "Field `SOFMSK` writer - Start of Frame Mask"]
pub type SOFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLVLMSK` reader - Receive FIFO Non-Empty Mask"]
pub type RXFLVLMSK_R = crate::BitReader;
#[doc = "Field `RXFLVLMSK` writer - Receive FIFO Non-Empty Mask"]
pub type RXFLVLMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINNAKEFFMSK` reader - Global Non-periodic IN NAK Effective Mask"]
pub type GINNAKEFFMSK_R = crate::BitReader;
#[doc = "Field `GINNAKEFFMSK` writer - Global Non-periodic IN NAK Effective Mask"]
pub type GINNAKEFFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOUTNAKEFFMSK` reader - Global OUT NAK Effective Mask"]
pub type GOUTNAKEFFMSK_R = crate::BitReader;
#[doc = "Field `GOUTNAKEFFMSK` writer - Global OUT NAK Effective Mask"]
pub type GOUTNAKEFFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERLYSUSPMSK` reader - Early Suspend Mask"]
pub type ERLYSUSPMSK_R = crate::BitReader;
#[doc = "Field `ERLYSUSPMSK` writer - Early Suspend Mask"]
pub type ERLYSUSPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSPMSK` reader - USB Suspend Mask"]
pub type USBSUSPMSK_R = crate::BitReader;
#[doc = "Field `USBSUSPMSK` writer - USB Suspend Mask"]
pub type USBSUSPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRSTMSK` reader - USB Reset Mask"]
pub type USBRSTMSK_R = crate::BitReader;
#[doc = "Field `USBRSTMSK` writer - USB Reset Mask"]
pub type USBRSTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDONEMSK` reader - Enumeration Done Mask"]
pub type ENUMDONEMSK_R = crate::BitReader;
#[doc = "Field `ENUMDONEMSK` writer - Enumeration Done Mask"]
pub type ENUMDONEMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOOUTDROPMSK` reader - Isochronous OUT Packet Dropped Interrupt Mask"]
pub type ISOOUTDROPMSK_R = crate::BitReader;
#[doc = "Field `ISOOUTDROPMSK` writer - Isochronous OUT Packet Dropped Interrupt Mask"]
pub type ISOOUTDROPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPFMSK` reader - End of Periodic Frame Interrupt Mask"]
pub type EOPFMSK_R = crate::BitReader;
#[doc = "Field `EOPFMSK` writer - End of Periodic Frame Interrupt Mask"]
pub type EOPFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPINTMSK` reader - IN Endpoints Interrupt Mask"]
pub type IEPINTMSK_R = crate::BitReader;
#[doc = "Field `IEPINTMSK` writer - IN Endpoints Interrupt Mask"]
pub type IEPINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEPINTMSK` reader - OUT Endpoints Interrupt Mask"]
pub type OEPINTMSK_R = crate::BitReader;
#[doc = "Field `OEPINTMSK` writer - OUT Endpoints Interrupt Mask"]
pub type OEPINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCOMPISOINMSK` reader - Incomplete Isochronous IN Transfer Mask"]
pub type INCOMPISOINMSK_R = crate::BitReader;
#[doc = "Field `INCOMPISOINMSK` writer - Incomplete Isochronous IN Transfer Mask"]
pub type INCOMPISOINMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCOMPLPMSK` reader - Incomplete Periodic Transfer Mask"]
pub type INCOMPLPMSK_R = crate::BitReader;
#[doc = "Field `INCOMPLPMSK` writer - Incomplete Periodic Transfer Mask"]
pub type INCOMPLPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETSUSPMSK` reader - Data Fetch Suspended Mask"]
pub type FETSUSPMSK_R = crate::BitReader;
#[doc = "Field `FETSUSPMSK` writer - Data Fetch Suspended Mask"]
pub type FETSUSPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETDETMSK` reader - Reset detected Interrupt Mask"]
pub type RESETDETMSK_R = crate::BitReader;
#[doc = "Field `RESETDETMSK` writer - Reset detected Interrupt Mask"]
pub type RESETDETMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPINTMSK` reader - Resume/Remote Wakeup Detected Interrupt Mask"]
pub type WKUPINTMSK_R = crate::BitReader;
#[doc = "Field `WKUPINTMSK` writer - Resume/Remote Wakeup Detected Interrupt Mask"]
pub type WKUPINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn modemismsk(&self) -> MODEMISMSK_R {
        MODEMISMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Frame Mask"]
    #[inline(always)]
    pub fn sofmsk(&self) -> SOFMSK_R {
        SOFMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    pub fn rxflvlmsk(&self) -> RXFLVLMSK_R {
        RXFLVLMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Global Non-periodic IN NAK Effective Mask"]
    #[inline(always)]
    pub fn ginnakeffmsk(&self) -> GINNAKEFFMSK_R {
        GINNAKEFFMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective Mask"]
    #[inline(always)]
    pub fn goutnakeffmsk(&self) -> GOUTNAKEFFMSK_R {
        GOUTNAKEFFMSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Suspend Mask"]
    #[inline(always)]
    pub fn erlysuspmsk(&self) -> ERLYSUSPMSK_R {
        ERLYSUSPMSK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB Suspend Mask"]
    #[inline(always)]
    pub fn usbsuspmsk(&self) -> USBSUSPMSK_R {
        USBSUSPMSK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB Reset Mask"]
    #[inline(always)]
    pub fn usbrstmsk(&self) -> USBRSTMSK_R {
        USBRSTMSK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration Done Mask"]
    #[inline(always)]
    pub fn enumdonemsk(&self) -> ENUMDONEMSK_R {
        ENUMDONEMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt Mask"]
    #[inline(always)]
    pub fn isooutdropmsk(&self) -> ISOOUTDROPMSK_R {
        ISOOUTDROPMSK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt Mask"]
    #[inline(always)]
    pub fn eopfmsk(&self) -> EOPFMSK_R {
        EOPFMSK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn iepintmsk(&self) -> IEPINTMSK_R {
        IEPINTMSK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn oepintmsk(&self) -> OEPINTMSK_R {
        OEPINTMSK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer Mask"]
    #[inline(always)]
    pub fn incompisoinmsk(&self) -> INCOMPISOINMSK_R {
        INCOMPISOINMSK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask"]
    #[inline(always)]
    pub fn incomplpmsk(&self) -> INCOMPLPMSK_R {
        INCOMPLPMSK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data Fetch Suspended Mask"]
    #[inline(always)]
    pub fn fetsuspmsk(&self) -> FETSUSPMSK_R {
        FETSUSPMSK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset detected Interrupt Mask"]
    #[inline(always)]
    pub fn resetdetmsk(&self) -> RESETDETMSK_R {
        RESETDETMSK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask"]
    #[inline(always)]
    pub fn wkupintmsk(&self) -> WKUPINTMSK_R {
        WKUPINTMSK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn modemismsk(&mut self) -> MODEMISMSK_W<GINTMSK_SPEC> {
        MODEMISMSK_W::new(self, 1)
    }
    #[doc = "Bit 3 - Start of Frame Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sofmsk(&mut self) -> SOFMSK_W<GINTMSK_SPEC> {
        SOFMSK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxflvlmsk(&mut self) -> RXFLVLMSK_W<GINTMSK_SPEC> {
        RXFLVLMSK_W::new(self, 4)
    }
    #[doc = "Bit 6 - Global Non-periodic IN NAK Effective Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ginnakeffmsk(&mut self) -> GINNAKEFFMSK_W<GINTMSK_SPEC> {
        GINNAKEFFMSK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective Mask"]
    #[inline(always)]
    #[must_use]
    pub fn goutnakeffmsk(&mut self) -> GOUTNAKEFFMSK_W<GINTMSK_SPEC> {
        GOUTNAKEFFMSK_W::new(self, 7)
    }
    #[doc = "Bit 10 - Early Suspend Mask"]
    #[inline(always)]
    #[must_use]
    pub fn erlysuspmsk(&mut self) -> ERLYSUSPMSK_W<GINTMSK_SPEC> {
        ERLYSUSPMSK_W::new(self, 10)
    }
    #[doc = "Bit 11 - USB Suspend Mask"]
    #[inline(always)]
    #[must_use]
    pub fn usbsuspmsk(&mut self) -> USBSUSPMSK_W<GINTMSK_SPEC> {
        USBSUSPMSK_W::new(self, 11)
    }
    #[doc = "Bit 12 - USB Reset Mask"]
    #[inline(always)]
    #[must_use]
    pub fn usbrstmsk(&mut self) -> USBRSTMSK_W<GINTMSK_SPEC> {
        USBRSTMSK_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enumeration Done Mask"]
    #[inline(always)]
    #[must_use]
    pub fn enumdonemsk(&mut self) -> ENUMDONEMSK_W<GINTMSK_SPEC> {
        ENUMDONEMSK_W::new(self, 13)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn isooutdropmsk(&mut self) -> ISOOUTDROPMSK_W<GINTMSK_SPEC> {
        ISOOUTDROPMSK_W::new(self, 14)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn eopfmsk(&mut self) -> EOPFMSK_W<GINTMSK_SPEC> {
        EOPFMSK_W::new(self, 15)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn iepintmsk(&mut self) -> IEPINTMSK_W<GINTMSK_SPEC> {
        IEPINTMSK_W::new(self, 18)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn oepintmsk(&mut self) -> OEPINTMSK_W<GINTMSK_SPEC> {
        OEPINTMSK_W::new(self, 19)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer Mask"]
    #[inline(always)]
    #[must_use]
    pub fn incompisoinmsk(&mut self) -> INCOMPISOINMSK_W<GINTMSK_SPEC> {
        INCOMPISOINMSK_W::new(self, 20)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask"]
    #[inline(always)]
    #[must_use]
    pub fn incomplpmsk(&mut self) -> INCOMPLPMSK_W<GINTMSK_SPEC> {
        INCOMPLPMSK_W::new(self, 21)
    }
    #[doc = "Bit 22 - Data Fetch Suspended Mask"]
    #[inline(always)]
    #[must_use]
    pub fn fetsuspmsk(&mut self) -> FETSUSPMSK_W<GINTMSK_SPEC> {
        FETSUSPMSK_W::new(self, 22)
    }
    #[doc = "Bit 23 - Reset detected Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn resetdetmsk(&mut self) -> RESETDETMSK_W<GINTMSK_SPEC> {
        RESETDETMSK_W::new(self, 23)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn wkupintmsk(&mut self) -> WKUPINTMSK_W<GINTMSK_SPEC> {
        WKUPINTMSK_W::new(self, 31)
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
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GINTMSK_SPEC;
impl crate::RegisterSpec for GINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintmsk::R`](R) reader structure"]
impl crate::Readable for GINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gintmsk::W`](W) writer structure"]
impl crate::Writable for GINTMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GINTMSK to value 0"]
impl crate::Resettable for GINTMSK_SPEC {
    const RESET_VALUE: u32 = 0;
}
