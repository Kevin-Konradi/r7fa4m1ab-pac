#[doc = "Register `GTICBSR` reader"]
pub type R = crate::R<GticbsrSpec>;
#[doc = "Register `GTICBSR` writer"]
pub type W = crate::W<GticbsrSpec>;
#[doc = "GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsgtrgar {
    #[doc = "0: GTCCRB input capture is disable at the rising edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the rising edge of GTETRGA input"]
    _1 = 1,
}
impl From<Bsgtrgar> for bool {
    #[inline(always)]
    fn from(variant: Bsgtrgar) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSGTRGAR` reader - GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable"]
pub type BsgtrgarR = crate::BitReader<Bsgtrgar>;
impl BsgtrgarR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bsgtrgar {
        match self.bits {
            false => Bsgtrgar::_0,
            true => Bsgtrgar::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bsgtrgar::_0
    }
    #[doc = "GTCCRB input capture is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bsgtrgar::_1
    }
}
#[doc = "Field `BSGTRGAR` writer - GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable"]
pub type BsgtrgarW<'a, REG> = crate::BitWriter<'a, REG, Bsgtrgar>;
impl<'a, REG> BsgtrgarW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bsgtrgar::_0)
    }
    #[doc = "GTCCRB input capture is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bsgtrgar::_1)
    }
}
#[doc = "GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsgtrgaf {
    #[doc = "0: GTCCRB input capture is disable at the falling edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the falling edge of GTETRGA input"]
    _1 = 1,
}
impl From<Bsgtrgaf> for bool {
    #[inline(always)]
    fn from(variant: Bsgtrgaf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSGTRGAF` reader - GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable"]
pub type BsgtrgafR = crate::BitReader<Bsgtrgaf>;
impl BsgtrgafR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bsgtrgaf {
        match self.bits {
            false => Bsgtrgaf::_0,
            true => Bsgtrgaf::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bsgtrgaf::_0
    }
    #[doc = "GTCCRB input capture is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bsgtrgaf::_1
    }
}
#[doc = "Field `BSGTRGAF` writer - GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable"]
pub type BsgtrgafW<'a, REG> = crate::BitWriter<'a, REG, Bsgtrgaf>;
impl<'a, REG> BsgtrgafW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bsgtrgaf::_0)
    }
    #[doc = "GTCCRB input capture is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bsgtrgaf::_1)
    }
}
#[doc = "GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsgtrgbr {
    #[doc = "0: GTCCRB input capture is disable at the rising edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the rising edge of GTETRGB input"]
    _1 = 1,
}
impl From<Bsgtrgbr> for bool {
    #[inline(always)]
    fn from(variant: Bsgtrgbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSGTRGBR` reader - GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable"]
pub type BsgtrgbrR = crate::BitReader<Bsgtrgbr>;
impl BsgtrgbrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bsgtrgbr {
        match self.bits {
            false => Bsgtrgbr::_0,
            true => Bsgtrgbr::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bsgtrgbr::_0
    }
    #[doc = "GTCCRB input capture is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bsgtrgbr::_1
    }
}
#[doc = "Field `BSGTRGBR` writer - GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable"]
pub type BsgtrgbrW<'a, REG> = crate::BitWriter<'a, REG, Bsgtrgbr>;
impl<'a, REG> BsgtrgbrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bsgtrgbr::_0)
    }
    #[doc = "GTCCRB input capture is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bsgtrgbr::_1)
    }
}
#[doc = "GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsgtrgbf {
    #[doc = "0: GTCCRB input capture is disable at the falling edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the falling edge of GTETRGB input"]
    _1 = 1,
}
impl From<Bsgtrgbf> for bool {
    #[inline(always)]
    fn from(variant: Bsgtrgbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSGTRGBF` reader - GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable"]
pub type BsgtrgbfR = crate::BitReader<Bsgtrgbf>;
impl BsgtrgbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bsgtrgbf {
        match self.bits {
            false => Bsgtrgbf::_0,
            true => Bsgtrgbf::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bsgtrgbf::_0
    }
    #[doc = "GTCCRB input capture is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bsgtrgbf::_1
    }
}
#[doc = "Field `BSGTRGBF` writer - GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable"]
pub type BsgtrgbfW<'a, REG> = crate::BitWriter<'a, REG, Bsgtrgbf>;
impl<'a, REG> BsgtrgbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bsgtrgbf::_0)
    }
    #[doc = "GTCCRB input capture is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bsgtrgbf::_1)
    }
}
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bscarbl {
    #[doc = "0: GTCCRB input capture is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<Bscarbl> for bool {
    #[inline(always)]
    fn from(variant: Bscarbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSCARBL` reader - GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRB Input Capture Enable"]
pub type BscarblR = crate::BitReader<Bscarbl>;
impl BscarblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bscarbl {
        match self.bits {
            false => Bscarbl::_0,
            true => Bscarbl::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bscarbl::_0
    }
    #[doc = "GTCCRB input capture is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bscarbl::_1
    }
}
#[doc = "Field `BSCARBL` writer - GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRB Input Capture Enable"]
pub type BscarblW<'a, REG> = crate::BitWriter<'a, REG, Bscarbl>;
impl<'a, REG> BscarblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bscarbl::_0)
    }
    #[doc = "GTCCRB input capture is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bscarbl::_1)
    }
}
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bscarbh {
    #[doc = "0: GTCCRB input capture is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<Bscarbh> for bool {
    #[inline(always)]
    fn from(variant: Bscarbh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSCARBH` reader - GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRB Input Capture Enable"]
pub type BscarbhR = crate::BitReader<Bscarbh>;
impl BscarbhR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bscarbh {
        match self.bits {
            false => Bscarbh::_0,
            true => Bscarbh::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bscarbh::_0
    }
    #[doc = "GTCCRB input capture is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bscarbh::_1
    }
}
#[doc = "Field `BSCARBH` writer - GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRB Input Capture Enable"]
pub type BscarbhW<'a, REG> = crate::BitWriter<'a, REG, Bscarbh>;
impl<'a, REG> BscarbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bscarbh::_0)
    }
    #[doc = "GTCCRB input capture is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bscarbh::_1)
    }
}
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bscafbl {
    #[doc = "0: GTCCRB input capture is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<Bscafbl> for bool {
    #[inline(always)]
    fn from(variant: Bscafbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSCAFBL` reader - GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRB Input Capture Enable"]
pub type BscafblR = crate::BitReader<Bscafbl>;
impl BscafblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bscafbl {
        match self.bits {
            false => Bscafbl::_0,
            true => Bscafbl::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bscafbl::_0
    }
    #[doc = "GTCCRB input capture is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bscafbl::_1
    }
}
#[doc = "Field `BSCAFBL` writer - GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRB Input Capture Enable"]
pub type BscafblW<'a, REG> = crate::BitWriter<'a, REG, Bscafbl>;
impl<'a, REG> BscafblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bscafbl::_0)
    }
    #[doc = "GTCCRB input capture is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bscafbl::_1)
    }
}
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bscafbh {
    #[doc = "0: GTCCRB input capture is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<Bscafbh> for bool {
    #[inline(always)]
    fn from(variant: Bscafbh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSCAFBH` reader - GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRB Input Capture Enable"]
pub type BscafbhR = crate::BitReader<Bscafbh>;
impl BscafbhR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bscafbh {
        match self.bits {
            false => Bscafbh::_0,
            true => Bscafbh::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bscafbh::_0
    }
    #[doc = "GTCCRB input capture is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bscafbh::_1
    }
}
#[doc = "Field `BSCAFBH` writer - GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRB Input Capture Enable"]
pub type BscafbhW<'a, REG> = crate::BitWriter<'a, REG, Bscafbh>;
impl<'a, REG> BscafbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bscafbh::_0)
    }
    #[doc = "GTCCRB input capture is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bscafbh::_1)
    }
}
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bscbral {
    #[doc = "0: GTCCRB input capture is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<Bscbral> for bool {
    #[inline(always)]
    fn from(variant: Bscbral) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSCBRAL` reader - GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRB Input Capture Enable"]
pub type BscbralR = crate::BitReader<Bscbral>;
impl BscbralR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bscbral {
        match self.bits {
            false => Bscbral::_0,
            true => Bscbral::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bscbral::_0
    }
    #[doc = "GTCCRB input capture is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bscbral::_1
    }
}
#[doc = "Field `BSCBRAL` writer - GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRB Input Capture Enable"]
pub type BscbralW<'a, REG> = crate::BitWriter<'a, REG, Bscbral>;
impl<'a, REG> BscbralW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bscbral::_0)
    }
    #[doc = "GTCCRB input capture is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bscbral::_1)
    }
}
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bscbrah {
    #[doc = "0: GTCCRB input capture is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<Bscbrah> for bool {
    #[inline(always)]
    fn from(variant: Bscbrah) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSCBRAH` reader - GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRB Input Capture Enable"]
pub type BscbrahR = crate::BitReader<Bscbrah>;
impl BscbrahR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bscbrah {
        match self.bits {
            false => Bscbrah::_0,
            true => Bscbrah::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bscbrah::_0
    }
    #[doc = "GTCCRB input capture is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bscbrah::_1
    }
}
#[doc = "Field `BSCBRAH` writer - GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRB Input Capture Enable"]
pub type BscbrahW<'a, REG> = crate::BitWriter<'a, REG, Bscbrah>;
impl<'a, REG> BscbrahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bscbrah::_0)
    }
    #[doc = "GTCCRB input capture is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bscbrah::_1)
    }
}
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bscbfal {
    #[doc = "0: GTCCRB input capture is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<Bscbfal> for bool {
    #[inline(always)]
    fn from(variant: Bscbfal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSCBFAL` reader - GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRB Input Capture Enable"]
pub type BscbfalR = crate::BitReader<Bscbfal>;
impl BscbfalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bscbfal {
        match self.bits {
            false => Bscbfal::_0,
            true => Bscbfal::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bscbfal::_0
    }
    #[doc = "GTCCRB input capture is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bscbfal::_1
    }
}
#[doc = "Field `BSCBFAL` writer - GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRB Input Capture Enable"]
pub type BscbfalW<'a, REG> = crate::BitWriter<'a, REG, Bscbfal>;
impl<'a, REG> BscbfalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bscbfal::_0)
    }
    #[doc = "GTCCRB input capture is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bscbfal::_1)
    }
}
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bscbfah {
    #[doc = "0: GTCCRB input capture is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<Bscbfah> for bool {
    #[inline(always)]
    fn from(variant: Bscbfah) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSCBFAH` reader - GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRB Input Capture Enable"]
pub type BscbfahR = crate::BitReader<Bscbfah>;
impl BscbfahR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bscbfah {
        match self.bits {
            false => Bscbfah::_0,
            true => Bscbfah::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bscbfah::_0
    }
    #[doc = "GTCCRB input capture is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bscbfah::_1
    }
}
#[doc = "Field `BSCBFAH` writer - GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRB Input Capture Enable"]
pub type BscbfahW<'a, REG> = crate::BitWriter<'a, REG, Bscbfah>;
impl<'a, REG> BscbfahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bscbfah::_0)
    }
    #[doc = "GTCCRB input capture is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bscbfah::_1)
    }
}
#[doc = "ELC_GPTA Event Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bselca {
    #[doc = "0: GTCCRB input capture is disable at the ELC_GPTA input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the ELC_GPTA input"]
    _1 = 1,
}
impl From<Bselca> for bool {
    #[inline(always)]
    fn from(variant: Bselca) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSELCA` reader - ELC_GPTA Event Source GTCCRB Input Capture Enable"]
pub type BselcaR = crate::BitReader<Bselca>;
impl BselcaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bselca {
        match self.bits {
            false => Bselca::_0,
            true => Bselca::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bselca::_0
    }
    #[doc = "GTCCRB input capture is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bselca::_1
    }
}
#[doc = "Field `BSELCA` writer - ELC_GPTA Event Source GTCCRB Input Capture Enable"]
pub type BselcaW<'a, REG> = crate::BitWriter<'a, REG, Bselca>;
impl<'a, REG> BselcaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bselca::_0)
    }
    #[doc = "GTCCRB input capture is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bselca::_1)
    }
}
#[doc = "ELC_GPTB Event Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bselcb {
    #[doc = "0: GTCCRB input capture is disable at the ELC_GPTB input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the ELC_GPTB input"]
    _1 = 1,
}
impl From<Bselcb> for bool {
    #[inline(always)]
    fn from(variant: Bselcb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSELCB` reader - ELC_GPTB Event Source GTCCRB Input Capture Enable"]
pub type BselcbR = crate::BitReader<Bselcb>;
impl BselcbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bselcb {
        match self.bits {
            false => Bselcb::_0,
            true => Bselcb::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bselcb::_0
    }
    #[doc = "GTCCRB input capture is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bselcb::_1
    }
}
#[doc = "Field `BSELCB` writer - ELC_GPTB Event Source GTCCRB Input Capture Enable"]
pub type BselcbW<'a, REG> = crate::BitWriter<'a, REG, Bselcb>;
impl<'a, REG> BselcbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bselcb::_0)
    }
    #[doc = "GTCCRB input capture is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bselcb::_1)
    }
}
#[doc = "ELC_GPTC Event Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bselcc {
    #[doc = "0: GTCCRB input capture is disable at the ELC_GPTC input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the ELC_GPTC input"]
    _1 = 1,
}
impl From<Bselcc> for bool {
    #[inline(always)]
    fn from(variant: Bselcc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSELCC` reader - ELC_GPTC Event Source GTCCRB Input Capture Enable"]
pub type BselccR = crate::BitReader<Bselcc>;
impl BselccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bselcc {
        match self.bits {
            false => Bselcc::_0,
            true => Bselcc::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bselcc::_0
    }
    #[doc = "GTCCRB input capture is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bselcc::_1
    }
}
#[doc = "Field `BSELCC` writer - ELC_GPTC Event Source GTCCRB Input Capture Enable"]
pub type BselccW<'a, REG> = crate::BitWriter<'a, REG, Bselcc>;
impl<'a, REG> BselccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bselcc::_0)
    }
    #[doc = "GTCCRB input capture is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bselcc::_1)
    }
}
#[doc = "ELC_GPTD Event Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bselcd {
    #[doc = "0: GTCCRB input capture is disable at the ELC_GPTD input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the ELC_GPTD input"]
    _1 = 1,
}
impl From<Bselcd> for bool {
    #[inline(always)]
    fn from(variant: Bselcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSELCD` reader - ELC_GPTD Event Source GTCCRB Input Capture Enable"]
pub type BselcdR = crate::BitReader<Bselcd>;
impl BselcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bselcd {
        match self.bits {
            false => Bselcd::_0,
            true => Bselcd::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bselcd::_0
    }
    #[doc = "GTCCRB input capture is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bselcd::_1
    }
}
#[doc = "Field `BSELCD` writer - ELC_GPTD Event Source GTCCRB Input Capture Enable"]
pub type BselcdW<'a, REG> = crate::BitWriter<'a, REG, Bselcd>;
impl<'a, REG> BselcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bselcd::_0)
    }
    #[doc = "GTCCRB input capture is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bselcd::_1)
    }
}
#[doc = "ELC_GPTE Event Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bselce {
    #[doc = "0: GTCCRB input capture is disable at the ELC_GPTE input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the ELC_GPTE input"]
    _1 = 1,
}
impl From<Bselce> for bool {
    #[inline(always)]
    fn from(variant: Bselce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSELCE` reader - ELC_GPTE Event Source GTCCRB Input Capture Enable"]
pub type BselceR = crate::BitReader<Bselce>;
impl BselceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bselce {
        match self.bits {
            false => Bselce::_0,
            true => Bselce::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bselce::_0
    }
    #[doc = "GTCCRB input capture is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bselce::_1
    }
}
#[doc = "Field `BSELCE` writer - ELC_GPTE Event Source GTCCRB Input Capture Enable"]
pub type BselceW<'a, REG> = crate::BitWriter<'a, REG, Bselce>;
impl<'a, REG> BselceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bselce::_0)
    }
    #[doc = "GTCCRB input capture is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bselce::_1)
    }
}
#[doc = "ELC_GPTF Event Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bselcf {
    #[doc = "0: GTCCRB input capture is disable at the ELC_GPTF input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the ELC_GPTF input"]
    _1 = 1,
}
impl From<Bselcf> for bool {
    #[inline(always)]
    fn from(variant: Bselcf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSELCF` reader - ELC_GPTF Event Source GTCCRB Input Capture Enable"]
pub type BselcfR = crate::BitReader<Bselcf>;
impl BselcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bselcf {
        match self.bits {
            false => Bselcf::_0,
            true => Bselcf::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bselcf::_0
    }
    #[doc = "GTCCRB input capture is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bselcf::_1
    }
}
#[doc = "Field `BSELCF` writer - ELC_GPTF Event Source GTCCRB Input Capture Enable"]
pub type BselcfW<'a, REG> = crate::BitWriter<'a, REG, Bselcf>;
impl<'a, REG> BselcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bselcf::_0)
    }
    #[doc = "GTCCRB input capture is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bselcf::_1)
    }
}
#[doc = "ELC_GPTG Event Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bselcg {
    #[doc = "0: GTCCRB input capture is disable at the ELC_GPTG input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the ELC_GPTG input"]
    _1 = 1,
}
impl From<Bselcg> for bool {
    #[inline(always)]
    fn from(variant: Bselcg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSELCG` reader - ELC_GPTG Event Source GTCCRB Input Capture Enable"]
pub type BselcgR = crate::BitReader<Bselcg>;
impl BselcgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bselcg {
        match self.bits {
            false => Bselcg::_0,
            true => Bselcg::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bselcg::_0
    }
    #[doc = "GTCCRB input capture is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bselcg::_1
    }
}
#[doc = "Field `BSELCG` writer - ELC_GPTG Event Source GTCCRB Input Capture Enable"]
pub type BselcgW<'a, REG> = crate::BitWriter<'a, REG, Bselcg>;
impl<'a, REG> BselcgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bselcg::_0)
    }
    #[doc = "GTCCRB input capture is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bselcg::_1)
    }
}
#[doc = "ELCH Event Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bselch {
    #[doc = "0: GTCCRB input capture is disable at the ELC_GPTH input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture is enable at the ELC_GPTH input"]
    _1 = 1,
}
impl From<Bselch> for bool {
    #[inline(always)]
    fn from(variant: Bselch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSELCH` reader - ELCH Event Source GTCCRB Input Capture Enable"]
pub type BselchR = crate::BitReader<Bselch>;
impl BselchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bselch {
        match self.bits {
            false => Bselch::_0,
            true => Bselch::_1,
        }
    }
    #[doc = "GTCCRB input capture is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bselch::_0
    }
    #[doc = "GTCCRB input capture is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bselch::_1
    }
}
#[doc = "Field `BSELCH` writer - ELCH Event Source GTCCRB Input Capture Enable"]
pub type BselchW<'a, REG> = crate::BitWriter<'a, REG, Bselch>;
impl<'a, REG> BselchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB input capture is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bselch::_0)
    }
    #[doc = "GTCCRB input capture is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bselch::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgar(&self) -> BsgtrgarR {
        BsgtrgarR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgaf(&self) -> BsgtrgafR {
        BsgtrgafR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgbr(&self) -> BsgtrgbrR {
        BsgtrgbrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgbf(&self) -> BsgtrgbfR {
        BsgtrgbfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscarbl(&self) -> BscarblR {
        BscarblR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscarbh(&self) -> BscarbhR {
        BscarbhR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscafbl(&self) -> BscafblR {
        BscafblR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscafbh(&self) -> BscafbhR {
        BscafbhR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbral(&self) -> BscbralR {
        BscbralR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbrah(&self) -> BscbrahR {
        BscbrahR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbfal(&self) -> BscbfalR {
        BscbfalR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbfah(&self) -> BscbfahR {
        BscbfahR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselca(&self) -> BselcaR {
        BselcaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcb(&self) -> BselcbR {
        BselcbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcc(&self) -> BselccR {
        BselccR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcd(&self) -> BselcdR {
        BselcdR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselce(&self) -> BselceR {
        BselceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcf(&self) -> BselcfR {
        BselcfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcg(&self) -> BselcgR {
        BselcgR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ELCH Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselch(&self) -> BselchR {
        BselchR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTICBSR")
            .field("bselch", &self.bselch())
            .field("bselcg", &self.bselcg())
            .field("bselcf", &self.bselcf())
            .field("bselce", &self.bselce())
            .field("bselcd", &self.bselcd())
            .field("bselcc", &self.bselcc())
            .field("bselcb", &self.bselcb())
            .field("bselca", &self.bselca())
            .field("bscbfah", &self.bscbfah())
            .field("bscbfal", &self.bscbfal())
            .field("bscbrah", &self.bscbrah())
            .field("bscbral", &self.bscbral())
            .field("bscafbh", &self.bscafbh())
            .field("bscafbl", &self.bscafbl())
            .field("bscarbh", &self.bscarbh())
            .field("bscarbl", &self.bscarbl())
            .field("bsgtrgbf", &self.bsgtrgbf())
            .field("bsgtrgbr", &self.bsgtrgbr())
            .field("bsgtrgaf", &self.bsgtrgaf())
            .field("bsgtrgar", &self.bsgtrgar())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgar(&mut self) -> BsgtrgarW<GticbsrSpec> {
        BsgtrgarW::new(self, 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgaf(&mut self) -> BsgtrgafW<GticbsrSpec> {
        BsgtrgafW::new(self, 1)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgbr(&mut self) -> BsgtrgbrW<GticbsrSpec> {
        BsgtrgbrW::new(self, 2)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgbf(&mut self) -> BsgtrgbfW<GticbsrSpec> {
        BsgtrgbfW::new(self, 3)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscarbl(&mut self) -> BscarblW<GticbsrSpec> {
        BscarblW::new(self, 8)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscarbh(&mut self) -> BscarbhW<GticbsrSpec> {
        BscarbhW::new(self, 9)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscafbl(&mut self) -> BscafblW<GticbsrSpec> {
        BscafblW::new(self, 10)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscafbh(&mut self) -> BscafbhW<GticbsrSpec> {
        BscafbhW::new(self, 11)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbral(&mut self) -> BscbralW<GticbsrSpec> {
        BscbralW::new(self, 12)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbrah(&mut self) -> BscbrahW<GticbsrSpec> {
        BscbrahW::new(self, 13)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbfal(&mut self) -> BscbfalW<GticbsrSpec> {
        BscbfalW::new(self, 14)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbfah(&mut self) -> BscbfahW<GticbsrSpec> {
        BscbfahW::new(self, 15)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselca(&mut self) -> BselcaW<GticbsrSpec> {
        BselcaW::new(self, 16)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcb(&mut self) -> BselcbW<GticbsrSpec> {
        BselcbW::new(self, 17)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcc(&mut self) -> BselccW<GticbsrSpec> {
        BselccW::new(self, 18)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcd(&mut self) -> BselcdW<GticbsrSpec> {
        BselcdW::new(self, 19)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselce(&mut self) -> BselceW<GticbsrSpec> {
        BselceW::new(self, 20)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcf(&mut self) -> BselcfW<GticbsrSpec> {
        BselcfW::new(self, 21)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcg(&mut self) -> BselcgW<GticbsrSpec> {
        BselcgW::new(self, 22)
    }
    #[doc = "Bit 23 - ELCH Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselch(&mut self) -> BselchW<GticbsrSpec> {
        BselchW::new(self, 23)
    }
}
#[doc = "General PWM Timer Input Capture Source Select Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`gticbsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gticbsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GticbsrSpec;
impl crate::RegisterSpec for GticbsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gticbsr::R`](R) reader structure"]
impl crate::Readable for GticbsrSpec {}
#[doc = "`write(|w| ..)` method takes [`gticbsr::W`](W) writer structure"]
impl crate::Writable for GticbsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTICBSR to value 0"]
impl crate::Resettable for GticbsrSpec {}
