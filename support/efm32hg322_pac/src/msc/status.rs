#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `BUSY` reader - Erase/Write Busy"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `LOCKED` reader - Access Locked"]
pub type LOCKED_R = crate::BitReader;
#[doc = "Field `INVADDR` reader - Invalid Write Address or Erase Page"]
pub type INVADDR_R = crate::BitReader;
#[doc = "Field `WDATAREADY` reader - WDATA Write Ready"]
pub type WDATAREADY_R = crate::BitReader;
#[doc = "Field `WORDTIMEOUT` reader - Flash Write Word Timeout"]
pub type WORDTIMEOUT_R = crate::BitReader;
#[doc = "Field `ERASEABORTED` reader - The Current Flash Erase Operation Aborted"]
pub type ERASEABORTED_R = crate::BitReader;
#[doc = "Field `PCRUNNING` reader - Performance Counters Running"]
pub type PCRUNNING_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Erase/Write Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Access Locked"]
    #[inline(always)]
    pub fn locked(&self) -> LOCKED_R {
        LOCKED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Invalid Write Address or Erase Page"]
    #[inline(always)]
    pub fn invaddr(&self) -> INVADDR_R {
        INVADDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WDATA Write Ready"]
    #[inline(always)]
    pub fn wdataready(&self) -> WDATAREADY_R {
        WDATAREADY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash Write Word Timeout"]
    #[inline(always)]
    pub fn wordtimeout(&self) -> WORDTIMEOUT_R {
        WORDTIMEOUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The Current Flash Erase Operation Aborted"]
    #[inline(always)]
    pub fn eraseaborted(&self) -> ERASEABORTED_R {
        ERASEABORTED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Performance Counters Running"]
    #[inline(always)]
    pub fn pcrunning(&self) -> PCRUNNING_R {
        PCRUNNING_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0x08"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
