#[doc = "Register `PC_MODEH` reader"]
pub type R = crate::R<PcModehSpec>;
#[doc = "Register `PC_MODEH` writer"]
pub type W = crate::W<PcModehSpec>;
#[doc = "Pin 8 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode8 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    Pushpulldrive = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    Wiredanddrive = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    Wiredanddrivefilter = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    Wiredanddrivepullup = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    Wiredanddrivepullupfilter = 15,
}
impl From<Mode8> for u8 {
    #[inline(always)]
    fn from(variant: Mode8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode8 {
    type Ux = u8;
}
impl crate::IsEnum for Mode8 {}
#[doc = "Field `MODE8` reader - Pin 8 Mode"]
pub type Mode8R = crate::FieldReader<Mode8>;
impl Mode8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode8 {
        match self.bits {
            0 => Mode8::Disabled,
            1 => Mode8::Input,
            2 => Mode8::Inputpull,
            3 => Mode8::Inputpullfilter,
            4 => Mode8::Pushpull,
            5 => Mode8::Pushpulldrive,
            6 => Mode8::Wiredor,
            7 => Mode8::Wiredorpulldown,
            8 => Mode8::Wiredand,
            9 => Mode8::Wiredandfilter,
            10 => Mode8::Wiredandpullup,
            11 => Mode8::Wiredandpullupfilter,
            12 => Mode8::Wiredanddrive,
            13 => Mode8::Wiredanddrivefilter,
            14 => Mode8::Wiredanddrivepullup,
            15 => Mode8::Wiredanddrivepullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mode8::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Mode8::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == Mode8::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == Mode8::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Mode8::Pushpull
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == Mode8::Pushpulldrive
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == Mode8::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == Mode8::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == Mode8::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == Mode8::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == Mode8::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == Mode8::Wiredandpullupfilter
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == Mode8::Wiredanddrive
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == Mode8::Wiredanddrivefilter
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == Mode8::Wiredanddrivepullup
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == Mode8::Wiredanddrivepullupfilter
    }
}
#[doc = "Field `MODE8` writer - Pin 8 Mode"]
pub type Mode8W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode8, crate::Safe>;
impl<'a, REG> Mode8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::Pushpull)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::Pushpulldrive)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::Wiredanddrive)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::Wiredanddrivefilter)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::Wiredanddrivepullup)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::Wiredanddrivepullupfilter)
    }
}
#[doc = "Pin 9 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode9 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    Pushpulldrive = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    Wiredanddrive = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    Wiredanddrivefilter = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    Wiredanddrivepullup = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    Wiredanddrivepullupfilter = 15,
}
impl From<Mode9> for u8 {
    #[inline(always)]
    fn from(variant: Mode9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode9 {
    type Ux = u8;
}
impl crate::IsEnum for Mode9 {}
#[doc = "Field `MODE9` reader - Pin 9 Mode"]
pub type Mode9R = crate::FieldReader<Mode9>;
impl Mode9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode9 {
        match self.bits {
            0 => Mode9::Disabled,
            1 => Mode9::Input,
            2 => Mode9::Inputpull,
            3 => Mode9::Inputpullfilter,
            4 => Mode9::Pushpull,
            5 => Mode9::Pushpulldrive,
            6 => Mode9::Wiredor,
            7 => Mode9::Wiredorpulldown,
            8 => Mode9::Wiredand,
            9 => Mode9::Wiredandfilter,
            10 => Mode9::Wiredandpullup,
            11 => Mode9::Wiredandpullupfilter,
            12 => Mode9::Wiredanddrive,
            13 => Mode9::Wiredanddrivefilter,
            14 => Mode9::Wiredanddrivepullup,
            15 => Mode9::Wiredanddrivepullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mode9::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Mode9::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == Mode9::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == Mode9::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Mode9::Pushpull
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == Mode9::Pushpulldrive
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == Mode9::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == Mode9::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == Mode9::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == Mode9::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == Mode9::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == Mode9::Wiredandpullupfilter
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == Mode9::Wiredanddrive
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == Mode9::Wiredanddrivefilter
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == Mode9::Wiredanddrivepullup
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == Mode9::Wiredanddrivepullupfilter
    }
}
#[doc = "Field `MODE9` writer - Pin 9 Mode"]
pub type Mode9W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode9, crate::Safe>;
impl<'a, REG> Mode9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::Pushpull)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::Pushpulldrive)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::Wiredanddrive)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::Wiredanddrivefilter)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::Wiredanddrivepullup)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::Wiredanddrivepullupfilter)
    }
}
#[doc = "Pin 10 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode10 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    Pushpulldrive = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    Wiredanddrive = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    Wiredanddrivefilter = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    Wiredanddrivepullup = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    Wiredanddrivepullupfilter = 15,
}
impl From<Mode10> for u8 {
    #[inline(always)]
    fn from(variant: Mode10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode10 {
    type Ux = u8;
}
impl crate::IsEnum for Mode10 {}
#[doc = "Field `MODE10` reader - Pin 10 Mode"]
pub type Mode10R = crate::FieldReader<Mode10>;
impl Mode10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode10 {
        match self.bits {
            0 => Mode10::Disabled,
            1 => Mode10::Input,
            2 => Mode10::Inputpull,
            3 => Mode10::Inputpullfilter,
            4 => Mode10::Pushpull,
            5 => Mode10::Pushpulldrive,
            6 => Mode10::Wiredor,
            7 => Mode10::Wiredorpulldown,
            8 => Mode10::Wiredand,
            9 => Mode10::Wiredandfilter,
            10 => Mode10::Wiredandpullup,
            11 => Mode10::Wiredandpullupfilter,
            12 => Mode10::Wiredanddrive,
            13 => Mode10::Wiredanddrivefilter,
            14 => Mode10::Wiredanddrivepullup,
            15 => Mode10::Wiredanddrivepullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mode10::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Mode10::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == Mode10::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == Mode10::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Mode10::Pushpull
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == Mode10::Pushpulldrive
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == Mode10::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == Mode10::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == Mode10::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == Mode10::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == Mode10::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == Mode10::Wiredandpullupfilter
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == Mode10::Wiredanddrive
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == Mode10::Wiredanddrivefilter
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == Mode10::Wiredanddrivepullup
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == Mode10::Wiredanddrivepullupfilter
    }
}
#[doc = "Field `MODE10` writer - Pin 10 Mode"]
pub type Mode10W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode10, crate::Safe>;
impl<'a, REG> Mode10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::Pushpull)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::Pushpulldrive)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::Wiredanddrive)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::Wiredanddrivefilter)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::Wiredanddrivepullup)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::Wiredanddrivepullupfilter)
    }
}
#[doc = "Pin 11 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode11 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    Pushpulldrive = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    Wiredanddrive = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    Wiredanddrivefilter = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    Wiredanddrivepullup = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    Wiredanddrivepullupfilter = 15,
}
impl From<Mode11> for u8 {
    #[inline(always)]
    fn from(variant: Mode11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode11 {
    type Ux = u8;
}
impl crate::IsEnum for Mode11 {}
#[doc = "Field `MODE11` reader - Pin 11 Mode"]
pub type Mode11R = crate::FieldReader<Mode11>;
impl Mode11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode11 {
        match self.bits {
            0 => Mode11::Disabled,
            1 => Mode11::Input,
            2 => Mode11::Inputpull,
            3 => Mode11::Inputpullfilter,
            4 => Mode11::Pushpull,
            5 => Mode11::Pushpulldrive,
            6 => Mode11::Wiredor,
            7 => Mode11::Wiredorpulldown,
            8 => Mode11::Wiredand,
            9 => Mode11::Wiredandfilter,
            10 => Mode11::Wiredandpullup,
            11 => Mode11::Wiredandpullupfilter,
            12 => Mode11::Wiredanddrive,
            13 => Mode11::Wiredanddrivefilter,
            14 => Mode11::Wiredanddrivepullup,
            15 => Mode11::Wiredanddrivepullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mode11::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Mode11::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == Mode11::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == Mode11::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Mode11::Pushpull
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == Mode11::Pushpulldrive
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == Mode11::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == Mode11::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == Mode11::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == Mode11::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == Mode11::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == Mode11::Wiredandpullupfilter
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == Mode11::Wiredanddrive
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == Mode11::Wiredanddrivefilter
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == Mode11::Wiredanddrivepullup
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == Mode11::Wiredanddrivepullupfilter
    }
}
#[doc = "Field `MODE11` writer - Pin 11 Mode"]
pub type Mode11W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode11, crate::Safe>;
impl<'a, REG> Mode11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::Pushpull)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::Pushpulldrive)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::Wiredanddrive)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::Wiredanddrivefilter)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::Wiredanddrivepullup)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::Wiredanddrivepullupfilter)
    }
}
#[doc = "Pin 12 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode12 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    Pushpulldrive = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    Wiredanddrive = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    Wiredanddrivefilter = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    Wiredanddrivepullup = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    Wiredanddrivepullupfilter = 15,
}
impl From<Mode12> for u8 {
    #[inline(always)]
    fn from(variant: Mode12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode12 {
    type Ux = u8;
}
impl crate::IsEnum for Mode12 {}
#[doc = "Field `MODE12` reader - Pin 12 Mode"]
pub type Mode12R = crate::FieldReader<Mode12>;
impl Mode12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode12 {
        match self.bits {
            0 => Mode12::Disabled,
            1 => Mode12::Input,
            2 => Mode12::Inputpull,
            3 => Mode12::Inputpullfilter,
            4 => Mode12::Pushpull,
            5 => Mode12::Pushpulldrive,
            6 => Mode12::Wiredor,
            7 => Mode12::Wiredorpulldown,
            8 => Mode12::Wiredand,
            9 => Mode12::Wiredandfilter,
            10 => Mode12::Wiredandpullup,
            11 => Mode12::Wiredandpullupfilter,
            12 => Mode12::Wiredanddrive,
            13 => Mode12::Wiredanddrivefilter,
            14 => Mode12::Wiredanddrivepullup,
            15 => Mode12::Wiredanddrivepullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mode12::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Mode12::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == Mode12::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == Mode12::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Mode12::Pushpull
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == Mode12::Pushpulldrive
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == Mode12::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == Mode12::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == Mode12::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == Mode12::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == Mode12::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == Mode12::Wiredandpullupfilter
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == Mode12::Wiredanddrive
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == Mode12::Wiredanddrivefilter
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == Mode12::Wiredanddrivepullup
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == Mode12::Wiredanddrivepullupfilter
    }
}
#[doc = "Field `MODE12` writer - Pin 12 Mode"]
pub type Mode12W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode12, crate::Safe>;
impl<'a, REG> Mode12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::Pushpull)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::Pushpulldrive)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::Wiredanddrive)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::Wiredanddrivefilter)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::Wiredanddrivepullup)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::Wiredanddrivepullupfilter)
    }
}
#[doc = "Pin 13 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode13 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    Pushpulldrive = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    Wiredanddrive = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    Wiredanddrivefilter = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    Wiredanddrivepullup = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    Wiredanddrivepullupfilter = 15,
}
impl From<Mode13> for u8 {
    #[inline(always)]
    fn from(variant: Mode13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode13 {
    type Ux = u8;
}
impl crate::IsEnum for Mode13 {}
#[doc = "Field `MODE13` reader - Pin 13 Mode"]
pub type Mode13R = crate::FieldReader<Mode13>;
impl Mode13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode13 {
        match self.bits {
            0 => Mode13::Disabled,
            1 => Mode13::Input,
            2 => Mode13::Inputpull,
            3 => Mode13::Inputpullfilter,
            4 => Mode13::Pushpull,
            5 => Mode13::Pushpulldrive,
            6 => Mode13::Wiredor,
            7 => Mode13::Wiredorpulldown,
            8 => Mode13::Wiredand,
            9 => Mode13::Wiredandfilter,
            10 => Mode13::Wiredandpullup,
            11 => Mode13::Wiredandpullupfilter,
            12 => Mode13::Wiredanddrive,
            13 => Mode13::Wiredanddrivefilter,
            14 => Mode13::Wiredanddrivepullup,
            15 => Mode13::Wiredanddrivepullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mode13::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Mode13::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == Mode13::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == Mode13::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Mode13::Pushpull
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == Mode13::Pushpulldrive
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == Mode13::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == Mode13::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == Mode13::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == Mode13::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == Mode13::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == Mode13::Wiredandpullupfilter
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == Mode13::Wiredanddrive
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == Mode13::Wiredanddrivefilter
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == Mode13::Wiredanddrivepullup
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == Mode13::Wiredanddrivepullupfilter
    }
}
#[doc = "Field `MODE13` writer - Pin 13 Mode"]
pub type Mode13W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode13, crate::Safe>;
impl<'a, REG> Mode13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::Pushpull)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::Pushpulldrive)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::Wiredanddrive)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::Wiredanddrivefilter)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::Wiredanddrivepullup)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::Wiredanddrivepullupfilter)
    }
}
#[doc = "Pin 14 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode14 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    Pushpulldrive = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    Wiredanddrive = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    Wiredanddrivefilter = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    Wiredanddrivepullup = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    Wiredanddrivepullupfilter = 15,
}
impl From<Mode14> for u8 {
    #[inline(always)]
    fn from(variant: Mode14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode14 {
    type Ux = u8;
}
impl crate::IsEnum for Mode14 {}
#[doc = "Field `MODE14` reader - Pin 14 Mode"]
pub type Mode14R = crate::FieldReader<Mode14>;
impl Mode14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode14 {
        match self.bits {
            0 => Mode14::Disabled,
            1 => Mode14::Input,
            2 => Mode14::Inputpull,
            3 => Mode14::Inputpullfilter,
            4 => Mode14::Pushpull,
            5 => Mode14::Pushpulldrive,
            6 => Mode14::Wiredor,
            7 => Mode14::Wiredorpulldown,
            8 => Mode14::Wiredand,
            9 => Mode14::Wiredandfilter,
            10 => Mode14::Wiredandpullup,
            11 => Mode14::Wiredandpullupfilter,
            12 => Mode14::Wiredanddrive,
            13 => Mode14::Wiredanddrivefilter,
            14 => Mode14::Wiredanddrivepullup,
            15 => Mode14::Wiredanddrivepullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mode14::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Mode14::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == Mode14::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == Mode14::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Mode14::Pushpull
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == Mode14::Pushpulldrive
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == Mode14::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == Mode14::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == Mode14::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == Mode14::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == Mode14::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == Mode14::Wiredandpullupfilter
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == Mode14::Wiredanddrive
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == Mode14::Wiredanddrivefilter
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == Mode14::Wiredanddrivepullup
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == Mode14::Wiredanddrivepullupfilter
    }
}
#[doc = "Field `MODE14` writer - Pin 14 Mode"]
pub type Mode14W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode14, crate::Safe>;
impl<'a, REG> Mode14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::Pushpull)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::Pushpulldrive)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::Wiredanddrive)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::Wiredanddrivefilter)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::Wiredanddrivepullup)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::Wiredanddrivepullupfilter)
    }
}
#[doc = "Pin 15 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode15 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    Pushpulldrive = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    Wiredanddrive = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    Wiredanddrivefilter = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    Wiredanddrivepullup = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    Wiredanddrivepullupfilter = 15,
}
impl From<Mode15> for u8 {
    #[inline(always)]
    fn from(variant: Mode15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode15 {
    type Ux = u8;
}
impl crate::IsEnum for Mode15 {}
#[doc = "Field `MODE15` reader - Pin 15 Mode"]
pub type Mode15R = crate::FieldReader<Mode15>;
impl Mode15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode15 {
        match self.bits {
            0 => Mode15::Disabled,
            1 => Mode15::Input,
            2 => Mode15::Inputpull,
            3 => Mode15::Inputpullfilter,
            4 => Mode15::Pushpull,
            5 => Mode15::Pushpulldrive,
            6 => Mode15::Wiredor,
            7 => Mode15::Wiredorpulldown,
            8 => Mode15::Wiredand,
            9 => Mode15::Wiredandfilter,
            10 => Mode15::Wiredandpullup,
            11 => Mode15::Wiredandpullupfilter,
            12 => Mode15::Wiredanddrive,
            13 => Mode15::Wiredanddrivefilter,
            14 => Mode15::Wiredanddrivepullup,
            15 => Mode15::Wiredanddrivepullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mode15::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Mode15::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == Mode15::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == Mode15::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Mode15::Pushpull
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == Mode15::Pushpulldrive
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == Mode15::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == Mode15::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == Mode15::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == Mode15::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == Mode15::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == Mode15::Wiredandpullupfilter
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == Mode15::Wiredanddrive
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == Mode15::Wiredanddrivefilter
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == Mode15::Wiredanddrivepullup
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == Mode15::Wiredanddrivepullupfilter
    }
}
#[doc = "Field `MODE15` writer - Pin 15 Mode"]
pub type Mode15W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode15, crate::Safe>;
impl<'a, REG> Mode15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::Pushpull)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::Pushpulldrive)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::Wiredanddrive)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::Wiredanddrivefilter)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::Wiredanddrivepullup)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::Wiredanddrivepullupfilter)
    }
}
impl R {
    #[doc = "Bits 0:3 - Pin 8 Mode"]
    #[inline(always)]
    pub fn mode8(&self) -> Mode8R {
        Mode8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pin 9 Mode"]
    #[inline(always)]
    pub fn mode9(&self) -> Mode9R {
        Mode9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Pin 10 Mode"]
    #[inline(always)]
    pub fn mode10(&self) -> Mode10R {
        Mode10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Pin 11 Mode"]
    #[inline(always)]
    pub fn mode11(&self) -> Mode11R {
        Mode11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Pin 12 Mode"]
    #[inline(always)]
    pub fn mode12(&self) -> Mode12R {
        Mode12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Pin 13 Mode"]
    #[inline(always)]
    pub fn mode13(&self) -> Mode13R {
        Mode13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Pin 14 Mode"]
    #[inline(always)]
    pub fn mode14(&self) -> Mode14R {
        Mode14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Pin 15 Mode"]
    #[inline(always)]
    pub fn mode15(&self) -> Mode15R {
        Mode15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pin 8 Mode"]
    #[inline(always)]
    pub fn mode8(&mut self) -> Mode8W<'_, PcModehSpec> {
        Mode8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Pin 9 Mode"]
    #[inline(always)]
    pub fn mode9(&mut self) -> Mode9W<'_, PcModehSpec> {
        Mode9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Pin 10 Mode"]
    #[inline(always)]
    pub fn mode10(&mut self) -> Mode10W<'_, PcModehSpec> {
        Mode10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Pin 11 Mode"]
    #[inline(always)]
    pub fn mode11(&mut self) -> Mode11W<'_, PcModehSpec> {
        Mode11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Pin 12 Mode"]
    #[inline(always)]
    pub fn mode12(&mut self) -> Mode12W<'_, PcModehSpec> {
        Mode12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Pin 13 Mode"]
    #[inline(always)]
    pub fn mode13(&mut self) -> Mode13W<'_, PcModehSpec> {
        Mode13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Pin 14 Mode"]
    #[inline(always)]
    pub fn mode14(&mut self) -> Mode14W<'_, PcModehSpec> {
        Mode14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Pin 15 Mode"]
    #[inline(always)]
    pub fn mode15(&mut self) -> Mode15W<'_, PcModehSpec> {
        Mode15W::new(self, 28)
    }
}
#[doc = "Port Pin Mode High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_modeh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_modeh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcModehSpec;
impl crate::RegisterSpec for PcModehSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc_modeh::R`](R) reader structure"]
impl crate::Readable for PcModehSpec {}
#[doc = "`write(|w| ..)` method takes [`pc_modeh::W`](W) writer structure"]
impl crate::Writable for PcModehSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PC_MODEH to value 0"]
impl crate::Resettable for PcModehSpec {}
