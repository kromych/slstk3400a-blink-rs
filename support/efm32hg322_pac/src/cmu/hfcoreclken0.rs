#[doc = "Register `HFCORECLKEN0` reader"]
pub type R = crate::R<HFCORECLKEN0_SPEC>;
#[doc = "Register `HFCORECLKEN0` writer"]
pub type W = crate::W<HFCORECLKEN0_SPEC>;
#[doc = "Field `AES` reader - Advanced Encryption Standard Accelerator Clock Enable"]
pub type AES_R = crate::BitReader;
#[doc = "Field `AES` writer - Advanced Encryption Standard Accelerator Clock Enable"]
pub type AES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA` reader - Direct Memory Access Controller Clock Enable"]
pub type DMA_R = crate::BitReader;
#[doc = "Field `DMA` writer - Direct Memory Access Controller Clock Enable"]
pub type DMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LE` reader - Low Energy Peripheral Interface Clock Enable"]
pub type LE_R = crate::BitReader;
#[doc = "Field `LE` writer - Low Energy Peripheral Interface Clock Enable"]
pub type LE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBC` reader - Universal Serial Bus Interface Core Clock Enable"]
pub type USBC_R = crate::BitReader;
#[doc = "Field `USBC` writer - Universal Serial Bus Interface Core Clock Enable"]
pub type USBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB` reader - Universal Serial Bus Interface Clock Enable"]
pub type USB_R = crate::BitReader;
#[doc = "Field `USB` writer - Universal Serial Bus Interface Clock Enable"]
pub type USB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Advanced Encryption Standard Accelerator Clock Enable"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direct Memory Access Controller Clock Enable"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Energy Peripheral Interface Clock Enable"]
    #[inline(always)]
    pub fn le(&self) -> LE_R {
        LE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Universal Serial Bus Interface Core Clock Enable"]
    #[inline(always)]
    pub fn usbc(&self) -> USBC_R {
        USBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Universal Serial Bus Interface Clock Enable"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Advanced Encryption Standard Accelerator Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AES_W<HFCORECLKEN0_SPEC> {
        AES_W::new(self, 0)
    }
    #[doc = "Bit 1 - Direct Memory Access Controller Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<HFCORECLKEN0_SPEC> {
        DMA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Low Energy Peripheral Interface Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn le(&mut self) -> LE_W<HFCORECLKEN0_SPEC> {
        LE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Universal Serial Bus Interface Core Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbc(&mut self) -> USBC_W<HFCORECLKEN0_SPEC> {
        USBC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Universal Serial Bus Interface Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> USB_W<HFCORECLKEN0_SPEC> {
        USB_W::new(self, 4)
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
#[doc = "High Frequency Core Clock Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfcoreclken0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfcoreclken0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFCORECLKEN0_SPEC;
impl crate::RegisterSpec for HFCORECLKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfcoreclken0::R`](R) reader structure"]
impl crate::Readable for HFCORECLKEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfcoreclken0::W`](W) writer structure"]
impl crate::Writable for HFCORECLKEN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFCORECLKEN0 to value 0"]
impl crate::Resettable for HFCORECLKEN0_SPEC {
    const RESET_VALUE: u32 = 0;
}
