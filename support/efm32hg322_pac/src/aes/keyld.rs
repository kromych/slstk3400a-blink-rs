#[doc = "Register `KEYLD` reader"]
pub type R = crate::R<KeyldSpec>;
#[doc = "Register `KEYLD` writer"]
pub type W = crate::W<KeyldSpec>;
#[doc = "Field `KEYLD` reader - Key Low Access D"]
pub type KeyldR = crate::FieldReader<u32>;
#[doc = "Field `KEYLD` writer - Key Low Access D"]
pub type KeyldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key Low Access D"]
    #[inline(always)]
    pub fn keyld(&self) -> KeyldR {
        KeyldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Low Access D"]
    #[inline(always)]
    pub fn keyld(&mut self) -> KeyldW<'_, KeyldSpec> {
        KeyldW::new(self, 0)
    }
}
#[doc = "KEY Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`keyld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct KeyldSpec;
impl crate::RegisterSpec for KeyldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyld::R`](R) reader structure"]
impl crate::Readable for KeyldSpec {}
#[doc = "`write(|w| ..)` method takes [`keyld::W`](W) writer structure"]
impl crate::Writable for KeyldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYLD to value 0"]
impl crate::Resettable for KeyldSpec {}
