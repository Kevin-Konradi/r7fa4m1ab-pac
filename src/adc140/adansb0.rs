#[doc = "Register `ADANSB0` reader"]
pub type R = crate::R<Adansb0Spec>;
#[doc = "Register `ADANSB0` writer"]
pub type W = crate::W<Adansb0Spec>;
#[doc = "AN000 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb00 {
    #[doc = "0: AN000 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN000 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb00> for bool {
    #[inline(always)]
    fn from(variant: Ansb00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB00` reader - AN000 Select"]
pub type Ansb00R = crate::BitReader<Ansb00>;
impl Ansb00R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb00 {
        match self.bits {
            false => Ansb00::_0,
            true => Ansb00::_1,
        }
    }
    #[doc = "AN000 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb00::_0
    }
    #[doc = "AN000 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb00::_1
    }
}
#[doc = "Field `ANSB00` writer - AN000 Select"]
pub type Ansb00W<'a, REG> = crate::BitWriter<'a, REG, Ansb00>;
impl<'a, REG> Ansb00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN000 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb00::_0)
    }
    #[doc = "AN000 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb00::_1)
    }
}
#[doc = "AN001 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb01 {
    #[doc = "0: AN001 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN001 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb01> for bool {
    #[inline(always)]
    fn from(variant: Ansb01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB01` reader - AN001 Select"]
pub type Ansb01R = crate::BitReader<Ansb01>;
impl Ansb01R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb01 {
        match self.bits {
            false => Ansb01::_0,
            true => Ansb01::_1,
        }
    }
    #[doc = "AN001 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb01::_0
    }
    #[doc = "AN001 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb01::_1
    }
}
#[doc = "Field `ANSB01` writer - AN001 Select"]
pub type Ansb01W<'a, REG> = crate::BitWriter<'a, REG, Ansb01>;
impl<'a, REG> Ansb01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN001 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb01::_0)
    }
    #[doc = "AN001 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb01::_1)
    }
}
#[doc = "AN002 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb02 {
    #[doc = "0: AN002 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN002 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb02> for bool {
    #[inline(always)]
    fn from(variant: Ansb02) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB02` reader - AN002 Select"]
pub type Ansb02R = crate::BitReader<Ansb02>;
impl Ansb02R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb02 {
        match self.bits {
            false => Ansb02::_0,
            true => Ansb02::_1,
        }
    }
    #[doc = "AN002 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb02::_0
    }
    #[doc = "AN002 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb02::_1
    }
}
#[doc = "Field `ANSB02` writer - AN002 Select"]
pub type Ansb02W<'a, REG> = crate::BitWriter<'a, REG, Ansb02>;
impl<'a, REG> Ansb02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN002 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb02::_0)
    }
    #[doc = "AN002 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb02::_1)
    }
}
#[doc = "AN003 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb03 {
    #[doc = "0: AN003 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN003 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb03> for bool {
    #[inline(always)]
    fn from(variant: Ansb03) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB03` reader - AN003 Select"]
pub type Ansb03R = crate::BitReader<Ansb03>;
impl Ansb03R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb03 {
        match self.bits {
            false => Ansb03::_0,
            true => Ansb03::_1,
        }
    }
    #[doc = "AN003 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb03::_0
    }
    #[doc = "AN003 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb03::_1
    }
}
#[doc = "Field `ANSB03` writer - AN003 Select"]
pub type Ansb03W<'a, REG> = crate::BitWriter<'a, REG, Ansb03>;
impl<'a, REG> Ansb03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN003 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb03::_0)
    }
    #[doc = "AN003 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb03::_1)
    }
}
#[doc = "AN004 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb04 {
    #[doc = "0: AN004 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN004 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb04> for bool {
    #[inline(always)]
    fn from(variant: Ansb04) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB04` reader - AN004 Select"]
pub type Ansb04R = crate::BitReader<Ansb04>;
impl Ansb04R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb04 {
        match self.bits {
            false => Ansb04::_0,
            true => Ansb04::_1,
        }
    }
    #[doc = "AN004 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb04::_0
    }
    #[doc = "AN004 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb04::_1
    }
}
#[doc = "Field `ANSB04` writer - AN004 Select"]
pub type Ansb04W<'a, REG> = crate::BitWriter<'a, REG, Ansb04>;
impl<'a, REG> Ansb04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN004 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb04::_0)
    }
    #[doc = "AN004 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb04::_1)
    }
}
#[doc = "AN005 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb05 {
    #[doc = "0: AN005 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN005 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb05> for bool {
    #[inline(always)]
    fn from(variant: Ansb05) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB05` reader - AN005 Select"]
