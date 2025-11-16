#[doc = "Register `GTSSR` reader"]
pub type R = crate::R<GtssrSpec>;
#[doc = "Register `GTSSR` writer"]
pub type W = crate::W<GtssrSpec>;
#[doc = "GTETRGA Pin Rising Input Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssgtrgar {
    #[doc = "0: Counter start is disable at the rising edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the rising edge of GTETRGA input"]
    _1 = 1,
}
impl From<Ssgtrgar> for bool {
    #[inline(always)]
    fn from(variant: Ssgtrgar) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Start Enable"]
pub type SsgtrgarR = crate::BitReader<Ssgtrgar>;
impl SsgtrgarR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssgtrgar {
        match self.bits {
            false => Ssgtrgar::_0,
            true => Ssgtrgar::_1,
        }
    }
    #[doc = "Counter start is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssgtrgar::_0
    }
    #[doc = "Counter start is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssgtrgar::_1
    }
}
#[doc = "Field `SSGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Start Enable"]
pub type SsgtrgarW<'a, REG> = crate::BitWriter<'a, REG, Ssgtrgar>;
impl<'a, REG> SsgtrgarW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgar::_0)
    }
    #[doc = "Counter start is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgar::_1)
    }
}
#[doc = "GTETRGA Pin Falling Input Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssgtrgaf {
    #[doc = "0: Counter start is disable at the falling edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the falling edge of GTETRGA input"]
    _1 = 1,
}
impl From<Ssgtrgaf> for bool {
    #[inline(always)]
    fn from(variant: Ssgtrgaf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Start Enable"]
pub type SsgtrgafR = crate::BitReader<Ssgtrgaf>;
impl SsgtrgafR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssgtrgaf {
        match self.bits {
            false => Ssgtrgaf::_0,
            true => Ssgtrgaf::_1,
        }
    }
    #[doc = "Counter start is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssgtrgaf::_0
    }
    #[doc = "Counter start is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssgtrgaf::_1
    }
}
#[doc = "Field `SSGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Start Enable"]
pub type SsgtrgafW<'a, REG> = crate::BitWriter<'a, REG, Ssgtrgaf>;
impl<'a, REG> SsgtrgafW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgaf::_0)
    }
    #[doc = "Counter start is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgaf::_1)
    }
}
#[doc = "GTETRGB Pin Rising Input Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssgtrgbr {
    #[doc = "0: Counter start is disable at the rising edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the rising edge of GTETRGB input"]
    _1 = 1,
}
impl From<Ssgtrgbr> for bool {
    #[inline(always)]
    fn from(variant: Ssgtrgbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Start Enable"]
pub type SsgtrgbrR = crate::BitReader<Ssgtrgbr>;
impl SsgtrgbrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssgtrgbr {
        match self.bits {
            false => Ssgtrgbr::_0,
            true => Ssgtrgbr::_1,
        }
    }
    #[doc = "Counter start is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssgtrgbr::_0
    }
    #[doc = "Counter start is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssgtrgbr::_1
    }
}
#[doc = "Field `SSGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Start Enable"]
pub type SsgtrgbrW<'a, REG> = crate::BitWriter<'a, REG, Ssgtrgbr>;
impl<'a, REG> SsgtrgbrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgbr::_0)
    }
    #[doc = "Counter start is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgbr::_1)
    }
}
#[doc = "GTETRGB Pin Falling Input Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssgtrgbf {
    #[doc = "0: Counter start is disable at the falling edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the falling edge of GTETRGB input"]
    _1 = 1,
}
impl From<Ssgtrgbf> for bool {
    #[inline(always)]
    fn from(variant: Ssgtrgbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Start Enable"]
pub type SsgtrgbfR = crate::BitReader<Ssgtrgbf>;
impl SsgtrgbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssgtrgbf {
        match self.bits {
            false => Ssgtrgbf::_0,
            true => Ssgtrgbf::_1,
        }
    }
    #[doc = "Counter start is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssgtrgbf::_0
    }
    #[doc = "Counter start is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssgtrgbf::_1
    }
}
#[doc = "Field `SSGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Start Enable"]
pub type SsgtrgbfW<'a, REG> = crate::BitWriter<'a, REG, Ssgtrgbf>;
impl<'a, REG> SsgtrgbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgbf::_0)
    }
    #[doc = "Counter start is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgbf::_1)
    }
}
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sscarbl {
    #[doc = "0: Counter start is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<Sscarbl> for bool {
    #[inline(always)]
    fn from(variant: Sscarbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSCARBL` reader - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable"]
pub type SscarblR = crate::BitReader<Sscarbl>;
impl SscarblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sscarbl {
        match self.bits {
            false => Sscarbl::_0,
            true => Sscarbl::_1,
        }
    }
    #[doc = "Counter start is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sscarbl::_0
    }
    #[doc = "Counter start is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sscarbl::_1
    }
}
#[doc = "Field `SSCARBL` writer - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable"]
pub type SscarblW<'a, REG> = crate::BitWriter<'a, REG, Sscarbl>;
impl<'a, REG> SscarblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sscarbl::_0)
    }
    #[doc = "Counter start is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sscarbl::_1)
    }
}
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sscarbh {
    #[doc = "0: Counter start is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<Sscarbh> for bool {
    #[inline(always)]
    fn from(variant: Sscarbh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSCARBH` reader - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable"]
pub type SscarbhR = crate::BitReader<Sscarbh>;
impl SscarbhR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sscarbh {
        match self.bits {
            false => Sscarbh::_0,
            true => Sscarbh::_1,
        }
    }
    #[doc = "Counter start is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sscarbh::_0
    }
    #[doc = "Counter start is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sscarbh::_1
    }
}
#[doc = "Field `SSCARBH` writer - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable"]
pub type SscarbhW<'a, REG> = crate::BitWriter<'a, REG, Sscarbh>;
impl<'a, REG> SscarbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sscarbh::_0)
    }
    #[doc = "Counter start is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sscarbh::_1)
    }
}
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sscafbl {
    #[doc = "0: Counter start is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<Sscafbl> for bool {
    #[inline(always)]
    fn from(variant: Sscafbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSCAFBL` reader - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable"]
pub type SscafblR = crate::BitReader<Sscafbl>;
impl SscafblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sscafbl {
        match self.bits {
            false => Sscafbl::_0,
            true => Sscafbl::_1,
        }
    }
    #[doc = "Counter start is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sscafbl::_0
    }
    #[doc = "Counter start is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sscafbl::_1
    }
}
#[doc = "Field `SSCAFBL` writer - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable"]
pub type SscafblW<'a, REG> = crate::BitWriter<'a, REG, Sscafbl>;
impl<'a, REG> SscafblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sscafbl::_0)
    }
    #[doc = "Counter start is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sscafbl::_1)
    }
}
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sscafbh {
    #[doc = "0: Counter start is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<Sscafbh> for bool {
    #[inline(always)]
    fn from(variant: Sscafbh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSCAFBH` reader - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable"]
pub type SscafbhR = crate::BitReader<Sscafbh>;
impl SscafbhR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sscafbh {
        match self.bits {
            false => Sscafbh::_0,
            true => Sscafbh::_1,
        }
    }
    #[doc = "Counter start is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sscafbh::_0
    }
    #[doc = "Counter start is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sscafbh::_1
    }
}
#[doc = "Field `SSCAFBH` writer - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable"]
pub type SscafbhW<'a, REG> = crate::BitWriter<'a, REG, Sscafbh>;
impl<'a, REG> SscafbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sscafbh::_0)
    }
    #[doc = "Counter start is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sscafbh::_1)
    }
}
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sscbral {
    #[doc = "0: Counter start is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<Sscbral> for bool {
    #[inline(always)]
    fn from(variant: Sscbral) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSCBRAL` reader - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable"]
pub type SscbralR = crate::BitReader<Sscbral>;
impl SscbralR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sscbral {
        match self.bits {
            false => Sscbral::_0,
            true => Sscbral::_1,
        }
    }
    #[doc = "Counter start is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sscbral::_0
    }
    #[doc = "Counter start is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sscbral::_1
    }
}
#[doc = "Field `SSCBRAL` writer - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable"]
pub type SscbralW<'a, REG> = crate::BitWriter<'a, REG, Sscbral>;
impl<'a, REG> SscbralW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sscbral::_0)
    }
    #[doc = "Counter start is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sscbral::_1)
    }
}
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sscbrah {
    #[doc = "0: Counter start is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<Sscbrah> for bool {
    #[inline(always)]
    fn from(variant: Sscbrah) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSCBRAH` reader - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable"]
pub type SscbrahR = crate::BitReader<Sscbrah>;
impl SscbrahR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sscbrah {
        match self.bits {
            false => Sscbrah::_0,
            true => Sscbrah::_1,
        }
    }
    #[doc = "Counter start is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sscbrah::_0
    }
    #[doc = "Counter start is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sscbrah::_1
    }
}
#[doc = "Field `SSCBRAH` writer - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable"]
pub type SscbrahW<'a, REG> = crate::BitWriter<'a, REG, Sscbrah>;
impl<'a, REG> SscbrahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sscbrah::_0)
    }
    #[doc = "Counter start is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sscbrah::_1)
    }
}
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sscbfal {
    #[doc = "0: Counter start is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<Sscbfal> for bool {
    #[inline(always)]
    fn from(variant: Sscbfal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSCBFAL` reader - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable"]
pub type SscbfalR = crate::BitReader<Sscbfal>;
impl SscbfalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sscbfal {
        match self.bits {
            false => Sscbfal::_0,
            true => Sscbfal::_1,
        }
    }
    #[doc = "Counter start is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sscbfal::_0
    }
    #[doc = "Counter start is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sscbfal::_1
    }
}
#[doc = "Field `SSCBFAL` writer - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable"]
pub type SscbfalW<'a, REG> = crate::BitWriter<'a, REG, Sscbfal>;
impl<'a, REG> SscbfalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sscbfal::_0)
    }
    #[doc = "Counter start is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sscbfal::_1)
    }
}
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sscbfah {
    #[doc = "0: Counter start is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<Sscbfah> for bool {
    #[inline(always)]
    fn from(variant: Sscbfah) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSCBFAH` reader - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable"]
pub type SscbfahR = crate::BitReader<Sscbfah>;
impl SscbfahR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sscbfah {
        match self.bits {
            false => Sscbfah::_0,
            true => Sscbfah::_1,
        }
    }
    #[doc = "Counter start is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sscbfah::_0
    }
    #[doc = "Counter start is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sscbfah::_1
    }
}
#[doc = "Field `SSCBFAH` writer - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable"]
pub type SscbfahW<'a, REG> = crate::BitWriter<'a, REG, Sscbfah>;
impl<'a, REG> SscbfahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sscbfah::_0)
    }
    #[doc = "Counter start is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sscbfah::_1)
    }
}
#[doc = "ELC_GPTA Event Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sselca {
    #[doc = "0: Counter start is disable at the ELC_GPTA input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the ELC_GPTA input"]
    _1 = 1,
}
impl From<Sselca> for bool {
    #[inline(always)]
    fn from(variant: Sselca) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSELCA` reader - ELC_GPTA Event Source Counter Start Enable"]
pub type SselcaR = crate::BitReader<Sselca>;
impl SselcaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sselca {
        match self.bits {
            false => Sselca::_0,
            true => Sselca::_1,
        }
    }
    #[doc = "Counter start is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sselca::_0
    }
    #[doc = "Counter start is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sselca::_1
    }
}
#[doc = "Field `SSELCA` writer - ELC_GPTA Event Source Counter Start Enable"]
pub type SselcaW<'a, REG> = crate::BitWriter<'a, REG, Sselca>;
impl<'a, REG> SselcaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sselca::_0)
    }
    #[doc = "Counter start is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sselca::_1)
    }
}
#[doc = "ELC_GPTB Event Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sselcb {
    #[doc = "0: Counter start is disable at the ELC_GPTB input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the ELC_GPTB input"]
    _1 = 1,
}
impl From<Sselcb> for bool {
    #[inline(always)]
    fn from(variant: Sselcb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSELCB` reader - ELC_GPTB Event Source Counter Start Enable"]
pub type SselcbR = crate::BitReader<Sselcb>;
impl SselcbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sselcb {
        match self.bits {
            false => Sselcb::_0,
            true => Sselcb::_1,
        }
    }
    #[doc = "Counter start is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sselcb::_0
    }
    #[doc = "Counter start is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sselcb::_1
    }
}
#[doc = "Field `SSELCB` writer - ELC_GPTB Event Source Counter Start Enable"]
pub type SselcbW<'a, REG> = crate::BitWriter<'a, REG, Sselcb>;
impl<'a, REG> SselcbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcb::_0)
    }
    #[doc = "Counter start is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcb::_1)
    }
}
#[doc = "ELC_GPTC Event Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sselcc {
    #[doc = "0: Counter start is disable at the ELC_GPTC input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the ELC_GPTC input"]
    _1 = 1,
}
impl From<Sselcc> for bool {
    #[inline(always)]
    fn from(variant: Sselcc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSELCC` reader - ELC_GPTC Event Source Counter Start Enable"]
pub type SselccR = crate::BitReader<Sselcc>;
impl SselccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sselcc {
        match self.bits {
            false => Sselcc::_0,
            true => Sselcc::_1,
        }
    }
    #[doc = "Counter start is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sselcc::_0
    }
    #[doc = "Counter start is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sselcc::_1
    }
}
#[doc = "Field `SSELCC` writer - ELC_GPTC Event Source Counter Start Enable"]
pub type SselccW<'a, REG> = crate::BitWriter<'a, REG, Sselcc>;
impl<'a, REG> SselccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcc::_0)
    }
    #[doc = "Counter start is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcc::_1)
    }
}
#[doc = "ELC_GPTD Event Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sselcd {
    #[doc = "0: Counter start is disable at the ELC_GPTD input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the ELC_GPTD input"]
    _1 = 1,
}
impl From<Sselcd> for bool {
    #[inline(always)]
    fn from(variant: Sselcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSELCD` reader - ELC_GPTD Event Source Counter Start Enable"]
pub type SselcdR = crate::BitReader<Sselcd>;
impl SselcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sselcd {
        match self.bits {
            false => Sselcd::_0,
            true => Sselcd::_1,
        }
    }
    #[doc = "Counter start is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sselcd::_0
    }
    #[doc = "Counter start is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sselcd::_1
    }
}
#[doc = "Field `SSELCD` writer - ELC_GPTD Event Source Counter Start Enable"]
pub type SselcdW<'a, REG> = crate::BitWriter<'a, REG, Sselcd>;
impl<'a, REG> SselcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcd::_0)
    }
    #[doc = "Counter start is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcd::_1)
    }
}
#[doc = "ELC_GPTE Event Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sselce {
    #[doc = "0: Counter start is disable at the ELC_GPTE input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the ELC_GPTE input"]
    _1 = 1,
}
impl From<Sselce> for bool {
    #[inline(always)]
    fn from(variant: Sselce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSELCE` reader - ELC_GPTE Event Source Counter Start Enable"]
pub type SselceR = crate::BitReader<Sselce>;
impl SselceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sselce {
        match self.bits {
            false => Sselce::_0,
            true => Sselce::_1,
        }
    }
    #[doc = "Counter start is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sselce::_0
    }
    #[doc = "Counter start is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sselce::_1
    }
}
#[doc = "Field `SSELCE` writer - ELC_GPTE Event Source Counter Start Enable"]
pub type SselceW<'a, REG> = crate::BitWriter<'a, REG, Sselce>;
impl<'a, REG> SselceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sselce::_0)
    }
    #[doc = "Counter start is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sselce::_1)
    }
}
#[doc = "ELC_GPTF Event Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sselcf {
    #[doc = "0: Counter start is disable at the ELC_GPTF input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the ELC_GPTF input"]
    _1 = 1,
}
impl From<Sselcf> for bool {
    #[inline(always)]
    fn from(variant: Sselcf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSELCF` reader - ELC_GPTF Event Source Counter Start Enable"]
pub type SselcfR = crate::BitReader<Sselcf>;
impl SselcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sselcf {
        match self.bits {
            false => Sselcf::_0,
            true => Sselcf::_1,
        }
    }
    #[doc = "Counter start is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sselcf::_0
    }
    #[doc = "Counter start is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sselcf::_1
    }
}
#[doc = "Field `SSELCF` writer - ELC_GPTF Event Source Counter Start Enable"]
pub type SselcfW<'a, REG> = crate::BitWriter<'a, REG, Sselcf>;
impl<'a, REG> SselcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcf::_0)
    }
    #[doc = "Counter start is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcf::_1)
    }
}
#[doc = "ELC_GPTG Event Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sselcg {
    #[doc = "0: Counter start is disable at the ELC_GPTG input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the ELC_GPTG input"]
    _1 = 1,
}
impl From<Sselcg> for bool {
    #[inline(always)]
    fn from(variant: Sselcg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSELCG` reader - ELC_GPTG Event Source Counter Start Enable"]
pub type SselcgR = crate::BitReader<Sselcg>;
impl SselcgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sselcg {
        match self.bits {
            false => Sselcg::_0,
            true => Sselcg::_1,
        }
    }
    #[doc = "Counter start is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sselcg::_0
    }
    #[doc = "Counter start is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sselcg::_1
    }
}
#[doc = "Field `SSELCG` writer - ELC_GPTG Event Source Counter Start Enable"]
pub type SselcgW<'a, REG> = crate::BitWriter<'a, REG, Sselcg>;
impl<'a, REG> SselcgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcg::_0)
    }
    #[doc = "Counter start is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcg::_1)
    }
}
#[doc = "ELCH Event Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sselch {
    #[doc = "0: Counter start is disable at the ELC_GPTH input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the ELC_GPTH input"]
    _1 = 1,
}
impl From<Sselch> for bool {
    #[inline(always)]
    fn from(variant: Sselch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSELCH` reader - ELCH Event Source Counter Start Enable"]
pub type SselchR = crate::BitReader<Sselch>;
impl SselchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sselch {
        match self.bits {
            false => Sselch::_0,
            true => Sselch::_1,
        }
    }
    #[doc = "Counter start is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sselch::_0
    }
    #[doc = "Counter start is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sselch::_1
    }
}
#[doc = "Field `SSELCH` writer - ELCH Event Source Counter Start Enable"]
pub type SselchW<'a, REG> = crate::BitWriter<'a, REG, Sselch>;
impl<'a, REG> SselchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sselch::_0)
    }
    #[doc = "Counter start is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sselch::_1)
    }
}
#[doc = "Software Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstrt {
    #[doc = "0: Counter start is disable by the GTSTR register"]
    _0 = 0,
    #[doc = "1: Counter start is enable by the GTSTR register"]
    _1 = 1,
}
impl From<Cstrt> for bool {
    #[inline(always)]
    fn from(variant: Cstrt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTRT` reader - Software Source Counter Start Enable"]
pub type CstrtR = crate::BitReader<Cstrt>;
impl CstrtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstrt {
        match self.bits {
            false => Cstrt::_0,
            true => Cstrt::_1,
        }
    }
    #[doc = "Counter start is disable by the GTSTR register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstrt::_0
    }
    #[doc = "Counter start is enable by the GTSTR register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstrt::_1
    }
}
#[doc = "Field `CSTRT` writer - Software Source Counter Start Enable"]
pub type CstrtW<'a, REG> = crate::BitWriter<'a, REG, Cstrt>;
impl<'a, REG> CstrtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable by the GTSTR register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt::_0)
    }
    #[doc = "Counter start is enable by the GTSTR register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgar(&self) -> SsgtrgarR {
        SsgtrgarR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgaf(&self) -> SsgtrgafR {
        SsgtrgafR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgbr(&self) -> SsgtrgbrR {
        SsgtrgbrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgbf(&self) -> SsgtrgbfR {
        SsgtrgbfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscarbl(&self) -> SscarblR {
        SscarblR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscarbh(&self) -> SscarbhR {
        SscarbhR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscafbl(&self) -> SscafblR {
        SscafblR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscafbh(&self) -> SscafbhR {
        SscafbhR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbral(&self) -> SscbralR {
        SscbralR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbrah(&self) -> SscbrahR {
        SscbrahR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbfal(&self) -> SscbfalR {
        SscbfalR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbfah(&self) -> SscbfahR {
        SscbfahR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselca(&self) -> SselcaR {
        SselcaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcb(&self) -> SselcbR {
        SselcbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcc(&self) -> SselccR {
        SselccR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcd(&self) -> SselcdR {
        SselcdR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselce(&self) -> SselceR {
        SselceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcf(&self) -> SselcfR {
        SselcfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcg(&self) -> SselcgR {
        SselcgR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ELCH Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselch(&self) -> SselchR {
        SselchR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - Software Source Counter Start Enable"]
    #[inline(always)]
    pub fn cstrt(&self) -> CstrtR {
        CstrtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTSSR")
            .field("cstrt", &self.cstrt())
            .field("sselch", &self.sselch())
            .field("sselcg", &self.sselcg())
            .field("sselcf", &self.sselcf())
            .field("sselce", &self.sselce())
            .field("sselcd", &self.sselcd())
            .field("sselcc", &self.sselcc())
            .field("sselcb", &self.sselcb())
            .field("sselca", &self.sselca())
            .field("sscbfah", &self.sscbfah())
            .field("sscbfal", &self.sscbfal())
            .field("sscbrah", &self.sscbrah())
            .field("sscbral", &self.sscbral())
            .field("sscafbh", &self.sscafbh())
            .field("sscafbl", &self.sscafbl())
            .field("sscarbh", &self.sscarbh())
            .field("sscarbl", &self.sscarbl())
            .field("ssgtrgbf", &self.ssgtrgbf())
            .field("ssgtrgbr", &self.ssgtrgbr())
            .field("ssgtrgaf", &self.ssgtrgaf())
            .field("ssgtrgar", &self.ssgtrgar())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgar(&mut self) -> SsgtrgarW<GtssrSpec> {
        SsgtrgarW::new(self, 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgaf(&mut self) -> SsgtrgafW<GtssrSpec> {
        SsgtrgafW::new(self, 1)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgbr(&mut self) -> SsgtrgbrW<GtssrSpec> {
        SsgtrgbrW::new(self, 2)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgbf(&mut self) -> SsgtrgbfW<GtssrSpec> {
        SsgtrgbfW::new(self, 3)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscarbl(&mut self) -> SscarblW<GtssrSpec> {
        SscarblW::new(self, 8)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscarbh(&mut self) -> SscarbhW<GtssrSpec> {
        SscarbhW::new(self, 9)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscafbl(&mut self) -> SscafblW<GtssrSpec> {
        SscafblW::new(self, 10)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscafbh(&mut self) -> SscafbhW<GtssrSpec> {
        SscafbhW::new(self, 11)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbral(&mut self) -> SscbralW<GtssrSpec> {
        SscbralW::new(self, 12)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbrah(&mut self) -> SscbrahW<GtssrSpec> {
        SscbrahW::new(self, 13)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbfal(&mut self) -> SscbfalW<GtssrSpec> {
        SscbfalW::new(self, 14)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbfah(&mut self) -> SscbfahW<GtssrSpec> {
        SscbfahW::new(self, 15)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselca(&mut self) -> SselcaW<GtssrSpec> {
        SselcaW::new(self, 16)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcb(&mut self) -> SselcbW<GtssrSpec> {
        SselcbW::new(self, 17)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcc(&mut self) -> SselccW<GtssrSpec> {
        SselccW::new(self, 18)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcd(&mut self) -> SselcdW<GtssrSpec> {
        SselcdW::new(self, 19)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselce(&mut self) -> SselceW<GtssrSpec> {
        SselceW::new(self, 20)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcf(&mut self) -> SselcfW<GtssrSpec> {
        SselcfW::new(self, 21)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcg(&mut self) -> SselcgW<GtssrSpec> {
        SselcgW::new(self, 22)
    }
    #[doc = "Bit 23 - ELCH Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselch(&mut self) -> SselchW<GtssrSpec> {
        SselchW::new(self, 23)
    }
    #[doc = "Bit 31 - Software Source Counter Start Enable"]
    #[inline(always)]
    pub fn cstrt(&mut self) -> CstrtW<GtssrSpec> {
        CstrtW::new(self, 31)
    }
}
#[doc = "General PWM Timer Start Source Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtssr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtssr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtssrSpec;
impl crate::RegisterSpec for GtssrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtssr::R`](R) reader structure"]
impl crate::Readable for GtssrSpec {}
#[doc = "`write(|w| ..)` method takes [`gtssr::W`](W) writer structure"]
impl crate::Writable for GtssrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTSSR to value 0"]
impl crate::Resettable for GtssrSpec {}
