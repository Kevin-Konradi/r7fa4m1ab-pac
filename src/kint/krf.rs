#[doc = "Register `KRF` reader"]
pub type R = crate::R<KrfSpec>;
#[doc = "Register `KRF` writer"]
pub type W = crate::W<KrfSpec>;
#[doc = "Key interrupt flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Krf0 {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected."]
    _1 = 1,
}
impl From<Krf0> for bool {
    #[inline(always)]
    fn from(variant: Krf0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KRF0` reader - Key interrupt flag 0\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Krf0R = crate::BitReader<Krf0>;
impl Krf0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Krf0 {
        match self.bits {
            false => Krf0::_0,
            true => Krf0::_1,
        }
    }
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Krf0::_0
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Krf0::_1
    }
}
#[doc = "Field `KRF0` writer - Key interrupt flag 0"]
pub type Krf0W<'a, REG> = crate::BitWriter0C<'a, REG, Krf0>;
impl<'a, REG> Krf0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Krf0::_0)
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Krf0::_1)
    }
}
#[doc = "Key interrupt flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Krf1 {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected."]
    _1 = 1,
}
impl From<Krf1> for bool {
    #[inline(always)]
    fn from(variant: Krf1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KRF1` reader - Key interrupt flag 1\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Krf1R = crate::BitReader<Krf1>;
impl Krf1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Krf1 {
        match self.bits {
            false => Krf1::_0,
            true => Krf1::_1,
        }
    }
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Krf1::_0
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Krf1::_1
    }
}
#[doc = "Field `KRF1` writer - Key interrupt flag 1"]
pub type Krf1W<'a, REG> = crate::BitWriter0C<'a, REG, Krf1>;
impl<'a, REG> Krf1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Krf1::_0)
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Krf1::_1)
    }
}
#[doc = "Key interrupt flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Krf2 {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected."]
    _1 = 1,
}
impl From<Krf2> for bool {
    #[inline(always)]
    fn from(variant: Krf2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KRF2` reader - Key interrupt flag 2\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Krf2R = crate::BitReader<Krf2>;
impl Krf2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Krf2 {
        match self.bits {
            false => Krf2::_0,
            true => Krf2::_1,
        }
    }
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Krf2::_0
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Krf2::_1
    }
}
#[doc = "Field `KRF2` writer - Key interrupt flag 2"]
pub type Krf2W<'a, REG> = crate::BitWriter0C<'a, REG, Krf2>;
impl<'a, REG> Krf2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Krf2::_0)
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Krf2::_1)
    }
}
#[doc = "Key interrupt flag 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Krf3 {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected."]
    _1 = 1,
}
impl From<Krf3> for bool {
    #[inline(always)]
    fn from(variant: Krf3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KRF3` reader - Key interrupt flag 3\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Krf3R = crate::BitReader<Krf3>;
impl Krf3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Krf3 {
        match self.bits {
            false => Krf3::_0,
            true => Krf3::_1,
        }
    }
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Krf3::_0
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Krf3::_1
    }
}
#[doc = "Field `KRF3` writer - Key interrupt flag 3"]
pub type Krf3W<'a, REG> = crate::BitWriter0C<'a, REG, Krf3>;
impl<'a, REG> Krf3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Krf3::_0)
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Krf3::_1)
    }
}
#[doc = "Key interrupt flag 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Krf4 {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected."]
    _1 = 1,
}
impl From<Krf4> for bool {
    #[inline(always)]
    fn from(variant: Krf4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KRF4` reader - Key interrupt flag 4\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Krf4R = crate::BitReader<Krf4>;
impl Krf4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Krf4 {
        match self.bits {
            false => Krf4::_0,
            true => Krf4::_1,
        }
    }
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Krf4::_0
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Krf4::_1
    }
}
#[doc = "Field `KRF4` writer - Key interrupt flag 4"]
pub type Krf4W<'a, REG> = crate::BitWriter0C<'a, REG, Krf4>;
impl<'a, REG> Krf4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Krf4::_0)
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Krf4::_1)
    }
}
#[doc = "Key interrupt flag 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Krf5 {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected."]
    _1 = 1,
}
impl From<Krf5> for bool {
    #[inline(always)]
    fn from(variant: Krf5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KRF5` reader - Key interrupt flag 5\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Krf5R = crate::BitReader<Krf5>;
impl Krf5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Krf5 {
        match self.bits {
            false => Krf5::_0,
            true => Krf5::_1,
        }
    }
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Krf5::_0
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Krf5::_1
    }
}
#[doc = "Field `KRF5` writer - Key interrupt flag 5"]
pub type Krf5W<'a, REG> = crate::BitWriter0C<'a, REG, Krf5>;
impl<'a, REG> Krf5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Krf5::_0)
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Krf5::_1)
    }
}
#[doc = "Key interrupt flag 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Krf6 {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected."]
    _1 = 1,
}
impl From<Krf6> for bool {
    #[inline(always)]
    fn from(variant: Krf6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KRF6` reader - Key interrupt flag 6\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Krf6R = crate::BitReader<Krf6>;
impl Krf6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Krf6 {
        match self.bits {
            false => Krf6::_0,
            true => Krf6::_1,
        }
    }
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Krf6::_0
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Krf6::_1
    }
}
#[doc = "Field `KRF6` writer - Key interrupt flag 6"]
pub type Krf6W<'a, REG> = crate::BitWriter0C<'a, REG, Krf6>;
impl<'a, REG> Krf6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Krf6::_0)
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Krf6::_1)
    }
}
#[doc = "Key interrupt flag 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Krf7 {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected."]
    _1 = 1,
}
impl From<Krf7> for bool {
    #[inline(always)]
    fn from(variant: Krf7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KRF7` reader - Key interrupt flag 7\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Krf7R = crate::BitReader<Krf7>;
impl Krf7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Krf7 {
        match self.bits {
            false => Krf7::_0,
            true => Krf7::_1,
        }
    }
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Krf7::_0
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Krf7::_1
    }
}
#[doc = "Field `KRF7` writer - Key interrupt flag 7"]
pub type Krf7W<'a, REG> = crate::BitWriter0C<'a, REG, Krf7>;
impl<'a, REG> Krf7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Krf7::_0)
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Krf7::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Key interrupt flag 0"]
    #[inline(always)]
    pub fn krf0(&self) -> Krf0R {
        Krf0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Key interrupt flag 1"]
    #[inline(always)]
    pub fn krf1(&self) -> Krf1R {
        Krf1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Key interrupt flag 2"]
    #[inline(always)]
    pub fn krf2(&self) -> Krf2R {
        Krf2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Key interrupt flag 3"]
    #[inline(always)]
    pub fn krf3(&self) -> Krf3R {
        Krf3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Key interrupt flag 4"]
    #[inline(always)]
    pub fn krf4(&self) -> Krf4R {
        Krf4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Key interrupt flag 5"]
    #[inline(always)]
    pub fn krf5(&self) -> Krf5R {
        Krf5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Key interrupt flag 6"]
    #[inline(always)]
    pub fn krf6(&self) -> Krf6R {
        Krf6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Key interrupt flag 7"]
    #[inline(always)]
    pub fn krf7(&self) -> Krf7R {
        Krf7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for crate::generic::Reg<KrfSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Key interrupt flag 0"]
    #[inline(always)]
    pub fn krf0(&mut self) -> Krf0W<KrfSpec> {
        Krf0W::new(self, 0)
    }
    #[doc = "Bit 1 - Key interrupt flag 1"]
    #[inline(always)]
    pub fn krf1(&mut self) -> Krf1W<KrfSpec> {
        Krf1W::new(self, 1)
    }
    #[doc = "Bit 2 - Key interrupt flag 2"]
    #[inline(always)]
    pub fn krf2(&mut self) -> Krf2W<KrfSpec> {
        Krf2W::new(self, 2)
    }
    #[doc = "Bit 3 - Key interrupt flag 3"]
    #[inline(always)]
    pub fn krf3(&mut self) -> Krf3W<KrfSpec> {
        Krf3W::new(self, 3)
    }
    #[doc = "Bit 4 - Key interrupt flag 4"]
    #[inline(always)]
    pub fn krf4(&mut self) -> Krf4W<KrfSpec> {
        Krf4W::new(self, 4)
    }
    #[doc = "Bit 5 - Key interrupt flag 5"]
    #[inline(always)]
    pub fn krf5(&mut self) -> Krf5W<KrfSpec> {
        Krf5W::new(self, 5)
    }
    #[doc = "Bit 6 - Key interrupt flag 6"]
    #[inline(always)]
    pub fn krf6(&mut self) -> Krf6W<KrfSpec> {
        Krf6W::new(self, 6)
    }
    #[doc = "Bit 7 - Key interrupt flag 7"]
    #[inline(always)]
    pub fn krf7(&mut self) -> Krf7W<KrfSpec> {
        Krf7W::new(self, 7)
    }
}
#[doc = "KEY Return Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`krf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`krf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>"]
pub struct KrfSpec;
impl crate::RegisterSpec for KrfSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`krf::R`](R) reader structure"]
impl crate::Readable for KrfSpec {}
#[doc = "`write(|w| ..)` method takes [`krf::W`](W) writer structure"]
impl crate::Writable for KrfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0xff;
}
#[doc = "`reset()` method sets KRF to value 0"]
impl crate::Resettable for KrfSpec {}