pub type Ansb05R = crate::BitReader<Ansb05>;
impl Ansb05R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb05 {
        match self.bits {
            false => Ansb05::_0,
            true => Ansb05::_1,
        }
    }
    #[doc = "AN005 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb05::_0
    }
    #[doc = "AN005 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb05::_1
    }
}
#[doc = "Field `ANSB05` writer - AN005 Select"]
pub type Ansb05W<'a, REG> = crate::BitWriter<'a, REG, Ansb05>;
impl<'a, REG> Ansb05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN005 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb05::_0)
    }
    #[doc = "AN005 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb05::_1)
    }
}
#[doc = "AN006 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb06 {
    #[doc = "0: AN006 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN006 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb06> for bool {
    #[inline(always)]
    fn from(variant: Ansb06) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB06` reader - AN006 Select"]
pub type Ansb06R = crate::BitReader<Ansb06>;
impl Ansb06R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb06 {
        match self.bits {
            false => Ansb06::_0,
            true => Ansb06::_1,
        }
    }
    #[doc = "AN006 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb06::_0
    }
    #[doc = "AN006 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb06::_1
    }
}
#[doc = "Field `ANSB06` writer - AN006 Select"]
pub type Ansb06W<'a, REG> = crate::BitWriter<'a, REG, Ansb06>;
impl<'a, REG> Ansb06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN006 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb06::_0)
    }
    #[doc = "AN006 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb06::_1)
    }
}
#[doc = "AN007 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb07 {
    #[doc = "0: AN007 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN007 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb07> for bool {
    #[inline(always)]
    fn from(variant: Ansb07) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB07` reader - AN007 Select"]
pub type Ansb07R = crate::BitReader<Ansb07>;
impl Ansb07R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb07 {
        match self.bits {
            false => Ansb07::_0,
            true => Ansb07::_1,
        }
    }
    #[doc = "AN007 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb07::_0
    }
    #[doc = "AN007 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb07::_1
    }
}
#[doc = "Field `ANSB07` writer - AN007 Select"]
pub type Ansb07W<'a, REG> = crate::BitWriter<'a, REG, Ansb07>;
impl<'a, REG> Ansb07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN007 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb07::_0)
    }
    #[doc = "AN007 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb07::_1)
    }
}
#[doc = "AN008 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb08 {
    #[doc = "0: AN008 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN008 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb08> for bool {
    #[inline(always)]
    fn from(variant: Ansb08) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB08` reader - AN008 Select"]
pub type Ansb08R = crate::BitReader<Ansb08>;
impl Ansb08R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb08 {
        match self.bits {
            false => Ansb08::_0,
            true => Ansb08::_1,
        }
    }
    #[doc = "AN008 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb08::_0
    }
    #[doc = "AN008 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb08::_1
    }
}
#[doc = "Field `ANSB08` writer - AN008 Select"]
pub type Ansb08W<'a, REG> = crate::BitWriter<'a, REG, Ansb08>;
impl<'a, REG> Ansb08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN008 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb08::_0)
    }
    #[doc = "AN008 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb08::_1)
    }
}
#[doc = "AN009 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb09 {
    #[doc = "0: AN009 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN009 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb09> for bool {
    #[inline(always)]
    fn from(variant: Ansb09) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB09` reader - AN009 Select"]
