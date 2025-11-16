#[doc = "Register `MSTPCRC` reader"]
pub type R = crate::R<MstpcrcSpec>;
#[doc = "Register `MSTPCRC` writer"]
pub type W = crate::W<MstpcrcSpec>;
#[doc = "Clock Frequency Accuracy Measurement Circuit Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc0 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpc0> for bool {
    #[inline(always)]
    fn from(variant: Mstpc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPC0` reader - Clock Frequency Accuracy Measurement Circuit Module Stop"]
pub type Mstpc0R = crate::BitReader<Mstpc0>;
impl Mstpc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc0 {
        match self.bits {
            false => Mstpc0::_0,
            true => Mstpc0::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc0::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc0::_1
    }
}
#[doc = "Field `MSTPC0` writer - Clock Frequency Accuracy Measurement Circuit Module Stop"]
pub type Mstpc0W<'a, REG> = crate::BitWriter<'a, REG, Mstpc0>;
impl<'a, REG> Mstpc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc0::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc0::_1)
    }
}
#[doc = "Cyclic Redundancy Check Calculator Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc1 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpc1> for bool {
    #[inline(always)]
    fn from(variant: Mstpc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPC1` reader - Cyclic Redundancy Check Calculator Module Stop"]
pub type Mstpc1R = crate::BitReader<Mstpc1>;
impl Mstpc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc1 {
        match self.bits {
            false => Mstpc1::_0,
            true => Mstpc1::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc1::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc1::_1
    }
}
#[doc = "Field `MSTPC1` writer - Cyclic Redundancy Check Calculator Module Stop"]
pub type Mstpc1W<'a, REG> = crate::BitWriter<'a, REG, Mstpc1>;
impl<'a, REG> Mstpc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc1::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc1::_1)
    }
}
#[doc = "Capacitive Touch Sensing Unit Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc3 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpc3> for bool {
    #[inline(always)]
    fn from(variant: Mstpc3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPC3` reader - Capacitive Touch Sensing Unit Module Stop"]
pub type Mstpc3R = crate::BitReader<Mstpc3>;
impl Mstpc3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc3 {
        match self.bits {
            false => Mstpc3::_0,
            true => Mstpc3::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc3::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc3::_1
    }
}
#[doc = "Field `MSTPC3` writer - Capacitive Touch Sensing Unit Module Stop"]
pub type Mstpc3W<'a, REG> = crate::BitWriter<'a, REG, Mstpc3>;
impl<'a, REG> Mstpc3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc3::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc3::_1)
    }
}
#[doc = "Segment LCD Controller Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc4 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpc4> for bool {
    #[inline(always)]
    fn from(variant: Mstpc4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPC4` reader - Segment LCD Controller Module Stop"]
pub type Mstpc4R = crate::BitReader<Mstpc4>;
impl Mstpc4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc4 {
        match self.bits {
            false => Mstpc4::_0,
            true => Mstpc4::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc4::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc4::_1
    }
}
#[doc = "Field `MSTPC4` writer - Segment LCD Controller Module Stop"]
pub type Mstpc4W<'a, REG> = crate::BitWriter<'a, REG, Mstpc4>;
impl<'a, REG> Mstpc4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc4::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc4::_1)
    }
}
#[doc = "Synchronous Serial Interface 0 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc8 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpc8> for bool {
    #[inline(always)]
    fn from(variant: Mstpc8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPC8` reader - Synchronous Serial Interface 0 Module Stop"]
pub type Mstpc8R = crate::BitReader<Mstpc8>;
impl Mstpc8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc8 {
        match self.bits {
            false => Mstpc8::_0,
            true => Mstpc8::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc8::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc8::_1
    }
}
#[doc = "Field `MSTPC8` writer - Synchronous Serial Interface 0 Module Stop"]
pub type Mstpc8W<'a, REG> = crate::BitWriter<'a, REG, Mstpc8>;
impl<'a, REG> Mstpc8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc8::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc8::_1)
    }
}
#[doc = "Data Operation Circuit Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc13 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpc13> for bool {
    #[inline(always)]
    fn from(variant: Mstpc13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPC13` reader - Data Operation Circuit Module Stop"]
pub type Mstpc13R = crate::BitReader<Mstpc13>;
impl Mstpc13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc13 {
        match self.bits {
            false => Mstpc13::_0,
            true => Mstpc13::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc13::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc13::_1
    }
}
#[doc = "Field `MSTPC13` writer - Data Operation Circuit Module Stop"]
pub type Mstpc13W<'a, REG> = crate::BitWriter<'a, REG, Mstpc13>;
impl<'a, REG> Mstpc13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc13::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc13::_1)
    }
}
#[doc = "Event Link Controller Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc14 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpc14> for bool {
    #[inline(always)]
    fn from(variant: Mstpc14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPC14` reader - Event Link Controller Module Stop"]
pub type Mstpc14R = crate::BitReader<Mstpc14>;
impl Mstpc14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc14 {
        match self.bits {
            false => Mstpc14::_0,
            true => Mstpc14::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc14::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc14::_1
    }
}
#[doc = "Field `MSTPC14` writer - Event Link Controller Module Stop"]
pub type Mstpc14W<'a, REG> = crate::BitWriter<'a, REG, Mstpc14>;
impl<'a, REG> Mstpc14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc14::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc14::_1)
    }
}
#[doc = "SCE5 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc31 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpc31> for bool {
    #[inline(always)]
    fn from(variant: Mstpc31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPC31` reader - SCE5 Module Stop"]
pub type Mstpc31R = crate::BitReader<Mstpc31>;
impl Mstpc31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc31 {
        match self.bits {
            false => Mstpc31::_0,
            true => Mstpc31::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc31::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc31::_1
    }
}
#[doc = "Field `MSTPC31` writer - SCE5 Module Stop"]
pub type Mstpc31W<'a, REG> = crate::BitWriter<'a, REG, Mstpc31>;
impl<'a, REG> Mstpc31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc31::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc31::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Frequency Accuracy Measurement Circuit Module Stop"]
    #[inline(always)]
    pub fn mstpc0(&self) -> Mstpc0R {
        Mstpc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cyclic Redundancy Check Calculator Module Stop"]
    #[inline(always)]
    pub fn mstpc1(&self) -> Mstpc1R {
        Mstpc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Capacitive Touch Sensing Unit Module Stop"]
    #[inline(always)]
    pub fn mstpc3(&self) -> Mstpc3R {
        Mstpc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Segment LCD Controller Module Stop"]
    #[inline(always)]
    pub fn mstpc4(&self) -> Mstpc4R {
        Mstpc4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Synchronous Serial Interface 0 Module Stop"]
    #[inline(always)]
    pub fn mstpc8(&self) -> Mstpc8R {
        Mstpc8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13 - Data Operation Circuit Module Stop"]
    #[inline(always)]
    pub fn mstpc13(&self) -> Mstpc13R {
        Mstpc13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Event Link Controller Module Stop"]
    #[inline(always)]
    pub fn mstpc14(&self) -> Mstpc14R {
        Mstpc14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 31 - SCE5 Module Stop"]
    #[inline(always)]
    pub fn mstpc31(&self) -> Mstpc31R {
        Mstpc31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTPCRC")
            .field("mstpc31", &self.mstpc31())
            .field("mstpc14", &self.mstpc14())
            .field("mstpc13", &self.mstpc13())
            .field("mstpc8", &self.mstpc8())
            .field("mstpc4", &self.mstpc4())
            .field("mstpc3", &self.mstpc3())
            .field("mstpc1", &self.mstpc1())
            .field("mstpc0", &self.mstpc0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Clock Frequency Accuracy Measurement Circuit Module Stop"]
    #[inline(always)]
    pub fn mstpc0(&mut self) -> Mstpc0W<MstpcrcSpec> {
        Mstpc0W::new(self, 0)
    }
    #[doc = "Bit 1 - Cyclic Redundancy Check Calculator Module Stop"]
    #[inline(always)]
    pub fn mstpc1(&mut self) -> Mstpc1W<MstpcrcSpec> {
        Mstpc1W::new(self, 1)
    }
    #[doc = "Bit 3 - Capacitive Touch Sensing Unit Module Stop"]
    #[inline(always)]
    pub fn mstpc3(&mut self) -> Mstpc3W<MstpcrcSpec> {
        Mstpc3W::new(self, 3)
    }
    #[doc = "Bit 4 - Segment LCD Controller Module Stop"]
    #[inline(always)]
    pub fn mstpc4(&mut self) -> Mstpc4W<MstpcrcSpec> {
        Mstpc4W::new(self, 4)
    }
    #[doc = "Bit 8 - Synchronous Serial Interface 0 Module Stop"]
    #[inline(always)]
    pub fn mstpc8(&mut self) -> Mstpc8W<MstpcrcSpec> {
        Mstpc8W::new(self, 8)
    }
    #[doc = "Bit 13 - Data Operation Circuit Module Stop"]
    #[inline(always)]
    pub fn mstpc13(&mut self) -> Mstpc13W<MstpcrcSpec> {
        Mstpc13W::new(self, 13)
    }
    #[doc = "Bit 14 - Event Link Controller Module Stop"]
    #[inline(always)]
    pub fn mstpc14(&mut self) -> Mstpc14W<MstpcrcSpec> {
        Mstpc14W::new(self, 14)
    }
    #[doc = "Bit 31 - SCE5 Module Stop"]
    #[inline(always)]
    pub fn mstpc31(&mut self) -> Mstpc31W<MstpcrcSpec> {
        Mstpc31W::new(self, 31)
    }
}
#[doc = "Module Stop Control Register C\n\nYou can [`read`](crate::Reg::read) this register and get [`mstpcrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstpcrcSpec;
impl crate::RegisterSpec for MstpcrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstpcrc::R`](R) reader structure"]
impl crate::Readable for MstpcrcSpec {}
#[doc = "`write(|w| ..)` method takes [`mstpcrc::W`](W) writer structure"]
impl crate::Writable for MstpcrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSTPCRC to value 0xffff_ffff"]
impl crate::Resettable for MstpcrcSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
