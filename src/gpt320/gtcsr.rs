#[doc = "Register `GTCSR` reader"]
pub type R = crate::R<GtcsrSpec>;
#[doc = "Register `GTCSR` writer"]
pub type W = crate::W<GtcsrSpec>;
#[doc = "GTETRGA Pin Rising Input Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csgtrgar {
    #[doc = "0: Counter clear is disable at the rising edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the rising edge of GTETRGA input"]
    _1 = 1,
}
impl From<Csgtrgar> for bool {
    #[inline(always)]
    fn from(variant: Csgtrgar) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Clear Enable"]
pub type CsgtrgarR = crate::BitReader<Csgtrgar>;
impl CsgtrgarR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csgtrgar {
        match self.bits {
            false => Csgtrgar::_0,
            true => Csgtrgar::_1,
        }
    }
    #[doc = "Counter clear is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Csgtrgar::_0
    }
    #[doc = "Counter clear is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Csgtrgar::_1
    }
}
#[doc = "Field `CSGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Clear Enable"]
pub type CsgtrgarW<'a, REG> = crate::BitWriter<'a, REG, Csgtrgar>;
impl<'a, REG> CsgtrgarW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Csgtrgar::_0)
    }
    #[doc = "Counter clear is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Csgtrgar::_1)
    }
}
#[doc = "GTETRGA Pin Falling Input Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csgtrgaf {
    #[doc = "0: Counter clear is disable at the falling edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the falling edge of GTETRGA input"]
    _1 = 1,
}
impl From<Csgtrgaf> for bool {
    #[inline(always)]
    fn from(variant: Csgtrgaf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Clear Enable"]
pub type CsgtrgafR = crate::BitReader<Csgtrgaf>;
impl CsgtrgafR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csgtrgaf {
        match self.bits {
            false => Csgtrgaf::_0,
            true => Csgtrgaf::_1,
        }
    }
    #[doc = "Counter clear is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Csgtrgaf::_0
    }
    #[doc = "Counter clear is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Csgtrgaf::_1
    }
}
#[doc = "Field `CSGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Clear Enable"]
pub type CsgtrgafW<'a, REG> = crate::BitWriter<'a, REG, Csgtrgaf>;
impl<'a, REG> CsgtrgafW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Csgtrgaf::_0)
    }
    #[doc = "Counter clear is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Csgtrgaf::_1)
    }
}
#[doc = "GTETRGB Pin Rising Input Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csgtrgbr {
    #[doc = "0: Counter clear is disable at the rising edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the rising edge of GTETRGB input"]
    _1 = 1,
}
impl From<Csgtrgbr> for bool {
    #[inline(always)]
    fn from(variant: Csgtrgbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Clear Enable"]
pub type CsgtrgbrR = crate::BitReader<Csgtrgbr>;
impl CsgtrgbrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csgtrgbr {
        match self.bits {
            false => Csgtrgbr::_0,
            true => Csgtrgbr::_1,
        }
    }
    #[doc = "Counter clear is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Csgtrgbr::_0
    }
    #[doc = "Counter clear is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Csgtrgbr::_1
    }
}
#[doc = "Field `CSGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Clear Enable"]
pub type CsgtrgbrW<'a, REG> = crate::BitWriter<'a, REG, Csgtrgbr>;
impl<'a, REG> CsgtrgbrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Csgtrgbr::_0)
    }
    #[doc = "Counter clear is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Csgtrgbr::_1)
    }
}
#[doc = "GTETRGB Pin Falling Input Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csgtrgbf {
    #[doc = "0: Counter clear is disable at the falling edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the falling edge of GTETRGB input"]
    _1 = 1,
}
impl From<Csgtrgbf> for bool {
    #[inline(always)]
    fn from(variant: Csgtrgbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Clear Enable"]
pub type CsgtrgbfR = crate::BitReader<Csgtrgbf>;
impl CsgtrgbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csgtrgbf {
        match self.bits {
            false => Csgtrgbf::_0,
            true => Csgtrgbf::_1,
        }
    }
    #[doc = "Counter clear is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Csgtrgbf::_0
    }
    #[doc = "Counter clear is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Csgtrgbf::_1
    }
}
#[doc = "Field `CSGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Clear Enable"]
pub type CsgtrgbfW<'a, REG> = crate::BitWriter<'a, REG, Csgtrgbf>;
impl<'a, REG> CsgtrgbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Csgtrgbf::_0)
    }
    #[doc = "Counter clear is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Csgtrgbf::_1)
    }
}
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cscarbl {
    #[doc = "0: Counter clear is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<Cscarbl> for bool {
    #[inline(always)]
    fn from(variant: Cscarbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSCARBL` reader - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable"]
pub type CscarblR = crate::BitReader<Cscarbl>;
impl CscarblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cscarbl {
        match self.bits {
            false => Cscarbl::_0,
            true => Cscarbl::_1,
        }
    }
    #[doc = "Counter clear is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cscarbl::_0
    }
    #[doc = "Counter clear is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cscarbl::_1
    }
}
#[doc = "Field `CSCARBL` writer - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable"]
pub type CscarblW<'a, REG> = crate::BitWriter<'a, REG, Cscarbl>;
impl<'a, REG> CscarblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cscarbl::_0)
    }
    #[doc = "Counter clear is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cscarbl::_1)
    }
}
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cscarbh {
    #[doc = "0: Counter clear is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<Cscarbh> for bool {
    #[inline(always)]
    fn from(variant: Cscarbh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSCARBH` reader - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable"]
pub type CscarbhR = crate::BitReader<Cscarbh>;
impl CscarbhR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cscarbh {
        match self.bits {
            false => Cscarbh::_0,
            true => Cscarbh::_1,
        }
    }
    #[doc = "Counter clear is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cscarbh::_0
    }
    #[doc = "Counter clear is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cscarbh::_1
    }
}
#[doc = "Field `CSCARBH` writer - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable"]
pub type CscarbhW<'a, REG> = crate::BitWriter<'a, REG, Cscarbh>;
impl<'a, REG> CscarbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cscarbh::_0)
    }
    #[doc = "Counter clear is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cscarbh::_1)
    }
}
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cscafbl {
    #[doc = "0: Counter clear is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<Cscafbl> for bool {
    #[inline(always)]
    fn from(variant: Cscafbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSCAFBL` reader - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable"]
pub type CscafblR = crate::BitReader<Cscafbl>;
impl CscafblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cscafbl {
        match self.bits {
            false => Cscafbl::_0,
            true => Cscafbl::_1,
        }
    }
    #[doc = "Counter clear is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cscafbl::_0
    }
    #[doc = "Counter clear is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cscafbl::_1
    }
}
#[doc = "Field `CSCAFBL` writer - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable"]
pub type CscafblW<'a, REG> = crate::BitWriter<'a, REG, Cscafbl>;
impl<'a, REG> CscafblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cscafbl::_0)
    }
    #[doc = "Counter clear is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cscafbl::_1)
    }
}
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cscafbh {
    #[doc = "0: Counter clear is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<Cscafbh> for bool {
    #[inline(always)]
    fn from(variant: Cscafbh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSCAFBH` reader - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable"]
pub type CscafbhR = crate::BitReader<Cscafbh>;
impl CscafbhR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cscafbh {
        match self.bits {
            false => Cscafbh::_0,
            true => Cscafbh::_1,
        }
    }
    #[doc = "Counter clear is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cscafbh::_0
    }
    #[doc = "Counter clear is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cscafbh::_1
    }
}
#[doc = "Field `CSCAFBH` writer - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable"]
pub type CscafbhW<'a, REG> = crate::BitWriter<'a, REG, Cscafbh>;
impl<'a, REG> CscafbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cscafbh::_0)
    }
    #[doc = "Counter clear is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cscafbh::_1)
    }
}
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cscbral {
    #[doc = "0: Counter clear is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<Cscbral> for bool {
    #[inline(always)]
    fn from(variant: Cscbral) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSCBRAL` reader - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable"]
pub type CscbralR = crate::BitReader<Cscbral>;
impl CscbralR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cscbral {
        match self.bits {
            false => Cscbral::_0,
            true => Cscbral::_1,
        }
    }
    #[doc = "Counter clear is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cscbral::_0
    }
    #[doc = "Counter clear is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cscbral::_1
    }
}
#[doc = "Field `CSCBRAL` writer - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable"]
pub type CscbralW<'a, REG> = crate::BitWriter<'a, REG, Cscbral>;
impl<'a, REG> CscbralW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cscbral::_0)
    }
    #[doc = "Counter clear is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cscbral::_1)
    }
}
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cscbrah {
    #[doc = "0: Counter clear is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<Cscbrah> for bool {
    #[inline(always)]
    fn from(variant: Cscbrah) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSCBRAH` reader - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable"]
pub type CscbrahR = crate::BitReader<Cscbrah>;
impl CscbrahR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cscbrah {
        match self.bits {
            false => Cscbrah::_0,
            true => Cscbrah::_1,
        }
    }
    #[doc = "Counter clear is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cscbrah::_0
    }
    #[doc = "Counter clear is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cscbrah::_1
    }
}
#[doc = "Field `CSCBRAH` writer - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable"]
pub type CscbrahW<'a, REG> = crate::BitWriter<'a, REG, Cscbrah>;
impl<'a, REG> CscbrahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cscbrah::_0)
    }
    #[doc = "Counter clear is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cscbrah::_1)
    }
}
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cscbfal {
    #[doc = "0: Counter clear is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<Cscbfal> for bool {
    #[inline(always)]
    fn from(variant: Cscbfal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSCBFAL` reader - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable"]
pub type CscbfalR = crate::BitReader<Cscbfal>;
impl CscbfalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cscbfal {
        match self.bits {
            false => Cscbfal::_0,
            true => Cscbfal::_1,
        }
    }
    #[doc = "Counter clear is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cscbfal::_0
    }
    #[doc = "Counter clear is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cscbfal::_1
    }
}
#[doc = "Field `CSCBFAL` writer - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable"]
pub type CscbfalW<'a, REG> = crate::BitWriter<'a, REG, Cscbfal>;
impl<'a, REG> CscbfalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cscbfal::_0)
    }
    #[doc = "Counter clear is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cscbfal::_1)
    }
}
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cscbfah {
    #[doc = "0: Counter clear is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<Cscbfah> for bool {
    #[inline(always)]
    fn from(variant: Cscbfah) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSCBFAH` reader - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable"]
pub type CscbfahR = crate::BitReader<Cscbfah>;
impl CscbfahR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cscbfah {
        match self.bits {
            false => Cscbfah::_0,
            true => Cscbfah::_1,
        }
    }
    #[doc = "Counter clear is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cscbfah::_0
    }
    #[doc = "Counter clear is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cscbfah::_1
    }
}
#[doc = "Field `CSCBFAH` writer - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable"]
pub type CscbfahW<'a, REG> = crate::BitWriter<'a, REG, Cscbfah>;
impl<'a, REG> CscbfahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cscbfah::_0)
    }
    #[doc = "Counter clear is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cscbfah::_1)
    }
}
#[doc = "ELC_GPTA Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cselca {
    #[doc = "0: Counter clear is disable at the ELC_GPTA input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the ELC_GPTA input"]
    _1 = 1,
}
impl From<Cselca> for bool {
    #[inline(always)]
    fn from(variant: Cselca) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSELCA` reader - ELC_GPTA Event Source Counter Clear Enable"]
pub type CselcaR = crate::BitReader<Cselca>;
impl CselcaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cselca {
        match self.bits {
            false => Cselca::_0,
            true => Cselca::_1,
        }
    }
    #[doc = "Counter clear is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cselca::_0
    }
    #[doc = "Counter clear is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cselca::_1
    }
}
#[doc = "Field `CSELCA` writer - ELC_GPTA Event Source Counter Clear Enable"]
pub type CselcaW<'a, REG> = crate::BitWriter<'a, REG, Cselca>;
impl<'a, REG> CselcaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cselca::_0)
    }
    #[doc = "Counter clear is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cselca::_1)
    }
}
#[doc = "ELC_GPTB Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cselcb {
    #[doc = "0: Counter clear is disable at the ELC_GPTB input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the ELC_GPTB input"]
    _1 = 1,
}
impl From<Cselcb> for bool {
    #[inline(always)]
    fn from(variant: Cselcb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSELCB` reader - ELC_GPTB Event Source Counter Clear Enable"]
pub type CselcbR = crate::BitReader<Cselcb>;
impl CselcbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cselcb {
        match self.bits {
            false => Cselcb::_0,
            true => Cselcb::_1,
        }
    }
    #[doc = "Counter clear is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cselcb::_0
    }
    #[doc = "Counter clear is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cselcb::_1
    }
}
#[doc = "Field `CSELCB` writer - ELC_GPTB Event Source Counter Clear Enable"]
pub type CselcbW<'a, REG> = crate::BitWriter<'a, REG, Cselcb>;
impl<'a, REG> CselcbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cselcb::_0)
    }
    #[doc = "Counter clear is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cselcb::_1)
    }
}
#[doc = "ELC_GPTC Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cselcc {
    #[doc = "0: Counter clear is disable at the ELC_GPTC input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the ELC_GPTC input"]
    _1 = 1,
}
impl From<Cselcc> for bool {
    #[inline(always)]
    fn from(variant: Cselcc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSELCC` reader - ELC_GPTC Event Source Counter Clear Enable"]
pub type CselccR = crate::BitReader<Cselcc>;
impl CselccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cselcc {
        match self.bits {
            false => Cselcc::_0,
            true => Cselcc::_1,
        }
    }
    #[doc = "Counter clear is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cselcc::_0
    }
    #[doc = "Counter clear is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cselcc::_1
    }
}
#[doc = "Field `CSELCC` writer - ELC_GPTC Event Source Counter Clear Enable"]
pub type CselccW<'a, REG> = crate::BitWriter<'a, REG, Cselcc>;
impl<'a, REG> CselccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cselcc::_0)
    }
    #[doc = "Counter clear is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cselcc::_1)
    }
}
#[doc = "ELC_GPTD Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cselcd {
    #[doc = "0: Counter clear is disable at the ELC_GPTD input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the ELC_GPTD input"]
    _1 = 1,
}
impl From<Cselcd> for bool {
    #[inline(always)]
    fn from(variant: Cselcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSELCD` reader - ELC_GPTD Event Source Counter Clear Enable"]
pub type CselcdR = crate::BitReader<Cselcd>;
impl CselcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cselcd {
        match self.bits {
            false => Cselcd::_0,
            true => Cselcd::_1,
        }
    }
    #[doc = "Counter clear is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cselcd::_0
    }
    #[doc = "Counter clear is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cselcd::_1
    }
}
#[doc = "Field `CSELCD` writer - ELC_GPTD Event Source Counter Clear Enable"]
pub type CselcdW<'a, REG> = crate::BitWriter<'a, REG, Cselcd>;
impl<'a, REG> CselcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cselcd::_0)
    }
    #[doc = "Counter clear is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cselcd::_1)
    }
}
#[doc = "ELC_GPTE Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cselce {
    #[doc = "0: Counter clear is disable at the ELC_GPTE input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the ELC_GPTE input"]
    _1 = 1,
}
impl From<Cselce> for bool {
    #[inline(always)]
    fn from(variant: Cselce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSELCE` reader - ELC_GPTE Event Source Counter Clear Enable"]
pub type CselceR = crate::BitReader<Cselce>;
impl CselceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cselce {
        match self.bits {
            false => Cselce::_0,
            true => Cselce::_1,
        }
    }
    #[doc = "Counter clear is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cselce::_0
    }
    #[doc = "Counter clear is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cselce::_1
    }
}
#[doc = "Field `CSELCE` writer - ELC_GPTE Event Source Counter Clear Enable"]
pub type CselceW<'a, REG> = crate::BitWriter<'a, REG, Cselce>;
impl<'a, REG> CselceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cselce::_0)
    }
    #[doc = "Counter clear is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cselce::_1)
    }
}
#[doc = "ELC_GPTF Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cselcf {
    #[doc = "0: Counter clear is disable at the ELC_GPTF input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the ELC_GPTF input"]
    _1 = 1,
}
impl From<Cselcf> for bool {
    #[inline(always)]
    fn from(variant: Cselcf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSELCF` reader - ELC_GPTF Event Source Counter Clear Enable"]
pub type CselcfR = crate::BitReader<Cselcf>;
impl CselcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cselcf {
        match self.bits {
            false => Cselcf::_0,
            true => Cselcf::_1,
        }
    }
    #[doc = "Counter clear is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cselcf::_0
    }
    #[doc = "Counter clear is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cselcf::_1
    }
}
#[doc = "Field `CSELCF` writer - ELC_GPTF Event Source Counter Clear Enable"]
pub type CselcfW<'a, REG> = crate::BitWriter<'a, REG, Cselcf>;
impl<'a, REG> CselcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cselcf::_0)
    }
    #[doc = "Counter clear is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cselcf::_1)
    }
}
#[doc = "ELC_GPTG Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cselcg {
    #[doc = "0: Counter clear is disable at the ELC_GPTG input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the ELC_GPTG input"]
    _1 = 1,
}
impl From<Cselcg> for bool {
    #[inline(always)]
    fn from(variant: Cselcg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSELCG` reader - ELC_GPTG Event Source Counter Clear Enable"]
pub type CselcgR = crate::BitReader<Cselcg>;
impl CselcgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cselcg {
        match self.bits {
            false => Cselcg::_0,
            true => Cselcg::_1,
        }
    }
    #[doc = "Counter clear is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cselcg::_0
    }
    #[doc = "Counter clear is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cselcg::_1
    }
}
#[doc = "Field `CSELCG` writer - ELC_GPTG Event Source Counter Clear Enable"]
pub type CselcgW<'a, REG> = crate::BitWriter<'a, REG, Cselcg>;
impl<'a, REG> CselcgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cselcg::_0)
    }
    #[doc = "Counter clear is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cselcg::_1)
    }
}
#[doc = "ELCH Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cselch {
    #[doc = "0: Counter clear is disable at the ELC_GPTH input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the ELC_GPTH input"]
    _1 = 1,
}
impl From<Cselch> for bool {
    #[inline(always)]
    fn from(variant: Cselch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSELCH` reader - ELCH Event Source Counter Clear Enable"]
pub type CselchR = crate::BitReader<Cselch>;
impl CselchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cselch {
        match self.bits {
            false => Cselch::_0,
            true => Cselch::_1,
        }
    }
    #[doc = "Counter clear is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cselch::_0
    }
    #[doc = "Counter clear is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cselch::_1
    }
}
#[doc = "Field `CSELCH` writer - ELCH Event Source Counter Clear Enable"]
pub type CselchW<'a, REG> = crate::BitWriter<'a, REG, Cselch>;
impl<'a, REG> CselchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cselch::_0)
    }
    #[doc = "Counter clear is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cselch::_1)
    }
}
#[doc = "Software Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclr {
    #[doc = "0: Counter clear is disable by the GTCLR register"]
    _0 = 0,
    #[doc = "1: Counter clear is enable by the GTCLR register"]
    _1 = 1,
}
impl From<Cclr> for bool {
    #[inline(always)]
    fn from(variant: Cclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR` reader - Software Source Counter Clear Enable"]
pub type CclrR = crate::BitReader<Cclr>;
impl CclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cclr {
        match self.bits {
            false => Cclr::_0,
            true => Cclr::_1,
        }
    }
    #[doc = "Counter clear is disable by the GTCLR register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cclr::_0
    }
    #[doc = "Counter clear is enable by the GTCLR register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cclr::_1
    }
}
#[doc = "Field `CCLR` writer - Software Source Counter Clear Enable"]
pub type CclrW<'a, REG> = crate::BitWriter<'a, REG, Cclr>;
impl<'a, REG> CclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable by the GTCLR register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr::_0)
    }
    #[doc = "Counter clear is enable by the GTCLR register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgar(&self) -> CsgtrgarR {
        CsgtrgarR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgaf(&self) -> CsgtrgafR {
        CsgtrgafR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgbr(&self) -> CsgtrgbrR {
        CsgtrgbrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgbf(&self) -> CsgtrgbfR {
        CsgtrgbfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscarbl(&self) -> CscarblR {
        CscarblR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscarbh(&self) -> CscarbhR {
        CscarbhR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscafbl(&self) -> CscafblR {
        CscafblR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscafbh(&self) -> CscafbhR {
        CscafbhR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbral(&self) -> CscbralR {
        CscbralR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbrah(&self) -> CscbrahR {
        CscbrahR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbfal(&self) -> CscbfalR {
        CscbfalR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbfah(&self) -> CscbfahR {
        CscbfahR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselca(&self) -> CselcaR {
        CselcaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcb(&self) -> CselcbR {
        CselcbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcc(&self) -> CselccR {
        CselccR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcd(&self) -> CselcdR {
        CselcdR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselce(&self) -> CselceR {
        CselceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcf(&self) -> CselcfR {
        CselcfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcg(&self) -> CselcgR {
        CselcgR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ELCH Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselch(&self) -> CselchR {
        CselchR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - Software Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cclr(&self) -> CclrR {
        CclrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTCSR")
            .field("cclr", &self.cclr())
            .field("cselch", &self.cselch())
            .field("cselcg", &self.cselcg())
            .field("cselcf", &self.cselcf())
            .field("cselce", &self.cselce())
            .field("cselcd", &self.cselcd())
            .field("cselcc", &self.cselcc())
            .field("cselcb", &self.cselcb())
            .field("cselca", &self.cselca())
            .field("cscbfah", &self.cscbfah())
            .field("cscbfal", &self.cscbfal())
            .field("cscbrah", &self.cscbrah())
            .field("cscbral", &self.cscbral())
            .field("cscafbh", &self.cscafbh())
            .field("cscafbl", &self.cscafbl())
            .field("cscarbh", &self.cscarbh())
            .field("cscarbl", &self.cscarbl())
            .field("csgtrgbf", &self.csgtrgbf())
            .field("csgtrgbr", &self.csgtrgbr())
            .field("csgtrgaf", &self.csgtrgaf())
            .field("csgtrgar", &self.csgtrgar())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgar(&mut self) -> CsgtrgarW<GtcsrSpec> {
        CsgtrgarW::new(self, 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgaf(&mut self) -> CsgtrgafW<GtcsrSpec> {
        CsgtrgafW::new(self, 1)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgbr(&mut self) -> CsgtrgbrW<GtcsrSpec> {
        CsgtrgbrW::new(self, 2)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgbf(&mut self) -> CsgtrgbfW<GtcsrSpec> {
        CsgtrgbfW::new(self, 3)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscarbl(&mut self) -> CscarblW<GtcsrSpec> {
        CscarblW::new(self, 8)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscarbh(&mut self) -> CscarbhW<GtcsrSpec> {
        CscarbhW::new(self, 9)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscafbl(&mut self) -> CscafblW<GtcsrSpec> {
        CscafblW::new(self, 10)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscafbh(&mut self) -> CscafbhW<GtcsrSpec> {
        CscafbhW::new(self, 11)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbral(&mut self) -> CscbralW<GtcsrSpec> {
        CscbralW::new(self, 12)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbrah(&mut self) -> CscbrahW<GtcsrSpec> {
        CscbrahW::new(self, 13)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbfal(&mut self) -> CscbfalW<GtcsrSpec> {
        CscbfalW::new(self, 14)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbfah(&mut self) -> CscbfahW<GtcsrSpec> {
        CscbfahW::new(self, 15)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselca(&mut self) -> CselcaW<GtcsrSpec> {
        CselcaW::new(self, 16)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcb(&mut self) -> CselcbW<GtcsrSpec> {
        CselcbW::new(self, 17)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcc(&mut self) -> CselccW<GtcsrSpec> {
        CselccW::new(self, 18)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcd(&mut self) -> CselcdW<GtcsrSpec> {
        CselcdW::new(self, 19)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselce(&mut self) -> CselceW<GtcsrSpec> {
        CselceW::new(self, 20)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcf(&mut self) -> CselcfW<GtcsrSpec> {
        CselcfW::new(self, 21)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcg(&mut self) -> CselcgW<GtcsrSpec> {
        CselcgW::new(self, 22)
    }
    #[doc = "Bit 23 - ELCH Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselch(&mut self) -> CselchW<GtcsrSpec> {
        CselchW::new(self, 23)
    }
    #[doc = "Bit 31 - Software Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cclr(&mut self) -> CclrW<GtcsrSpec> {
        CclrW::new(self, 31)
    }
}
#[doc = "General PWM Timer Clear Source Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtcsrSpec;
impl crate::RegisterSpec for GtcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtcsr::R`](R) reader structure"]
impl crate::Readable for GtcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`gtcsr::W`](W) writer structure"]
impl crate::Writable for GtcsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTCSR to value 0"]
impl crate::Resettable for GtcsrSpec {}
