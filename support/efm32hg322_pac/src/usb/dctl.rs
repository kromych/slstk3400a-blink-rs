#[doc = "Register `DCTL` reader"]
pub type R = crate::R<DctlSpec>;
#[doc = "Register `DCTL` writer"]
pub type W = crate::W<DctlSpec>;
#[doc = "Field `RMTWKUPSIG` reader - Remote Wakeup Signaling"]
pub type RmtwkupsigR = crate::BitReader;
#[doc = "Field `RMTWKUPSIG` writer - Remote Wakeup Signaling"]
pub type RmtwkupsigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTDISCON` reader - Soft Disconnect"]
pub type SftdisconR = crate::BitReader;
#[doc = "Field `SFTDISCON` writer - Soft Disconnect"]
pub type SftdisconW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GNPINNAKSTS` reader - Global Non-periodic IN NAK Status"]
pub type GnpinnakstsR = crate::BitReader;
#[doc = "Field `GOUTNAKSTS` reader - Global OUT NAK Status"]
pub type GoutnakstsR = crate::BitReader;
#[doc = "Test Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tstctl {
    #[doc = "0: Test mode disabled."]
    Disable = 0,
    #[doc = "1: Test_J mode."]
    J = 1,
    #[doc = "2: Test_K mode."]
    K = 2,
    #[doc = "3: Test_SE0_NAK mode."]
    Se0nak = 3,
    #[doc = "4: Test_Packet mode."]
    Packet = 4,
    #[doc = "5: Test_Force_Enable."]
    Force = 5,
}
impl From<Tstctl> for u8 {
    #[inline(always)]
    fn from(variant: Tstctl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tstctl {
    type Ux = u8;
}
impl crate::IsEnum for Tstctl {}
#[doc = "Field `TSTCTL` reader - Test Control"]
pub type TstctlR = crate::FieldReader<Tstctl>;
impl TstctlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tstctl> {
        match self.bits {
            0 => Some(Tstctl::Disable),
            1 => Some(Tstctl::J),
            2 => Some(Tstctl::K),
            3 => Some(Tstctl::Se0nak),
            4 => Some(Tstctl::Packet),
            5 => Some(Tstctl::Force),
            _ => None,
        }
    }
    #[doc = "Test mode disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tstctl::Disable
    }
    #[doc = "Test_J mode."]
    #[inline(always)]
    pub fn is_j(&self) -> bool {
        *self == Tstctl::J
    }
    #[doc = "Test_K mode."]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        *self == Tstctl::K
    }
    #[doc = "Test_SE0_NAK mode."]
    #[inline(always)]
    pub fn is_se0nak(&self) -> bool {
        *self == Tstctl::Se0nak
    }
    #[doc = "Test_Packet mode."]
    #[inline(always)]
    pub fn is_packet(&self) -> bool {
        *self == Tstctl::Packet
    }
    #[doc = "Test_Force_Enable."]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == Tstctl::Force
    }
}
#[doc = "Field `TSTCTL` writer - Test Control"]
pub type TstctlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tstctl>;
impl<'a, REG> TstctlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Test mode disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstctl::Disable)
    }
    #[doc = "Test_J mode."]
    #[inline(always)]
    pub fn j(self) -> &'a mut crate::W<REG> {
        self.variant(Tstctl::J)
    }
    #[doc = "Test_K mode."]
    #[inline(always)]
    pub fn k(self) -> &'a mut crate::W<REG> {
        self.variant(Tstctl::K)
    }
    #[doc = "Test_SE0_NAK mode."]
    #[inline(always)]
    pub fn se0nak(self) -> &'a mut crate::W<REG> {
        self.variant(Tstctl::Se0nak)
    }
    #[doc = "Test_Packet mode."]
    #[inline(always)]
    pub fn packet(self) -> &'a mut crate::W<REG> {
        self.variant(Tstctl::Packet)
    }
    #[doc = "Test_Force_Enable."]
    #[inline(always)]
    pub fn force(self) -> &'a mut crate::W<REG> {
        self.variant(Tstctl::Force)
    }
}
#[doc = "Field `SGNPINNAK` writer - Set Global Non-periodic IN NAK"]
pub type SgnpinnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGNPINNAK` writer - Clear Global Non-periodic IN NAK"]
pub type CgnpinnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGOUTNAK` writer - Set Global OUT NAK"]
pub type SgoutnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGOUTNAK` writer - Clear Global OUT NAK"]
pub type CgoutnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRONPRGDONE` reader - Power-On Programming Done"]
pub type PwronprgdoneR = crate::BitReader;
#[doc = "Field `PWRONPRGDONE` writer - Power-On Programming Done"]
pub type PwronprgdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNRFRMNUM` reader - Ignore Frame number For Isochronous End points"]
pub type IgnrfrmnumR = crate::BitReader;
#[doc = "Field `IGNRFRMNUM` writer - Ignore Frame number For Isochronous End points"]
pub type IgnrfrmnumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKONBBLE` reader - NAK on Babble Error"]
pub type NakonbbleR = crate::BitReader;
#[doc = "Field `NAKONBBLE` writer - NAK on Babble Error"]
pub type NakonbbleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline(always)]
    pub fn rmtwkupsig(&self) -> RmtwkupsigR {
        RmtwkupsigR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline(always)]
    pub fn sftdiscon(&self) -> SftdisconR {
        SftdisconR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global Non-periodic IN NAK Status"]
    #[inline(always)]
    pub fn gnpinnaksts(&self) -> GnpinnakstsR {
        GnpinnakstsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK Status"]
    #[inline(always)]
    pub fn goutnaksts(&self) -> GoutnakstsR {
        GoutnakstsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Test Control"]
    #[inline(always)]
    pub fn tstctl(&self) -> TstctlR {
        TstctlR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 11 - Power-On Programming Done"]
    #[inline(always)]
    pub fn pwronprgdone(&self) -> PwronprgdoneR {
        PwronprgdoneR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Ignore Frame number For Isochronous End points"]
    #[inline(always)]
    pub fn ignrfrmnum(&self) -> IgnrfrmnumR {
        IgnrfrmnumR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NAK on Babble Error"]
    #[inline(always)]
    pub fn nakonbble(&self) -> NakonbbleR {
        NakonbbleR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline(always)]
    pub fn rmtwkupsig(&mut self) -> RmtwkupsigW<'_, DctlSpec> {
        RmtwkupsigW::new(self, 0)
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline(always)]
    pub fn sftdiscon(&mut self) -> SftdisconW<'_, DctlSpec> {
        SftdisconW::new(self, 1)
    }
    #[doc = "Bits 4:6 - Test Control"]
    #[inline(always)]
    pub fn tstctl(&mut self) -> TstctlW<'_, DctlSpec> {
        TstctlW::new(self, 4)
    }
    #[doc = "Bit 7 - Set Global Non-periodic IN NAK"]
    #[inline(always)]
    pub fn sgnpinnak(&mut self) -> SgnpinnakW<'_, DctlSpec> {
        SgnpinnakW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear Global Non-periodic IN NAK"]
    #[inline(always)]
    pub fn cgnpinnak(&mut self) -> CgnpinnakW<'_, DctlSpec> {
        CgnpinnakW::new(self, 8)
    }
    #[doc = "Bit 9 - Set Global OUT NAK"]
    #[inline(always)]
    pub fn sgoutnak(&mut self) -> SgoutnakW<'_, DctlSpec> {
        SgoutnakW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear Global OUT NAK"]
    #[inline(always)]
    pub fn cgoutnak(&mut self) -> CgoutnakW<'_, DctlSpec> {
        CgoutnakW::new(self, 10)
    }
    #[doc = "Bit 11 - Power-On Programming Done"]
    #[inline(always)]
    pub fn pwronprgdone(&mut self) -> PwronprgdoneW<'_, DctlSpec> {
        PwronprgdoneW::new(self, 11)
    }
    #[doc = "Bit 15 - Ignore Frame number For Isochronous End points"]
    #[inline(always)]
    pub fn ignrfrmnum(&mut self) -> IgnrfrmnumW<'_, DctlSpec> {
        IgnrfrmnumW::new(self, 15)
    }
    #[doc = "Bit 16 - NAK on Babble Error"]
    #[inline(always)]
    pub fn nakonbble(&mut self) -> NakonbbleW<'_, DctlSpec> {
        NakonbbleW::new(self, 16)
    }
}
#[doc = "Device Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DctlSpec;
impl crate::RegisterSpec for DctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctl::R`](R) reader structure"]
impl crate::Readable for DctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dctl::W`](W) writer structure"]
impl crate::Writable for DctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCTL to value 0x02"]
impl crate::Resettable for DctlSpec {
    const RESET_VALUE: u32 = 0x02;
}
