#[doc = "Register `ADANSA0` reader"]
pub type R = crate::R<Adansa0Spec>;
#[doc = "Register `ADANSA0` writer"]
pub type W = crate::W<Adansa0Spec>;
#[doc = "AN000 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa00 {
    #[doc = "0: AN000 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN000 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa00> for bool {
    #[inline(always)]
    fn from(variant: Ansa00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA00` reader - AN000 Select"]
pub type Ansa00R = crate::BitReader<Ansa00>;
impl Ansa00R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa00 {
        match self.bits {
            false => Ansa00::_0,
            true => Ansa00::_1,
        }
    }
    #[doc = "AN000 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa00::_0
    }
    #[doc = "AN000 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa00::_1
    }
}
#[doc = "Field `ANSA00` writer - AN000 Select"]
pub type Ansa00W<'a, REG> = crate::BitWriter<'a, REG, Ansa00>;
impl<'a, REG> Ansa00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN000 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa00::_0)
    }
    #[doc = "AN000 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa00::_1)
    }
}
#[doc = "AN001 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa01 {
    #[doc = "0: AN001 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN001 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa01> for bool {
    #[inline(always)]
    fn from(variant: Ansa01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA01` reader - AN001 Select"]
pub type Ansa01R = crate::BitReader<Ansa01>;
impl Ansa01R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa01 {
        match self.bits {
            false => Ansa01::_0,
            true => Ansa01::_1,
        }
    }
    #[doc = "AN001 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa01::_0
    }
    #[doc = "AN001 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa01::_1
    }
}
#[doc = "Field `ANSA01` writer - AN001 Select"]
pub type Ansa01W<'a, REG> = crate::BitWriter<'a, REG, Ansa01>;
impl<'a, REG> Ansa01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN001 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa01::_0)
    }
    #[doc = "AN001 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa01::_1)
    }
}
#[doc = "AN002 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa02 {
    #[doc = "0: AN002 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN002 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa02> for bool {
    #[inline(always)]
    fn from(variant: Ansa02) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA02` reader - AN002 Select"]
pub type Ansa02R = crate::BitReader<Ansa02>;
impl Ansa02R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa02 {
        match self.bits {
            false => Ansa02::_0,
            true => Ansa02::_1,
        }
    }
    #[doc = "AN002 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa02::_0
    }
    #[doc = "AN002 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa02::_1
    }
}
#[doc = "Field `ANSA02` writer - AN002 Select"]
pub type Ansa02W<'a, REG> = crate::BitWriter<'a, REG, Ansa02>;
impl<'a, REG> Ansa02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN002 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa02::_0)
    }
    #[doc = "AN002 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa02::_1)
    }
}
#[doc = "AN003 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa03 {
    #[doc = "0: AN003 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN003 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa03> for bool {
    #[inline(always)]
    fn from(variant: Ansa03) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA03` reader - AN003 Select"]
pub type Ansa03R = crate::BitReader<Ansa03>;
impl Ansa03R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa03 {
        match self.bits {
            false => Ansa03::_0,
            true => Ansa03::_1,
        }
    }
    #[doc = "AN003 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa03::_0
    }
    #[doc = "AN003 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa03::_1
    }
}
#[doc = "Field `ANSA03` writer - AN003 Select"]
pub type Ansa03W<'a, REG> = crate::BitWriter<'a, REG, Ansa03>;
impl<'a, REG> Ansa03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN003 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa03::_0)
    }
    #[doc = "AN003 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa03::_1)
    }
}
#[doc = "AN004 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa04 {
    #[doc = "0: AN004 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN004 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa04> for bool {
    #[inline(always)]
    fn from(variant: Ansa04) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA04` reader - AN004 Select"]
pub type Ansa04R = crate::BitReader<Ansa04>;
impl Ansa04R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa04 {
        match self.bits {
            false => Ansa04::_0,
            true => Ansa04::_1,
        }
    }
    #[doc = "AN004 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa04::_0
    }
    #[doc = "AN004 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa04::_1
    }
}
#[doc = "Field `ANSA04` writer - AN004 Select"]
pub type Ansa04W<'a, REG> = crate::BitWriter<'a, REG, Ansa04>;
impl<'a, REG> Ansa04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN004 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa04::_0)
    }
    #[doc = "AN004 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa04::_1)
    }
}
#[doc = "AN005 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa05 {
    #[doc = "0: AN005 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN005 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa05> for bool {
    #[inline(always)]
    fn from(variant: Ansa05) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA05` reader - AN005 Select"]
