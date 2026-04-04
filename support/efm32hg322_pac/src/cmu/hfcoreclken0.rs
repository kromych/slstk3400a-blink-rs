#[doc = "Register `HFCORECLKEN0` reader"]
pub type R = crate::R<Hfcoreclken0Spec>;
#[doc = "Register `HFCORECLKEN0` writer"]
pub type W = crate::W<Hfcoreclken0Spec>;
#[doc = "Field `AES` reader - Advanced Encryption Standard Accelerator Clock Enable"]
pub type AesR = crate::BitReader;
#[doc = "Field `AES` writer - Advanced Encryption Standard Accelerator Clock Enable"]
pub type AesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA` reader - Direct Memory Access Controller Clock Enable"]
pub type DmaR = crate::BitReader;
#[doc = "Field `DMA` writer - Direct Memory Access Controller Clock Enable"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LE` reader - Low Energy Peripheral Interface Clock Enable"]
pub type LeR = crate::BitReader;
#[doc = "Field `LE` writer - Low Energy Peripheral Interface Clock Enable"]
pub type LeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBC` reader - Universal Serial Bus Interface Core Clock Enable"]
pub type UsbcR = crate::BitReader;
#[doc = "Field `USBC` writer - Universal Serial Bus Interface Core Clock Enable"]
pub type UsbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB` reader - Universal Serial Bus Interface Clock Enable"]
pub type UsbR = crate::BitReader;
#[doc = "Field `USB` writer - Universal Serial Bus Interface Clock Enable"]
pub type UsbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Advanced Encryption Standard Accelerator Clock Enable"]
    #[inline(always)]
    pub fn aes(&self) -> AesR {
        AesR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direct Memory Access Controller Clock Enable"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Energy Peripheral Interface Clock Enable"]
    #[inline(always)]
    pub fn le(&self) -> LeR {
        LeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Universal Serial Bus Interface Core Clock Enable"]
    #[inline(always)]
    pub fn usbc(&self) -> UsbcR {
        UsbcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Universal Serial Bus Interface Clock Enable"]
    #[inline(always)]
    pub fn usb(&self) -> UsbR {
        UsbR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Advanced Encryption Standard Accelerator Clock Enable"]
    #[inline(always)]
    pub fn aes(&mut self) -> AesW<'_, Hfcoreclken0Spec> {
        AesW::new(self, 0)
    }
    #[doc = "Bit 1 - Direct Memory Access Controller Clock Enable"]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<'_, Hfcoreclken0Spec> {
        DmaW::new(self, 1)
    }
    #[doc = "Bit 2 - Low Energy Peripheral Interface Clock Enable"]
    #[inline(always)]
    pub fn le(&mut self) -> LeW<'_, Hfcoreclken0Spec> {
        LeW::new(self, 2)
    }
    #[doc = "Bit 3 - Universal Serial Bus Interface Core Clock Enable"]
    #[inline(always)]
    pub fn usbc(&mut self) -> UsbcW<'_, Hfcoreclken0Spec> {
        UsbcW::new(self, 3)
    }
    #[doc = "Bit 4 - Universal Serial Bus Interface Clock Enable"]
    #[inline(always)]
    pub fn usb(&mut self) -> UsbW<'_, Hfcoreclken0Spec> {
        UsbW::new(self, 4)
    }
}
#[doc = "High Frequency Core Clock Enable Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcoreclken0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcoreclken0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hfcoreclken0Spec;
impl crate::RegisterSpec for Hfcoreclken0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfcoreclken0::R`](R) reader structure"]
impl crate::Readable for Hfcoreclken0Spec {}
#[doc = "`write(|w| ..)` method takes [`hfcoreclken0::W`](W) writer structure"]
impl crate::Writable for Hfcoreclken0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFCORECLKEN0 to value 0"]
impl crate::Resettable for Hfcoreclken0Spec {}
