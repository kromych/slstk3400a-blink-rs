#[doc = "Register `USBCRCTRL` reader"]
pub type R = crate::R<UsbcrctrlSpec>;
#[doc = "Register `USBCRCTRL` writer"]
pub type W = crate::W<UsbcrctrlSpec>;
#[doc = "Field `EN` reader - Clock Recovery Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Clock Recovery Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSMODE` reader - Low Speed Clock Recovery Mode"]
pub type LsmodeR = crate::BitReader;
#[doc = "Field `LSMODE` writer - Low Speed Clock Recovery Mode"]
pub type LsmodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock Recovery Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Speed Clock Recovery Mode"]
    #[inline(always)]
    pub fn lsmode(&self) -> LsmodeR {
        LsmodeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Recovery Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, UsbcrctrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Low Speed Clock Recovery Mode"]
    #[inline(always)]
    pub fn lsmode(&mut self) -> LsmodeW<'_, UsbcrctrlSpec> {
        LsmodeW::new(self, 1)
    }
}
#[doc = "USB Clock Recovery Control\n\nYou can [`read`](crate::Reg::read) this register and get [`usbcrctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbcrctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbcrctrlSpec;
impl crate::RegisterSpec for UsbcrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbcrctrl::R`](R) reader structure"]
impl crate::Readable for UsbcrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`usbcrctrl::W`](W) writer structure"]
impl crate::Writable for UsbcrctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBCRCTRL to value 0"]
impl crate::Resettable for UsbcrctrlSpec {}
