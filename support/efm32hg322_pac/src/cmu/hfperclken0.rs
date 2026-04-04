#[doc = "Register `HFPERCLKEN0` reader"]
pub type R = crate::R<Hfperclken0Spec>;
#[doc = "Register `HFPERCLKEN0` writer"]
pub type W = crate::W<Hfperclken0Spec>;
#[doc = "Field `TIMER0` reader - Timer 0 Clock Enable"]
pub type Timer0R = crate::BitReader;
#[doc = "Field `TIMER0` writer - Timer 0 Clock Enable"]
pub type Timer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1` reader - Timer 1 Clock Enable"]
pub type Timer1R = crate::BitReader;
#[doc = "Field `TIMER1` writer - Timer 1 Clock Enable"]
pub type Timer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2` reader - Timer 2 Clock Enable"]
pub type Timer2R = crate::BitReader;
#[doc = "Field `TIMER2` writer - Timer 2 Clock Enable"]
pub type Timer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART0` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
pub type Usart0R = crate::BitReader;
#[doc = "Field `USART0` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
pub type Usart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
pub type Usart1R = crate::BitReader;
#[doc = "Field `USART1` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
pub type Usart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP0` reader - Analog Comparator 0 Clock Enable"]
pub type Acmp0R = crate::BitReader;
#[doc = "Field `ACMP0` writer - Analog Comparator 0 Clock Enable"]
pub type Acmp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRS` reader - Peripheral Reflex System Clock Enable"]
pub type PrsR = crate::BitReader;
#[doc = "Field `PRS` writer - Peripheral Reflex System Clock Enable"]
pub type PrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDAC0` reader - Current Digital to Analog Converter 0 Clock Enable"]
pub type Idac0R = crate::BitReader;
#[doc = "Field `IDAC0` writer - Current Digital to Analog Converter 0 Clock Enable"]
pub type Idac0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO` reader - General purpose Input/Output Clock Enable"]
pub type GpioR = crate::BitReader;
#[doc = "Field `GPIO` writer - General purpose Input/Output Clock Enable"]
pub type GpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCMP` reader - Voltage Comparator Clock Enable"]
pub type VcmpR = crate::BitReader;
#[doc = "Field `VCMP` writer - Voltage Comparator Clock Enable"]
pub type VcmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0` reader - Analog to Digital Converter 0 Clock Enable"]
pub type Adc0R = crate::BitReader;
#[doc = "Field `ADC0` writer - Analog to Digital Converter 0 Clock Enable"]
pub type Adc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0` reader - I2C 0 Clock Enable"]
pub type I2c0R = crate::BitReader;
#[doc = "Field `I2C0` writer - I2C 0 Clock Enable"]
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn timer0(&self) -> Timer0R {
        Timer0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 1 Clock Enable"]
    #[inline(always)]
    pub fn timer1(&self) -> Timer1R {
        Timer1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 2 Clock Enable"]
    #[inline(always)]
    pub fn timer2(&self) -> Timer2R {
        Timer2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    pub fn usart0(&self) -> Usart0R {
        Usart0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    pub fn usart1(&self) -> Usart1R {
        Usart1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog Comparator 0 Clock Enable"]
    #[inline(always)]
    pub fn acmp0(&self) -> Acmp0R {
        Acmp0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral Reflex System Clock Enable"]
    #[inline(always)]
    pub fn prs(&self) -> PrsR {
        PrsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Current Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn idac0(&self) -> Idac0R {
        Idac0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - General purpose Input/Output Clock Enable"]
    #[inline(always)]
    pub fn gpio(&self) -> GpioR {
        GpioR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Voltage Comparator Clock Enable"]
    #[inline(always)]
    pub fn vcmp(&self) -> VcmpR {
        VcmpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Analog to Digital Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn adc0(&self) -> Adc0R {
        Adc0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2C 0 Clock Enable"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn timer0(&mut self) -> Timer0W<'_, Hfperclken0Spec> {
        Timer0W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer 1 Clock Enable"]
    #[inline(always)]
    pub fn timer1(&mut self) -> Timer1W<'_, Hfperclken0Spec> {
        Timer1W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer 2 Clock Enable"]
    #[inline(always)]
    pub fn timer2(&mut self) -> Timer2W<'_, Hfperclken0Spec> {
        Timer2W::new(self, 2)
    }
    #[doc = "Bit 3 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    pub fn usart0(&mut self) -> Usart0W<'_, Hfperclken0Spec> {
        Usart0W::new(self, 3)
    }
    #[doc = "Bit 4 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    pub fn usart1(&mut self) -> Usart1W<'_, Hfperclken0Spec> {
        Usart1W::new(self, 4)
    }
    #[doc = "Bit 5 - Analog Comparator 0 Clock Enable"]
    #[inline(always)]
    pub fn acmp0(&mut self) -> Acmp0W<'_, Hfperclken0Spec> {
        Acmp0W::new(self, 5)
    }
    #[doc = "Bit 6 - Peripheral Reflex System Clock Enable"]
    #[inline(always)]
    pub fn prs(&mut self) -> PrsW<'_, Hfperclken0Spec> {
        PrsW::new(self, 6)
    }
    #[doc = "Bit 7 - Current Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn idac0(&mut self) -> Idac0W<'_, Hfperclken0Spec> {
        Idac0W::new(self, 7)
    }
    #[doc = "Bit 8 - General purpose Input/Output Clock Enable"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GpioW<'_, Hfperclken0Spec> {
        GpioW::new(self, 8)
    }
    #[doc = "Bit 9 - Voltage Comparator Clock Enable"]
    #[inline(always)]
    pub fn vcmp(&mut self) -> VcmpW<'_, Hfperclken0Spec> {
        VcmpW::new(self, 9)
    }
    #[doc = "Bit 10 - Analog to Digital Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn adc0(&mut self) -> Adc0W<'_, Hfperclken0Spec> {
        Adc0W::new(self, 10)
    }
    #[doc = "Bit 11 - I2C 0 Clock Enable"]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2c0W<'_, Hfperclken0Spec> {
        I2c0W::new(self, 11)
    }
}
#[doc = "High Frequency Peripheral Clock Enable Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hfperclken0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfperclken0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hfperclken0Spec;
impl crate::RegisterSpec for Hfperclken0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfperclken0::R`](R) reader structure"]
impl crate::Readable for Hfperclken0Spec {}
#[doc = "`write(|w| ..)` method takes [`hfperclken0::W`](W) writer structure"]
impl crate::Writable for Hfperclken0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFPERCLKEN0 to value 0"]
impl crate::Resettable for Hfperclken0Spec {}
