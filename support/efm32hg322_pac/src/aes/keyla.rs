#[doc = "Register `KEYLA` reader"]
pub type R = crate::R<KeylaSpec>;
#[doc = "Register `KEYLA` writer"]
pub type W = crate::W<KeylaSpec>;
#[doc = "Field `KEYLA` reader - Key Low Access A"]
pub type KeylaR = crate::FieldReader<u32>;
#[doc = "Field `KEYLA` writer - Key Low Access A"]
pub type KeylaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key Low Access A"]
    #[inline(always)]
    pub fn keyla(&self) -> KeylaR {
        KeylaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Low Access A"]
    #[inline(always)]
    pub fn keyla(&mut self) -> KeylaW<'_, KeylaSpec> {
        KeylaW::new(self, 0)
    }
}
#[doc = "KEY Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`keyla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct KeylaSpec;
impl crate::RegisterSpec for KeylaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyla::R`](R) reader structure"]
impl crate::Readable for KeylaSpec {}
#[doc = "`write(|w| ..)` method takes [`keyla::W`](W) writer structure"]
impl crate::Writable for KeylaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYLA to value 0"]
impl crate::Resettable for KeylaSpec {}
