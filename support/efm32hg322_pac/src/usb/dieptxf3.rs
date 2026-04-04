#[doc = "Register `DIEPTXF3` reader"]
pub type R = crate::R<Dieptxf3Spec>;
#[doc = "Register `DIEPTXF3` writer"]
pub type W = crate::W<Dieptxf3Spec>;
#[doc = "Field `INEPNTXFSTADDR` reader - IN Endpoint FIFO 3 Transmit RAM Start Address"]
pub type InepntxfstaddrR = crate::FieldReader<u16>;
#[doc = "Field `INEPNTXFSTADDR` writer - IN Endpoint FIFO 3 Transmit RAM Start Address"]
pub type InepntxfstaddrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `INEPNTXFDEP` reader - IN Endpoint TxFIFO Depth"]
pub type InepntxfdepR = crate::FieldReader<u16>;
#[doc = "Field `INEPNTXFDEP` writer - IN Endpoint TxFIFO Depth"]
pub type InepntxfdepW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:11 - IN Endpoint FIFO 3 Transmit RAM Start Address"]
    #[inline(always)]
    pub fn inepntxfstaddr(&self) -> InepntxfstaddrR {
        InepntxfstaddrR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:25 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    pub fn inepntxfdep(&self) -> InepntxfdepR {
        InepntxfdepR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - IN Endpoint FIFO 3 Transmit RAM Start Address"]
    #[inline(always)]
    pub fn inepntxfstaddr(&mut self) -> InepntxfstaddrW<'_, Dieptxf3Spec> {
        InepntxfstaddrW::new(self, 0)
    }
    #[doc = "Bits 16:25 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    pub fn inepntxfdep(&mut self) -> InepntxfdepW<'_, Dieptxf3Spec> {
        InepntxfdepW::new(self, 16)
    }
}
#[doc = "Device IN Endpoint Transmit FIFO 3 Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dieptxf3Spec;
impl crate::RegisterSpec for Dieptxf3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf3::R`](R) reader structure"]
impl crate::Readable for Dieptxf3Spec {}
#[doc = "`write(|w| ..)` method takes [`dieptxf3::W`](W) writer structure"]
impl crate::Writable for Dieptxf3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPTXF3 to value 0x0200_0800"]
impl crate::Resettable for Dieptxf3Spec {
    const RESET_VALUE: u32 = 0x0200_0800;
}
