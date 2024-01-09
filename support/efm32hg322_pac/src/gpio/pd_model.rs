#[doc = "Register `PD_MODEL` reader"]
pub type R = crate::R<PD_MODEL_SPEC>;
#[doc = "Register `PD_MODEL` writer"]
pub type W = crate::W<PD_MODEL_SPEC>;
#[doc = "Field `MODE0` reader - Pin 0 Mode"]
pub type MODE0_R = crate::FieldReader<MODE0_A>;
#[doc = "Pin 0 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE0_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER = 15,
}
impl From<MODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE0_A {
    type Ux = u8;
}
impl MODE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE0_A {
        match self.bits {
            0 => MODE0_A::DISABLED,
            1 => MODE0_A::INPUT,
            2 => MODE0_A::INPUTPULL,
            3 => MODE0_A::INPUTPULLFILTER,
            4 => MODE0_A::PUSHPULL,
            5 => MODE0_A::PUSHPULLDRIVE,
            6 => MODE0_A::WIREDOR,
            7 => MODE0_A::WIREDORPULLDOWN,
            8 => MODE0_A::WIREDAND,
            9 => MODE0_A::WIREDANDFILTER,
            10 => MODE0_A::WIREDANDPULLUP,
            11 => MODE0_A::WIREDANDPULLUPFILTER,
            12 => MODE0_A::WIREDANDDRIVE,
            13 => MODE0_A::WIREDANDDRIVEFILTER,
            14 => MODE0_A::WIREDANDDRIVEPULLUP,
            15 => MODE0_A::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE0_A::DISABLED
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE0_A::INPUT
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE0_A::INPUTPULL
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE0_A::INPUTPULLFILTER
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE0_A::PUSHPULL
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE0_A::PUSHPULLDRIVE
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE0_A::WIREDOR
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE0_A::WIREDORPULLDOWN
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE0_A::WIREDAND
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE0_A::WIREDANDFILTER
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE0_A::WIREDANDPULLUP
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE0_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE0_A::WIREDANDDRIVE
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE0_A::WIREDANDDRIVEFILTER
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE0_A::WIREDANDDRIVEPULLUP
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE0_A::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Field `MODE0` writer - Pin 0 Mode"]
pub type MODE0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, MODE0_A>;
impl<'a, REG> MODE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::WIREDANDDRIVEPULLUPFILTER)
    }
}
#[doc = "Field `MODE1` reader - Pin 1 Mode"]
pub type MODE1_R = crate::FieldReader<MODE1_A>;
#[doc = "Pin 1 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE1_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER = 15,
}
impl From<MODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE1_A {
    type Ux = u8;
}
impl MODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE1_A {
        match self.bits {
            0 => MODE1_A::DISABLED,
            1 => MODE1_A::INPUT,
            2 => MODE1_A::INPUTPULL,
            3 => MODE1_A::INPUTPULLFILTER,
            4 => MODE1_A::PUSHPULL,
            5 => MODE1_A::PUSHPULLDRIVE,
            6 => MODE1_A::WIREDOR,
            7 => MODE1_A::WIREDORPULLDOWN,
            8 => MODE1_A::WIREDAND,
            9 => MODE1_A::WIREDANDFILTER,
            10 => MODE1_A::WIREDANDPULLUP,
            11 => MODE1_A::WIREDANDPULLUPFILTER,
            12 => MODE1_A::WIREDANDDRIVE,
            13 => MODE1_A::WIREDANDDRIVEFILTER,
            14 => MODE1_A::WIREDANDDRIVEPULLUP,
            15 => MODE1_A::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE1_A::DISABLED
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE1_A::INPUT
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE1_A::INPUTPULL
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE1_A::INPUTPULLFILTER
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE1_A::PUSHPULL
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE1_A::PUSHPULLDRIVE
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE1_A::WIREDOR
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE1_A::WIREDORPULLDOWN
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE1_A::WIREDAND
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE1_A::WIREDANDFILTER
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE1_A::WIREDANDPULLUP
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE1_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE1_A::WIREDANDDRIVE
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE1_A::WIREDANDDRIVEFILTER
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE1_A::WIREDANDDRIVEPULLUP
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE1_A::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Field `MODE1` writer - Pin 1 Mode"]
pub type MODE1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, MODE1_A>;
impl<'a, REG> MODE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::WIREDANDDRIVEPULLUPFILTER)
    }
}
#[doc = "Field `MODE2` reader - Pin 2 Mode"]
pub type MODE2_R = crate::FieldReader<MODE2_A>;
#[doc = "Pin 2 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE2_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER = 15,
}
impl From<MODE2_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE2_A {
    type Ux = u8;
}
impl MODE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE2_A {
        match self.bits {
            0 => MODE2_A::DISABLED,
            1 => MODE2_A::INPUT,
            2 => MODE2_A::INPUTPULL,
            3 => MODE2_A::INPUTPULLFILTER,
            4 => MODE2_A::PUSHPULL,
            5 => MODE2_A::PUSHPULLDRIVE,
            6 => MODE2_A::WIREDOR,
            7 => MODE2_A::WIREDORPULLDOWN,
            8 => MODE2_A::WIREDAND,
            9 => MODE2_A::WIREDANDFILTER,
            10 => MODE2_A::WIREDANDPULLUP,
            11 => MODE2_A::WIREDANDPULLUPFILTER,
            12 => MODE2_A::WIREDANDDRIVE,
            13 => MODE2_A::WIREDANDDRIVEFILTER,
            14 => MODE2_A::WIREDANDDRIVEPULLUP,
            15 => MODE2_A::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE2_A::DISABLED
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE2_A::INPUT
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE2_A::INPUTPULL
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE2_A::INPUTPULLFILTER
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE2_A::PUSHPULL
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE2_A::PUSHPULLDRIVE
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE2_A::WIREDOR
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE2_A::WIREDORPULLDOWN
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE2_A::WIREDAND
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE2_A::WIREDANDFILTER
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE2_A::WIREDANDPULLUP
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE2_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE2_A::WIREDANDDRIVE
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE2_A::WIREDANDDRIVEFILTER
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE2_A::WIREDANDDRIVEPULLUP
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE2_A::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Field `MODE2` writer - Pin 2 Mode"]
pub type MODE2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, MODE2_A>;
impl<'a, REG> MODE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::WIREDANDDRIVEPULLUPFILTER)
    }
}
#[doc = "Field `MODE3` reader - Pin 3 Mode"]
pub type MODE3_R = crate::FieldReader<MODE3_A>;
#[doc = "Pin 3 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE3_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER = 15,
}
impl From<MODE3_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE3_A {
    type Ux = u8;
}
impl MODE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE3_A {
        match self.bits {
            0 => MODE3_A::DISABLED,
            1 => MODE3_A::INPUT,
            2 => MODE3_A::INPUTPULL,
            3 => MODE3_A::INPUTPULLFILTER,
            4 => MODE3_A::PUSHPULL,
            5 => MODE3_A::PUSHPULLDRIVE,
            6 => MODE3_A::WIREDOR,
            7 => MODE3_A::WIREDORPULLDOWN,
            8 => MODE3_A::WIREDAND,
            9 => MODE3_A::WIREDANDFILTER,
            10 => MODE3_A::WIREDANDPULLUP,
            11 => MODE3_A::WIREDANDPULLUPFILTER,
            12 => MODE3_A::WIREDANDDRIVE,
            13 => MODE3_A::WIREDANDDRIVEFILTER,
            14 => MODE3_A::WIREDANDDRIVEPULLUP,
            15 => MODE3_A::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE3_A::DISABLED
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE3_A::INPUT
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE3_A::INPUTPULL
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE3_A::INPUTPULLFILTER
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE3_A::PUSHPULL
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE3_A::PUSHPULLDRIVE
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE3_A::WIREDOR
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE3_A::WIREDORPULLDOWN
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE3_A::WIREDAND
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE3_A::WIREDANDFILTER
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE3_A::WIREDANDPULLUP
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE3_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE3_A::WIREDANDDRIVE
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE3_A::WIREDANDDRIVEFILTER
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE3_A::WIREDANDDRIVEPULLUP
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE3_A::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Field `MODE3` writer - Pin 3 Mode"]
pub type MODE3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, MODE3_A>;
impl<'a, REG> MODE3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::WIREDANDDRIVEPULLUPFILTER)
    }
}
#[doc = "Field `MODE4` reader - Pin 4 Mode"]
pub type MODE4_R = crate::FieldReader<MODE4_A>;
#[doc = "Pin 4 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE4_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER = 15,
}
impl From<MODE4_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE4_A {
    type Ux = u8;
}
impl MODE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE4_A {
        match self.bits {
            0 => MODE4_A::DISABLED,
            1 => MODE4_A::INPUT,
            2 => MODE4_A::INPUTPULL,
            3 => MODE4_A::INPUTPULLFILTER,
            4 => MODE4_A::PUSHPULL,
            5 => MODE4_A::PUSHPULLDRIVE,
            6 => MODE4_A::WIREDOR,
            7 => MODE4_A::WIREDORPULLDOWN,
            8 => MODE4_A::WIREDAND,
            9 => MODE4_A::WIREDANDFILTER,
            10 => MODE4_A::WIREDANDPULLUP,
            11 => MODE4_A::WIREDANDPULLUPFILTER,
            12 => MODE4_A::WIREDANDDRIVE,
            13 => MODE4_A::WIREDANDDRIVEFILTER,
            14 => MODE4_A::WIREDANDDRIVEPULLUP,
            15 => MODE4_A::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE4_A::DISABLED
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE4_A::INPUT
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE4_A::INPUTPULL
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE4_A::INPUTPULLFILTER
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE4_A::PUSHPULL
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE4_A::PUSHPULLDRIVE
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE4_A::WIREDOR
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE4_A::WIREDORPULLDOWN
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE4_A::WIREDAND
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE4_A::WIREDANDFILTER
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE4_A::WIREDANDPULLUP
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE4_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE4_A::WIREDANDDRIVE
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE4_A::WIREDANDDRIVEFILTER
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE4_A::WIREDANDDRIVEPULLUP
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE4_A::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Field `MODE4` writer - Pin 4 Mode"]
pub type MODE4_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, MODE4_A>;
impl<'a, REG> MODE4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::WIREDANDDRIVEPULLUPFILTER)
    }
}
#[doc = "Field `MODE5` reader - Pin 5 Mode"]
pub type MODE5_R = crate::FieldReader<MODE5_A>;
#[doc = "Pin 5 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE5_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER = 15,
}
impl From<MODE5_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE5_A {
    type Ux = u8;
}
impl MODE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE5_A {
        match self.bits {
            0 => MODE5_A::DISABLED,
            1 => MODE5_A::INPUT,
            2 => MODE5_A::INPUTPULL,
            3 => MODE5_A::INPUTPULLFILTER,
            4 => MODE5_A::PUSHPULL,
            5 => MODE5_A::PUSHPULLDRIVE,
            6 => MODE5_A::WIREDOR,
            7 => MODE5_A::WIREDORPULLDOWN,
            8 => MODE5_A::WIREDAND,
            9 => MODE5_A::WIREDANDFILTER,
            10 => MODE5_A::WIREDANDPULLUP,
            11 => MODE5_A::WIREDANDPULLUPFILTER,
            12 => MODE5_A::WIREDANDDRIVE,
            13 => MODE5_A::WIREDANDDRIVEFILTER,
            14 => MODE5_A::WIREDANDDRIVEPULLUP,
            15 => MODE5_A::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE5_A::DISABLED
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE5_A::INPUT
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE5_A::INPUTPULL
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE5_A::INPUTPULLFILTER
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE5_A::PUSHPULL
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE5_A::PUSHPULLDRIVE
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE5_A::WIREDOR
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE5_A::WIREDORPULLDOWN
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE5_A::WIREDAND
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE5_A::WIREDANDFILTER
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE5_A::WIREDANDPULLUP
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE5_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE5_A::WIREDANDDRIVE
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE5_A::WIREDANDDRIVEFILTER
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE5_A::WIREDANDDRIVEPULLUP
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE5_A::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Field `MODE5` writer - Pin 5 Mode"]
pub type MODE5_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, MODE5_A>;
impl<'a, REG> MODE5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::WIREDANDDRIVEPULLUPFILTER)
    }
}
#[doc = "Field `MODE6` reader - Pin 6 Mode"]
pub type MODE6_R = crate::FieldReader<MODE6_A>;
#[doc = "Pin 6 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE6_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER = 15,
}
impl From<MODE6_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE6_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE6_A {
    type Ux = u8;
}
impl MODE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE6_A {
        match self.bits {
            0 => MODE6_A::DISABLED,
            1 => MODE6_A::INPUT,
            2 => MODE6_A::INPUTPULL,
            3 => MODE6_A::INPUTPULLFILTER,
            4 => MODE6_A::PUSHPULL,
            5 => MODE6_A::PUSHPULLDRIVE,
            6 => MODE6_A::WIREDOR,
            7 => MODE6_A::WIREDORPULLDOWN,
            8 => MODE6_A::WIREDAND,
            9 => MODE6_A::WIREDANDFILTER,
            10 => MODE6_A::WIREDANDPULLUP,
            11 => MODE6_A::WIREDANDPULLUPFILTER,
            12 => MODE6_A::WIREDANDDRIVE,
            13 => MODE6_A::WIREDANDDRIVEFILTER,
            14 => MODE6_A::WIREDANDDRIVEPULLUP,
            15 => MODE6_A::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE6_A::DISABLED
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE6_A::INPUT
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE6_A::INPUTPULL
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE6_A::INPUTPULLFILTER
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE6_A::PUSHPULL
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE6_A::PUSHPULLDRIVE
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE6_A::WIREDOR
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE6_A::WIREDORPULLDOWN
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE6_A::WIREDAND
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE6_A::WIREDANDFILTER
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE6_A::WIREDANDPULLUP
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE6_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE6_A::WIREDANDDRIVE
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE6_A::WIREDANDDRIVEFILTER
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE6_A::WIREDANDDRIVEPULLUP
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE6_A::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Field `MODE6` writer - Pin 6 Mode"]
pub type MODE6_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, MODE6_A>;
impl<'a, REG> MODE6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::WIREDANDDRIVEPULLUPFILTER)
    }
}
#[doc = "Field `MODE7` reader - Pin 7 Mode"]
pub type MODE7_R = crate::FieldReader<MODE7_A>;
#[doc = "Pin 7 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE7_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER = 15,
}
impl From<MODE7_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE7_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE7_A {
    type Ux = u8;
}
impl MODE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE7_A {
        match self.bits {
            0 => MODE7_A::DISABLED,
            1 => MODE7_A::INPUT,
            2 => MODE7_A::INPUTPULL,
            3 => MODE7_A::INPUTPULLFILTER,
            4 => MODE7_A::PUSHPULL,
            5 => MODE7_A::PUSHPULLDRIVE,
            6 => MODE7_A::WIREDOR,
            7 => MODE7_A::WIREDORPULLDOWN,
            8 => MODE7_A::WIREDAND,
            9 => MODE7_A::WIREDANDFILTER,
            10 => MODE7_A::WIREDANDPULLUP,
            11 => MODE7_A::WIREDANDPULLUPFILTER,
            12 => MODE7_A::WIREDANDDRIVE,
            13 => MODE7_A::WIREDANDDRIVEFILTER,
            14 => MODE7_A::WIREDANDDRIVEPULLUP,
            15 => MODE7_A::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE7_A::DISABLED
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE7_A::INPUT
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE7_A::INPUTPULL
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE7_A::INPUTPULLFILTER
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE7_A::PUSHPULL
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE7_A::PUSHPULLDRIVE
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE7_A::WIREDOR
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE7_A::WIREDORPULLDOWN
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE7_A::WIREDAND
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE7_A::WIREDANDFILTER
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE7_A::WIREDANDPULLUP
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE7_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE7_A::WIREDANDDRIVE
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE7_A::WIREDANDDRIVEFILTER
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE7_A::WIREDANDDRIVEPULLUP
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE7_A::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Field `MODE7` writer - Pin 7 Mode"]
pub type MODE7_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, MODE7_A>;
impl<'a, REG> MODE7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::WIREDANDDRIVEPULLUPFILTER)
    }
}
impl R {
    #[doc = "Bits 0:3 - Pin 0 Mode"]
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pin 1 Mode"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Pin 2 Mode"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Pin 3 Mode"]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Pin 4 Mode"]
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Pin 5 Mode"]
    #[inline(always)]
    pub fn mode5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Pin 6 Mode"]
    #[inline(always)]
    pub fn mode6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Pin 7 Mode"]
    #[inline(always)]
    pub fn mode7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pin 0 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode0(&mut self) -> MODE0_W<PD_MODEL_SPEC> {
        MODE0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Pin 1 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<PD_MODEL_SPEC> {
        MODE1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Pin 2 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<PD_MODEL_SPEC> {
        MODE2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Pin 3 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode3(&mut self) -> MODE3_W<PD_MODEL_SPEC> {
        MODE3_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Pin 4 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode4(&mut self) -> MODE4_W<PD_MODEL_SPEC> {
        MODE4_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Pin 5 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode5(&mut self) -> MODE5_W<PD_MODEL_SPEC> {
        MODE5_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Pin 6 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode6(&mut self) -> MODE6_W<PD_MODEL_SPEC> {
        MODE6_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Pin 7 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode7(&mut self) -> MODE7_W<PD_MODEL_SPEC> {
        MODE7_W::new(self, 28)
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
#[doc = "Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_model::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_model::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PD_MODEL_SPEC;
impl crate::RegisterSpec for PD_MODEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_model::R`](R) reader structure"]
impl crate::Readable for PD_MODEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pd_model::W`](W) writer structure"]
impl crate::Writable for PD_MODEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PD_MODEL to value 0"]
impl crate::Resettable for PD_MODEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
