#[doc = "Register `DCTL` reader"]
pub type R = crate::R<DCTL_SPEC>;
#[doc = "Register `DCTL` writer"]
pub type W = crate::W<DCTL_SPEC>;
#[doc = "Field `RMTWKUPSIG` reader - Remote Wakeup Signaling"]
pub type RMTWKUPSIG_R = crate::BitReader;
#[doc = "Field `RMTWKUPSIG` writer - Remote Wakeup Signaling"]
pub type RMTWKUPSIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTDISCON` reader - Soft Disconnect"]
pub type SFTDISCON_R = crate::BitReader;
#[doc = "Field `SFTDISCON` writer - Soft Disconnect"]
pub type SFTDISCON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GNPINNAKSTS` reader - Global Non-periodic IN NAK Status"]
pub type GNPINNAKSTS_R = crate::BitReader;
#[doc = "Field `GOUTNAKSTS` reader - Global OUT NAK Status"]
pub type GOUTNAKSTS_R = crate::BitReader;
#[doc = "Field `TSTCTL` reader - Test Control"]
pub type TSTCTL_R = crate::FieldReader<TSTCTL_A>;
#[doc = "Test Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTCTL_A {
    #[doc = "0: Test mode disabled."]
    DISABLE = 0,
    #[doc = "1: Test_J mode."]
    J = 1,
    #[doc = "2: Test_K mode."]
    K = 2,
    #[doc = "3: Test_SE0_NAK mode."]
    SE0NAK = 3,
    #[doc = "4: Test_Packet mode."]
    PACKET = 4,
    #[doc = "5: Test_Force_Enable."]
    FORCE = 5,
}
impl From<TSTCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTCTL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSTCTL_A {
    type Ux = u8;
}
impl TSTCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSTCTL_A> {
        match self.bits {
            0 => Some(TSTCTL_A::DISABLE),
            1 => Some(TSTCTL_A::J),
            2 => Some(TSTCTL_A::K),
            3 => Some(TSTCTL_A::SE0NAK),
            4 => Some(TSTCTL_A::PACKET),
            5 => Some(TSTCTL_A::FORCE),
            _ => None,
        }
    }
    #[doc = "Test mode disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TSTCTL_A::DISABLE
    }
    #[doc = "Test_J mode."]
    #[inline(always)]
    pub fn is_j(&self) -> bool {
        *self == TSTCTL_A::J
    }
    #[doc = "Test_K mode."]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        *self == TSTCTL_A::K
    }
    #[doc = "Test_SE0_NAK mode."]
    #[inline(always)]
    pub fn is_se0nak(&self) -> bool {
        *self == TSTCTL_A::SE0NAK
    }
    #[doc = "Test_Packet mode."]
    #[inline(always)]
    pub fn is_packet(&self) -> bool {
        *self == TSTCTL_A::PACKET
    }
    #[doc = "Test_Force_Enable."]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == TSTCTL_A::FORCE
    }
}
#[doc = "Field `TSTCTL` writer - Test Control"]
pub type TSTCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TSTCTL_A>;
impl<'a, REG> TSTCTL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Test mode disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TSTCTL_A::DISABLE)
    }
    #[doc = "Test_J mode."]
    #[inline(always)]
    pub fn j(self) -> &'a mut crate::W<REG> {
        self.variant(TSTCTL_A::J)
    }
    #[doc = "Test_K mode."]
    #[inline(always)]
    pub fn k(self) -> &'a mut crate::W<REG> {
        self.variant(TSTCTL_A::K)
    }
    #[doc = "Test_SE0_NAK mode."]
    #[inline(always)]
    pub fn se0nak(self) -> &'a mut crate::W<REG> {
        self.variant(TSTCTL_A::SE0NAK)
    }
    #[doc = "Test_Packet mode."]
    #[inline(always)]
    pub fn packet(self) -> &'a mut crate::W<REG> {
        self.variant(TSTCTL_A::PACKET)
    }
    #[doc = "Test_Force_Enable."]
    #[inline(always)]
    pub fn force(self) -> &'a mut crate::W<REG> {
        self.variant(TSTCTL_A::FORCE)
    }
}
#[doc = "Field `SGNPINNAK` writer - Set Global Non-periodic IN NAK"]
pub type SGNPINNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGNPINNAK` writer - Clear Global Non-periodic IN NAK"]
pub type CGNPINNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGOUTNAK` writer - Set Global OUT NAK"]
pub type SGOUTNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGOUTNAK` writer - Clear Global OUT NAK"]
pub type CGOUTNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRONPRGDONE` reader - Power-On Programming Done"]
pub type PWRONPRGDONE_R = crate::BitReader;
#[doc = "Field `PWRONPRGDONE` writer - Power-On Programming Done"]
pub type PWRONPRGDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNRFRMNUM` reader - Ignore Frame number For Isochronous End points"]
pub type IGNRFRMNUM_R = crate::BitReader;
#[doc = "Field `IGNRFRMNUM` writer - Ignore Frame number For Isochronous End points"]
pub type IGNRFRMNUM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKONBBLE` reader - NAK on Babble Error"]
pub type NAKONBBLE_R = crate::BitReader;
#[doc = "Field `NAKONBBLE` writer - NAK on Babble Error"]
pub type NAKONBBLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline(always)]
    pub fn rmtwkupsig(&self) -> RMTWKUPSIG_R {
        RMTWKUPSIG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline(always)]
    pub fn sftdiscon(&self) -> SFTDISCON_R {
        SFTDISCON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global Non-periodic IN NAK Status"]
    #[inline(always)]
    pub fn gnpinnaksts(&self) -> GNPINNAKSTS_R {
        GNPINNAKSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK Status"]
    #[inline(always)]
    pub fn goutnaksts(&self) -> GOUTNAKSTS_R {
        GOUTNAKSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Test Control"]
    #[inline(always)]
    pub fn tstctl(&self) -> TSTCTL_R {
        TSTCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 11 - Power-On Programming Done"]
    #[inline(always)]
    pub fn pwronprgdone(&self) -> PWRONPRGDONE_R {
        PWRONPRGDONE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Ignore Frame number For Isochronous End points"]
    #[inline(always)]
    pub fn ignrfrmnum(&self) -> IGNRFRMNUM_R {
        IGNRFRMNUM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NAK on Babble Error"]
    #[inline(always)]
    pub fn nakonbble(&self) -> NAKONBBLE_R {
        NAKONBBLE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline(always)]
    #[must_use]
    pub fn rmtwkupsig(&mut self) -> RMTWKUPSIG_W<DCTL_SPEC> {
        RMTWKUPSIG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn sftdiscon(&mut self) -> SFTDISCON_W<DCTL_SPEC> {
        SFTDISCON_W::new(self, 1)
    }
    #[doc = "Bits 4:6 - Test Control"]
    #[inline(always)]
    #[must_use]
    pub fn tstctl(&mut self) -> TSTCTL_W<DCTL_SPEC> {
        TSTCTL_W::new(self, 4)
    }
    #[doc = "Bit 7 - Set Global Non-periodic IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sgnpinnak(&mut self) -> SGNPINNAK_W<DCTL_SPEC> {
        SGNPINNAK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear Global Non-periodic IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cgnpinnak(&mut self) -> CGNPINNAK_W<DCTL_SPEC> {
        CGNPINNAK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set Global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sgoutnak(&mut self) -> SGOUTNAK_W<DCTL_SPEC> {
        SGOUTNAK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear Global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cgoutnak(&mut self) -> CGOUTNAK_W<DCTL_SPEC> {
        CGOUTNAK_W::new(self, 10)
    }
    #[doc = "Bit 11 - Power-On Programming Done"]
    #[inline(always)]
    #[must_use]
    pub fn pwronprgdone(&mut self) -> PWRONPRGDONE_W<DCTL_SPEC> {
        PWRONPRGDONE_W::new(self, 11)
    }
    #[doc = "Bit 15 - Ignore Frame number For Isochronous End points"]
    #[inline(always)]
    #[must_use]
    pub fn ignrfrmnum(&mut self) -> IGNRFRMNUM_W<DCTL_SPEC> {
        IGNRFRMNUM_W::new(self, 15)
    }
    #[doc = "Bit 16 - NAK on Babble Error"]
    #[inline(always)]
    #[must_use]
    pub fn nakonbble(&mut self) -> NAKONBBLE_W<DCTL_SPEC> {
        NAKONBBLE_W::new(self, 16)
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
#[doc = "Device Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCTL_SPEC;
impl crate::RegisterSpec for DCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctl::R`](R) reader structure"]
impl crate::Readable for DCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dctl::W`](W) writer structure"]
impl crate::Writable for DCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCTL to value 0x02"]
impl crate::Resettable for DCTL_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