pub type Ansa05R = crate::BitReader<Ansa05>;
impl Ansa05R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa05 {
        match self.bits {
            false => Ansa05::_0,
            true => Ansa05::_1,
        }
    }
    #[doc = "AN005 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa05::_0
    }
    #[doc = "AN005 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa05::_1
    }
}
#[doc = "Field `ANSA05` writer - AN005 Select"]
pub type Ansa05W<'a, REG> = crate::BitWriter<'a, REG, Ansa05>;
impl<'a, REG> Ansa05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN005 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa05::_0)
    }
    #[doc = "AN005 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa05::_1)
    }
}
#[doc = "AN006 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa06 {
    #[doc = "0: AN006 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN006 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa06> for bool {
    #[inline(always)]
    fn from(variant: Ansa06) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA06` reader - AN006 Select"]
pub type Ansa06R = crate::BitReader<Ansa06>;
impl Ansa06R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa06 {
        match self.bits {
            false => Ansa06::_0,
            true => Ansa06::_1,
        }
    }
    #[doc = "AN006 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa06::_0
    }
    #[doc = "AN006 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa06::_1
    }
}
#[doc = "Field `ANSA06` writer - AN006 Select"]
pub type Ansa06W<'a, REG> = crate::BitWriter<'a, REG, Ansa06>;
impl<'a, REG> Ansa06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN006 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa06::_0)
    }
    #[doc = "AN006 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa06::_1)
    }
}
#[doc = "AN007 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa07 {
    #[doc = "0: AN007 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN007 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa07> for bool {
    #[inline(always)]
    fn from(variant: Ansa07) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA07` reader - AN007 Select"]
pub type Ansa07R = crate::BitReader<Ansa07>;
impl Ansa07R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa07 {
        match self.bits {
            false => Ansa07::_0,
            true => Ansa07::_1,
        }
    }
    #[doc = "AN007 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa07::_0
    }
    #[doc = "AN007 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa07::_1
    }
}
#[doc = "Field `ANSA07` writer - AN007 Select"]
pub type Ansa07W<'a, REG> = crate::BitWriter<'a, REG, Ansa07>;
impl<'a, REG> Ansa07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN007 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa07::_0)
    }
    #[doc = "AN007 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa07::_1)
    }
}
#[doc = "AN008 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa08 {
    #[doc = "0: AN008 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN008 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa08> for bool {
    #[inline(always)]
    fn from(variant: Ansa08) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA08` reader - AN008 Select"]
pub type Ansa08R = crate::BitReader<Ansa08>;
impl Ansa08R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa08 {
        match self.bits {
            false => Ansa08::_0,
            true => Ansa08::_1,
        }
    }
    #[doc = "AN008 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa08::_0
    }
    #[doc = "AN008 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa08::_1
    }
}
#[doc = "Field `ANSA08` writer - AN008 Select"]
pub type Ansa08W<'a, REG> = crate::BitWriter<'a, REG, Ansa08>;
impl<'a, REG> Ansa08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN008 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa08::_0)
    }
    #[doc = "AN008 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa08::_1)
    }
}
#[doc = "AN009 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa09 {
    #[doc = "0: AN009 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN009 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa09> for bool {
    #[inline(always)]
    fn from(variant: Ansa09) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA09` reader - AN009 Select"]
pub type Ansa09R = crate::BitReader<Ansa09>;
impl Ansa09R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa09 {
        match self.bits {
            false => Ansa09::_0,
            true => Ansa09::_1,
        }
    }
    #[doc = "AN009 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa09::_0
    }
    #[doc = "AN009 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa09::_1
    }
}
#[doc = "Field `ANSA09` writer - AN009 Select"]
pub type Ansa09W<'a, REG> = crate::BitWriter<'a, REG, Ansa09>;
impl<'a, REG> Ansa09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN009 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa09::_0)
    }
    #[doc = "AN009 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa09::_1)
    }
}
#[doc = "AN010 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa010 {
    #[doc = "0: AN010 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN010 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa010> for bool {
    #[inline(always)]
    fn from(variant: Ansa010) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA010` reader - AN010 Select"]
pub type Ansa010R = crate::BitReader<Ansa010>;
impl Ansa010R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa010 {
        match self.bits {
            false => Ansa010::_0,
            true => Ansa010::_1,
        }
    }
    #[doc = "AN010 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa010::_0
    }
    #[doc = "AN010 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa010::_1
    }
}
#[doc = "Field `ANSA010` writer - AN010 Select"]
pub type Ansa010W<'a, REG> = crate::BitWriter<'a, REG, Ansa010>;
impl<'a, REG> Ansa010W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN010 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa010::_0)
    }
    #[doc = "AN010 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa010::_1)
    }
}
#[doc = "AN011 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa011 {
    #[doc = "0: AN011 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN011 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa011> for bool {
    #[inline(always)]
    fn from(variant: Ansa011) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA011` reader - AN011 Select"]
