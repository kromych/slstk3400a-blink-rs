#[doc = "Register `CH3_CTRL` reader"]
pub type R = crate::R<Ch3CtrlSpec>;
#[doc = "Register `CH3_CTRL` writer"]
pub type W = crate::W<Ch3CtrlSpec>;
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SigselR = crate::FieldReader;
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SigselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sourcesel {
    #[doc = "0: No source selected"]
    None = 0,
    #[doc = "1: Voltage Comparator"]
    Vcmp = 1,
    #[doc = "2: Analog Comparator 0"]
    Acmp0 = 2,
    #[doc = "8: Analog to Digital Converter 0"]
    Adc0 = 8,
    #[doc = "16: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    Usart0 = 16,
    #[doc = "17: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    Usart1 = 17,
    #[doc = "28: Timer 0"]
    Timer0 = 28,
    #[doc = "29: Timer 1"]
    Timer1 = 29,
    #[doc = "30: Timer 2"]
    Timer2 = 30,
    #[doc = "36: Universal Serial Bus Interface"]
    Usb = 36,
    #[doc = "40: Real-Time Counter"]
    Rtc = 40,
    #[doc = "48: General purpose Input/Output"]
    Gpiol = 48,
    #[doc = "49: General purpose Input/Output"]
    Gpioh = 49,
    #[doc = "54: Pulse Counter 0"]
    Pcnt0 = 54,
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
            1 => Some(Sourcesel::Vcmp),
            2 => Some(Sourcesel::Acmp0),
            8 => Some(Sourcesel::Adc0),
            16 => Some(Sourcesel::Usart0),
            17 => Some(Sourcesel::Usart1),
            28 => Some(Sourcesel::Timer0),
            29 => Some(Sourcesel::Timer1),
            30 => Some(Sourcesel::Timer2),
            36 => Some(Sourcesel::Usb),
            40 => Some(Sourcesel::Rtc),
            48 => Some(Sourcesel::Gpiol),
            49 => Some(Sourcesel::Gpioh),
            54 => Some(Sourcesel::Pcnt0),
            _ => None,
        }
    }
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sourcesel::None
    }
    #[doc = "Voltage Comparator"]
    #[inline(always)]
    pub fn is_vcmp(&self) -> bool {
        *self == Sourcesel::Vcmp
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Sourcesel::Acmp0
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
    #[doc = "Universal Serial Bus Interface"]
    #[inline(always)]
    pub fn is_usb(&self) -> bool {
        *self == Sourcesel::Usb
    }
    #[doc = "Real-Time Counter"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == Sourcesel::Rtc
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn is_gpiol(&self) -> bool {
        *self == Sourcesel::Gpiol
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn is_gpioh(&self) -> bool {
        *self == Sourcesel::Gpioh
    }
    #[doc = "Pulse Counter 0"]
    #[inline(always)]
    pub fn is_pcnt0(&self) -> bool {
        *self == Sourcesel::Pcnt0
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
    #[doc = "Voltage Comparator"]
    #[inline(always)]
    pub fn vcmp(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Vcmp)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Acmp0)
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
    #[doc = "Universal Serial Bus Interface"]
    #[inline(always)]
    pub fn usb(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Usb)
    }
    #[doc = "Real-Time Counter"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Rtc)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpiol(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Gpiol)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpioh(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Gpioh)
    }
    #[doc = "Pulse Counter 0"]
    #[inline(always)]
    pub fn pcnt0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Pcnt0)
    }
}
#[doc = "Edge Detect Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Edsel {
    #[doc = "0: Signal is left as it is"]
    Off = 0,
    #[doc = "1: A one HFPERCLK cycle pulse is generated for every positive edge of the incoming signal"]
    Posedge = 1,
    #[doc = "2: A one HFPERCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    Negedge = 2,
    #[doc = "3: A one HFPERCLK clock cycle pulse is generated for every edge of the incoming signal"]
    Bothedges = 3,
}
impl From<Edsel> for u8 {
    #[inline(always)]
    fn from(variant: Edsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Edsel {
    type Ux = u8;
}
impl crate::IsEnum for Edsel {}
#[doc = "Field `EDSEL` reader - Edge Detect Select"]
pub type EdselR = crate::FieldReader<Edsel>;
impl EdselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edsel {
        match self.bits {
            0 => Edsel::Off,
            1 => Edsel::Posedge,
            2 => Edsel::Negedge,
            3 => Edsel::Bothedges,
            _ => unreachable!(),
        }
    }
    #[doc = "Signal is left as it is"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Edsel::Off
    }
    #[doc = "A one HFPERCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == Edsel::Posedge
    }
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == Edsel::Negedge
    }
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline(always)]
    pub fn is_bothedges(&self) -> bool {
        *self == Edsel::Bothedges
    }
}
#[doc = "Field `EDSEL` writer - Edge Detect Select"]
pub type EdselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Edsel, crate::Safe>;
impl<'a, REG> EdselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Signal is left as it is"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Edsel::Off)
    }
    #[doc = "A one HFPERCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut crate::W<REG> {
        self.variant(Edsel::Posedge)
    }
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut crate::W<REG> {
        self.variant(Edsel::Negedge)
    }
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline(always)]
    pub fn bothedges(self) -> &'a mut crate::W<REG> {
        self.variant(Edsel::Bothedges)
    }
}
#[doc = "Field `ASYNC` reader - Asynchronous reflex"]
pub type AsyncR = crate::BitReader;
#[doc = "Field `ASYNC` writer - Asynchronous reflex"]
pub type AsyncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SigselR {
        SigselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SourceselR {
        SourceselR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - Edge Detect Select"]
    #[inline(always)]
    pub fn edsel(&self) -> EdselR {
        EdselR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Asynchronous reflex"]
    #[inline(always)]
    pub fn async_(&self) -> AsyncR {
        AsyncR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&mut self) -> SigselW<'_, Ch3CtrlSpec> {
        SigselW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&mut self) -> SourceselW<'_, Ch3CtrlSpec> {
        SourceselW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Edge Detect Select"]
    #[inline(always)]
    pub fn edsel(&mut self) -> EdselW<'_, Ch3CtrlSpec> {
        EdselW::new(self, 24)
    }
    #[doc = "Bit 28 - Asynchronous reflex"]
    #[inline(always)]
    pub fn async_(&mut self) -> AsyncW<'_, Ch3CtrlSpec> {
        AsyncW::new(self, 28)
    }
}
#[doc = "Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3CtrlSpec;
impl crate::RegisterSpec for Ch3CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3_ctrl::R`](R) reader structure"]
impl crate::Readable for Ch3CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3_ctrl::W`](W) writer structure"]
impl crate::Writable for Ch3CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH3_CTRL to value 0"]
impl crate::Resettable for Ch3CtrlSpec {}
