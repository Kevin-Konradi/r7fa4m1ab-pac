#[doc = "Register `ICSR1` reader"]
pub type R = crate::R<Icsr1Spec>;
#[doc = "Register `ICSR1` writer"]
pub type W = crate::W<Icsr1Spec>;
#[doc = "Slave Address 0 Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aas0 {
    #[doc = "0: Slave address 0 is not detected."]
    _0 = 0,
    #[doc = "1: Slave address 0 is detected."]
    _1 = 1,
}
impl From<Aas0> for bool {
    #[inline(always)]
    fn from(variant: Aas0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AAS0` reader - Slave Address 0 Detection Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Aas0R = crate::BitReader<Aas0>;
impl Aas0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aas0 {
        match self.bits {
            false => Aas0::_0,
            true => Aas0::_1,
        }
    }
    #[doc = "Slave address 0 is not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aas0::_0
    }
    #[doc = "Slave address 0 is detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aas0::_1
    }
}
#[doc = "Field `AAS0` writer - Slave Address 0 Detection Flag"]
pub type Aas0W<'a, REG> = crate::BitWriter0C<'a, REG, Aas0>;
impl<'a, REG> Aas0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave address 0 is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aas0::_0)
    }
    #[doc = "Slave address 0 is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aas0::_1)
    }
}
#[doc = "Slave Address 1 Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aas1 {
    #[doc = "0: Slave address 1 is not detected."]
    _0 = 0,
    #[doc = "1: Slave address 1 is detected."]
    _1 = 1,
}
impl From<Aas1> for bool {
    #[inline(always)]
    fn from(variant: Aas1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AAS1` reader - Slave Address 1 Detection Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Aas1R = crate::BitReader<Aas1>;
impl Aas1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aas1 {
        match self.bits {
            false => Aas1::_0,
            true => Aas1::_1,
        }
    }
    #[doc = "Slave address 1 is not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aas1::_0
    }
    #[doc = "Slave address 1 is detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aas1::_1
    }
}
#[doc = "Field `AAS1` writer - Slave Address 1 Detection Flag"]
pub type Aas1W<'a, REG> = crate::BitWriter0C<'a, REG, Aas1>;
impl<'a, REG> Aas1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave address 1 is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aas1::_0)
    }
    #[doc = "Slave address 1 is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aas1::_1)
    }
}
#[doc = "Slave Address 2 Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aas2 {
    #[doc = "0: Slave address 2 is not detected."]
    _0 = 0,
    #[doc = "1: Slave address 2 is detected"]
    _1 = 1,
}
impl From<Aas2> for bool {
    #[inline(always)]
    fn from(variant: Aas2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AAS2` reader - Slave Address 2 Detection Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Aas2R = crate::BitReader<Aas2>;
impl Aas2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aas2 {
        match self.bits {
            false => Aas2::_0,
            true => Aas2::_1,
        }
    }
    #[doc = "Slave address 2 is not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aas2::_0
    }
    #[doc = "Slave address 2 is detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aas2::_1
    }
}
#[doc = "Field `AAS2` writer - Slave Address 2 Detection Flag"]
pub type Aas2W<'a, REG> = crate::BitWriter0C<'a, REG, Aas2>;
impl<'a, REG> Aas2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave address 2 is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aas2::_0)
    }
    #[doc = "Slave address 2 is detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aas2::_1)
    }
}
#[doc = "General Call Address Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gca {
    #[doc = "0: General call address is not detected."]
    _0 = 0,
    #[doc = "1: General call address is detected."]
    _1 = 1,
}
impl From<Gca> for bool {
    #[inline(always)]
    fn from(variant: Gca) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCA` reader - General Call Address Detection Flag"]
pub type GcaR = crate::BitReader<Gca>;
impl GcaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gca {
        match self.bits {
            false => Gca::_0,
            true => Gca::_1,
        }
    }
    #[doc = "General call address is not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gca::_0
    }
    #[doc = "General call address is detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gca::_1
    }
}
#[doc = "Field `GCA` writer - General Call Address Detection Flag"]
pub type GcaW<'a, REG> = crate::BitWriter<'a, REG, Gca>;
impl<'a, REG> GcaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "General call address is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gca::_0)
    }
    #[doc = "General call address is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gca::_1)
    }
}
#[doc = "Device-ID Address Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Did {
    #[doc = "0: Device-ID command is not detected."]
    _0 = 0,
    #[doc = "1: Device-ID command is detected."]
    _1 = 1,
}
impl From<Did> for bool {
    #[inline(always)]
    fn from(variant: Did) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DID` reader - Device-ID Address Detection Flag"]
pub type DidR = crate::BitReader<Did>;
impl DidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Did {
        match self.bits {
            false => Did::_0,
            true => Did::_1,
        }
    }
    #[doc = "Device-ID command is not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Did::_0
    }
    #[doc = "Device-ID command is detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Did::_1
    }
}
#[doc = "Field `DID` writer - Device-ID Address Detection Flag"]
pub type DidW<'a, REG> = crate::BitWriter<'a, REG, Did>;
impl<'a, REG> DidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Device-ID command is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Did::_0)
    }
    #[doc = "Device-ID command is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Did::_1)
    }
}
#[doc = "Host Address Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hoa {
    #[doc = "0: Host address is not detected."]
    _0 = 0,
    #[doc = "1: Host address is detected."]
    _1 = 1,
}
impl From<Hoa> for bool {
    #[inline(always)]
    fn from(variant: Hoa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOA` reader - Host Address Detection Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type HoaR = crate::BitReader<Hoa>;
impl HoaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hoa {
        match self.bits {
            false => Hoa::_0,
            true => Hoa::_1,
        }
    }
    #[doc = "Host address is not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hoa::_0
    }
    #[doc = "Host address is detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hoa::_1
    }
}
#[doc = "Field `HOA` writer - Host Address Detection Flag"]
pub type HoaW<'a, REG> = crate::BitWriter0C<'a, REG, Hoa>;
impl<'a, REG> HoaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Host address is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hoa::_0)
    }
    #[doc = "Host address is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hoa::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Slave Address 0 Detection Flag"]
    #[inline(always)]
    pub fn aas0(&self) -> Aas0R {
        Aas0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave Address 1 Detection Flag"]
    #[inline(always)]
    pub fn aas1(&self) -> Aas1R {
        Aas1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave Address 2 Detection Flag"]
    #[inline(always)]
    pub fn aas2(&self) -> Aas2R {
        Aas2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - General Call Address Detection Flag"]
    #[inline(always)]
    pub fn gca(&self) -> GcaR {
        GcaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Device-ID Address Detection Flag"]
    #[inline(always)]
    pub fn did(&self) -> DidR {
        DidR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Host Address Detection Flag"]
    #[inline(always)]
    pub fn hoa(&self) -> HoaR {
        HoaR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICSR1")
            .field("did", &self.did())
            .field("gca", &self.gca())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Slave Address 0 Detection Flag"]
    #[inline(always)]
    pub fn aas0(&mut self) -> Aas0W<Icsr1Spec> {
        Aas0W::new(self, 0)
    }
    #[doc = "Bit 1 - Slave Address 1 Detection Flag"]
    #[inline(always)]
    pub fn aas1(&mut self) -> Aas1W<Icsr1Spec> {
        Aas1W::new(self, 1)
    }
    #[doc = "Bit 2 - Slave Address 2 Detection Flag"]
    #[inline(always)]
    pub fn aas2(&mut self) -> Aas2W<Icsr1Spec> {
        Aas2W::new(self, 2)
    }
    #[doc = "Bit 3 - General Call Address Detection Flag"]
    #[inline(always)]
    pub fn gca(&mut self) -> GcaW<Icsr1Spec> {
        GcaW::new(self, 3)
    }
    #[doc = "Bit 5 - Device-ID Address Detection Flag"]
    #[inline(always)]
    pub fn did(&mut self) -> DidW<Icsr1Spec> {
        DidW::new(self, 5)
    }
    #[doc = "Bit 7 - Host Address Detection Flag"]
    #[inline(always)]
    pub fn hoa(&mut self) -> HoaW<Icsr1Spec> {
        HoaW::new(self, 7)
    }
}
#[doc = "I2C Bus Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`icsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icsr1Spec;
impl crate::RegisterSpec for Icsr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icsr1::R`](R) reader structure"]
impl crate::Readable for Icsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`icsr1::W`](W) writer structure"]
impl crate::Writable for Icsr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x87;
}
#[doc = "`reset()` method sets ICSR1 to value 0"]
impl crate::Resettable for Icsr1Spec {}