pub type Ansa011R = crate::BitReader<Ansa011>;
impl Ansa011R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa011 {
        match self.bits {
            false => Ansa011::_0,
            true => Ansa011::_1,
        }
    }
    #[doc = "AN011 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa011::_0
    }
    #[doc = "AN011 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa011::_1
    }
}
#[doc = "Field `ANSA011` writer - AN011 Select"]
pub type Ansa011W<'a, REG> = crate::BitWriter<'a, REG, Ansa011>;
impl<'a, REG> Ansa011W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN011 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa011::_0)
    }
    #[doc = "AN011 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa011::_1)
    }
}
#[doc = "AN012 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa012 {
    #[doc = "0: AN012 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN012 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa012> for bool {
    #[inline(always)]
    fn from(variant: Ansa012) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA012` reader - AN012 Select"]
pub type Ansa012R = crate::BitReader<Ansa012>;
impl Ansa012R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa012 {
        match self.bits {
            false => Ansa012::_0,
            true => Ansa012::_1,
        }
    }
    #[doc = "AN012 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa012::_0
    }
    #[doc = "AN012 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa012::_1
    }
}
#[doc = "Field `ANSA012` writer - AN012 Select"]
pub type Ansa012W<'a, REG> = crate::BitWriter<'a, REG, Ansa012>;
impl<'a, REG> Ansa012W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN012 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa012::_0)
    }
    #[doc = "AN012 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa012::_1)
    }
}
#[doc = "AN013 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa013 {
    #[doc = "0: AN013 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN013 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa013> for bool {
    #[inline(always)]
    fn from(variant: Ansa013) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA013` reader - AN013 Select"]