pub type Ansb09R = crate::BitReader<Ansb09>;
impl Ansb09R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb09 {
        match self.bits {
            false => Ansb09::_0,
            true => Ansb09::_1,
        }
    }
    #[doc = "AN009 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb09::_0
    }
    #[doc = "AN009 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb09::_1
    }
}
#[doc = "Field `ANSB09` writer - AN009 Select"]
pub type Ansb09W<'a, REG> = crate::BitWriter<'a, REG, Ansb09>;
impl<'a, REG> Ansb09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN009 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb09::_0)
    }
    #[doc = "AN009 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb09::_1)
    }
}
#[doc = "AN010 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb10 {
    #[doc = "0: AN010 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN010 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb10> for bool {
    #[inline(always)]
    fn from(variant: Ansb10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB10` reader - AN010 Select"]
pub type Ansb10R = crate::BitReader<Ansb10>;
impl Ansb10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb10 {
        match self.bits {
            false => Ansb10::_0,
            true => Ansb10::_1,
        }
    }
    #[doc = "AN010 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb10::_0
    }
    #[doc = "AN010 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb10::_1
    }
}
#[doc = "Field `ANSB10` writer - AN010 Select"]
pub type Ansb10W<'a, REG> = crate::BitWriter<'a, REG, Ansb10>;
impl<'a, REG> Ansb10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN010 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb10::_0)
    }
    #[doc = "AN010 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb10::_1)
    }
}
#[doc = "AN011 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb11 {
    #[doc = "0: AN011 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN011 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb11> for bool {
    #[inline(always)]
    fn from(variant: Ansb11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB11` reader - AN011 Select"]
pub type Ansb11R = crate::BitReader<Ansb11>;
impl Ansb11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb11 {
        match self.bits {
            false => Ansb11::_0,
            true => Ansb11::_1,
        }
    }
    #[doc = "AN011 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb11::_0
    }
    #[doc = "AN011 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb11::_1
    }
}
#[doc = "Field `ANSB11` writer - AN011 Select"]
pub type Ansb11W<'a, REG> = crate::BitWriter<'a, REG, Ansb11>;
impl<'a, REG> Ansb11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN011 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb11::_0)
    }
    #[doc = "AN011 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb11::_1)
    }
}
#[doc = "AN012 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb12 {
    #[doc = "0: AN012 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN012 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb12> for bool {
    #[inline(always)]
    fn from(variant: Ansb12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB12` reader - AN012 Select"]
pub type Ansb12R = crate::BitReader<Ansb12>;
impl Ansb12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb12 {
        match self.bits {
            false => Ansb12::_0,
            true => Ansb12::_1,
        }
    }
    #[doc = "AN012 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb12::_0
    }
    #[doc = "AN012 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb12::_1
    }
}
#[doc = "Field `ANSB12` writer - AN012 Select"]
pub type Ansb12W<'a, REG> = crate::BitWriter<'a, REG, Ansb12>;
impl<'a, REG> Ansb12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN012 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb12::_0)
    }
    #[doc = "AN012 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb12::_1)
    }
}
#[doc = "AN013 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb13 {
    #[doc = "0: AN013 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN013 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb13> for bool {
    #[inline(always)]
    fn from(variant: Ansb13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB13` reader - AN013 Select"]
pub type Ansb13R = crate::BitReader<Ansb13>;
impl Ansb13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb13 {
        match self.bits {
            false => Ansb13::_0,
            true => Ansb13::_1,
        }
    }
    #[doc = "AN013 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb13::_0
    }
    #[doc = "AN013 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb13::_1
    }
}
#[doc = "Field `ANSB13` writer - AN013 Select"]
pub type Ansb13W<'a, REG> = crate::BitWriter<'a, REG, Ansb13>;
impl<'a, REG> Ansb13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN013 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb13::_0)
    }
    #[doc = "AN013 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb13::_1)
    }
}
#[doc = "AN014 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb14 {
    #[doc = "0: AN014 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN014 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb14> for bool {
    #[inline(always)]
    fn from(variant: Ansb14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB14` reader - AN014 Select"]
pub type Ansb14R = crate::BitReader<Ansb14>;
impl Ansb14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb14 {
        match self.bits {
            false => Ansb14::_0,
            true => Ansb14::_1,
        }
    }
    #[doc = "AN014 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb14::_0
    }
    #[doc = "AN014 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb14::_1
    }
}
#[doc = "Field `ANSB14` writer - AN014 Select"]
pub type Ansb14W<'a, REG> = crate::BitWriter<'a, REG, Ansb14>;
impl<'a, REG> Ansb14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN014 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb14::_0)
    }
    #[doc = "AN014 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb14::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AN000 Select"]
    #[inline(always)]
    pub fn ansb00(&self) -> Ansb00R {
        Ansb00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AN001 Select"]
    #[inline(always)]
    pub fn ansb01(&self) -> Ansb01R {
        Ansb01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AN002 Select"]
    #[inline(always)]
    pub fn ansb02(&self) -> Ansb02R {
        Ansb02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AN003 Select"]
    #[inline(always)]
    pub fn ansb03(&self) -> Ansb03R {
        Ansb03R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AN004 Select"]
    #[inline(always)]
    pub fn ansb04(&self) -> Ansb04R {
        Ansb04R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AN005 Select"]
    #[inline(always)]
    pub fn ansb05(&self) -> Ansb05R {
        Ansb05R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AN006 Select"]
    #[inline(always)]
    pub fn ansb06(&self) -> Ansb06R {
        Ansb06R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AN007 Select"]
    #[inline(always)]
    pub fn ansb07(&self) -> Ansb07R {
        Ansb07R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AN008 Select"]
    #[inline(always)]
    pub fn ansb08(&self) -> Ansb08R {
        Ansb08R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AN009 Select"]
    #[inline(always)]
    pub fn ansb09(&self) -> Ansb09R {
        Ansb09R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AN010 Select"]
    #[inline(always)]
    pub fn ansb10(&self) -> Ansb10R {
        Ansb10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AN011 Select"]
    #[inline(always)]
    pub fn ansb11(&self) -> Ansb11R {
        Ansb11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AN012 Select"]
    #[inline(always)]
    pub fn ansb12(&self) -> Ansb12R {
        Ansb12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AN013 Select"]
    #[inline(always)]
    pub fn ansb13(&self) -> Ansb13R {
        Ansb13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AN014 Select"]
    #[inline(always)]
    pub fn ansb14(&self) -> Ansb14R {
        Ansb14R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADANSB0")
            .field("ansb14", &self.ansb14())
            .field("ansb13", &self.ansb13())
            .field("ansb12", &self.ansb12())
            .field("ansb11", &self.ansb11())
            .field("ansb10", &self.ansb10())
            .field("ansb09", &self.ansb09())
            .field("ansb08", &self.ansb08())
            .field("ansb07", &self.ansb07())
            .field("ansb06", &self.ansb06())
            .field("ansb05", &self.ansb05())
            .field("ansb04", &self.ansb04())
            .field("ansb03", &self.ansb03())
            .field("ansb02", &self.ansb02())
            .field("ansb01", &self.ansb01())
            .field("ansb00", &self.ansb00())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - AN000 Select"]
    #[inline(always)]
    pub fn ansb00(&mut self) -> Ansb00W<Adansb0Spec> {
        Ansb00W::new(self, 0)
    }
    #[doc = "Bit 1 - AN001 Select"]
    #[inline(always)]
    pub fn ansb01(&mut self) -> Ansb01W<Adansb0Spec> {
        Ansb01W::new(self, 1)
    }
    #[doc = "Bit 2 - AN002 Select"]
    #[inline(always)]
    pub fn ansb02(&mut self) -> Ansb02W<Adansb0Spec> {
        Ansb02W::new(self, 2)
    }
    #[doc = "Bit 3 - AN003 Select"]
    #[inline(always)]
    pub fn ansb03(&mut self) -> Ansb03W<Adansb0Spec> {
        Ansb03W::new(self, 3)
    }
    #[doc = "Bit 4 - AN004 Select"]
    #[inline(always)]
    pub fn ansb04(&mut self) -> Ansb04W<Adansb0Spec> {
        Ansb04W::new(self, 4)
    }
    #[doc = "Bit 5 - AN005 Select"]
    #[inline(always)]
    pub fn ansb05(&mut self) -> Ansb05W<Adansb0Spec> {
        Ansb05W::new(self, 5)
    }
    #[doc = "Bit 6 - AN006 Select"]
    #[inline(always)]
    pub fn ansb06(&mut self) -> Ansb06W<Adansb0Spec> {
        Ansb06W::new(self, 6)
    }
    #[doc = "Bit 7 - AN007 Select"]
    #[inline(always)]
    pub fn ansb07(&mut self) -> Ansb07W<Adansb0Spec> {
        Ansb07W::new(self, 7)
    }
    #[doc = "Bit 8 - AN008 Select"]
    #[inline(always)]
    pub fn ansb08(&mut self) -> Ansb08W<Adansb0Spec> {
        Ansb08W::new(self, 8)
    }
    #[doc = "Bit 9 - AN009 Select"]
    #[inline(always)]
    pub fn ansb09(&mut self) -> Ansb09W<Adansb0Spec> {
        Ansb09W::new(self, 9)
    }
    #[doc = "Bit 10 - AN010 Select"]
    #[inline(always)]
    pub fn ansb10(&mut self) -> Ansb10W<Adansb0Spec> {
        Ansb10W::new(self, 10)
    }
    #[doc = "Bit 11 - AN011 Select"]
    #[inline(always)]
    pub fn ansb11(&mut self) -> Ansb11W<Adansb0Spec> {
        Ansb11W::new(self, 11)
    }
    #[doc = "Bit 12 - AN012 Select"]
    #[inline(always)]
    pub fn ansb12(&mut self) -> Ansb12W<Adansb0Spec> {
        Ansb12W::new(self, 12)
    }
    #[doc = "Bit 13 - AN013 Select"]
    #[inline(always)]
    pub fn ansb13(&mut self) -> Ansb13W<Adansb0Spec> {
        Ansb13W::new(self, 13)
    }
    #[doc = "Bit 14 - AN014 Select"]
    #[inline(always)]
    pub fn ansb14(&mut self) -> Ansb14W<Adansb0Spec> {
        Ansb14W::new(self, 14)
    }
}
#[doc = "A/D Channel Select Register B0\n\nYou can [`read`](crate::Reg::read) this register and get [`adansb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adansb0Spec;
impl crate::RegisterSpec for Adansb0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adansb0::R`](R) reader structure"]
impl crate::Readable for Adansb0Spec {}
#[doc = "`write(|w| ..)` method takes [`adansb0::W`](W) writer structure"]
impl crate::Writable for Adansb0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADANSB0 to value 0"]
impl crate::Resettable for Adansb0Spec {}
