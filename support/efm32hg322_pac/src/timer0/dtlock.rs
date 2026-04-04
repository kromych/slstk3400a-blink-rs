#[doc = "Register `DTLOCK` reader"]
pub type R = crate::R<DtlockSpec>;
#[doc = "Register `DTLOCK` writer"]
pub type W = crate::W<DtlockSpec>;
#[doc = "DTI Lock Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Lockkey {
    #[doc = "0: `0`"]
    Unlocked = 0,
    #[doc = "1: `1`"]
    Locked = 1,
}
impl From<Lockkey> for u16 {
    #[inline(always)]
    fn from(variant: Lockkey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lockkey {
    type Ux = u16;
}
impl crate::IsEnum for Lockkey {}
#[doc = "Field `LOCKKEY` reader - DTI Lock Key"]
pub type LockkeyR = crate::FieldReader<Lockkey>;
impl LockkeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lockkey> {
        match self.bits {
            0 => Some(Lockkey::Unlocked),
            1 => Some(Lockkey::Locked),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lockkey::Unlocked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lockkey::Locked
    }
}
#[doc = "Field `LOCKKEY` writer - DTI Lock Key"]
pub type LockkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, Lockkey>;
impl<'a, REG> LockkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Lockkey::Unlocked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lockkey::Locked)
    }
}
impl R {
    #[doc = "Bits 0:15 - DTI Lock Key"]
    #[inline(always)]
    pub fn lockkey(&self) -> LockkeyR {
        LockkeyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DTI Lock Key"]
    #[inline(always)]
    pub fn lockkey(&mut self) -> LockkeyW<'_, DtlockSpec> {
        LockkeyW::new(self, 0)
    }
}
#[doc = "DTI Configuration Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtlockSpec;
impl crate::RegisterSpec for DtlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtlock::R`](R) reader structure"]
impl crate::Readable for DtlockSpec {}
#[doc = "`write(|w| ..)` method takes [`dtlock::W`](W) writer structure"]
impl crate::Writable for DtlockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTLOCK to value 0"]
impl crate::Resettable for DtlockSpec {}
