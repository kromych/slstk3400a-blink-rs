#[doc = "Register `KEYLB` reader"]
pub type R = crate::R<KeylbSpec>;
#[doc = "Register `KEYLB` writer"]
pub type W = crate::W<KeylbSpec>;
#[doc = "Field `KEYLB` reader - Key Low Access B"]
pub type KeylbR = crate::FieldReader<u32>;
#[doc = "Field `KEYLB` writer - Key Low Access B"]
pub type KeylbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key Low Access B"]
    #[inline(always)]
    pub fn keylb(&self) -> KeylbR {
        KeylbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Low Access B"]
    #[inline(always)]
    pub fn keylb(&mut self) -> KeylbW<'_, KeylbSpec> {
        KeylbW::new(self, 0)
    }
}
#[doc = "KEY Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`keylb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keylb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct KeylbSpec;
impl crate::RegisterSpec for KeylbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keylb::R`](R) reader structure"]
impl crate::Readable for KeylbSpec {}
#[doc = "`write(|w| ..)` method takes [`keylb::W`](W) writer structure"]
impl crate::Writable for KeylbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYLB to value 0"]
impl crate::Resettable for KeylbSpec {}
