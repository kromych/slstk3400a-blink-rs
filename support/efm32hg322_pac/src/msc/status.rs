#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `BUSY` reader - Erase/Write Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `LOCKED` reader - Access Locked"]
pub type LockedR = crate::BitReader;
#[doc = "Field `INVADDR` reader - Invalid Write Address or Erase Page"]
pub type InvaddrR = crate::BitReader;
#[doc = "Field `WDATAREADY` reader - WDATA Write Ready"]
pub type WdatareadyR = crate::BitReader;
#[doc = "Field `WORDTIMEOUT` reader - Flash Write Word Timeout"]
pub type WordtimeoutR = crate::BitReader;
#[doc = "Field `ERASEABORTED` reader - The Current Flash Erase Operation Aborted"]
pub type EraseabortedR = crate::BitReader;
#[doc = "Field `PCRUNNING` reader - Performance Counters Running"]
pub type PcrunningR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Erase/Write Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Access Locked"]
    #[inline(always)]
    pub fn locked(&self) -> LockedR {
        LockedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Invalid Write Address or Erase Page"]
    #[inline(always)]
    pub fn invaddr(&self) -> InvaddrR {
        InvaddrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WDATA Write Ready"]
    #[inline(always)]
    pub fn wdataready(&self) -> WdatareadyR {
        WdatareadyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash Write Word Timeout"]
    #[inline(always)]
    pub fn wordtimeout(&self) -> WordtimeoutR {
        WordtimeoutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The Current Flash Erase Operation Aborted"]
    #[inline(always)]
    pub fn eraseaborted(&self) -> EraseabortedR {
        EraseabortedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Performance Counters Running"]
    #[inline(always)]
    pub fn pcrunning(&self) -> PcrunningR {
        PcrunningR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0x08"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x08;
}
