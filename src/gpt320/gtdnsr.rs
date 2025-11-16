#[doc = "Register `GTDNSR` reader"]
pub type R = crate::R<GtdnsrSpec>;
#[doc = "Register `GTDNSR` writer"]
pub type W = crate::W<GtdnsrSpec>;
#[doc = "GTETRGA Pin Rising Input Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsgtrgar {
    #[doc = "0: Counter count down is disable at the rising edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the rising edge of GTETRGA input"]
    _1 = 1,
}
impl From<Dsgtrgar> for bool {
    #[inline(always)]
    fn from(variant: Dsgtrgar) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Count Down Enable"]
pub type DsgtrgarR = crate::BitReader<Dsgtrgar>;
impl DsgtrgarR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsgtrgar {
        match self.bits {
            false => Dsgtrgar::_0,
            true => Dsgtrgar::_1,
        }
    }
    #[doc = "Counter count down is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dsgtrgar::_0
    }
    #[doc = "Counter count down is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dsgtrgar::_1
    }
}
#[doc = "Field `DSGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Count Down Enable"]
pub type DsgtrgarW<'a, REG> = crate::BitWriter<'a, REG, Dsgtrgar>;
impl<'a, REG> DsgtrgarW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgar::_0)
    }
    #[doc = "Counter count down is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgar::_1)
    }
}
#[doc = "GTETRGA Pin Falling Input Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsgtrgaf {
    #[doc = "0: Counter count down is disable at the falling edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the falling edge of GTETRGA input"]
    _1 = 1,
}
impl From<Dsgtrgaf> for bool {
    #[inline(always)]
    fn from(variant: Dsgtrgaf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Count Down Enable"]
pub type DsgtrgafR = crate::BitReader<Dsgtrgaf>;
impl DsgtrgafR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsgtrgaf {
        match self.bits {
            false => Dsgtrgaf::_0,
            true => Dsgtrgaf::_1,
        }
    }
    #[doc = "Counter count down is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dsgtrgaf::_0
    }
    #[doc = "Counter count down is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dsgtrgaf::_1
    }
}
#[doc = "Field `DSGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Count Down Enable"]
pub type DsgtrgafW<'a, REG> = crate::BitWriter<'a, REG, Dsgtrgaf>;
impl<'a, REG> DsgtrgafW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgaf::_0)
    }
    #[doc = "Counter count down is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgaf::_1)
    }
}
#[doc = "GTETRGB Pin Rising Input Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsgtrgbr {
    #[doc = "0: Counter count down is disable at the rising edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the rising edge of GTETRGB input"]
    _1 = 1,
}
impl From<Dsgtrgbr> for bool {
    #[inline(always)]
    fn from(variant: Dsgtrgbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Count Down Enable"]
pub type DsgtrgbrR = crate::BitReader<Dsgtrgbr>;
impl DsgtrgbrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsgtrgbr {
        match self.bits {
            false => Dsgtrgbr::_0,
            true => Dsgtrgbr::_1,
        }
    }
    #[doc = "Counter count down is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dsgtrgbr::_0
    }
    #[doc = "Counter count down is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dsgtrgbr::_1
    }
}
#[doc = "Field `DSGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Count Down Enable"]
pub type DsgtrgbrW<'a, REG> = crate::BitWriter<'a, REG, Dsgtrgbr>;
impl<'a, REG> DsgtrgbrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgbr::_0)
    }
    #[doc = "Counter count down is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgbr::_1)
    }
}
#[doc = "GTETRGB Pin Falling Input Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsgtrgbf {
    #[doc = "0: Counter count down is disable at the falling edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the falling edge of GTETRGB input"]
    _1 = 1,
}
impl From<Dsgtrgbf> for bool {
    #[inline(always)]
    fn from(variant: Dsgtrgbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Count Down Enable"]
pub type DsgtrgbfR = crate::BitReader<Dsgtrgbf>;
impl DsgtrgbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsgtrgbf {
        match self.bits {
            false => Dsgtrgbf::_0,
            true => Dsgtrgbf::_1,
        }
    }
    #[doc = "Counter count down is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dsgtrgbf::_0
    }
    #[doc = "Counter count down is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dsgtrgbf::_1
    }
}
#[doc = "Field `DSGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Count Down Enable"]
pub type DsgtrgbfW<'a, REG> = crate::BitWriter<'a, REG, Dsgtrgbf>;
impl<'a, REG> DsgtrgbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgbf::_0)
    }
    #[doc = "Counter count down is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgbf::_1)
    }
}
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dscarbl {
    #[doc = "0: Counter count down is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<Dscarbl> for bool {
    #[inline(always)]
    fn from(variant: Dscarbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSCARBL` reader - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Down Enable"]
pub type DscarblR = crate::BitReader<Dscarbl>;
impl DscarblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dscarbl {
        match self.bits {
            false => Dscarbl::_0,
            true => Dscarbl::_1,
        }
    }
    #[doc = "Counter count down is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dscarbl::_0
    }
    #[doc = "Counter count down is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dscarbl::_1
    }
}
#[doc = "Field `DSCARBL` writer - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Down Enable"]
pub type DscarblW<'a, REG> = crate::BitWriter<'a, REG, Dscarbl>;
impl<'a, REG> DscarblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dscarbl::_0)
    }
    #[doc = "Counter count down is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dscarbl::_1)
    }
}
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dscarbh {
    #[doc = "0: Counter count down is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<Dscarbh> for bool {
    #[inline(always)]
    fn from(variant: Dscarbh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSCARBH` reader - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Down Enable"]
pub type DscarbhR = crate::BitReader<Dscarbh>;
impl DscarbhR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dscarbh {
        match self.bits {
            false => Dscarbh::_0,
            true => Dscarbh::_1,
        }
    }
    #[doc = "Counter count down is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dscarbh::_0
    }
    #[doc = "Counter count down is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dscarbh::_1
    }
}
#[doc = "Field `DSCARBH` writer - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Down Enable"]
pub type DscarbhW<'a, REG> = crate::BitWriter<'a, REG, Dscarbh>;
impl<'a, REG> DscarbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dscarbh::_0)
    }
    #[doc = "Counter count down is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dscarbh::_1)
    }
}
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dscafbl {
    #[doc = "0: Counter count down is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<Dscafbl> for bool {
    #[inline(always)]
    fn from(variant: Dscafbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSCAFBL` reader - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Down Enable"]
pub type DscafblR = crate::BitReader<Dscafbl>;
impl DscafblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dscafbl {
        match self.bits {
            false => Dscafbl::_0,
            true => Dscafbl::_1,
        }
    }
    #[doc = "Counter count down is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dscafbl::_0
    }
    #[doc = "Counter count down is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dscafbl::_1
    }
}
#[doc = "Field `DSCAFBL` writer - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Down Enable"]
pub type DscafblW<'a, REG> = crate::BitWriter<'a, REG, Dscafbl>;
impl<'a, REG> DscafblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dscafbl::_0)
    }
    #[doc = "Counter count down is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dscafbl::_1)
    }
}
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dscafbh {
    #[doc = "0: Counter count down is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<Dscafbh> for bool {
    #[inline(always)]
    fn from(variant: Dscafbh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSCAFBH` reader - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Down Enable"]
pub type DscafbhR = crate::BitReader<Dscafbh>;
impl DscafbhR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dscafbh {
        match self.bits {
            false => Dscafbh::_0,
            true => Dscafbh::_1,
        }
    }
    #[doc = "Counter count down is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dscafbh::_0
    }
    #[doc = "Counter count down is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dscafbh::_1
    }
}
#[doc = "Field `DSCAFBH` writer - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Down Enable"]
pub type DscafbhW<'a, REG> = crate::BitWriter<'a, REG, Dscafbh>;
impl<'a, REG> DscafbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dscafbh::_0)
    }
    #[doc = "Counter count down is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dscafbh::_1)
    }
}
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dscbral {
    #[doc = "0: Counter count down is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<Dscbral> for bool {
    #[inline(always)]
    fn from(variant: Dscbral) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSCBRAL` reader - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Down Enable"]
pub type DscbralR = crate::BitReader<Dscbral>;
impl DscbralR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dscbral {
        match self.bits {
            false => Dscbral::_0,
            true => Dscbral::_1,
        }
    }
    #[doc = "Counter count down is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dscbral::_0
    }
    #[doc = "Counter count down is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dscbral::_1
    }
}
#[doc = "Field `DSCBRAL` writer - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Down Enable"]
pub type DscbralW<'a, REG> = crate::BitWriter<'a, REG, Dscbral>;
impl<'a, REG> DscbralW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dscbral::_0)
    }
    #[doc = "Counter count down is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dscbral::_1)
    }
}
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dscbrah {
    #[doc = "0: Counter count down is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<Dscbrah> for bool {
    #[inline(always)]
    fn from(variant: Dscbrah) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSCBRAH` reader - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Down Enable"]
pub type DscbrahR = crate::BitReader<Dscbrah>;
impl DscbrahR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dscbrah {
        match self.bits {
            false => Dscbrah::_0,
            true => Dscbrah::_1,
        }
    }
    #[doc = "Counter count down is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dscbrah::_0
    }
    #[doc = "Counter count down is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dscbrah::_1
    }
}
#[doc = "Field `DSCBRAH` writer - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Down Enable"]
pub type DscbrahW<'a, REG> = crate::BitWriter<'a, REG, Dscbrah>;
impl<'a, REG> DscbrahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dscbrah::_0)
    }
    #[doc = "Counter count down is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dscbrah::_1)
    }
}
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dscbfal {
    #[doc = "0: Counter count down is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<Dscbfal> for bool {
    #[inline(always)]
    fn from(variant: Dscbfal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSCBFAL` reader - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Down Enable"]
pub type DscbfalR = crate::BitReader<Dscbfal>;
impl DscbfalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dscbfal {
        match self.bits {
            false => Dscbfal::_0,
            true => Dscbfal::_1,
        }
    }
    #[doc = "Counter count down is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dscbfal::_0
    }
    #[doc = "Counter count down is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dscbfal::_1
    }
}
#[doc = "Field `DSCBFAL` writer - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Down Enable"]
pub type DscbfalW<'a, REG> = crate::BitWriter<'a, REG, Dscbfal>;
impl<'a, REG> DscbfalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dscbfal::_0)
    }
    #[doc = "Counter count down is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dscbfal::_1)
    }
}
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dscbfah {
    #[doc = "0: Counter count down is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<Dscbfah> for bool {
    #[inline(always)]
    fn from(variant: Dscbfah) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSCBFAH` reader - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Down Enable"]
pub type DscbfahR = crate::BitReader<Dscbfah>;
impl DscbfahR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dscbfah {
        match self.bits {
            false => Dscbfah::_0,
            true => Dscbfah::_1,
        }
    }
    #[doc = "Counter count down is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dscbfah::_0
    }
    #[doc = "Counter count down is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dscbfah::_1
    }
}
#[doc = "Field `DSCBFAH` writer - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Down Enable"]
pub type DscbfahW<'a, REG> = crate::BitWriter<'a, REG, Dscbfah>;
impl<'a, REG> DscbfahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dscbfah::_0)
    }
    #[doc = "Counter count down is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dscbfah::_1)
    }
}
#[doc = "ELC_GPTA Event Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dselca {
    #[doc = "0: Counter count down is disable at the ELC_GPTA input"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the ELC_GPTA input"]
    _1 = 1,
}
impl From<Dselca> for bool {
    #[inline(always)]
    fn from(variant: Dselca) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSELCA` reader - ELC_GPTA Event Source Counter Count Down Enable"]
pub type DselcaR = crate::BitReader<Dselca>;
impl DselcaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dselca {
        match self.bits {
            false => Dselca::_0,
            true => Dselca::_1,
        }
    }
    #[doc = "Counter count down is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dselca::_0
    }
    #[doc = "Counter count down is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dselca::_1
    }
}
#[doc = "Field `DSELCA` writer - ELC_GPTA Event Source Counter Count Down Enable"]
pub type DselcaW<'a, REG> = crate::BitWriter<'a, REG, Dselca>;
impl<'a, REG> DselcaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dselca::_0)
    }
    #[doc = "Counter count down is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dselca::_1)
    }
}
#[doc = "ELC_GPTB Event Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dselcb {
    #[doc = "0: Counter count down is disable at the ELC_GPTB input"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the ELC_GPTB input"]
    _1 = 1,
}
impl From<Dselcb> for bool {
    #[inline(always)]
    fn from(variant: Dselcb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSELCB` reader - ELC_GPTB Event Source Counter Count Down Enable"]
pub type DselcbR = crate::BitReader<Dselcb>;
impl DselcbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dselcb {
        match self.bits {
            false => Dselcb::_0,
            true => Dselcb::_1,
        }
    }
    #[doc = "Counter count down is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dselcb::_0
    }
    #[doc = "Counter count down is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dselcb::_1
    }
}
#[doc = "Field `DSELCB` writer - ELC_GPTB Event Source Counter Count Down Enable"]
pub type DselcbW<'a, REG> = crate::BitWriter<'a, REG, Dselcb>;
impl<'a, REG> DselcbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcb::_0)
    }
    #[doc = "Counter count down is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcb::_1)
    }
}
#[doc = "ELC_GPTC Event Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dselcc {
    #[doc = "0: Counter count down is disable at the ELC_GPTC input"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the ELC_GPTC input"]
    _1 = 1,
}
impl From<Dselcc> for bool {
    #[inline(always)]
    fn from(variant: Dselcc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSELCC` reader - ELC_GPTC Event Source Counter Count Down Enable"]
pub type DselccR = crate::BitReader<Dselcc>;
impl DselccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dselcc {
        match self.bits {
            false => Dselcc::_0,
            true => Dselcc::_1,
        }
    }
    #[doc = "Counter count down is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dselcc::_0
    }
    #[doc = "Counter count down is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dselcc::_1
    }
}
#[doc = "Field `DSELCC` writer - ELC_GPTC Event Source Counter Count Down Enable"]
pub type DselccW<'a, REG> = crate::BitWriter<'a, REG, Dselcc>;
impl<'a, REG> DselccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcc::_0)
    }
    #[doc = "Counter count down is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcc::_1)
    }
}
#[doc = "ELC_GPTD Event Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dselcd {
    #[doc = "0: Counter count down is disable at the ELC_GPTD input"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the ELC_GPTD input"]
    _1 = 1,
}
impl From<Dselcd> for bool {
    #[inline(always)]
    fn from(variant: Dselcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSELCD` reader - ELC_GPTD Event Source Counter Count Down Enable"]
pub type DselcdR = crate::BitReader<Dselcd>;
impl DselcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dselcd {
        match self.bits {
            false => Dselcd::_0,
            true => Dselcd::_1,
        }
    }
    #[doc = "Counter count down is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dselcd::_0
    }
    #[doc = "Counter count down is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dselcd::_1
    }
}
#[doc = "Field `DSELCD` writer - ELC_GPTD Event Source Counter Count Down Enable"]
pub type DselcdW<'a, REG> = crate::BitWriter<'a, REG, Dselcd>;
impl<'a, REG> DselcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcd::_0)
    }
    #[doc = "Counter count down is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcd::_1)
    }
}
#[doc = "ELC_GPTE Event Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dselce {
    #[doc = "0: Counter count down is disable at the ELC_GPTE input"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the ELC_GPTE input"]
    _1 = 1,
}
impl From<Dselce> for bool {
    #[inline(always)]
    fn from(variant: Dselce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSELCE` reader - ELC_GPTE Event Source Counter Count Down Enable"]
pub type DselceR = crate::BitReader<Dselce>;
impl DselceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dselce {
        match self.bits {
            false => Dselce::_0,
            true => Dselce::_1,
        }
    }
    #[doc = "Counter count down is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dselce::_0
    }
    #[doc = "Counter count down is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dselce::_1
    }
}
#[doc = "Field `DSELCE` writer - ELC_GPTE Event Source Counter Count Down Enable"]
pub type DselceW<'a, REG> = crate::BitWriter<'a, REG, Dselce>;
impl<'a, REG> DselceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dselce::_0)
    }
    #[doc = "Counter count down is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dselce::_1)
    }
}
#[doc = "ELC_GPTF Event Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dselcf {
    #[doc = "0: Counter count down is disable at the ELC_GPTF input"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the ELC_GPTF input"]
    _1 = 1,
}
impl From<Dselcf> for bool {
    #[inline(always)]
    fn from(variant: Dselcf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSELCF` reader - ELC_GPTF Event Source Counter Count Down Enable"]
pub type DselcfR = crate::BitReader<Dselcf>;
impl DselcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dselcf {
        match self.bits {
            false => Dselcf::_0,
            true => Dselcf::_1,
        }
    }
    #[doc = "Counter count down is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dselcf::_0
    }
    #[doc = "Counter count down is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dselcf::_1
    }
}
#[doc = "Field `DSELCF` writer - ELC_GPTF Event Source Counter Count Down Enable"]
pub type DselcfW<'a, REG> = crate::BitWriter<'a, REG, Dselcf>;
impl<'a, REG> DselcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcf::_0)
    }
    #[doc = "Counter count down is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcf::_1)
    }
}
#[doc = "ELC_GPTG Event Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dselcg {
    #[doc = "0: Counter count down is disable at the ELC_GPTG input"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the ELC_GPTG input"]
    _1 = 1,
}
impl From<Dselcg> for bool {
    #[inline(always)]
    fn from(variant: Dselcg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSELCG` reader - ELC_GPTG Event Source Counter Count Down Enable"]
pub type DselcgR = crate::BitReader<Dselcg>;
impl DselcgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dselcg {
        match self.bits {
            false => Dselcg::_0,
            true => Dselcg::_1,
        }
    }
    #[doc = "Counter count down is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dselcg::_0
    }
    #[doc = "Counter count down is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dselcg::_1
    }
}
#[doc = "Field `DSELCG` writer - ELC_GPTG Event Source Counter Count Down Enable"]
pub type DselcgW<'a, REG> = crate::BitWriter<'a, REG, Dselcg>;
impl<'a, REG> DselcgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcg::_0)
    }
    #[doc = "Counter count down is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcg::_1)
    }
}
#[doc = "ELCH Event Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dselch {
    #[doc = "0: Counter count down is disable at the ELC_GPTH input"]
    _0 = 0,
    #[doc = "1: Counter count down is enable at the ELC_GPTH input"]
    _1 = 1,
}
impl From<Dselch> for bool {
    #[inline(always)]
    fn from(variant: Dselch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSELCH` reader - ELCH Event Source Counter Count Down Enable"]
pub type DselchR = crate::BitReader<Dselch>;
impl DselchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dselch {
        match self.bits {
            false => Dselch::_0,
            true => Dselch::_1,
        }
    }
    #[doc = "Counter count down is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dselch::_0
    }
    #[doc = "Counter count down is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dselch::_1
    }
}
#[doc = "Field `DSELCH` writer - ELCH Event Source Counter Count Down Enable"]
pub type DselchW<'a, REG> = crate::BitWriter<'a, REG, Dselch>;
impl<'a, REG> DselchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count down is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dselch::_0)
    }
    #[doc = "Counter count down is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dselch::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgar(&self) -> DsgtrgarR {
        DsgtrgarR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgaf(&self) -> DsgtrgafR {
        DsgtrgafR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgbr(&self) -> DsgtrgbrR {
        DsgtrgbrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgbf(&self) -> DsgtrgbfR {
        DsgtrgbfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscarbl(&self) -> DscarblR {
        DscarblR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscarbh(&self) -> DscarbhR {
        DscarbhR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscafbl(&self) -> DscafblR {
        DscafblR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscafbh(&self) -> DscafbhR {
        DscafbhR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbral(&self) -> DscbralR {
        DscbralR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbrah(&self) -> DscbrahR {
        DscbrahR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbfal(&self) -> DscbfalR {
        DscbfalR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbfah(&self) -> DscbfahR {
        DscbfahR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselca(&self) -> DselcaR {
        DselcaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcb(&self) -> DselcbR {
        DselcbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcc(&self) -> DselccR {
        DselccR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcd(&self) -> DselcdR {
        DselcdR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselce(&self) -> DselceR {
        DselceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcf(&self) -> DselcfR {
        DselcfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcg(&self) -> DselcgR {
        DselcgR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ELCH Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselch(&self) -> DselchR {
        DselchR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTDNSR")
            .field("dselch", &self.dselch())
            .field("dselcg", &self.dselcg())
            .field("dselcf", &self.dselcf())
            .field("dselce", &self.dselce())
            .field("dselcd", &self.dselcd())
            .field("dselcc", &self.dselcc())
            .field("dselcb", &self.dselcb())
            .field("dselca", &self.dselca())
            .field("dscbfah", &self.dscbfah())
            .field("dscbfal", &self.dscbfal())
            .field("dscbrah", &self.dscbrah())
            .field("dscbral", &self.dscbral())
            .field("dscafbh", &self.dscafbh())
            .field("dscafbl", &self.dscafbl())
            .field("dscarbh", &self.dscarbh())
            .field("dscarbl", &self.dscarbl())
            .field("dsgtrgbf", &self.dsgtrgbf())
            .field("dsgtrgbr", &self.dsgtrgbr())
            .field("dsgtrgaf", &self.dsgtrgaf())
            .field("dsgtrgar", &self.dsgtrgar())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgar(&mut self) -> DsgtrgarW<GtdnsrSpec> {
        DsgtrgarW::new(self, 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgaf(&mut self) -> DsgtrgafW<GtdnsrSpec> {
        DsgtrgafW::new(self, 1)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgbr(&mut self) -> DsgtrgbrW<GtdnsrSpec> {
        DsgtrgbrW::new(self, 2)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgbf(&mut self) -> DsgtrgbfW<GtdnsrSpec> {
        DsgtrgbfW::new(self, 3)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscarbl(&mut self) -> DscarblW<GtdnsrSpec> {
        DscarblW::new(self, 8)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscarbh(&mut self) -> DscarbhW<GtdnsrSpec> {
        DscarbhW::new(self, 9)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscafbl(&mut self) -> DscafblW<GtdnsrSpec> {
        DscafblW::new(self, 10)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscafbh(&mut self) -> DscafbhW<GtdnsrSpec> {
        DscafbhW::new(self, 11)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbral(&mut self) -> DscbralW<GtdnsrSpec> {
        DscbralW::new(self, 12)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbrah(&mut self) -> DscbrahW<GtdnsrSpec> {
        DscbrahW::new(self, 13)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbfal(&mut self) -> DscbfalW<GtdnsrSpec> {
        DscbfalW::new(self, 14)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbfah(&mut self) -> DscbfahW<GtdnsrSpec> {
        DscbfahW::new(self, 15)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselca(&mut self) -> DselcaW<GtdnsrSpec> {
        DselcaW::new(self, 16)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcb(&mut self) -> DselcbW<GtdnsrSpec> {
        DselcbW::new(self, 17)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcc(&mut self) -> DselccW<GtdnsrSpec> {
        DselccW::new(self, 18)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcd(&mut self) -> DselcdW<GtdnsrSpec> {
        DselcdW::new(self, 19)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselce(&mut self) -> DselceW<GtdnsrSpec> {
        DselceW::new(self, 20)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcf(&mut self) -> DselcfW<GtdnsrSpec> {
        DselcfW::new(self, 21)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcg(&mut self) -> DselcgW<GtdnsrSpec> {
        DselcgW::new(self, 22)
    }
    #[doc = "Bit 23 - ELCH Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselch(&mut self) -> DselchW<GtdnsrSpec> {
        DselchW::new(self, 23)
    }
}
#[doc = "General PWM Timer Down Count Source Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtdnsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdnsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtdnsrSpec;
impl crate::RegisterSpec for GtdnsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtdnsr::R`](R) reader structure"]
impl crate::Readable for GtdnsrSpec {}
#[doc = "`write(|w| ..)` method takes [`gtdnsr::W`](W) writer structure"]
impl crate::Writable for GtdnsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTDNSR to value 0"]
impl crate::Resettable for GtdnsrSpec {}
