#[doc = "Register `MASSLOCK` reader"]
pub type R = crate::R<MasslockSpec>;
#[doc = "Register `MASSLOCK` writer"]
pub type W = crate::W<MasslockSpec>;
#[doc = "Mass Erase Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Lockkey {
    #[doc = "0: Mass erase unlocked."]
    Unlocked = 0,
    #[doc = "1: Mass erase locked."]
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
#[doc = "Field `LOCKKEY` reader - Mass Erase Lock"]
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
    #[doc = "Mass erase unlocked."]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lockkey::Unlocked
    }
    #[doc = "Mass erase locked."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lockkey::Locked
    }
}
#[doc = "Field `LOCKKEY` writer - Mass Erase Lock"]
pub type LockkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, Lockkey>;
impl<'a, REG> LockkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Mass erase unlocked."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Lockkey::Unlocked)
    }
    #[doc = "Mass erase locked."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lockkey::Locked)
    }
}
impl R {
    #[doc = "Bits 0:15 - Mass Erase Lock"]
    #[inline(always)]
    pub fn lockkey(&self) -> LockkeyR {
        LockkeyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mass Erase Lock"]
    #[inline(always)]
    pub fn lockkey(&mut self) -> LockkeyW<'_, MasslockSpec> {
        LockkeyW::new(self, 0)
    }
}
#[doc = "Mass Erase Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`masslock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`masslock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MasslockSpec;
impl crate::RegisterSpec for MasslockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`masslock::R`](R) reader structure"]
impl crate::Readable for MasslockSpec {}
#[doc = "`write(|w| ..)` method takes [`masslock::W`](W) writer structure"]
impl crate::Writable for MasslockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASSLOCK to value 0x01"]
impl crate::Resettable for MasslockSpec {
    const RESET_VALUE: u32 = 0x01;
}
