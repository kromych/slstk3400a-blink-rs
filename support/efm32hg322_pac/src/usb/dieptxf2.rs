#[doc = "Register `DIEPTXF2` reader"]
pub type R = crate::R<Dieptxf2Spec>;
#[doc = "Register `DIEPTXF2` writer"]
pub type W = crate::W<Dieptxf2Spec>;
#[doc = "Field `INEPNTXFSTADDR` reader - IN Endpoint FIFO 2 Transmit RAM Start Address"]
pub type InepntxfstaddrR = crate::FieldReader<u16>;
#[doc = "Field `INEPNTXFSTADDR` writer - IN Endpoint FIFO 2 Transmit RAM Start Address"]
pub type InepntxfstaddrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `INEPNTXFDEP` reader - IN Endpoint TxFIFO Depth"]
pub type InepntxfdepR = crate::FieldReader<u16>;
#[doc = "Field `INEPNTXFDEP` writer - IN Endpoint TxFIFO Depth"]
pub type InepntxfdepW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:10 - IN Endpoint FIFO 2 Transmit RAM Start Address"]
    #[inline(always)]
    pub fn inepntxfstaddr(&self) -> InepntxfstaddrR {
        InepntxfstaddrR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:25 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    pub fn inepntxfdep(&self) -> InepntxfdepR {
        InepntxfdepR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - IN Endpoint FIFO 2 Transmit RAM Start Address"]
    #[inline(always)]
    pub fn inepntxfstaddr(&mut self) -> InepntxfstaddrW<'_, Dieptxf2Spec> {
        InepntxfstaddrW::new(self, 0)
    }
    #[doc = "Bits 16:25 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    pub fn inepntxfdep(&mut self) -> InepntxfdepW<'_, Dieptxf2Spec> {
        InepntxfdepW::new(self, 16)
    }
}
#[doc = "Device IN Endpoint Transmit FIFO 2 Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dieptxf2Spec;
impl crate::RegisterSpec for Dieptxf2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf2::R`](R) reader structure"]
impl crate::Readable for Dieptxf2Spec {}
#[doc = "`write(|w| ..)` method takes [`dieptxf2::W`](W) writer structure"]
impl crate::Writable for Dieptxf2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPTXF2 to value 0x0200_0600"]
impl crate::Resettable for Dieptxf2Spec {
    const RESET_VALUE: u32 = 0x0200_0600;
}
