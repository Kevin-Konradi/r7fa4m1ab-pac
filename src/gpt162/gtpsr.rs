#[doc = "Register `GTPSR` reader"]
pub type R = crate::R<GtpsrSpec>;
#[doc = "Register `GTPSR` writer"]
pub type W = crate::W<GtpsrSpec>;
#[doc = "GTETRGA Pin Rising Input Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psgtrgar {
    #[doc = "0: Counter stop is disable at the rising edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the rising edge of GTETRGA input"]
    _1 = 1,
}
impl From<Psgtrgar> for bool {
    #[inline(always)]
    fn from(variant: Psgtrgar) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Stop Enable"]
pub type PsgtrgarR = crate::BitReader<Psgtrgar>;
impl PsgtrgarR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psgtrgar {
        match self.bits {
            false => Psgtrgar::_0,
            true => Psgtrgar::_1,
        }
    }
    #[doc = "Counter stop is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psgtrgar::_0
    }
    #[doc = "Counter stop is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psgtrgar::_1
    }
}
#[doc = "Field `PSGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Stop Enable"]
pub type PsgtrgarW<'a, REG> = crate::BitWriter<'a, REG, Psgtrgar>;
impl<'a, REG> PsgtrgarW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgar::_0)
    }
    #[doc = "Counter stop is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgar::_1)
    }
}
#[doc = "GTETRGA Pin Falling Input Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psgtrgaf {
    #[doc = "0: Counter stop is disable at the falling edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the falling edge of GTETRGA input"]
    _1 = 1,
}
impl From<Psgtrgaf> for bool {
    #[inline(always)]
    fn from(variant: Psgtrgaf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Stop Enable"]
pub type PsgtrgafR = crate::BitReader<Psgtrgaf>;
impl PsgtrgafR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psgtrgaf {
        match self.bits {
            false => Psgtrgaf::_0,
            true => Psgtrgaf::_1,
        }
    }
    #[doc = "Counter stop is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psgtrgaf::_0
    }
    #[doc = "Counter stop is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psgtrgaf::_1
    }
}
#[doc = "Field `PSGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Stop Enable"]
pub type PsgtrgafW<'a, REG> = crate::BitWriter<'a, REG, Psgtrgaf>;
impl<'a, REG> PsgtrgafW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgaf::_0)
    }
    #[doc = "Counter stop is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgaf::_1)
    }
}
#[doc = "GTETRGB Pin Rising Input Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psgtrgbr {
    #[doc = "0: Counter stop is disable at the rising edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the rising edge of GTETRGB input"]
    _1 = 1,
}
impl From<Psgtrgbr> for bool {
    #[inline(always)]
    fn from(variant: Psgtrgbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Stop Enable"]
pub type PsgtrgbrR = crate::BitReader<Psgtrgbr>;
impl PsgtrgbrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psgtrgbr {
        match self.bits {
            false => Psgtrgbr::_0,
            true => Psgtrgbr::_1,
        }
    }
    #[doc = "Counter stop is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psgtrgbr::_0
    }
    #[doc = "Counter stop is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psgtrgbr::_1
    }
}
#[doc = "Field `PSGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Stop Enable"]
pub type PsgtrgbrW<'a, REG> = crate::BitWriter<'a, REG, Psgtrgbr>;
impl<'a, REG> PsgtrgbrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgbr::_0)
    }
    #[doc = "Counter stop is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgbr::_1)
    }
}
#[doc = "GTETRGB Pin Falling Input Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psgtrgbf {
    #[doc = "0: Counter stop is disable at the falling edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the falling edge of GTETRGB input"]
    _1 = 1,
}
impl From<Psgtrgbf> for bool {
    #[inline(always)]
    fn from(variant: Psgtrgbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Stop Enable"]
pub type PsgtrgbfR = crate::BitReader<Psgtrgbf>;
impl PsgtrgbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psgtrgbf {
        match self.bits {
            false => Psgtrgbf::_0,
            true => Psgtrgbf::_1,
        }
    }
    #[doc = "Counter stop is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psgtrgbf::_0
    }
    #[doc = "Counter stop is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psgtrgbf::_1
    }
}
#[doc = "Field `PSGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Stop Enable"]
pub type PsgtrgbfW<'a, REG> = crate::BitWriter<'a, REG, Psgtrgbf>;
impl<'a, REG> PsgtrgbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgbf::_0)
    }
    #[doc = "Counter stop is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgbf::_1)
    }
}
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pscarbl {
    #[doc = "0: Counter stop is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<Pscarbl> for bool {
    #[inline(always)]
    fn from(variant: Pscarbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSCARBL` reader - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable"]
pub type PscarblR = crate::BitReader<Pscarbl>;
impl PscarblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pscarbl {
        match self.bits {
            false => Pscarbl::_0,
            true => Pscarbl::_1,
        }
    }
    #[doc = "Counter stop is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pscarbl::_0
    }
    #[doc = "Counter stop is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pscarbl::_1
    }
}
#[doc = "Field `PSCARBL` writer - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable"]
pub type PscarblW<'a, REG> = crate::BitWriter<'a, REG, Pscarbl>;
impl<'a, REG> PscarblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pscarbl::_0)
    }
    #[doc = "Counter stop is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pscarbl::_1)
    }
}
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pscarbh {
    #[doc = "0: Counter stop is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<Pscarbh> for bool {
    #[inline(always)]
    fn from(variant: Pscarbh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSCARBH` reader - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable"]
pub type PscarbhR = crate::BitReader<Pscarbh>;
impl PscarbhR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pscarbh {
        match self.bits {
            false => Pscarbh::_0,
            true => Pscarbh::_1,
        }
    }
    #[doc = "Counter stop is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pscarbh::_0
    }
    #[doc = "Counter stop is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pscarbh::_1
    }
}
#[doc = "Field `PSCARBH` writer - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable"]
pub type PscarbhW<'a, REG> = crate::BitWriter<'a, REG, Pscarbh>;
impl<'a, REG> PscarbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pscarbh::_0)
    }
    #[doc = "Counter stop is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pscarbh::_1)
    }
}
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pscafbl {
    #[doc = "0: Counter stop is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<Pscafbl> for bool {
    #[inline(always)]
    fn from(variant: Pscafbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSCAFBL` reader - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable"]
pub type PscafblR = crate::BitReader<Pscafbl>;
impl PscafblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pscafbl {
        match self.bits {
            false => Pscafbl::_0,
            true => Pscafbl::_1,
        }
    }
    #[doc = "Counter stop is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pscafbl::_0
    }
    #[doc = "Counter stop is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pscafbl::_1
    }
}
#[doc = "Field `PSCAFBL` writer - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable"]
pub type PscafblW<'a, REG> = crate::BitWriter<'a, REG, Pscafbl>;
impl<'a, REG> PscafblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pscafbl::_0)
    }
    #[doc = "Counter stop is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pscafbl::_1)
    }
}
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pscafbh {
    #[doc = "0: Counter stop is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<Pscafbh> for bool {
    #[inline(always)]
    fn from(variant: Pscafbh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSCAFBH` reader - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable"]
pub type PscafbhR = crate::BitReader<Pscafbh>;
impl PscafbhR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pscafbh {
        match self.bits {
            false => Pscafbh::_0,
            true => Pscafbh::_1,
        }
    }
    #[doc = "Counter stop is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pscafbh::_0
    }
    #[doc = "Counter stop is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pscafbh::_1
    }
}
#[doc = "Field `PSCAFBH` writer - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable"]
pub type PscafbhW<'a, REG> = crate::BitWriter<'a, REG, Pscafbh>;
impl<'a, REG> PscafbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pscafbh::_0)
    }
    #[doc = "Counter stop is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pscafbh::_1)
    }
}
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pscbral {
    #[doc = "0: Counter stop is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<Pscbral> for bool {
    #[inline(always)]
    fn from(variant: Pscbral) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSCBRAL` reader - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable"]
pub type PscbralR = crate::BitReader<Pscbral>;
impl PscbralR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pscbral {
        match self.bits {
            false => Pscbral::_0,
            true => Pscbral::_1,
        }
    }
    #[doc = "Counter stop is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pscbral::_0
    }
    #[doc = "Counter stop is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pscbral::_1
    }
}
#[doc = "Field `PSCBRAL` writer - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable"]
pub type PscbralW<'a, REG> = crate::BitWriter<'a, REG, Pscbral>;
impl<'a, REG> PscbralW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pscbral::_0)
    }
    #[doc = "Counter stop is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pscbral::_1)
    }
}
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pscbrah {
    #[doc = "0: Counter stop is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<Pscbrah> for bool {
    #[inline(always)]
    fn from(variant: Pscbrah) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSCBRAH` reader - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable"]
pub type PscbrahR = crate::BitReader<Pscbrah>;
impl PscbrahR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pscbrah {
        match self.bits {
            false => Pscbrah::_0,
            true => Pscbrah::_1,
        }
    }
    #[doc = "Counter stop is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pscbrah::_0
    }
    #[doc = "Counter stop is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pscbrah::_1
    }
}
#[doc = "Field `PSCBRAH` writer - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable"]
pub type PscbrahW<'a, REG> = crate::BitWriter<'a, REG, Pscbrah>;
impl<'a, REG> PscbrahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pscbrah::_0)
    }
    #[doc = "Counter stop is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pscbrah::_1)
    }
}
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pscbfal {
    #[doc = "0: Counter stop is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<Pscbfal> for bool {
    #[inline(always)]
    fn from(variant: Pscbfal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSCBFAL` reader - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable"]
pub type PscbfalR = crate::BitReader<Pscbfal>;
impl PscbfalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pscbfal {
        match self.bits {
            false => Pscbfal::_0,
            true => Pscbfal::_1,
        }
    }
    #[doc = "Counter stop is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pscbfal::_0
    }
    #[doc = "Counter stop is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pscbfal::_1
    }
}
#[doc = "Field `PSCBFAL` writer - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable"]
pub type PscbfalW<'a, REG> = crate::BitWriter<'a, REG, Pscbfal>;
impl<'a, REG> PscbfalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pscbfal::_0)
    }
    #[doc = "Counter stop is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pscbfal::_1)
    }
}
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pscbfah {
    #[doc = "0: Counter stop is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<Pscbfah> for bool {
    #[inline(always)]
    fn from(variant: Pscbfah) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSCBFAH` reader - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable"]
pub type PscbfahR = crate::BitReader<Pscbfah>;
impl PscbfahR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pscbfah {
        match self.bits {
            false => Pscbfah::_0,
            true => Pscbfah::_1,
        }
    }
    #[doc = "Counter stop is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pscbfah::_0
    }
    #[doc = "Counter stop is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pscbfah::_1
    }
}
#[doc = "Field `PSCBFAH` writer - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable"]
pub type PscbfahW<'a, REG> = crate::BitWriter<'a, REG, Pscbfah>;
impl<'a, REG> PscbfahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pscbfah::_0)
    }
    #[doc = "Counter stop is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pscbfah::_1)
    }
}
#[doc = "ELC_GPTA Event Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pselca {
    #[doc = "0: Counter stop is disable at the ELC_GPTA input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the ELC_GPTA input"]
    _1 = 1,
}
impl From<Pselca> for bool {
    #[inline(always)]
    fn from(variant: Pselca) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSELCA` reader - ELC_GPTA Event Source Counter Stop Enable"]
pub type PselcaR = crate::BitReader<Pselca>;
impl PselcaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pselca {
        match self.bits {
            false => Pselca::_0,
            true => Pselca::_1,
        }
    }
    #[doc = "Counter stop is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pselca::_0
    }
    #[doc = "Counter stop is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pselca::_1
    }
}
#[doc = "Field `PSELCA` writer - ELC_GPTA Event Source Counter Stop Enable"]
pub type PselcaW<'a, REG> = crate::BitWriter<'a, REG, Pselca>;
impl<'a, REG> PselcaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pselca::_0)
    }
    #[doc = "Counter stop is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pselca::_1)
    }
}
#[doc = "ELC_GPTB Event Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pselcb {
    #[doc = "0: Counter stop is disable at the ELC_GPTB input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the ELC_GPTB input"]
    _1 = 1,
}
impl From<Pselcb> for bool {
    #[inline(always)]
    fn from(variant: Pselcb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSELCB` reader - ELC_GPTB Event Source Counter Stop Enable"]
pub type PselcbR = crate::BitReader<Pselcb>;
impl PselcbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pselcb {
        match self.bits {
            false => Pselcb::_0,
            true => Pselcb::_1,
        }
    }
    #[doc = "Counter stop is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pselcb::_0
    }
    #[doc = "Counter stop is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pselcb::_1
    }
}
#[doc = "Field `PSELCB` writer - ELC_GPTB Event Source Counter Stop Enable"]
pub type PselcbW<'a, REG> = crate::BitWriter<'a, REG, Pselcb>;
impl<'a, REG> PselcbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcb::_0)
    }
    #[doc = "Counter stop is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcb::_1)
    }
}
#[doc = "ELC_GPTC Event Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pselcc {
    #[doc = "0: Counter stop is disable at the ELC_GPTC input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the ELC_GPTC input"]
    _1 = 1,
}
impl From<Pselcc> for bool {
    #[inline(always)]
    fn from(variant: Pselcc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSELCC` reader - ELC_GPTC Event Source Counter Stop Enable"]
pub type PselccR = crate::BitReader<Pselcc>;
impl PselccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pselcc {
        match self.bits {
            false => Pselcc::_0,
            true => Pselcc::_1,
        }
    }
    #[doc = "Counter stop is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pselcc::_0
    }
    #[doc = "Counter stop is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pselcc::_1
    }
}
#[doc = "Field `PSELCC` writer - ELC_GPTC Event Source Counter Stop Enable"]
pub type PselccW<'a, REG> = crate::BitWriter<'a, REG, Pselcc>;
impl<'a, REG> PselccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcc::_0)
    }
    #[doc = "Counter stop is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcc::_1)
    }
}
#[doc = "ELC_GPTD Event Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pselcd {
    #[doc = "0: Counter stop is disable at the ELC_GPTD input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the ELC_GPTD input"]
    _1 = 1,
}
impl From<Pselcd> for bool {
    #[inline(always)]
    fn from(variant: Pselcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSELCD` reader - ELC_GPTD Event Source Counter Stop Enable"]
pub type PselcdR = crate::BitReader<Pselcd>;
impl PselcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pselcd {
        match self.bits {
            false => Pselcd::_0,
            true => Pselcd::_1,
        }
    }
    #[doc = "Counter stop is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pselcd::_0
    }
    #[doc = "Counter stop is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pselcd::_1
    }
}
#[doc = "Field `PSELCD` writer - ELC_GPTD Event Source Counter Stop Enable"]
pub type PselcdW<'a, REG> = crate::BitWriter<'a, REG, Pselcd>;
impl<'a, REG> PselcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcd::_0)
    }
    #[doc = "Counter stop is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcd::_1)
    }
}
#[doc = "ELC_GPTE Event Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pselce {
    #[doc = "0: Counter stop is disable at the ELC_GPTE input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the ELC_GPTE input"]
    _1 = 1,
}
impl From<Pselce> for bool {
    #[inline(always)]
    fn from(variant: Pselce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSELCE` reader - ELC_GPTE Event Source Counter Stop Enable"]
pub type PselceR = crate::BitReader<Pselce>;
impl PselceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pselce {
        match self.bits {
            false => Pselce::_0,
            true => Pselce::_1,
        }
    }
    #[doc = "Counter stop is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pselce::_0
    }
    #[doc = "Counter stop is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pselce::_1
    }
}
#[doc = "Field `PSELCE` writer - ELC_GPTE Event Source Counter Stop Enable"]
pub type PselceW<'a, REG> = crate::BitWriter<'a, REG, Pselce>;
impl<'a, REG> PselceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pselce::_0)
    }
    #[doc = "Counter stop is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pselce::_1)
    }
}
#[doc = "ELC_GPTF Event Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pselcf {
    #[doc = "0: Counter stop is disable at the ELC_GPTF input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the ELC_GPTF input"]
    _1 = 1,
}
impl From<Pselcf> for bool {
    #[inline(always)]
    fn from(variant: Pselcf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSELCF` reader - ELC_GPTF Event Source Counter Stop Enable"]
pub type PselcfR = crate::BitReader<Pselcf>;
impl PselcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pselcf {
        match self.bits {
            false => Pselcf::_0,
            true => Pselcf::_1,
        }
    }
    #[doc = "Counter stop is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pselcf::_0
    }
    #[doc = "Counter stop is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pselcf::_1
    }
}
#[doc = "Field `PSELCF` writer - ELC_GPTF Event Source Counter Stop Enable"]
pub type PselcfW<'a, REG> = crate::BitWriter<'a, REG, Pselcf>;
impl<'a, REG> PselcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcf::_0)
    }
    #[doc = "Counter stop is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcf::_1)
    }
}
#[doc = "ELC_GPTG Event Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pselcg {
    #[doc = "0: Counter stop is disable at the ELC_GPTG input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the ELC_GPTG input"]
    _1 = 1,
}
impl From<Pselcg> for bool {
    #[inline(always)]
    fn from(variant: Pselcg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSELCG` reader - ELC_GPTG Event Source Counter Stop Enable"]
pub type PselcgR = crate::BitReader<Pselcg>;
impl PselcgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pselcg {
        match self.bits {
            false => Pselcg::_0,
            true => Pselcg::_1,
        }
    }
    #[doc = "Counter stop is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pselcg::_0
    }
    #[doc = "Counter stop is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pselcg::_1
    }
}
#[doc = "Field `PSELCG` writer - ELC_GPTG Event Source Counter Stop Enable"]
pub type PselcgW<'a, REG> = crate::BitWriter<'a, REG, Pselcg>;
impl<'a, REG> PselcgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcg::_0)
    }
    #[doc = "Counter stop is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcg::_1)
    }
}
#[doc = "ELC_GPTH Event Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pselch {
    #[doc = "0: Counter stop is disable at the ELC_GPTH input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the ELC_GPTH input"]
    _1 = 1,
}
impl From<Pselch> for bool {
    #[inline(always)]
    fn from(variant: Pselch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSELCH` reader - ELC_GPTH Event Source Counter Stop Enable"]
pub type PselchR = crate::BitReader<Pselch>;
impl PselchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pselch {
        match self.bits {
            false => Pselch::_0,
            true => Pselch::_1,
        }
    }
    #[doc = "Counter stop is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pselch::_0
    }
    #[doc = "Counter stop is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pselch::_1
    }
}
#[doc = "Field `PSELCH` writer - ELC_GPTH Event Source Counter Stop Enable"]
pub type PselchW<'a, REG> = crate::BitWriter<'a, REG, Pselch>;
impl<'a, REG> PselchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pselch::_0)
    }
    #[doc = "Counter stop is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pselch::_1)
    }
}
#[doc = "Software Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop {
    #[doc = "0: Counter stop is disable by the GTSTP register"]
    _0 = 0,
    #[doc = "1: Counter stop is enable by the GTSTP register"]
    _1 = 1,
}
impl From<Cstop> for bool {
    #[inline(always)]
    fn from(variant: Cstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTOP` reader - Software Source Counter Stop Enable"]
pub type CstopR = crate::BitReader<Cstop>;
impl CstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstop {
        match self.bits {
            false => Cstop::_0,
            true => Cstop::_1,
        }
    }
    #[doc = "Counter stop is disable by the GTSTP register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop::_0
    }
    #[doc = "Counter stop is enable by the GTSTP register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop::_1
    }
}
#[doc = "Field `CSTOP` writer - Software Source Counter Stop Enable"]
pub type CstopW<'a, REG> = crate::BitWriter<'a, REG, Cstop>;
impl<'a, REG> CstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable by the GTSTP register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop::_0)
    }
    #[doc = "Counter stop is enable by the GTSTP register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgar(&self) -> PsgtrgarR {
        PsgtrgarR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgaf(&self) -> PsgtrgafR {
        PsgtrgafR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgbr(&self) -> PsgtrgbrR {
        PsgtrgbrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgbf(&self) -> PsgtrgbfR {
        PsgtrgbfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscarbl(&self) -> PscarblR {
        PscarblR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscarbh(&self) -> PscarbhR {
        PscarbhR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscafbl(&self) -> PscafblR {
        PscafblR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscafbh(&self) -> PscafbhR {
        PscafbhR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbral(&self) -> PscbralR {
        PscbralR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbrah(&self) -> PscbrahR {
        PscbrahR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbfal(&self) -> PscbfalR {
        PscbfalR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbfah(&self) -> PscbfahR {
        PscbfahR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselca(&self) -> PselcaR {
        PselcaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcb(&self) -> PselcbR {
        PselcbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcc(&self) -> PselccR {
        PselccR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcd(&self) -> PselcdR {
        PselcdR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselce(&self) -> PselceR {
        PselceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcf(&self) -> PselcfR {
        PselcfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcg(&self) -> PselcgR {
        PselcgR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ELC_GPTH Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselch(&self) -> PselchR {
        PselchR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - Software Source Counter Stop Enable"]
    #[inline(always)]
    pub fn cstop(&self) -> CstopR {
        CstopR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTPSR")
            .field("cstop", &self.cstop())
            .field("pselch", &self.pselch())
            .field("pselcg", &self.pselcg())
            .field("pselcf", &self.pselcf())
            .field("pselce", &self.pselce())
            .field("pselcd", &self.pselcd())
            .field("pselcc", &self.pselcc())
            .field("pselcb", &self.pselcb())
            .field("pselca", &self.pselca())
            .field("pscbfah", &self.pscbfah())
            .field("pscbfal", &self.pscbfal())
            .field("pscbrah", &self.pscbrah())
            .field("pscbral", &self.pscbral())
            .field("pscafbh", &self.pscafbh())
            .field("pscafbl", &self.pscafbl())
            .field("pscarbh", &self.pscarbh())
            .field("pscarbl", &self.pscarbl())
            .field("psgtrgbf", &self.psgtrgbf())
            .field("psgtrgbr", &self.psgtrgbr())
            .field("psgtrgaf", &self.psgtrgaf())
            .field("psgtrgar", &self.psgtrgar())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgar(&mut self) -> PsgtrgarW<GtpsrSpec> {
        PsgtrgarW::new(self, 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgaf(&mut self) -> PsgtrgafW<GtpsrSpec> {
        PsgtrgafW::new(self, 1)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgbr(&mut self) -> PsgtrgbrW<GtpsrSpec> {
        PsgtrgbrW::new(self, 2)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgbf(&mut self) -> PsgtrgbfW<GtpsrSpec> {
        PsgtrgbfW::new(self, 3)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscarbl(&mut self) -> PscarblW<GtpsrSpec> {
        PscarblW::new(self, 8)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscarbh(&mut self) -> PscarbhW<GtpsrSpec> {
        PscarbhW::new(self, 9)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscafbl(&mut self) -> PscafblW<GtpsrSpec> {
        PscafblW::new(self, 10)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscafbh(&mut self) -> PscafbhW<GtpsrSpec> {
        PscafbhW::new(self, 11)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbral(&mut self) -> PscbralW<GtpsrSpec> {
        PscbralW::new(self, 12)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbrah(&mut self) -> PscbrahW<GtpsrSpec> {
        PscbrahW::new(self, 13)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbfal(&mut self) -> PscbfalW<GtpsrSpec> {
        PscbfalW::new(self, 14)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbfah(&mut self) -> PscbfahW<GtpsrSpec> {
        PscbfahW::new(self, 15)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselca(&mut self) -> PselcaW<GtpsrSpec> {
        PselcaW::new(self, 16)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcb(&mut self) -> PselcbW<GtpsrSpec> {
        PselcbW::new(self, 17)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcc(&mut self) -> PselccW<GtpsrSpec> {
        PselccW::new(self, 18)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcd(&mut self) -> PselcdW<GtpsrSpec> {
        PselcdW::new(self, 19)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselce(&mut self) -> PselceW<GtpsrSpec> {
        PselceW::new(self, 20)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcf(&mut self) -> PselcfW<GtpsrSpec> {
        PselcfW::new(self, 21)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcg(&mut self) -> PselcgW<GtpsrSpec> {
        PselcgW::new(self, 22)
    }
    #[doc = "Bit 23 - ELC_GPTH Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselch(&mut self) -> PselchW<GtpsrSpec> {
        PselchW::new(self, 23)
    }
    #[doc = "Bit 31 - Software Source Counter Stop Enable"]
    #[inline(always)]
    pub fn cstop(&mut self) -> CstopW<GtpsrSpec> {
        CstopW::new(self, 31)
    }
}
#[doc = "General PWM Timer Stop Source Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtpsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtpsrSpec;
impl crate::RegisterSpec for GtpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtpsr::R`](R) reader structure"]
impl crate::Readable for GtpsrSpec {}
#[doc = "`write(|w| ..)` method takes [`gtpsr::W`](W) writer structure"]
impl crate::Writable for GtpsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTPSR to value 0"]
impl crate::Resettable for GtpsrSpec {}
