#[doc = "Register `MASSLOCK` reader"]
pub type R = crate::R<MASSLOCK_SPEC>;
#[doc = "Register `MASSLOCK` writer"]
pub type W = crate::W<MASSLOCK_SPEC>;
#[doc = "Field `LOCKKEY` reader - Mass Erase Lock"]
pub type LOCKKEY_R = crate::FieldReader<LOCKKEY_A>;
#[doc = "Mass Erase Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum LOCKKEY_A {
    #[doc = "0: Mass erase unlocked."]
    UNLOCKED = 0,
    #[doc = "1: Mass erase locked."]
    LOCKED = 1,
}
impl From<LOCKKEY_A> for u16 {
    #[inline(always)]
    fn from(variant: LOCKKEY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LOCKKEY_A {
    type Ux = u16;
}
impl LOCKKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LOCKKEY_A> {
        match self.bits {
            0 => Some(LOCKKEY_A::UNLOCKED),
            1 => Some(LOCKKEY_A::LOCKED),
            _ => None,
        }
    }
    #[doc = "Mass erase unlocked."]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKKEY_A::UNLOCKED
    }
    #[doc = "Mass erase locked."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKKEY_A::LOCKED
    }
}
#[doc = "Field `LOCKKEY` writer - Mass Erase Lock"]
pub type LOCKKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, LOCKKEY_A>;
impl<'a, REG> LOCKKEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Mass erase unlocked."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKKEY_A::UNLOCKED)
    }
    #[doc = "Mass erase locked."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKKEY_A::LOCKED)
    }
}
impl R {
    #[doc = "Bits 0:15 - Mass Erase Lock"]
    #[inline(always)]
    pub fn lockkey(&self) -> LOCKKEY_R {
        LOCKKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mass Erase Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lockkey(&mut self) -> LOCKKEY_W<MASSLOCK_SPEC> {
        LOCKKEY_W::new(self, 0)
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
#[doc = "Mass Erase Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`masslock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`masslock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MASSLOCK_SPEC;
impl crate::RegisterSpec for MASSLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`masslock::R`](R) reader structure"]
impl crate::Readable for MASSLOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`masslock::W`](W) writer structure"]
impl crate::Writable for MASSLOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASSLOCK to value 0x01"]
impl crate::Resettable for MASSLOCK_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
