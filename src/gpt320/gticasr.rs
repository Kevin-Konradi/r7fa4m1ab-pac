#[doc = "Register `GTICASR` reader"]
pub type R = crate::R<GticasrSpec>;
#[doc = "Register `GTICASR` writer"]
pub type W = crate::W<GticasrSpec>;
#[doc = "GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asgtrgar {
    #[doc = "0: GTCCRA input capture is disable at the rising edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the rising edge of GTETRGA input"]
    _1 = 1,
}
impl From<Asgtrgar> for bool {
    #[inline(always)]
    fn from(variant: Asgtrgar) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASGTRGAR` reader - GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable"]
pub type AsgtrgarR = crate::BitReader<Asgtrgar>;
impl AsgtrgarR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asgtrgar {
        match self.bits {
            false => Asgtrgar::_0,
            true => Asgtrgar::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Asgtrgar::_0
    }
    #[doc = "GTCCRA input capture is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Asgtrgar::_1
    }
}
#[doc = "Field `ASGTRGAR` writer - GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable"]
pub type AsgtrgarW<'a, REG> = crate::BitWriter<'a, REG, Asgtrgar>;
impl<'a, REG> AsgtrgarW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgar::_0)
    }
    #[doc = "GTCCRA input capture is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgar::_1)
    }
}
#[doc = "GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asgtrgaf {
    #[doc = "0: GTCCRA input capture is disable at the falling edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the falling edge of GTETRGA input"]
    _1 = 1,
}
impl From<Asgtrgaf> for bool {
    #[inline(always)]
    fn from(variant: Asgtrgaf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASGTRGAF` reader - GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable"]
pub type AsgtrgafR = crate::BitReader<Asgtrgaf>;
impl AsgtrgafR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asgtrgaf {
        match self.bits {
            false => Asgtrgaf::_0,
            true => Asgtrgaf::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Asgtrgaf::_0
    }
    #[doc = "GTCCRA input capture is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Asgtrgaf::_1
    }
}
#[doc = "Field `ASGTRGAF` writer - GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable"]
pub type AsgtrgafW<'a, REG> = crate::BitWriter<'a, REG, Asgtrgaf>;
impl<'a, REG> AsgtrgafW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgaf::_0)
    }
    #[doc = "GTCCRA input capture is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgaf::_1)
    }
}
#[doc = "GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asgtrgbr {
    #[doc = "0: GTCCRA input capture is disable at the rising edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the rising edge of GTETRGB input"]
    _1 = 1,
}
impl From<Asgtrgbr> for bool {
    #[inline(always)]
    fn from(variant: Asgtrgbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASGTRGBR` reader - GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable"]
pub type AsgtrgbrR = crate::BitReader<Asgtrgbr>;
impl AsgtrgbrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asgtrgbr {
        match self.bits {
            false => Asgtrgbr::_0,
            true => Asgtrgbr::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Asgtrgbr::_0
    }
    #[doc = "GTCCRA input capture is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Asgtrgbr::_1
    }
}
#[doc = "Field `ASGTRGBR` writer - GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable"]
pub type AsgtrgbrW<'a, REG> = crate::BitWriter<'a, REG, Asgtrgbr>;
impl<'a, REG> AsgtrgbrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgbr::_0)
    }
    #[doc = "GTCCRA input capture is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgbr::_1)
    }
}
#[doc = "GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asgtrgbf {
    #[doc = "0: GTCCRA input capture is disable at the falling edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the falling edge of GTETRGB input"]
    _1 = 1,
}
impl From<Asgtrgbf> for bool {
    #[inline(always)]
    fn from(variant: Asgtrgbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASGTRGBF` reader - GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable"]
pub type AsgtrgbfR = crate::BitReader<Asgtrgbf>;
impl AsgtrgbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asgtrgbf {
        match self.bits {
            false => Asgtrgbf::_0,
            true => Asgtrgbf::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Asgtrgbf::_0
    }
    #[doc = "GTCCRA input capture is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Asgtrgbf::_1
    }
}
#[doc = "Field `ASGTRGBF` writer - GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable"]
pub type AsgtrgbfW<'a, REG> = crate::BitWriter<'a, REG, Asgtrgbf>;
impl<'a, REG> AsgtrgbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgbf::_0)
    }
    #[doc = "GTCCRA input capture is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgbf::_1)
    }
}
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ascarbl {
    #[doc = "0: GTCCRA input capture is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<Ascarbl> for bool {
    #[inline(always)]
    fn from(variant: Ascarbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASCARBL` reader - GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRA Input Capture Enable"]
pub type AscarblR = crate::BitReader<Ascarbl>;
impl AscarblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ascarbl {
        match self.bits {
            false => Ascarbl::_0,
            true => Ascarbl::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ascarbl::_0
    }
    #[doc = "GTCCRA input capture is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ascarbl::_1
    }
}
#[doc = "Field `ASCARBL` writer - GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRA Input Capture Enable"]
pub type AscarblW<'a, REG> = crate::BitWriter<'a, REG, Ascarbl>;
impl<'a, REG> AscarblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ascarbl::_0)
    }
    #[doc = "GTCCRA input capture is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ascarbl::_1)
    }
}
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ascarbh {
    #[doc = "0: GTCCRA input capture is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<Ascarbh> for bool {
    #[inline(always)]
    fn from(variant: Ascarbh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASCARBH` reader - GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRA Input Capture Enable"]
pub type AscarbhR = crate::BitReader<Ascarbh>;
impl AscarbhR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ascarbh {
        match self.bits {
            false => Ascarbh::_0,
            true => Ascarbh::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ascarbh::_0
    }
    #[doc = "GTCCRA input capture is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ascarbh::_1
    }
}
#[doc = "Field `ASCARBH` writer - GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRA Input Capture Enable"]
pub type AscarbhW<'a, REG> = crate::BitWriter<'a, REG, Ascarbh>;
impl<'a, REG> AscarbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ascarbh::_0)
    }
    #[doc = "GTCCRA input capture is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ascarbh::_1)
    }
}
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ascafbl {
    #[doc = "0: GTCCRA input capture is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<Ascafbl> for bool {
    #[inline(always)]
    fn from(variant: Ascafbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASCAFBL` reader - GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRA Input Capture Enable"]
pub type AscafblR = crate::BitReader<Ascafbl>;
impl AscafblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ascafbl {
        match self.bits {
            false => Ascafbl::_0,
            true => Ascafbl::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ascafbl::_0
    }
    #[doc = "GTCCRA input capture is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ascafbl::_1
    }
}
#[doc = "Field `ASCAFBL` writer - GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRA Input Capture Enable"]
pub type AscafblW<'a, REG> = crate::BitWriter<'a, REG, Ascafbl>;
impl<'a, REG> AscafblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ascafbl::_0)
    }
    #[doc = "GTCCRA input capture is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ascafbl::_1)
    }
}
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ascafbh {
    #[doc = "0: GTCCRA input capture is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<Ascafbh> for bool {
    #[inline(always)]
    fn from(variant: Ascafbh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASCAFBH` reader - GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRA Input Capture Enable"]
pub type AscafbhR = crate::BitReader<Ascafbh>;
impl AscafbhR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ascafbh {
        match self.bits {
            false => Ascafbh::_0,
            true => Ascafbh::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ascafbh::_0
    }
    #[doc = "GTCCRA input capture is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ascafbh::_1
    }
}
#[doc = "Field `ASCAFBH` writer - GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRA Input Capture Enable"]
pub type AscafbhW<'a, REG> = crate::BitWriter<'a, REG, Ascafbh>;
impl<'a, REG> AscafbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ascafbh::_0)
    }
    #[doc = "GTCCRA input capture is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ascafbh::_1)
    }
}
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ascbral {
    #[doc = "0: GTCCRA input capture is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<Ascbral> for bool {
    #[inline(always)]
    fn from(variant: Ascbral) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASCBRAL` reader - GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRA Input Capture Enable"]
pub type AscbralR = crate::BitReader<Ascbral>;
impl AscbralR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ascbral {
        match self.bits {
            false => Ascbral::_0,
            true => Ascbral::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ascbral::_0
    }
    #[doc = "GTCCRA input capture is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ascbral::_1
    }
}
#[doc = "Field `ASCBRAL` writer - GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRA Input Capture Enable"]
pub type AscbralW<'a, REG> = crate::BitWriter<'a, REG, Ascbral>;
impl<'a, REG> AscbralW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ascbral::_0)
    }
    #[doc = "GTCCRA input capture is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ascbral::_1)
    }
}
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ascbrah {
    #[doc = "0: GTCCRA input capture is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<Ascbrah> for bool {
    #[inline(always)]
    fn from(variant: Ascbrah) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASCBRAH` reader - GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRA Input Capture Enable"]
pub type AscbrahR = crate::BitReader<Ascbrah>;
impl AscbrahR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ascbrah {
        match self.bits {
            false => Ascbrah::_0,
            true => Ascbrah::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ascbrah::_0
    }
    #[doc = "GTCCRA input capture is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ascbrah::_1
    }
}
#[doc = "Field `ASCBRAH` writer - GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRA Input Capture Enable"]
pub type AscbrahW<'a, REG> = crate::BitWriter<'a, REG, Ascbrah>;
impl<'a, REG> AscbrahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ascbrah::_0)
    }
    #[doc = "GTCCRA input capture is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ascbrah::_1)
    }
}
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ascbfal {
    #[doc = "0: GTCCRA input capture is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<Ascbfal> for bool {
    #[inline(always)]
    fn from(variant: Ascbfal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASCBFAL` reader - GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRA Input Capture Enable"]
pub type AscbfalR = crate::BitReader<Ascbfal>;
impl AscbfalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ascbfal {
        match self.bits {
            false => Ascbfal::_0,
            true => Ascbfal::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ascbfal::_0
    }
    #[doc = "GTCCRA input capture is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ascbfal::_1
    }
}
#[doc = "Field `ASCBFAL` writer - GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRA Input Capture Enable"]
pub type AscbfalW<'a, REG> = crate::BitWriter<'a, REG, Ascbfal>;
impl<'a, REG> AscbfalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ascbfal::_0)
    }
    #[doc = "GTCCRA input capture is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ascbfal::_1)
    }
}
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ascbfah {
    #[doc = "0: GTCCRA input capture is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<Ascbfah> for bool {
    #[inline(always)]
    fn from(variant: Ascbfah) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASCBFAH` reader - GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRA Input Capture Enable"]
pub type AscbfahR = crate::BitReader<Ascbfah>;
impl AscbfahR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ascbfah {
        match self.bits {
            false => Ascbfah::_0,
            true => Ascbfah::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ascbfah::_0
    }
    #[doc = "GTCCRA input capture is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ascbfah::_1
    }
}
#[doc = "Field `ASCBFAH` writer - GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRA Input Capture Enable"]
pub type AscbfahW<'a, REG> = crate::BitWriter<'a, REG, Ascbfah>;
impl<'a, REG> AscbfahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ascbfah::_0)
    }
    #[doc = "GTCCRA input capture is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ascbfah::_1)
    }
}
#[doc = "ELC_GPTA Event Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aselca {
    #[doc = "0: GTCCRA input capture is disable at the ELC_GPTA input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the ELC_GPTA input"]
    _1 = 1,
}
impl From<Aselca> for bool {
    #[inline(always)]
    fn from(variant: Aselca) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASELCA` reader - ELC_GPTA Event Source GTCCRA Input Capture Enable"]
pub type AselcaR = crate::BitReader<Aselca>;
impl AselcaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aselca {
        match self.bits {
            false => Aselca::_0,
            true => Aselca::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aselca::_0
    }
    #[doc = "GTCCRA input capture is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aselca::_1
    }
}
#[doc = "Field `ASELCA` writer - ELC_GPTA Event Source GTCCRA Input Capture Enable"]
pub type AselcaW<'a, REG> = crate::BitWriter<'a, REG, Aselca>;
impl<'a, REG> AselcaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aselca::_0)
    }
    #[doc = "GTCCRA input capture is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aselca::_1)
    }
}
#[doc = "ELC_GPTB Event Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aselcb {
    #[doc = "0: GTCCRA input capture is disable at the ELC_GPTB input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the ELC_GPTB input"]
    _1 = 1,
}
impl From<Aselcb> for bool {
    #[inline(always)]
    fn from(variant: Aselcb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASELCB` reader - ELC_GPTB Event Source GTCCRA Input Capture Enable"]
pub type AselcbR = crate::BitReader<Aselcb>;
impl AselcbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aselcb {
        match self.bits {
            false => Aselcb::_0,
            true => Aselcb::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aselcb::_0
    }
    #[doc = "GTCCRA input capture is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aselcb::_1
    }
}
#[doc = "Field `ASELCB` writer - ELC_GPTB Event Source GTCCRA Input Capture Enable"]
pub type AselcbW<'a, REG> = crate::BitWriter<'a, REG, Aselcb>;
impl<'a, REG> AselcbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcb::_0)
    }
    #[doc = "GTCCRA input capture is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcb::_1)
    }
}
#[doc = "ELC_GPTC Event Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aselcc {
    #[doc = "0: GTCCRA input capture is disable at the ELC_GPTC input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the ELC_GPTC input"]
    _1 = 1,
}
impl From<Aselcc> for bool {
    #[inline(always)]
    fn from(variant: Aselcc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASELCC` reader - ELC_GPTC Event Source GTCCRA Input Capture Enable"]
pub type AselccR = crate::BitReader<Aselcc>;
impl AselccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aselcc {
        match self.bits {
            false => Aselcc::_0,
            true => Aselcc::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aselcc::_0
    }
    #[doc = "GTCCRA input capture is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aselcc::_1
    }
}
#[doc = "Field `ASELCC` writer - ELC_GPTC Event Source GTCCRA Input Capture Enable"]
pub type AselccW<'a, REG> = crate::BitWriter<'a, REG, Aselcc>;
impl<'a, REG> AselccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcc::_0)
    }
    #[doc = "GTCCRA input capture is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcc::_1)
    }
}
#[doc = "ELC_GPTD Event Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aselcd {
    #[doc = "0: GTCCRA input capture is disable at the ELC_GPTD input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the ELC_GPTD input"]
    _1 = 1,
}
impl From<Aselcd> for bool {
    #[inline(always)]
    fn from(variant: Aselcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASELCD` reader - ELC_GPTD Event Source GTCCRA Input Capture Enable"]
pub type AselcdR = crate::BitReader<Aselcd>;
impl AselcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aselcd {
        match self.bits {
            false => Aselcd::_0,
            true => Aselcd::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aselcd::_0
    }
    #[doc = "GTCCRA input capture is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aselcd::_1
    }
}
#[doc = "Field `ASELCD` writer - ELC_GPTD Event Source GTCCRA Input Capture Enable"]
pub type AselcdW<'a, REG> = crate::BitWriter<'a, REG, Aselcd>;
impl<'a, REG> AselcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcd::_0)
    }
    #[doc = "GTCCRA input capture is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcd::_1)
    }
}
#[doc = "ELC_GPTE Event Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aselce {
    #[doc = "0: GTCCRA input capture is disable at the ELC_GPTE input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the ELC_GPTE input"]
    _1 = 1,
}
impl From<Aselce> for bool {
    #[inline(always)]
    fn from(variant: Aselce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASELCE` reader - ELC_GPTE Event Source GTCCRA Input Capture Enable"]
pub type AselceR = crate::BitReader<Aselce>;
impl AselceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aselce {
        match self.bits {
            false => Aselce::_0,
            true => Aselce::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aselce::_0
    }
    #[doc = "GTCCRA input capture is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aselce::_1
    }
}
#[doc = "Field `ASELCE` writer - ELC_GPTE Event Source GTCCRA Input Capture Enable"]
pub type AselceW<'a, REG> = crate::BitWriter<'a, REG, Aselce>;
impl<'a, REG> AselceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aselce::_0)
    }
    #[doc = "GTCCRA input capture is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aselce::_1)
    }
}
#[doc = "ELC_GPTF Event Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aselcf {
    #[doc = "0: GTCCRA input capture is disable at the ELC_GPTF input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the ELC_GPTF input"]
    _1 = 1,
}
impl From<Aselcf> for bool {
    #[inline(always)]
    fn from(variant: Aselcf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASELCF` reader - ELC_GPTF Event Source GTCCRA Input Capture Enable"]
pub type AselcfR = crate::BitReader<Aselcf>;
impl AselcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aselcf {
        match self.bits {
            false => Aselcf::_0,
            true => Aselcf::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aselcf::_0
    }
    #[doc = "GTCCRA input capture is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aselcf::_1
    }
}
#[doc = "Field `ASELCF` writer - ELC_GPTF Event Source GTCCRA Input Capture Enable"]
pub type AselcfW<'a, REG> = crate::BitWriter<'a, REG, Aselcf>;
impl<'a, REG> AselcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcf::_0)
    }
    #[doc = "GTCCRA input capture is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcf::_1)
    }
}
#[doc = "ELC_GPTG Event Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aselcg {
    #[doc = "0: GTCCRA input capture is disable at the ELC_GPTG input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the ELC_GPTG input"]
    _1 = 1,
}
impl From<Aselcg> for bool {
    #[inline(always)]
    fn from(variant: Aselcg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASELCG` reader - ELC_GPTG Event Source GTCCRA Input Capture Enable"]
pub type AselcgR = crate::BitReader<Aselcg>;
impl AselcgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aselcg {
        match self.bits {
            false => Aselcg::_0,
            true => Aselcg::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aselcg::_0
    }
    #[doc = "GTCCRA input capture is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aselcg::_1
    }
}
#[doc = "Field `ASELCG` writer - ELC_GPTG Event Source GTCCRA Input Capture Enable"]
pub type AselcgW<'a, REG> = crate::BitWriter<'a, REG, Aselcg>;
impl<'a, REG> AselcgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcg::_0)
    }
    #[doc = "GTCCRA input capture is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcg::_1)
    }
}
#[doc = "ELCH Event Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aselch {
    #[doc = "0: GTCCRA input capture is disable at the ELC_GPTH input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture is enable at the ELC_GPTH input"]
    _1 = 1,
}
impl From<Aselch> for bool {
    #[inline(always)]
    fn from(variant: Aselch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASELCH` reader - ELCH Event Source GTCCRA Input Capture Enable"]
pub type AselchR = crate::BitReader<Aselch>;
impl AselchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aselch {
        match self.bits {
            false => Aselch::_0,
            true => Aselch::_1,
        }
    }
    #[doc = "GTCCRA input capture is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aselch::_0
    }
    #[doc = "GTCCRA input capture is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aselch::_1
    }
}
#[doc = "Field `ASELCH` writer - ELCH Event Source GTCCRA Input Capture Enable"]
pub type AselchW<'a, REG> = crate::BitWriter<'a, REG, Aselch>;
impl<'a, REG> AselchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRA input capture is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aselch::_0)
    }
    #[doc = "GTCCRA input capture is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aselch::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgar(&self) -> AsgtrgarR {
        AsgtrgarR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgaf(&self) -> AsgtrgafR {
        AsgtrgafR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgbr(&self) -> AsgtrgbrR {
        AsgtrgbrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgbf(&self) -> AsgtrgbfR {
        AsgtrgbfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascarbl(&self) -> AscarblR {
        AscarblR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascarbh(&self) -> AscarbhR {
        AscarbhR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascafbl(&self) -> AscafblR {
        AscafblR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascafbh(&self) -> AscafbhR {
        AscafbhR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbral(&self) -> AscbralR {
        AscbralR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbrah(&self) -> AscbrahR {
        AscbrahR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbfal(&self) -> AscbfalR {
        AscbfalR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbfah(&self) -> AscbfahR {
        AscbfahR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselca(&self) -> AselcaR {
        AselcaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcb(&self) -> AselcbR {
        AselcbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcc(&self) -> AselccR {
        AselccR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcd(&self) -> AselcdR {
        AselcdR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselce(&self) -> AselceR {
        AselceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcf(&self) -> AselcfR {
        AselcfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcg(&self) -> AselcgR {
        AselcgR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ELCH Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselch(&self) -> AselchR {
        AselchR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTICASR")
            .field("aselch", &self.aselch())
            .field("aselcg", &self.aselcg())
            .field("aselcf", &self.aselcf())
            .field("aselce", &self.aselce())
            .field("aselcd", &self.aselcd())
            .field("aselcc", &self.aselcc())
            .field("aselcb", &self.aselcb())
            .field("aselca", &self.aselca())
            .field("ascbfah", &self.ascbfah())
            .field("ascbfal", &self.ascbfal())
            .field("ascbrah", &self.ascbrah())
            .field("ascbral", &self.ascbral())
            .field("ascafbh", &self.ascafbh())
            .field("ascafbl", &self.ascafbl())
            .field("ascarbh", &self.ascarbh())
            .field("ascarbl", &self.ascarbl())
            .field("asgtrgbf", &self.asgtrgbf())
            .field("asgtrgbr", &self.asgtrgbr())
            .field("asgtrgaf", &self.asgtrgaf())
            .field("asgtrgar", &self.asgtrgar())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgar(&mut self) -> AsgtrgarW<GticasrSpec> {
        AsgtrgarW::new(self, 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgaf(&mut self) -> AsgtrgafW<GticasrSpec> {
        AsgtrgafW::new(self, 1)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgbr(&mut self) -> AsgtrgbrW<GticasrSpec> {
        AsgtrgbrW::new(self, 2)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgbf(&mut self) -> AsgtrgbfW<GticasrSpec> {
        AsgtrgbfW::new(self, 3)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascarbl(&mut self) -> AscarblW<GticasrSpec> {
        AscarblW::new(self, 8)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascarbh(&mut self) -> AscarbhW<GticasrSpec> {
        AscarbhW::new(self, 9)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascafbl(&mut self) -> AscafblW<GticasrSpec> {
        AscafblW::new(self, 10)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascafbh(&mut self) -> AscafbhW<GticasrSpec> {
        AscafbhW::new(self, 11)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbral(&mut self) -> AscbralW<GticasrSpec> {
        AscbralW::new(self, 12)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbrah(&mut self) -> AscbrahW<GticasrSpec> {
        AscbrahW::new(self, 13)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbfal(&mut self) -> AscbfalW<GticasrSpec> {
        AscbfalW::new(self, 14)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbfah(&mut self) -> AscbfahW<GticasrSpec> {
        AscbfahW::new(self, 15)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselca(&mut self) -> AselcaW<GticasrSpec> {
        AselcaW::new(self, 16)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcb(&mut self) -> AselcbW<GticasrSpec> {
        AselcbW::new(self, 17)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcc(&mut self) -> AselccW<GticasrSpec> {
        AselccW::new(self, 18)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcd(&mut self) -> AselcdW<GticasrSpec> {
        AselcdW::new(self, 19)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselce(&mut self) -> AselceW<GticasrSpec> {
        AselceW::new(self, 20)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcf(&mut self) -> AselcfW<GticasrSpec> {
        AselcfW::new(self, 21)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcg(&mut self) -> AselcgW<GticasrSpec> {
        AselcgW::new(self, 22)
    }
    #[doc = "Bit 23 - ELCH Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselch(&mut self) -> AselchW<GticasrSpec> {
        AselchW::new(self, 23)
    }
}
#[doc = "General PWM Timer Input Capture Source Select Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`gticasr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gticasr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GticasrSpec;
impl crate::RegisterSpec for GticasrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gticasr::R`](R) reader structure"]
impl crate::Readable for GticasrSpec {}
#[doc = "`write(|w| ..)` method takes [`gticasr::W`](W) writer structure"]
impl crate::Writable for GticasrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTICASR to value 0"]
impl crate::Resettable for GticasrSpec {}
