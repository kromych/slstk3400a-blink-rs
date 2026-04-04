#[doc = "Register `RSTCAUSE` reader"]
pub type R = crate::R<RstcauseSpec>;
#[doc = "Field `PORST` reader - Power On Reset"]
pub type PorstR = crate::BitReader;
#[doc = "Field `BODUNREGRST` reader - Brown Out Detector Unregulated Domain Reset"]
pub type BodunregrstR = crate::BitReader;
#[doc = "Field `BODREGRST` reader - Brown Out Detector Regulated Domain Reset"]
pub type BodregrstR = crate::BitReader;
#[doc = "Field `EXTRST` reader - External Pin Reset"]
pub type ExtrstR = crate::BitReader;
#[doc = "Field `WDOGRST` reader - Watchdog Reset"]
pub type WdogrstR = crate::BitReader;
#[doc = "Field `LOCKUPRST` reader - LOCKUP Reset"]
pub type LockuprstR = crate::BitReader;
#[doc = "Field `SYSREQRST` reader - System Request Reset"]
pub type SysreqrstR = crate::BitReader;
#[doc = "Field `EM4RST` reader - EM4 Reset"]
pub type Em4rstR = crate::BitReader;
#[doc = "Field `EM4WURST` reader - EM4 Wake-up Reset"]
pub type Em4wurstR = crate::BitReader;
#[doc = "Field `BODAVDD0` reader - AVDD0 Bod Reset"]
pub type Bodavdd0R = crate::BitReader;
#[doc = "Field `BODAVDD1` reader - AVDD1 Bod Reset"]
pub type Bodavdd1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Power On Reset"]
    #[inline(always)]
    pub fn porst(&self) -> PorstR {
        PorstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Brown Out Detector Unregulated Domain Reset"]
    #[inline(always)]
    pub fn bodunregrst(&self) -> BodunregrstR {
        BodunregrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Brown Out Detector Regulated Domain Reset"]
    #[inline(always)]
    pub fn bodregrst(&self) -> BodregrstR {
        BodregrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Pin Reset"]
    #[inline(always)]
    pub fn extrst(&self) -> ExtrstR {
        ExtrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog Reset"]
    #[inline(always)]
    pub fn wdogrst(&self) -> WdogrstR {
        WdogrstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LOCKUP Reset"]
    #[inline(always)]
    pub fn lockuprst(&self) -> LockuprstR {
        LockuprstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - System Request Reset"]
    #[inline(always)]
    pub fn sysreqrst(&self) -> SysreqrstR {
        SysreqrstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EM4 Reset"]
    #[inline(always)]
    pub fn em4rst(&self) -> Em4rstR {
        Em4rstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EM4 Wake-up Reset"]
    #[inline(always)]
    pub fn em4wurst(&self) -> Em4wurstR {
        Em4wurstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AVDD0 Bod Reset"]
    #[inline(always)]
    pub fn bodavdd0(&self) -> Bodavdd0R {
        Bodavdd0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AVDD1 Bod Reset"]
    #[inline(always)]
    pub fn bodavdd1(&self) -> Bodavdd1R {
        Bodavdd1R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Reset Cause Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstcause::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstcauseSpec;
impl crate::RegisterSpec for RstcauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstcause::R`](R) reader structure"]
impl crate::Readable for RstcauseSpec {}
#[doc = "`reset()` method sets RSTCAUSE to value 0"]
impl crate::Resettable for RstcauseSpec {}
