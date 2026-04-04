#[doc = "Register `CH5_CTRL` reader"]
pub type R = crate::R<Ch5CtrlSpec>;
#[doc = "Register `CH5_CTRL` writer"]
pub type W = crate::W<Ch5CtrlSpec>;
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SigselR = crate::FieldReader;
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SigselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sourcesel {
    #[doc = "0: No source selected"]
    None = 0,
    #[doc = "8: Analog to Digital Converter 0"]
    Adc0 = 8,
    #[doc = "12: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    Usart0 = 12,
    #[doc = "13: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    Usart1 = 13,
    #[doc = "16: Low Energy UART 0"]
    Leuart0 = 16,
    #[doc = "20: I2C 0"]
    I2c0 = 20,
    #[doc = "24: Timer 0"]
    Timer0 = 24,
    #[doc = "25: Timer 1"]
    Timer1 = 25,
    #[doc = "26: Timer 2"]
    Timer2 = 26,
    #[doc = "48: `110000`"]
    Msc = 48,
    #[doc = "49: Advanced Encryption Standard Accelerator"]
    Aes = 49,
}
impl From<Sourcesel> for u8 {
    #[inline(always)]
    fn from(variant: Sourcesel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sourcesel {
    type Ux = u8;
}
impl crate::IsEnum for Sourcesel {}
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SourceselR = crate::FieldReader<Sourcesel>;
impl SourceselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sourcesel> {
        match self.bits {
            0 => Some(Sourcesel::None),
            8 => Some(Sourcesel::Adc0),
            12 => Some(Sourcesel::Usart0),
            13 => Some(Sourcesel::Usart1),
            16 => Some(Sourcesel::Leuart0),
            20 => Some(Sourcesel::I2c0),
            24 => Some(Sourcesel::Timer0),
            25 => Some(Sourcesel::Timer1),
            26 => Some(Sourcesel::Timer2),
            48 => Some(Sourcesel::Msc),
            49 => Some(Sourcesel::Aes),
            _ => None,
        }
    }
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sourcesel::None
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == Sourcesel::Adc0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == Sourcesel::Usart0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == Sourcesel::Usart1
    }
    #[doc = "Low Energy UART 0"]
    #[inline(always)]
    pub fn is_leuart0(&self) -> bool {
        *self == Sourcesel::Leuart0
    }
    #[doc = "I2C 0"]
    #[inline(always)]
    pub fn is_i2c0(&self) -> bool {
        *self == Sourcesel::I2c0
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == Sourcesel::Timer0
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == Sourcesel::Timer1
    }
    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn is_timer2(&self) -> bool {
        *self == Sourcesel::Timer2
    }
    #[doc = "`110000`"]
    #[inline(always)]
    pub fn is_msc(&self) -> bool {
        *self == Sourcesel::Msc
    }
    #[doc = "Advanced Encryption Standard Accelerator"]
    #[inline(always)]
    pub fn is_aes(&self) -> bool {
        *self == Sourcesel::Aes
    }
}
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SourceselW<'a, REG> = crate::FieldWriter<'a, REG, 6, Sourcesel>;
impl<'a, REG> SourceselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::None)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Adc0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn usart0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Usart0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn usart1(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Usart1)
    }
    #[doc = "Low Energy UART 0"]
    #[inline(always)]
    pub fn leuart0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Leuart0)
    }
    #[doc = "I2C 0"]
    #[inline(always)]
    pub fn i2c0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::I2c0)
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn timer0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Timer0)
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn timer1(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Timer1)
    }
    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn timer2(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Timer2)
    }
    #[doc = "`110000`"]
    #[inline(always)]
    pub fn msc(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Msc)
    }
    #[doc = "Advanced Encryption Standard Accelerator"]
    #[inline(always)]
    pub fn aes(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Aes)
    }
}
impl R {
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SigselR {
        SigselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SourceselR {
        SourceselR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&mut self) -> SigselW<'_, Ch5CtrlSpec> {
        SigselW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&mut self) -> SourceselW<'_, Ch5CtrlSpec> {
        SourceselW::new(self, 16)
    }
}
#[doc = "Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch5CtrlSpec;
impl crate::RegisterSpec for Ch5CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch5_ctrl::R`](R) reader structure"]
impl crate::Readable for Ch5CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch5_ctrl::W`](W) writer structure"]
impl crate::Writable for Ch5CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH5_CTRL to value 0"]
impl crate::Resettable for Ch5CtrlSpec {}
