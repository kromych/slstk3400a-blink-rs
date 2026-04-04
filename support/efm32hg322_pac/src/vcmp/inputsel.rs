#[doc = "Register `INPUTSEL` reader"]
pub type R = crate::R<InputselSpec>;
#[doc = "Register `INPUTSEL` writer"]
pub type W = crate::W<InputselSpec>;
#[doc = "Field `TRIGLEVEL` reader - Trigger Level"]
pub type TriglevelR = crate::FieldReader;
#[doc = "Field `TRIGLEVEL` writer - Trigger Level"]
pub type TriglevelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LPREF` reader - Low Power Reference"]
pub type LprefR = crate::BitReader;
#[doc = "Field `LPREF` writer - Low Power Reference"]
pub type LprefW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Trigger Level"]
    #[inline(always)]
    pub fn triglevel(&self) -> TriglevelR {
        TriglevelR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Low Power Reference"]
    #[inline(always)]
    pub fn lpref(&self) -> LprefR {
        LprefR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trigger Level"]
    #[inline(always)]
    pub fn triglevel(&mut self) -> TriglevelW<'_, InputselSpec> {
        TriglevelW::new(self, 0)
    }
    #[doc = "Bit 8 - Low Power Reference"]
    #[inline(always)]
    pub fn lpref(&mut self) -> LprefW<'_, InputselSpec> {
        LprefW::new(self, 8)
    }
}
#[doc = "Input Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inputsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputselSpec;
impl crate::RegisterSpec for InputselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inputsel::R`](R) reader structure"]
impl crate::Readable for InputselSpec {}
#[doc = "`write(|w| ..)` method takes [`inputsel::W`](W) writer structure"]
impl crate::Writable for InputselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUTSEL to value 0"]
impl crate::Resettable for InputselSpec {}
