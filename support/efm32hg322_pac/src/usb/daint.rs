#[doc = "Register `DAINT` reader"]
pub type R = crate::R<DaintSpec>;
#[doc = "Field `INEPINT0` reader - IN Endpoint 0 Interrupt Bit"]
pub type Inepint0R = crate::BitReader;
#[doc = "Field `INEPINT1` reader - IN Endpoint 1 Interrupt Bit"]
pub type Inepint1R = crate::BitReader;
#[doc = "Field `INEPINT2` reader - IN Endpoint 2 Interrupt Bit"]
pub type Inepint2R = crate::BitReader;
#[doc = "Field `INEPINT3` reader - IN Endpoint 3 Interrupt Bit"]
pub type Inepint3R = crate::BitReader;
#[doc = "Field `OUTEPINT0` reader - OUT Endpoint 0 Interrupt Bit"]
pub type Outepint0R = crate::BitReader;
#[doc = "Field `OUTEPINT1` reader - OUT Endpoint 1 Interrupt Bit"]
pub type Outepint1R = crate::BitReader;
#[doc = "Field `OUTEPINT2` reader - OUT Endpoint 2 Interrupt Bit"]
pub type Outepint2R = crate::BitReader;
#[doc = "Field `OUTEPINT3` reader - OUT Endpoint 3 Interrupt Bit"]
pub type Outepint3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - IN Endpoint 0 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint0(&self) -> Inepint0R {
        Inepint0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IN Endpoint 1 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint1(&self) -> Inepint1R {
        Inepint1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IN Endpoint 2 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint2(&self) -> Inepint2R {
        Inepint2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IN Endpoint 3 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint3(&self) -> Inepint3R {
        Inepint3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - OUT Endpoint 0 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint0(&self) -> Outepint0R {
        Outepint0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint1(&self) -> Outepint1R {
        Outepint1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OUT Endpoint 2 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint2(&self) -> Outepint2R {
        Outepint2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoint 3 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint3(&self) -> Outepint3R {
        Outepint3R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Device All Endpoints Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`daint::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaintSpec;
impl crate::RegisterSpec for DaintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daint::R`](R) reader structure"]
impl crate::Readable for DaintSpec {}
#[doc = "`reset()` method sets DAINT to value 0"]
impl crate::Resettable for DaintSpec {}
