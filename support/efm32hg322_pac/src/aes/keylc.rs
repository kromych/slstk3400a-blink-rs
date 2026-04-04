#[doc = "Register `KEYLC` reader"]
pub type R = crate::R<KeylcSpec>;
#[doc = "Register `KEYLC` writer"]
pub type W = crate::W<KeylcSpec>;
#[doc = "Field `KEYLC` reader - Key Low Access C"]
pub type KeylcR = crate::FieldReader<u32>;
#[doc = "Field `KEYLC` writer - Key Low Access C"]
pub type KeylcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key Low Access C"]
    #[inline(always)]
    pub fn keylc(&self) -> KeylcR {
        KeylcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Low Access C"]
    #[inline(always)]
    pub fn keylc(&mut self) -> KeylcW<'_, KeylcSpec> {
        KeylcW::new(self, 0)
    }
}
#[doc = "KEY Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`keylc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keylc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct KeylcSpec;
impl crate::RegisterSpec for KeylcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keylc::R`](R) reader structure"]
impl crate::Readable for KeylcSpec {}
#[doc = "`write(|w| ..)` method takes [`keylc::W`](W) writer structure"]
impl crate::Writable for KeylcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYLC to value 0"]
impl crate::Resettable for KeylcSpec {}
