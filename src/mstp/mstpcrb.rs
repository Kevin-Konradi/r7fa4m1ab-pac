#[doc = "Register `MSTPCRB` reader"]
pub type R = crate::R<MstpcrbSpec>;
#[doc = "Register `MSTPCRB` writer"]
pub type W = crate::W<MstpcrbSpec>;
#[doc = "Controller Area Network Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb2 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpb2> for bool {
    #[inline(always)]
    fn from(variant: Mstpb2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPB2` reader - Controller Area Network Module Stop"]
pub type Mstpb2R = crate::BitReader<Mstpb2>;
impl Mstpb2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb2 {
        match self.bits {
            false => Mstpb2::_0,
            true => Mstpb2::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb2::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb2::_1
    }
}
#[doc = "Field `MSTPB2` writer - Controller Area Network Module Stop"]
pub type Mstpb2W<'a, REG> = crate::BitWriter<'a, REG, Mstpb2>;
impl<'a, REG> Mstpb2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb2::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb2::_1)
    }
}
#[doc = "I2C Bus Interface 1 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb8 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpb8> for bool {
    #[inline(always)]
    fn from(variant: Mstpb8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPB8` reader - I2C Bus Interface 1 Module Stop"]
pub type Mstpb8R = crate::BitReader<Mstpb8>;
impl Mstpb8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb8 {
        match self.bits {
            false => Mstpb8::_0,
            true => Mstpb8::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb8::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb8::_1
    }
}
#[doc = "Field `MSTPB8` writer - I2C Bus Interface 1 Module Stop"]
pub type Mstpb8W<'a, REG> = crate::BitWriter<'a, REG, Mstpb8>;
impl<'a, REG> Mstpb8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb8::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb8::_1)
    }
}
#[doc = "I2C Bus Interface 0 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb9 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpb9> for bool {
    #[inline(always)]
    fn from(variant: Mstpb9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPB9` reader - I2C Bus Interface 0 Module Stop"]
pub type Mstpb9R = crate::BitReader<Mstpb9>;
impl Mstpb9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb9 {
        match self.bits {
            false => Mstpb9::_0,
            true => Mstpb9::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb9::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb9::_1
    }
}
#[doc = "Field `MSTPB9` writer - I2C Bus Interface 0 Module Stop"]
pub type Mstpb9W<'a, REG> = crate::BitWriter<'a, REG, Mstpb9>;
impl<'a, REG> Mstpb9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb9::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb9::_1)
    }
}
#[doc = "Universal Serial Bus 2.0 FS Interface Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb11 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpb11> for bool {
    #[inline(always)]
    fn from(variant: Mstpb11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPB11` reader - Universal Serial Bus 2.0 FS Interface Module Stop"]
pub type Mstpb11R = crate::BitReader<Mstpb11>;
impl Mstpb11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb11 {
        match self.bits {
            false => Mstpb11::_0,
            true => Mstpb11::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb11::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb11::_1
    }
}
#[doc = "Field `MSTPB11` writer - Universal Serial Bus 2.0 FS Interface Module Stop"]
pub type Mstpb11W<'a, REG> = crate::BitWriter<'a, REG, Mstpb11>;
impl<'a, REG> Mstpb11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb11::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb11::_1)
    }
}
#[doc = "Serial Peripheral Interface 1 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb18 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpb18> for bool {
    #[inline(always)]
    fn from(variant: Mstpb18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPB18` reader - Serial Peripheral Interface 1 Module Stop"]
pub type Mstpb18R = crate::BitReader<Mstpb18>;
impl Mstpb18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb18 {
        match self.bits {
            false => Mstpb18::_0,
            true => Mstpb18::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb18::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb18::_1
    }
}
#[doc = "Field `MSTPB18` writer - Serial Peripheral Interface 1 Module Stop"]
pub type Mstpb18W<'a, REG> = crate::BitWriter<'a, REG, Mstpb18>;
impl<'a, REG> Mstpb18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb18::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb18::_1)
    }
}
#[doc = "Serial Peripheral Interface 0 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb19 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpb19> for bool {
    #[inline(always)]
    fn from(variant: Mstpb19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPB19` reader - Serial Peripheral Interface 0 Module Stop"]
pub type Mstpb19R = crate::BitReader<Mstpb19>;
impl Mstpb19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb19 {
        match self.bits {
            false => Mstpb19::_0,
            true => Mstpb19::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb19::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb19::_1
    }
}
#[doc = "Field `MSTPB19` writer - Serial Peripheral Interface 0 Module Stop"]
pub type Mstpb19W<'a, REG> = crate::BitWriter<'a, REG, Mstpb19>;
impl<'a, REG> Mstpb19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb19::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb19::_1)
    }
}
#[doc = "Serial Communication Interface 9 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb22 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpb22> for bool {
    #[inline(always)]
    fn from(variant: Mstpb22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPB22` reader - Serial Communication Interface 9 Module Stop"]
pub type Mstpb22R = crate::BitReader<Mstpb22>;
impl Mstpb22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb22 {
        match self.bits {
            false => Mstpb22::_0,
            true => Mstpb22::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb22::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb22::_1
    }
}
#[doc = "Field `MSTPB22` writer - Serial Communication Interface 9 Module Stop"]
pub type Mstpb22W<'a, REG> = crate::BitWriter<'a, REG, Mstpb22>;
impl<'a, REG> Mstpb22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb22::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb22::_1)
    }
}
#[doc = "Serial Communication Interface 2 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb29 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpb29> for bool {
    #[inline(always)]
    fn from(variant: Mstpb29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPB29` reader - Serial Communication Interface 2 Module Stop"]
pub type Mstpb29R = crate::BitReader<Mstpb29>;
impl Mstpb29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb29 {
        match self.bits {
            false => Mstpb29::_0,
            true => Mstpb29::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb29::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb29::_1
    }
}
#[doc = "Field `MSTPB29` writer - Serial Communication Interface 2 Module Stop"]
pub type Mstpb29W<'a, REG> = crate::BitWriter<'a, REG, Mstpb29>;
impl<'a, REG> Mstpb29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb29::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb29::_1)
    }
}
#[doc = "Serial Communication Interface 1 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb30 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpb30> for bool {
    #[inline(always)]
    fn from(variant: Mstpb30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPB30` reader - Serial Communication Interface 1 Module Stop"]
pub type Mstpb30R = crate::BitReader<Mstpb30>;
impl Mstpb30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb30 {
        match self.bits {
            false => Mstpb30::_0,
            true => Mstpb30::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb30::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb30::_1
    }
}
#[doc = "Field `MSTPB30` writer - Serial Communication Interface 1 Module Stop"]
pub type Mstpb30W<'a, REG> = crate::BitWriter<'a, REG, Mstpb30>;
impl<'a, REG> Mstpb30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb30::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb30::_1)
    }
}
#[doc = "Serial Communication Interface 0 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb31 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpb31> for bool {
    #[inline(always)]
    fn from(variant: Mstpb31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPB31` reader - Serial Communication Interface 0 Module Stop"]
pub type Mstpb31R = crate::BitReader<Mstpb31>;
impl Mstpb31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb31 {
        match self.bits {
            false => Mstpb31::_0,
            true => Mstpb31::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb31::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb31::_1
    }
}
#[doc = "Field `MSTPB31` writer - Serial Communication Interface 0 Module Stop"]
pub type Mstpb31W<'a, REG> = crate::BitWriter<'a, REG, Mstpb31>;
impl<'a, REG> Mstpb31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb31::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb31::_1)
    }
}
impl R {
    #[doc = "Bit 2 - Controller Area Network Module Stop"]
    #[inline(always)]
    pub fn mstpb2(&self) -> Mstpb2R {
        Mstpb2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Bus Interface 1 Module Stop"]
    #[inline(always)]
    pub fn mstpb8(&self) -> Mstpb8R {
        Mstpb8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Bus Interface 0 Module Stop"]
    #[inline(always)]
    pub fn mstpb9(&self) -> Mstpb9R {
        Mstpb9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Universal Serial Bus 2.0 FS Interface Module Stop"]
    #[inline(always)]
    pub fn mstpb11(&self) -> Mstpb11R {
        Mstpb11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 18 - Serial Peripheral Interface 1 Module Stop"]
    #[inline(always)]
    pub fn mstpb18(&self) -> Mstpb18R {
        Mstpb18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Serial Peripheral Interface 0 Module Stop"]
    #[inline(always)]
    pub fn mstpb19(&self) -> Mstpb19R {
        Mstpb19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - Serial Communication Interface 9 Module Stop"]
    #[inline(always)]
    pub fn mstpb22(&self) -> Mstpb22R {
        Mstpb22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 29 - Serial Communication Interface 2 Module Stop"]
    #[inline(always)]
    pub fn mstpb29(&self) -> Mstpb29R {
        Mstpb29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Serial Communication Interface 1 Module Stop"]
    #[inline(always)]
    pub fn mstpb30(&self) -> Mstpb30R {
        Mstpb30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Serial Communication Interface 0 Module Stop"]
    #[inline(always)]
    pub fn mstpb31(&self) -> Mstpb31R {
        Mstpb31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTPCRB")
            .field("mstpb31", &self.mstpb31())
            .field("mstpb30", &self.mstpb30())
            .field("mstpb29", &self.mstpb29())
            .field("mstpb22", &self.mstpb22())
            .field("mstpb19", &self.mstpb19())
            .field("mstpb18", &self.mstpb18())
            .field("mstpb11", &self.mstpb11())
            .field("mstpb9", &self.mstpb9())
            .field("mstpb8", &self.mstpb8())
            .field("mstpb2", &self.mstpb2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Controller Area Network Module Stop"]
    #[inline(always)]
    pub fn mstpb2(&mut self) -> Mstpb2W<MstpcrbSpec> {
        Mstpb2W::new(self, 2)
    }
    #[doc = "Bit 8 - I2C Bus Interface 1 Module Stop"]
    #[inline(always)]
    pub fn mstpb8(&mut self) -> Mstpb8W<MstpcrbSpec> {
        Mstpb8W::new(self, 8)
    }
    #[doc = "Bit 9 - I2C Bus Interface 0 Module Stop"]
    #[inline(always)]
    pub fn mstpb9(&mut self) -> Mstpb9W<MstpcrbSpec> {
        Mstpb9W::new(self, 9)
    }
    #[doc = "Bit 11 - Universal Serial Bus 2.0 FS Interface Module Stop"]
    #[inline(always)]
    pub fn mstpb11(&mut self) -> Mstpb11W<MstpcrbSpec> {
        Mstpb11W::new(self, 11)
    }
    #[doc = "Bit 18 - Serial Peripheral Interface 1 Module Stop"]
    #[inline(always)]
    pub fn mstpb18(&mut self) -> Mstpb18W<MstpcrbSpec> {
        Mstpb18W::new(self, 18)
    }
    #[doc = "Bit 19 - Serial Peripheral Interface 0 Module Stop"]
    #[inline(always)]
    pub fn mstpb19(&mut self) -> Mstpb19W<MstpcrbSpec> {
        Mstpb19W::new(self, 19)
    }
    #[doc = "Bit 22 - Serial Communication Interface 9 Module Stop"]
    #[inline(always)]
    pub fn mstpb22(&mut self) -> Mstpb22W<MstpcrbSpec> {
        Mstpb22W::new(self, 22)
    }
    #[doc = "Bit 29 - Serial Communication Interface 2 Module Stop"]
    #[inline(always)]
    pub fn mstpb29(&mut self) -> Mstpb29W<MstpcrbSpec> {
        Mstpb29W::new(self, 29)
    }
    #[doc = "Bit 30 - Serial Communication Interface 1 Module Stop"]
    #[inline(always)]
    pub fn mstpb30(&mut self) -> Mstpb30W<MstpcrbSpec> {
        Mstpb30W::new(self, 30)
    }
    #[doc = "Bit 31 - Serial Communication Interface 0 Module Stop"]
    #[inline(always)]
    pub fn mstpb31(&mut self) -> Mstpb31W<MstpcrbSpec> {
        Mstpb31W::new(self, 31)
    }
}
#[doc = "Module Stop Control Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`mstpcrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstpcrbSpec;
impl crate::RegisterSpec for MstpcrbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstpcrb::R`](R) reader structure"]
impl crate::Readable for MstpcrbSpec {}
#[doc = "`write(|w| ..)` method takes [`mstpcrb::W`](W) writer structure"]
impl crate::Writable for MstpcrbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSTPCRB to value 0xffff_ffff"]
impl crate::Resettable for MstpcrbSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