pub type Ansa013R = crate::BitReader<Ansa013>;
impl Ansa013R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa013 {
        match self.bits {
            false => Ansa013::_0,
            true => Ansa013::_1,
        }
    }
    #[doc = "AN013 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa013::_0
    }
    #[doc = "AN013 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa013::_1
    }
}
#[doc = "Field `ANSA013` writer - AN013 Select"]
pub type Ansa013W<'a, REG> = crate::BitWriter<'a, REG, Ansa013>;
impl<'a, REG> Ansa013W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN013 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa013::_0)
    }
    #[doc = "AN013 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa013::_1)
    }
}
#[doc = "AN014 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa014 {
    #[doc = "0: AN014 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN014 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa014> for bool {
    #[inline(always)]
    fn from(variant: Ansa014) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA014` reader - AN014 Select"]
pub type Ansa014R = crate::BitReader<Ansa014>;
impl Ansa014R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa014 {
        match self.bits {
            false => Ansa014::_0,
            true => Ansa014::_1,
        }
    }
    #[doc = "AN014 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa014::_0
    }
    #[doc = "AN014 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa014::_1
    }
}
#[doc = "Field `ANSA014` writer - AN014 Select"]
pub type Ansa014W<'a, REG> = crate::BitWriter<'a, REG, Ansa014>;
impl<'a, REG> Ansa014W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN014 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa014::_0)
    }
    #[doc = "AN014 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa014::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AN000 Select"]
    #[inline(always)]
    pub fn ansa00(&self) -> Ansa00R {
        Ansa00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AN001 Select"]
    #[inline(always)]
    pub fn ansa01(&self) -> Ansa01R {
        Ansa01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AN002 Select"]
    #[inline(always)]
    pub fn ansa02(&self) -> Ansa02R {
        Ansa02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AN003 Select"]
    #[inline(always)]
    pub fn ansa03(&self) -> Ansa03R {
        Ansa03R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AN004 Select"]
    #[inline(always)]
    pub fn ansa04(&self) -> Ansa04R {
        Ansa04R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AN005 Select"]
    #[inline(always)]
    pub fn ansa05(&self) -> Ansa05R {
        Ansa05R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AN006 Select"]
    #[inline(always)]
    pub fn ansa06(&self) -> Ansa06R {
        Ansa06R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AN007 Select"]
    #[inline(always)]
    pub fn ansa07(&self) -> Ansa07R {
        Ansa07R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AN008 Select"]
    #[inline(always)]
    pub fn ansa08(&self) -> Ansa08R {
        Ansa08R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AN009 Select"]
    #[inline(always)]
    pub fn ansa09(&self) -> Ansa09R {
        Ansa09R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AN010 Select"]
    #[inline(always)]
    pub fn ansa010(&self) -> Ansa010R {
        Ansa010R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AN011 Select"]
    #[inline(always)]
    pub fn ansa011(&self) -> Ansa011R {
        Ansa011R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AN012 Select"]
    #[inline(always)]
    pub fn ansa012(&self) -> Ansa012R {
        Ansa012R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AN013 Select"]
    #[inline(always)]
    pub fn ansa013(&self) -> Ansa013R {
        Ansa013R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AN014 Select"]
    #[inline(always)]
    pub fn ansa014(&self) -> Ansa014R {
        Ansa014R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADANSA0")
            .field("ansa014", &self.ansa014())
            .field("ansa013", &self.ansa013())
            .field("ansa012", &self.ansa012())
            .field("ansa011", &self.ansa011())
            .field("ansa010", &self.ansa010())
            .field("ansa09", &self.ansa09())
            .field("ansa08", &self.ansa08())
            .field("ansa07", &self.ansa07())
            .field("ansa06", &self.ansa06())
            .field("ansa05", &self.ansa05())
            .field("ansa04", &self.ansa04())
            .field("ansa03", &self.ansa03())
            .field("ansa02", &self.ansa02())
            .field("ansa01", &self.ansa01())
            .field("ansa00", &self.ansa00())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - AN000 Select"]
    #[inline(always)]
    pub fn ansa00(&mut self) -> Ansa00W<Adansa0Spec> {
        Ansa00W::new(self, 0)
    }
    #[doc = "Bit 1 - AN001 Select"]
    #[inline(always)]
    pub fn ansa01(&mut self) -> Ansa01W<Adansa0Spec> {
        Ansa01W::new(self, 1)
    }
    #[doc = "Bit 2 - AN002 Select"]
    #[inline(always)]
    pub fn ansa02(&mut self) -> Ansa02W<Adansa0Spec> {
        Ansa02W::new(self, 2)
    }
    #[doc = "Bit 3 - AN003 Select"]
    #[inline(always)]
    pub fn ansa03(&mut self) -> Ansa03W<Adansa0Spec> {
        Ansa03W::new(self, 3)
    }
    #[doc = "Bit 4 - AN004 Select"]
    #[inline(always)]
    pub fn ansa04(&mut self) -> Ansa04W<Adansa0Spec> {
        Ansa04W::new(self, 4)
    }
    #[doc = "Bit 5 - AN005 Select"]
    #[inline(always)]
    pub fn ansa05(&mut self) -> Ansa05W<Adansa0Spec> {
        Ansa05W::new(self, 5)
    }
    #[doc = "Bit 6 - AN006 Select"]
    #[inline(always)]
    pub fn ansa06(&mut self) -> Ansa06W<Adansa0Spec> {
        Ansa06W::new(self, 6)
    }
    #[doc = "Bit 7 - AN007 Select"]
    #[inline(always)]
    pub fn ansa07(&mut self) -> Ansa07W<Adansa0Spec> {
        Ansa07W::new(self, 7)
    }
    #[doc = "Bit 8 - AN008 Select"]
    #[inline(always)]
    pub fn ansa08(&mut self) -> Ansa08W<Adansa0Spec> {
        Ansa08W::new(self, 8)
    }
    #[doc = "Bit 9 - AN009 Select"]
    #[inline(always)]
    pub fn ansa09(&mut self) -> Ansa09W<Adansa0Spec> {
        Ansa09W::new(self, 9)
    }
    #[doc = "Bit 10 - AN010 Select"]
    #[inline(always)]
    pub fn ansa010(&mut self) -> Ansa010W<Adansa0Spec> {
        Ansa010W::new(self, 10)
    }
    #[doc = "Bit 11 - AN011 Select"]
    #[inline(always)]
    pub fn ansa011(&mut self) -> Ansa011W<Adansa0Spec> {
        Ansa011W::new(self, 11)
    }
    #[doc = "Bit 12 - AN012 Select"]
    #[inline(always)]
    pub fn ansa012(&mut self) -> Ansa012W<Adansa0Spec> {
        Ansa012W::new(self, 12)
    }
    #[doc = "Bit 13 - AN013 Select"]
    #[inline(always)]
    pub fn ansa013(&mut self) -> Ansa013W<Adansa0Spec> {
        Ansa013W::new(self, 13)
    }
    #[doc = "Bit 14 - AN014 Select"]
    #[inline(always)]
    pub fn ansa014(&mut self) -> Ansa014W<Adansa0Spec> {
        Ansa014W::new(self, 14)
    }
}
#[doc = "A/D Channel Select Register A0\n\nYou can [`read`](crate::Reg::read) this register and get [`adansa0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansa0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adansa0Spec;
impl crate::RegisterSpec for Adansa0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adansa0::R`](R) reader structure"]
impl crate::Readable for Adansa0Spec {}
#[doc = "`write(|w| ..)` method takes [`adansa0::W`](W) writer structure"]
impl crate::Writable for Adansa0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADANSA0 to value 0"]
impl crate::Resettable for Adansa0Spec {}
