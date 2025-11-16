#[doc = "Register `MSTPCRD` reader"]
pub type R = crate::R<MstpcrdSpec>;
#[doc = "Register `MSTPCRD` writer"]
pub type W = crate::W<MstpcrdSpec>;
#[doc = "Asynchronous General Purpose Timer 1 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd2 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpd2> for bool {
    #[inline(always)]
    fn from(variant: Mstpd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPD2` reader - Asynchronous General Purpose Timer 1 Module Stop"]
pub type Mstpd2R = crate::BitReader<Mstpd2>;
impl Mstpd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd2 {
        match self.bits {
            false => Mstpd2::_0,
            true => Mstpd2::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd2::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd2::_1
    }
}
#[doc = "Field `MSTPD2` writer - Asynchronous General Purpose Timer 1 Module Stop"]
pub type Mstpd2W<'a, REG> = crate::BitWriter<'a, REG, Mstpd2>;
impl<'a, REG> Mstpd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd2::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd2::_1)
    }
}
#[doc = "Asynchronous General Purpose Timer 0 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd3 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpd3> for bool {
    #[inline(always)]
    fn from(variant: Mstpd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPD3` reader - Asynchronous General Purpose Timer 0 Module Stop"]
pub type Mstpd3R = crate::BitReader<Mstpd3>;
impl Mstpd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd3 {
        match self.bits {
            false => Mstpd3::_0,
            true => Mstpd3::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd3::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd3::_1
    }
}
#[doc = "Field `MSTPD3` writer - Asynchronous General Purpose Timer 0 Module Stop"]
pub type Mstpd3W<'a, REG> = crate::BitWriter<'a, REG, Mstpd3>;
impl<'a, REG> Mstpd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd3::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd3::_1)
    }
}
#[doc = "General PWM Timer 323 to 320 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd5 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpd5> for bool {
    #[inline(always)]
    fn from(variant: Mstpd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPD5` reader - General PWM Timer 323 to 320 Module Stop"]
pub type Mstpd5R = crate::BitReader<Mstpd5>;
impl Mstpd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd5 {
        match self.bits {
            false => Mstpd5::_0,
            true => Mstpd5::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd5::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd5::_1
    }
}
#[doc = "Field `MSTPD5` writer - General PWM Timer 323 to 320 Module Stop"]
pub type Mstpd5W<'a, REG> = crate::BitWriter<'a, REG, Mstpd5>;
impl<'a, REG> Mstpd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd5::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd5::_1)
    }
}
#[doc = "General PWM Timer 169 to 164 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd6 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpd6> for bool {
    #[inline(always)]
    fn from(variant: Mstpd6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPD6` reader - General PWM Timer 169 to 164 Module Stop"]
pub type Mstpd6R = crate::BitReader<Mstpd6>;
impl Mstpd6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd6 {
        match self.bits {
            false => Mstpd6::_0,
            true => Mstpd6::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd6::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd6::_1
    }
}
#[doc = "Field `MSTPD6` writer - General PWM Timer 169 to 164 Module Stop"]
pub type Mstpd6W<'a, REG> = crate::BitWriter<'a, REG, Mstpd6>;
impl<'a, REG> Mstpd6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd6::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd6::_1)
    }
}
#[doc = "Port Output Enable for GPT Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd14 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpd14> for bool {
    #[inline(always)]
    fn from(variant: Mstpd14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPD14` reader - Port Output Enable for GPT Module Stop"]
pub type Mstpd14R = crate::BitReader<Mstpd14>;
impl Mstpd14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd14 {
        match self.bits {
            false => Mstpd14::_0,
            true => Mstpd14::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd14::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd14::_1
    }
}
#[doc = "Field `MSTPD14` writer - Port Output Enable for GPT Module Stop"]
pub type Mstpd14W<'a, REG> = crate::BitWriter<'a, REG, Mstpd14>;
impl<'a, REG> Mstpd14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd14::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd14::_1)
    }
}
#[doc = "14-Bit A/D Converter Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd16 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpd16> for bool {
    #[inline(always)]
    fn from(variant: Mstpd16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPD16` reader - 14-Bit A/D Converter Module Stop"]
pub type Mstpd16R = crate::BitReader<Mstpd16>;
impl Mstpd16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd16 {
        match self.bits {
            false => Mstpd16::_0,
            true => Mstpd16::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd16::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd16::_1
    }
}
#[doc = "Field `MSTPD16` writer - 14-Bit A/D Converter Module Stop"]
pub type Mstpd16W<'a, REG> = crate::BitWriter<'a, REG, Mstpd16>;
impl<'a, REG> Mstpd16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd16::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd16::_1)
    }
}
#[doc = "8-bit D/A Converter Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd19 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpd19> for bool {
    #[inline(always)]
    fn from(variant: Mstpd19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPD19` reader - 8-bit D/A Converter Module Stop"]
pub type Mstpd19R = crate::BitReader<Mstpd19>;
impl Mstpd19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd19 {
        match self.bits {
            false => Mstpd19::_0,
            true => Mstpd19::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd19::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd19::_1
    }
}
#[doc = "Field `MSTPD19` writer - 8-bit D/A Converter Module Stop"]
pub type Mstpd19W<'a, REG> = crate::BitWriter<'a, REG, Mstpd19>;
impl<'a, REG> Mstpd19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd19::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd19::_1)
    }
}
#[doc = "12-Bit D/A Converter Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd20 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpd20> for bool {
    #[inline(always)]
    fn from(variant: Mstpd20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPD20` reader - 12-Bit D/A Converter Module Stop"]
pub type Mstpd20R = crate::BitReader<Mstpd20>;
impl Mstpd20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd20 {
        match self.bits {
            false => Mstpd20::_0,
            true => Mstpd20::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd20::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd20::_1
    }
}
#[doc = "Field `MSTPD20` writer - 12-Bit D/A Converter Module Stop"]
pub type Mstpd20W<'a, REG> = crate::BitWriter<'a, REG, Mstpd20>;
impl<'a, REG> Mstpd20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd20::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd20::_1)
    }
}
#[doc = "Low-Power Analog Comparator Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd29 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpd29> for bool {
    #[inline(always)]
    fn from(variant: Mstpd29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPD29` reader - Low-Power Analog Comparator Module Stop"]
pub type Mstpd29R = crate::BitReader<Mstpd29>;
impl Mstpd29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd29 {
        match self.bits {
            false => Mstpd29::_0,
            true => Mstpd29::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd29::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd29::_1
    }
}
#[doc = "Field `MSTPD29` writer - Low-Power Analog Comparator Module Stop"]
pub type Mstpd29W<'a, REG> = crate::BitWriter<'a, REG, Mstpd29>;
impl<'a, REG> Mstpd29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd29::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd29::_1)
    }
}
#[doc = "Operational Amplifier Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd31 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpd31> for bool {
    #[inline(always)]
    fn from(variant: Mstpd31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPD31` reader - Operational Amplifier Module Stop"]
pub type Mstpd31R = crate::BitReader<Mstpd31>;
impl Mstpd31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd31 {
        match self.bits {
            false => Mstpd31::_0,
            true => Mstpd31::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd31::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd31::_1
    }
}
#[doc = "Field `MSTPD31` writer - Operational Amplifier Module Stop"]
pub type Mstpd31W<'a, REG> = crate::BitWriter<'a, REG, Mstpd31>;
impl<'a, REG> Mstpd31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd31::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd31::_1)
    }
}
impl R {
    #[doc = "Bit 2 - Asynchronous General Purpose Timer 1 Module Stop"]
    #[inline(always)]
    pub fn mstpd2(&self) -> Mstpd2R {
        Mstpd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Asynchronous General Purpose Timer 0 Module Stop"]
    #[inline(always)]
    pub fn mstpd3(&self) -> Mstpd3R {
        Mstpd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - General PWM Timer 323 to 320 Module Stop"]
    #[inline(always)]
    pub fn mstpd5(&self) -> Mstpd5R {
        Mstpd5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General PWM Timer 169 to 164 Module Stop"]
    #[inline(always)]
    pub fn mstpd6(&self) -> Mstpd6R {
        Mstpd6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - Port Output Enable for GPT Module Stop"]
    #[inline(always)]
    pub fn mstpd14(&self) -> Mstpd14R {
        Mstpd14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 14-Bit A/D Converter Module Stop"]
    #[inline(always)]
    pub fn mstpd16(&self) -> Mstpd16R {
        Mstpd16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - 8-bit D/A Converter Module Stop"]
    #[inline(always)]
    pub fn mstpd19(&self) -> Mstpd19R {
        Mstpd19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 12-Bit D/A Converter Module Stop"]
    #[inline(always)]
    pub fn mstpd20(&self) -> Mstpd20R {
        Mstpd20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 29 - Low-Power Analog Comparator Module Stop"]
    #[inline(always)]
    pub fn mstpd29(&self) -> Mstpd29R {
        Mstpd29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Operational Amplifier Module Stop"]
    #[inline(always)]
    pub fn mstpd31(&self) -> Mstpd31R {
        Mstpd31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTPCRD")
            .field("mstpd31", &self.mstpd31())
            .field("mstpd29", &self.mstpd29())
            .field("mstpd20", &self.mstpd20())
            .field("mstpd19", &self.mstpd19())
            .field("mstpd16", &self.mstpd16())
            .field("mstpd14", &self.mstpd14())
            .field("mstpd6", &self.mstpd6())
            .field("mstpd5", &self.mstpd5())
            .field("mstpd3", &self.mstpd3())
            .field("mstpd2", &self.mstpd2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Asynchronous General Purpose Timer 1 Module Stop"]
    #[inline(always)]
    pub fn mstpd2(&mut self) -> Mstpd2W<MstpcrdSpec> {
        Mstpd2W::new(self, 2)
    }
    #[doc = "Bit 3 - Asynchronous General Purpose Timer 0 Module Stop"]
    #[inline(always)]
    pub fn mstpd3(&mut self) -> Mstpd3W<MstpcrdSpec> {
        Mstpd3W::new(self, 3)
    }
    #[doc = "Bit 5 - General PWM Timer 323 to 320 Module Stop"]
    #[inline(always)]
    pub fn mstpd5(&mut self) -> Mstpd5W<MstpcrdSpec> {
        Mstpd5W::new(self, 5)
    }
    #[doc = "Bit 6 - General PWM Timer 169 to 164 Module Stop"]
    #[inline(always)]
    pub fn mstpd6(&mut self) -> Mstpd6W<MstpcrdSpec> {
        Mstpd6W::new(self, 6)
    }
    #[doc = "Bit 14 - Port Output Enable for GPT Module Stop"]
    #[inline(always)]
    pub fn mstpd14(&mut self) -> Mstpd14W<MstpcrdSpec> {
        Mstpd14W::new(self, 14)
    }
    #[doc = "Bit 16 - 14-Bit A/D Converter Module Stop"]
    #[inline(always)]
    pub fn mstpd16(&mut self) -> Mstpd16W<MstpcrdSpec> {
        Mstpd16W::new(self, 16)
    }
    #[doc = "Bit 19 - 8-bit D/A Converter Module Stop"]
    #[inline(always)]
    pub fn mstpd19(&mut self) -> Mstpd19W<MstpcrdSpec> {
        Mstpd19W::new(self, 19)
    }
    #[doc = "Bit 20 - 12-Bit D/A Converter Module Stop"]
    #[inline(always)]
    pub fn mstpd20(&mut self) -> Mstpd20W<MstpcrdSpec> {
        Mstpd20W::new(self, 20)
    }
    #[doc = "Bit 29 - Low-Power Analog Comparator Module Stop"]
    #[inline(always)]
    pub fn mstpd29(&mut self) -> Mstpd29W<MstpcrdSpec> {
        Mstpd29W::new(self, 29)
    }
    #[doc = "Bit 31 - Operational Amplifier Module Stop"]
    #[inline(always)]
    pub fn mstpd31(&mut self) -> Mstpd31W<MstpcrdSpec> {
        Mstpd31W::new(self, 31)
    }
}
#[doc = "Module Stop Control Register D\n\nYou can [`read`](crate::Reg::read) this register and get [`mstpcrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstpcrdSpec;
impl crate::RegisterSpec for MstpcrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstpcrd::R`](R) reader structure"]
impl crate::Readable for MstpcrdSpec {}
#[doc = "`write(|w| ..)` method takes [`mstpcrd::W`](W) writer structure"]
impl crate::Writable for MstpcrdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSTPCRD to value 0xffff_ffff"]
impl crate::Resettable for MstpcrdSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
