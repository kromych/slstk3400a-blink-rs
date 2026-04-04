#[doc = "Register `GINTMSK` reader"]
pub type R = crate::R<GintmskSpec>;
#[doc = "Register `GINTMSK` writer"]
pub type W = crate::W<GintmskSpec>;
#[doc = "Field `MODEMISMSK` reader - Mode Mismatch Interrupt Mask"]
pub type ModemismskR = crate::BitReader;
#[doc = "Field `MODEMISMSK` writer - Mode Mismatch Interrupt Mask"]
pub type ModemismskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFMSK` reader - Start of Frame Mask"]
pub type SofmskR = crate::BitReader;
#[doc = "Field `SOFMSK` writer - Start of Frame Mask"]
pub type SofmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLVLMSK` reader - Receive FIFO Non-Empty Mask"]
pub type RxflvlmskR = crate::BitReader;
#[doc = "Field `RXFLVLMSK` writer - Receive FIFO Non-Empty Mask"]
pub type RxflvlmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINNAKEFFMSK` reader - Global Non-periodic IN NAK Effective Mask"]
pub type GinnakeffmskR = crate::BitReader;
#[doc = "Field `GINNAKEFFMSK` writer - Global Non-periodic IN NAK Effective Mask"]
pub type GinnakeffmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOUTNAKEFFMSK` reader - Global OUT NAK Effective Mask"]
pub type GoutnakeffmskR = crate::BitReader;
#[doc = "Field `GOUTNAKEFFMSK` writer - Global OUT NAK Effective Mask"]
pub type GoutnakeffmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERLYSUSPMSK` reader - Early Suspend Mask"]
pub type ErlysuspmskR = crate::BitReader;
#[doc = "Field `ERLYSUSPMSK` writer - Early Suspend Mask"]
pub type ErlysuspmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSPMSK` reader - USB Suspend Mask"]
pub type UsbsuspmskR = crate::BitReader;
#[doc = "Field `USBSUSPMSK` writer - USB Suspend Mask"]
pub type UsbsuspmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRSTMSK` reader - USB Reset Mask"]
pub type UsbrstmskR = crate::BitReader;
#[doc = "Field `USBRSTMSK` writer - USB Reset Mask"]
pub type UsbrstmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDONEMSK` reader - Enumeration Done Mask"]
pub type EnumdonemskR = crate::BitReader;
#[doc = "Field `ENUMDONEMSK` writer - Enumeration Done Mask"]
pub type EnumdonemskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOOUTDROPMSK` reader - Isochronous OUT Packet Dropped Interrupt Mask"]
pub type IsooutdropmskR = crate::BitReader;
#[doc = "Field `ISOOUTDROPMSK` writer - Isochronous OUT Packet Dropped Interrupt Mask"]
pub type IsooutdropmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPFMSK` reader - End of Periodic Frame Interrupt Mask"]
pub type EopfmskR = crate::BitReader;
#[doc = "Field `EOPFMSK` writer - End of Periodic Frame Interrupt Mask"]
pub type EopfmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPINTMSK` reader - IN Endpoints Interrupt Mask"]
pub type IepintmskR = crate::BitReader;
#[doc = "Field `IEPINTMSK` writer - IN Endpoints Interrupt Mask"]
pub type IepintmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEPINTMSK` reader - OUT Endpoints Interrupt Mask"]
pub type OepintmskR = crate::BitReader;
#[doc = "Field `OEPINTMSK` writer - OUT Endpoints Interrupt Mask"]
pub type OepintmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCOMPISOINMSK` reader - Incomplete Isochronous IN Transfer Mask"]
pub type IncompisoinmskR = crate::BitReader;
#[doc = "Field `INCOMPISOINMSK` writer - Incomplete Isochronous IN Transfer Mask"]
pub type IncompisoinmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCOMPLPMSK` reader - Incomplete Periodic Transfer Mask"]
pub type IncomplpmskR = crate::BitReader;
#[doc = "Field `INCOMPLPMSK` writer - Incomplete Periodic Transfer Mask"]
pub type IncomplpmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETSUSPMSK` reader - Data Fetch Suspended Mask"]
pub type FetsuspmskR = crate::BitReader;
#[doc = "Field `FETSUSPMSK` writer - Data Fetch Suspended Mask"]
pub type FetsuspmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETDETMSK` reader - Reset detected Interrupt Mask"]
pub type ResetdetmskR = crate::BitReader;
#[doc = "Field `RESETDETMSK` writer - Reset detected Interrupt Mask"]
pub type ResetdetmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPINTMSK` reader - Resume/Remote Wakeup Detected Interrupt Mask"]
pub type WkupintmskR = crate::BitReader;
#[doc = "Field `WKUPINTMSK` writer - Resume/Remote Wakeup Detected Interrupt Mask"]
pub type WkupintmskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn modemismsk(&self) -> ModemismskR {
        ModemismskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Frame Mask"]
    #[inline(always)]
    pub fn sofmsk(&self) -> SofmskR {
        SofmskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    pub fn rxflvlmsk(&self) -> RxflvlmskR {
        RxflvlmskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Global Non-periodic IN NAK Effective Mask"]
    #[inline(always)]
    pub fn ginnakeffmsk(&self) -> GinnakeffmskR {
        GinnakeffmskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective Mask"]
    #[inline(always)]
    pub fn goutnakeffmsk(&self) -> GoutnakeffmskR {
        GoutnakeffmskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Suspend Mask"]
    #[inline(always)]
    pub fn erlysuspmsk(&self) -> ErlysuspmskR {
        ErlysuspmskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB Suspend Mask"]
    #[inline(always)]
    pub fn usbsuspmsk(&self) -> UsbsuspmskR {
        UsbsuspmskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB Reset Mask"]
    #[inline(always)]
    pub fn usbrstmsk(&self) -> UsbrstmskR {
        UsbrstmskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration Done Mask"]
    #[inline(always)]
    pub fn enumdonemsk(&self) -> EnumdonemskR {
        EnumdonemskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt Mask"]
    #[inline(always)]
    pub fn isooutdropmsk(&self) -> IsooutdropmskR {
        IsooutdropmskR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt Mask"]
    #[inline(always)]
    pub fn eopfmsk(&self) -> EopfmskR {
        EopfmskR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn iepintmsk(&self) -> IepintmskR {
        IepintmskR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn oepintmsk(&self) -> OepintmskR {
        OepintmskR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer Mask"]
    #[inline(always)]
    pub fn incompisoinmsk(&self) -> IncompisoinmskR {
        IncompisoinmskR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask"]
    #[inline(always)]
    pub fn incomplpmsk(&self) -> IncomplpmskR {
        IncomplpmskR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data Fetch Suspended Mask"]
    #[inline(always)]
    pub fn fetsuspmsk(&self) -> FetsuspmskR {
        FetsuspmskR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset detected Interrupt Mask"]
    #[inline(always)]
    pub fn resetdetmsk(&self) -> ResetdetmskR {
        ResetdetmskR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask"]
    #[inline(always)]
    pub fn wkupintmsk(&self) -> WkupintmskR {
        WkupintmskR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn modemismsk(&mut self) -> ModemismskW<'_, GintmskSpec> {
        ModemismskW::new(self, 1)
    }
    #[doc = "Bit 3 - Start of Frame Mask"]
    #[inline(always)]
    pub fn sofmsk(&mut self) -> SofmskW<'_, GintmskSpec> {
        SofmskW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    pub fn rxflvlmsk(&mut self) -> RxflvlmskW<'_, GintmskSpec> {
        RxflvlmskW::new(self, 4)
    }
    #[doc = "Bit 6 - Global Non-periodic IN NAK Effective Mask"]
    #[inline(always)]
    pub fn ginnakeffmsk(&mut self) -> GinnakeffmskW<'_, GintmskSpec> {
        GinnakeffmskW::new(self, 6)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective Mask"]
    #[inline(always)]
    pub fn goutnakeffmsk(&mut self) -> GoutnakeffmskW<'_, GintmskSpec> {
        GoutnakeffmskW::new(self, 7)
    }
    #[doc = "Bit 10 - Early Suspend Mask"]
    #[inline(always)]
    pub fn erlysuspmsk(&mut self) -> ErlysuspmskW<'_, GintmskSpec> {
        ErlysuspmskW::new(self, 10)
    }
    #[doc = "Bit 11 - USB Suspend Mask"]
    #[inline(always)]
    pub fn usbsuspmsk(&mut self) -> UsbsuspmskW<'_, GintmskSpec> {
        UsbsuspmskW::new(self, 11)
    }
    #[doc = "Bit 12 - USB Reset Mask"]
    #[inline(always)]
    pub fn usbrstmsk(&mut self) -> UsbrstmskW<'_, GintmskSpec> {
        UsbrstmskW::new(self, 12)
    }
    #[doc = "Bit 13 - Enumeration Done Mask"]
    #[inline(always)]
    pub fn enumdonemsk(&mut self) -> EnumdonemskW<'_, GintmskSpec> {
        EnumdonemskW::new(self, 13)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt Mask"]
    #[inline(always)]
    pub fn isooutdropmsk(&mut self) -> IsooutdropmskW<'_, GintmskSpec> {
        IsooutdropmskW::new(self, 14)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt Mask"]
    #[inline(always)]
    pub fn eopfmsk(&mut self) -> EopfmskW<'_, GintmskSpec> {
        EopfmskW::new(self, 15)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn iepintmsk(&mut self) -> IepintmskW<'_, GintmskSpec> {
        IepintmskW::new(self, 18)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn oepintmsk(&mut self) -> OepintmskW<'_, GintmskSpec> {
        OepintmskW::new(self, 19)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer Mask"]
    #[inline(always)]
    pub fn incompisoinmsk(&mut self) -> IncompisoinmskW<'_, GintmskSpec> {
        IncompisoinmskW::new(self, 20)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask"]
    #[inline(always)]
    pub fn incomplpmsk(&mut self) -> IncomplpmskW<'_, GintmskSpec> {
        IncomplpmskW::new(self, 21)
    }
    #[doc = "Bit 22 - Data Fetch Suspended Mask"]
    #[inline(always)]
    pub fn fetsuspmsk(&mut self) -> FetsuspmskW<'_, GintmskSpec> {
        FetsuspmskW::new(self, 22)
    }
    #[doc = "Bit 23 - Reset detected Interrupt Mask"]
    #[inline(always)]
    pub fn resetdetmsk(&mut self) -> ResetdetmskW<'_, GintmskSpec> {
        ResetdetmskW::new(self, 23)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask"]
    #[inline(always)]
    pub fn wkupintmsk(&mut self) -> WkupintmskW<'_, GintmskSpec> {
        WkupintmskW::new(self, 31)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GintmskSpec;
impl crate::RegisterSpec for GintmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintmsk::R`](R) reader structure"]
impl crate::Readable for GintmskSpec {}
#[doc = "`write(|w| ..)` method takes [`gintmsk::W`](W) writer structure"]
impl crate::Writable for GintmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GINTMSK to value 0"]
impl crate::Resettable for GintmskSpec {}
