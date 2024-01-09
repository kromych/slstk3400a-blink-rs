#[doc = "Register `HFPERCLKEN0` reader"]
pub type R = crate::R<HFPERCLKEN0_SPEC>;
#[doc = "Register `HFPERCLKEN0` writer"]
pub type W = crate::W<HFPERCLKEN0_SPEC>;
#[doc = "Field `TIMER0` reader - Timer 0 Clock Enable"]
pub type TIMER0_R = crate::BitReader;
#[doc = "Field `TIMER0` writer - Timer 0 Clock Enable"]
pub type TIMER0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1` reader - Timer 1 Clock Enable"]
pub type TIMER1_R = crate::BitReader;
#[doc = "Field `TIMER1` writer - Timer 1 Clock Enable"]
pub type TIMER1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2` reader - Timer 2 Clock Enable"]
pub type TIMER2_R = crate::BitReader;
#[doc = "Field `TIMER2` writer - Timer 2 Clock Enable"]
pub type TIMER2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART0` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
pub type USART0_R = crate::BitReader;
#[doc = "Field `USART0` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
pub type USART0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
pub type USART1_R = crate::BitReader;
#[doc = "Field `USART1` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
pub type USART1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP0` reader - Analog Comparator 0 Clock Enable"]
pub type ACMP0_R = crate::BitReader;
#[doc = "Field `ACMP0` writer - Analog Comparator 0 Clock Enable"]
pub type ACMP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRS` reader - Peripheral Reflex System Clock Enable"]
pub type PRS_R = crate::BitReader;
#[doc = "Field `PRS` writer - Peripheral Reflex System Clock Enable"]
pub type PRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDAC0` reader - Current Digital to Analog Converter 0 Clock Enable"]
pub type IDAC0_R = crate::BitReader;
#[doc = "Field `IDAC0` writer - Current Digital to Analog Converter 0 Clock Enable"]
pub type IDAC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO` reader - General purpose Input/Output Clock Enable"]
pub type GPIO_R = crate::BitReader;
#[doc = "Field `GPIO` writer - General purpose Input/Output Clock Enable"]
pub type GPIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCMP` reader - Voltage Comparator Clock Enable"]
pub type VCMP_R = crate::BitReader;
#[doc = "Field `VCMP` writer - Voltage Comparator Clock Enable"]
pub type VCMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0` reader - Analog to Digital Converter 0 Clock Enable"]
pub type ADC0_R = crate::BitReader;
#[doc = "Field `ADC0` writer - Analog to Digital Converter 0 Clock Enable"]
pub type ADC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0` reader - I2C 0 Clock Enable"]
pub type I2C0_R = crate::BitReader;
#[doc = "Field `I2C0` writer - I2C 0 Clock Enable"]
pub type I2C0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn timer0(&self) -> TIMER0_R {
        TIMER0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 1 Clock Enable"]
    #[inline(always)]
    pub fn timer1(&self) -> TIMER1_R {
        TIMER1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 2 Clock Enable"]
    #[inline(always)]
    pub fn timer2(&self) -> TIMER2_R {
        TIMER2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    pub fn usart0(&self) -> USART0_R {
        USART0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog Comparator 0 Clock Enable"]
    #[inline(always)]
    pub fn acmp0(&self) -> ACMP0_R {
        ACMP0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral Reflex System Clock Enable"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Current Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn idac0(&self) -> IDAC0_R {
        IDAC0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - General purpose Input/Output Clock Enable"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Voltage Comparator Clock Enable"]
    #[inline(always)]
    pub fn vcmp(&self) -> VCMP_R {
        VCMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Analog to Digital Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2C 0 Clock Enable"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer0(&mut self) -> TIMER0_W<HFPERCLKEN0_SPEC> {
        TIMER0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer 1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer1(&mut self) -> TIMER1_W<HFPERCLKEN0_SPEC> {
        TIMER1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer 2 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer2(&mut self) -> TIMER2_W<HFPERCLKEN0_SPEC> {
        TIMER2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart0(&mut self) -> USART0_W<HFPERCLKEN0_SPEC> {
        USART0_W::new(self, 3)
    }
    #[doc = "Bit 4 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1(&mut self) -> USART1_W<HFPERCLKEN0_SPEC> {
        USART1_W::new(self, 4)
    }
    #[doc = "Bit 5 - Analog Comparator 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0(&mut self) -> ACMP0_W<HFPERCLKEN0_SPEC> {
        ACMP0_W::new(self, 5)
    }
    #[doc = "Bit 6 - Peripheral Reflex System Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prs(&mut self) -> PRS_W<HFPERCLKEN0_SPEC> {
        PRS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Current Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn idac0(&mut self) -> IDAC0_W<HFPERCLKEN0_SPEC> {
        IDAC0_W::new(self, 7)
    }
    #[doc = "Bit 8 - General purpose Input/Output Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GPIO_W<HFPERCLKEN0_SPEC> {
        GPIO_W::new(self, 8)
    }
    #[doc = "Bit 9 - Voltage Comparator Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vcmp(&mut self) -> VCMP_W<HFPERCLKEN0_SPEC> {
        VCMP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Analog to Digital Converter 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0(&mut self) -> ADC0_W<HFPERCLKEN0_SPEC> {
        ADC0_W::new(self, 10)
    }
    #[doc = "Bit 11 - I2C 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2C0_W<HFPERCLKEN0_SPEC> {
        I2C0_W::new(self, 11)
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
#[doc = "High Frequency Peripheral Clock Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfperclken0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfperclken0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFPERCLKEN0_SPEC;
impl crate::RegisterSpec for HFPERCLKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfperclken0::R`](R) reader structure"]
impl crate::Readable for HFPERCLKEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfperclken0::W`](W) writer structure"]
impl crate::Writable for HFPERCLKEN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFPERCLKEN0 to value 0"]
impl crate::Resettable for HFPERCLKEN0_SPEC {
    const RESET_VALUE: u32 = 0;
}
