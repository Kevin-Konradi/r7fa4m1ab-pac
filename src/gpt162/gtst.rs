#[doc = "Register `GTST` reader"]
pub type R = crate::R<GtstSpec>;
#[doc = "Register `GTST` writer"]
pub type W = crate::W<GtstSpec>;
#[doc = "Input Capture/Compare Match Flag A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcfa {
    #[doc = "0: No input capture/compare match of GTCCRA is generated."]
    _0 = 0,
    #[doc = "1: An input capture/compare match of GTCCRA is generated."]
    _1 = 1,
}
impl From<Tcfa> for bool {
    #[inline(always)]
    fn from(variant: Tcfa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCFA` reader - Input Capture/Compare Match Flag A"]
pub type TcfaR = crate::BitReader<Tcfa>;
impl TcfaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcfa {
        match self.bits {
            false => Tcfa::_0,
            true => Tcfa::_1,
        }
    }
    #[doc = "No input capture/compare match of GTCCRA is generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcfa::_0
    }
    #[doc = "An input capture/compare match of GTCCRA is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcfa::_1
    }
}
#[doc = "Field `TCFA` writer - Input Capture/Compare Match Flag A"]
pub type TcfaW<'a, REG> = crate::BitWriter<'a, REG, Tcfa>;
impl<'a, REG> TcfaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No input capture/compare match of GTCCRA is generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfa::_0)
    }
    #[doc = "An input capture/compare match of GTCCRA is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfa::_1)
    }
}
#[doc = "Input Capture/Compare Match Flag B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcfb {
    #[doc = "0: No input capture/compare match of GTCCRB is generated."]
    _0 = 0,
    #[doc = "1: An input capture/compare match of GTCCRB is generated."]
    _1 = 1,
}
impl From<Tcfb> for bool {
    #[inline(always)]
    fn from(variant: Tcfb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCFB` reader - Input Capture/Compare Match Flag B"]
pub type TcfbR = crate::BitReader<Tcfb>;
impl TcfbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcfb {
        match self.bits {
            false => Tcfb::_0,
            true => Tcfb::_1,
        }
    }
    #[doc = "No input capture/compare match of GTCCRB is generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcfb::_0
    }
    #[doc = "An input capture/compare match of GTCCRB is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcfb::_1
    }
}
#[doc = "Field `TCFB` writer - Input Capture/Compare Match Flag B"]
pub type TcfbW<'a, REG> = crate::BitWriter<'a, REG, Tcfb>;
impl<'a, REG> TcfbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No input capture/compare match of GTCCRB is generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfb::_0)
    }
    #[doc = "An input capture/compare match of GTCCRB is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfb::_1)
    }
}
#[doc = "Input Compare Match Flag C\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcfc {
    #[doc = "0: No compare match of GTCCRC is generated."]
    _0 = 0,
    #[doc = "1: A compare match of GTCCRC is generated."]
    _1 = 1,
}
impl From<Tcfc> for bool {
    #[inline(always)]
    fn from(variant: Tcfc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCFC` reader - Input Compare Match Flag C"]
pub type TcfcR = crate::BitReader<Tcfc>;
impl TcfcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcfc {
        match self.bits {
            false => Tcfc::_0,
            true => Tcfc::_1,
        }
    }
    #[doc = "No compare match of GTCCRC is generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcfc::_0
    }
    #[doc = "A compare match of GTCCRC is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcfc::_1
    }
}
#[doc = "Field `TCFC` writer - Input Compare Match Flag C"]
pub type TcfcW<'a, REG> = crate::BitWriter<'a, REG, Tcfc>;
impl<'a, REG> TcfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No compare match of GTCCRC is generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfc::_0)
    }
    #[doc = "A compare match of GTCCRC is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfc::_1)
    }
}
#[doc = "Input Compare Match Flag D\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcfd {
    #[doc = "0: No compare match of GTCCRD is generated."]
    _0 = 0,
    #[doc = "1: A compare match of GTCCRD is generated."]
    _1 = 1,
}
impl From<Tcfd> for bool {
    #[inline(always)]
    fn from(variant: Tcfd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCFD` reader - Input Compare Match Flag D"]
pub type TcfdR = crate::BitReader<Tcfd>;
impl TcfdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcfd {
        match self.bits {
            false => Tcfd::_0,
            true => Tcfd::_1,
        }
    }
    #[doc = "No compare match of GTCCRD is generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcfd::_0
    }
    #[doc = "A compare match of GTCCRD is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcfd::_1
    }
}
#[doc = "Field `TCFD` writer - Input Compare Match Flag D"]
pub type TcfdW<'a, REG> = crate::BitWriter<'a, REG, Tcfd>;
impl<'a, REG> TcfdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No compare match of GTCCRD is generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfd::_0)
    }
    #[doc = "A compare match of GTCCRD is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfd::_1)
    }
}
#[doc = "Input Compare Match Flag E\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcfe {
    #[doc = "0: No compare match of GTCCRE is generated."]
    _0 = 0,
    #[doc = "1: A compare match of GTCCRE is generated."]
    _1 = 1,
}
impl From<Tcfe> for bool {
    #[inline(always)]
    fn from(variant: Tcfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCFE` reader - Input Compare Match Flag E"]
pub type TcfeR = crate::BitReader<Tcfe>;
impl TcfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcfe {
        match self.bits {
            false => Tcfe::_0,
            true => Tcfe::_1,
        }
    }
    #[doc = "No compare match of GTCCRE is generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcfe::_0
    }
    #[doc = "A compare match of GTCCRE is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcfe::_1
    }
}
#[doc = "Field `TCFE` writer - Input Compare Match Flag E"]
pub type TcfeW<'a, REG> = crate::BitWriter<'a, REG, Tcfe>;
impl<'a, REG> TcfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No compare match of GTCCRE is generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfe::_0)
    }
    #[doc = "A compare match of GTCCRE is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfe::_1)
    }
}
#[doc = "Input Compare Match Flag F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcff {
    #[doc = "0: No compare match of GTCCRF is generated."]
    _0 = 0,
    #[doc = "1: A compare match of GTCCRF is generated."]
    _1 = 1,
}
impl From<Tcff> for bool {
    #[inline(always)]
    fn from(variant: Tcff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCFF` reader - Input Compare Match Flag F"]
pub type TcffR = crate::BitReader<Tcff>;
impl TcffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcff {
        match self.bits {
            false => Tcff::_0,
            true => Tcff::_1,
        }
    }
    #[doc = "No compare match of GTCCRF is generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcff::_0
    }
    #[doc = "A compare match of GTCCRF is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcff::_1
    }
}
#[doc = "Field `TCFF` writer - Input Compare Match Flag F"]
pub type TcffW<'a, REG> = crate::BitWriter<'a, REG, Tcff>;
impl<'a, REG> TcffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No compare match of GTCCRF is generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcff::_0)
    }
    #[doc = "A compare match of GTCCRF is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcff::_1)
    }
}
#[doc = "Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcfpo {
    #[doc = "0: No overflow (crest) has occurred."]
    _0 = 0,
    #[doc = "1: An overflow (crest) has occurred."]
    _1 = 1,
}
impl From<Tcfpo> for bool {
    #[inline(always)]
    fn from(variant: Tcfpo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCFPO` reader - Overflow Flag"]
pub type TcfpoR = crate::BitReader<Tcfpo>;
impl TcfpoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcfpo {
        match self.bits {
            false => Tcfpo::_0,
            true => Tcfpo::_1,
        }
    }
    #[doc = "No overflow (crest) has occurred."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcfpo::_0
    }
    #[doc = "An overflow (crest) has occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcfpo::_1
    }
}
#[doc = "Field `TCFPO` writer - Overflow Flag"]
pub type TcfpoW<'a, REG> = crate::BitWriter<'a, REG, Tcfpo>;
impl<'a, REG> TcfpoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overflow (crest) has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfpo::_0)
    }
    #[doc = "An overflow (crest) has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfpo::_1)
    }
}
#[doc = "Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcfpu {
    #[doc = "0: No underflow (trough) has occurred."]
    _0 = 0,
    #[doc = "1: An underflow (trough) has occurred."]
    _1 = 1,
}
impl From<Tcfpu> for bool {
    #[inline(always)]
    fn from(variant: Tcfpu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCFPU` reader - Underflow Flag"]
pub type TcfpuR = crate::BitReader<Tcfpu>;
impl TcfpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcfpu {
        match self.bits {
            false => Tcfpu::_0,
            true => Tcfpu::_1,
        }
    }
    #[doc = "No underflow (trough) has occurred."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcfpu::_0
    }
    #[doc = "An underflow (trough) has occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcfpu::_1
    }
}
#[doc = "Field `TCFPU` writer - Underflow Flag"]
pub type TcfpuW<'a, REG> = crate::BitWriter<'a, REG, Tcfpu>;
impl<'a, REG> TcfpuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No underflow (trough) has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfpu::_0)
    }
    #[doc = "An underflow (trough) has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfpu::_1)
    }
}
#[doc = "Count Direction Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tucf {
    #[doc = "0: The GTCNT counter counts downward."]
    _0 = 0,
    #[doc = "1: The GTCNT counter counts upward."]
    _1 = 1,
}
impl From<Tucf> for bool {
    #[inline(always)]
    fn from(variant: Tucf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TUCF` reader - Count Direction Flag"]
pub type TucfR = crate::BitReader<Tucf>;
impl TucfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tucf {
        match self.bits {
            false => Tucf::_0,
            true => Tucf::_1,
        }
    }
    #[doc = "The GTCNT counter counts downward."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tucf::_0
    }
    #[doc = "The GTCNT counter counts upward."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tucf::_1
    }
}
#[doc = "Output Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Odf {
    #[doc = "0: No output disable request is generated."]
    _0 = 0,
    #[doc = "1: An output disable request is generated."]
    _1 = 1,
}
impl From<Odf> for bool {
    #[inline(always)]
    fn from(variant: Odf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODF` reader - Output Disable Flag"]
pub type OdfR = crate::BitReader<Odf>;
impl OdfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Odf {
        match self.bits {
            false => Odf::_0,
            true => Odf::_1,
        }
    }
    #[doc = "No output disable request is generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Odf::_0
    }
    #[doc = "An output disable request is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Odf::_1
    }
}
#[doc = "Same Time Output Level High Disable Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oabhf {
    #[doc = "0: GTIOCA pin and GTIOCB pin don't output 1 at the same time."]
    _0 = 0,
    #[doc = "1: GTIOCA pin and GTIOCB pin output 1 at the same time."]
    _1 = 1,
}
impl From<Oabhf> for bool {
    #[inline(always)]
    fn from(variant: Oabhf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OABHF` reader - Same Time Output Level High Disable Request Enable"]
pub type OabhfR = crate::BitReader<Oabhf>;
impl OabhfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oabhf {
        match self.bits {
            false => Oabhf::_0,
            true => Oabhf::_1,
        }
    }
    #[doc = "GTIOCA pin and GTIOCB pin don't output 1 at the same time."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oabhf::_0
    }
    #[doc = "GTIOCA pin and GTIOCB pin output 1 at the same time."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oabhf::_1
    }
}
#[doc = "Same Time Output Level Low Disable Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oablf {
    #[doc = "0: GTIOCA pin and GTIOCB pin don't output 0 at the same time."]
    _0 = 0,
    #[doc = "1: GTIOCA pin and GTIOCB pin output 0 at the same time."]
    _1 = 1,
}
impl From<Oablf> for bool {
    #[inline(always)]
    fn from(variant: Oablf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OABLF` reader - Same Time Output Level Low Disable Request Enable"]
pub type OablfR = crate::BitReader<Oablf>;
impl OablfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oablf {
        match self.bits {
            false => Oablf::_0,
            true => Oablf::_1,
        }
    }
    #[doc = "GTIOCA pin and GTIOCB pin don't output 0 at the same time."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oablf::_0
    }
    #[doc = "GTIOCA pin and GTIOCB pin output 0 at the same time."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oablf::_1
    }
}
impl R {
    #[doc = "Bit 0 - Input Capture/Compare Match Flag A"]
    #[inline(always)]
    pub fn tcfa(&self) -> TcfaR {
        TcfaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Capture/Compare Match Flag B"]
    #[inline(always)]
    pub fn tcfb(&self) -> TcfbR {
        TcfbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input Compare Match Flag C"]
    #[inline(always)]
    pub fn tcfc(&self) -> TcfcR {
        TcfcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input Compare Match Flag D"]
    #[inline(always)]
    pub fn tcfd(&self) -> TcfdR {
        TcfdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input Compare Match Flag E"]
    #[inline(always)]
    pub fn tcfe(&self) -> TcfeR {
        TcfeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input Compare Match Flag F"]
    #[inline(always)]
    pub fn tcff(&self) -> TcffR {
        TcffR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overflow Flag"]
    #[inline(always)]
    pub fn tcfpo(&self) -> TcfpoR {
        TcfpoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Underflow Flag"]
    #[inline(always)]
    pub fn tcfpu(&self) -> TcfpuR {
        TcfpuR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Count Direction Flag"]
    #[inline(always)]
    pub fn tucf(&self) -> TucfR {
        TucfR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - Output Disable Flag"]
    #[inline(always)]
    pub fn odf(&self) -> OdfR {
        OdfR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 29 - Same Time Output Level High Disable Request Enable"]
    #[inline(always)]
    pub fn oabhf(&self) -> OabhfR {
        OabhfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Same Time Output Level Low Disable Request Enable"]
    #[inline(always)]
    pub fn oablf(&self) -> OablfR {
        OablfR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTST")
            .field("oablf", &self.oablf())
            .field("oabhf", &self.oabhf())
            .field("odf", &self.odf())
            .field("tucf", &self.tucf())
            .field("tcfpu", &self.tcfpu())
            .field("tcfpo", &self.tcfpo())
            .field("tcff", &self.tcff())
            .field("tcfe", &self.tcfe())
            .field("tcfd", &self.tcfd())
            .field("tcfc", &self.tcfc())
            .field("tcfb", &self.tcfb())
            .field("tcfa", &self.tcfa())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Input Capture/Compare Match Flag A"]
    #[inline(always)]
    pub fn tcfa(&mut self) -> TcfaW<GtstSpec> {
        TcfaW::new(self, 0)
    }
    #[doc = "Bit 1 - Input Capture/Compare Match Flag B"]
    #[inline(always)]
    pub fn tcfb(&mut self) -> TcfbW<GtstSpec> {
        TcfbW::new(self, 1)
    }
    #[doc = "Bit 2 - Input Compare Match Flag C"]
    #[inline(always)]
    pub fn tcfc(&mut self) -> TcfcW<GtstSpec> {
        TcfcW::new(self, 2)
    }
    #[doc = "Bit 3 - Input Compare Match Flag D"]
    #[inline(always)]
    pub fn tcfd(&mut self) -> TcfdW<GtstSpec> {
        TcfdW::new(self, 3)
    }
    #[doc = "Bit 4 - Input Compare Match Flag E"]
    #[inline(always)]
    pub fn tcfe(&mut self) -> TcfeW<GtstSpec> {
        TcfeW::new(self, 4)
    }
    #[doc = "Bit 5 - Input Compare Match Flag F"]
    #[inline(always)]
    pub fn tcff(&mut self) -> TcffW<GtstSpec> {
        TcffW::new(self, 5)
    }
    #[doc = "Bit 6 - Overflow Flag"]
    #[inline(always)]
    pub fn tcfpo(&mut self) -> TcfpoW<GtstSpec> {
        TcfpoW::new(self, 6)
    }
    #[doc = "Bit 7 - Underflow Flag"]
    #[inline(always)]
    pub fn tcfpu(&mut self) -> TcfpuW<GtstSpec> {
        TcfpuW::new(self, 7)
    }
}
#[doc = "General PWM Timer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtstSpec;
impl crate::RegisterSpec for GtstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtst::R`](R) reader structure"]
impl crate::Readable for GtstSpec {}
#[doc = "`write(|w| ..)` method takes [`gtst::W`](W) writer structure"]
impl crate::Writable for GtstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTST to value 0x8000"]
impl crate::Resettable for GtstSpec {
    const RESET_VALUE: u32 = 0x8000;
}
