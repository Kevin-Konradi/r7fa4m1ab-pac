#[doc = "Register `MIER_FIFO` reader"]
pub type R = crate::R<MierFifoSpec>;
#[doc = "Register `MIER_FIFO` writer"]
pub type W = crate::W<MierFifoSpec>;
#[doc = "mailbox 0 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb0 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb0> for bool {
    #[inline(always)]
    fn from(variant: Mb0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB0` reader - mailbox 0 Interrupt Enable"]
pub type Mb0R = crate::BitReader<Mb0>;
impl Mb0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb0 {
        match self.bits {
            false => Mb0::_0,
            true => Mb0::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb0::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb0::_1
    }
}
#[doc = "Field `MB0` writer - mailbox 0 Interrupt Enable"]
pub type Mb0W<'a, REG> = crate::BitWriter<'a, REG, Mb0>;
impl<'a, REG> Mb0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb0::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb0::_1)
    }
}
#[doc = "mailbox 1 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb1 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb1> for bool {
    #[inline(always)]
    fn from(variant: Mb1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB1` reader - mailbox 1 Interrupt Enable"]
pub type Mb1R = crate::BitReader<Mb1>;
impl Mb1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb1 {
        match self.bits {
            false => Mb1::_0,
            true => Mb1::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb1::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb1::_1
    }
}
#[doc = "Field `MB1` writer - mailbox 1 Interrupt Enable"]
pub type Mb1W<'a, REG> = crate::BitWriter<'a, REG, Mb1>;
impl<'a, REG> Mb1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb1::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb1::_1)
    }
}
#[doc = "mailbox 2 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb2 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb2> for bool {
    #[inline(always)]
    fn from(variant: Mb2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB2` reader - mailbox 2 Interrupt Enable"]
pub type Mb2R = crate::BitReader<Mb2>;
impl Mb2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb2 {
        match self.bits {
            false => Mb2::_0,
            true => Mb2::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb2::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb2::_1
    }
}
#[doc = "Field `MB2` writer - mailbox 2 Interrupt Enable"]
pub type Mb2W<'a, REG> = crate::BitWriter<'a, REG, Mb2>;
impl<'a, REG> Mb2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb2::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb2::_1)
    }
}
#[doc = "mailbox 3 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb3 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb3> for bool {
    #[inline(always)]
    fn from(variant: Mb3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB3` reader - mailbox 3 Interrupt Enable"]
pub type Mb3R = crate::BitReader<Mb3>;
impl Mb3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb3 {
        match self.bits {
            false => Mb3::_0,
            true => Mb3::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb3::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb3::_1
    }
}
#[doc = "Field `MB3` writer - mailbox 3 Interrupt Enable"]
pub type Mb3W<'a, REG> = crate::BitWriter<'a, REG, Mb3>;
impl<'a, REG> Mb3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb3::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb3::_1)
    }
}
#[doc = "mailbox 4 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb4 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb4> for bool {
    #[inline(always)]
    fn from(variant: Mb4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB4` reader - mailbox 4 Interrupt Enable"]
pub type Mb4R = crate::BitReader<Mb4>;
impl Mb4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb4 {
        match self.bits {
            false => Mb4::_0,
            true => Mb4::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb4::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb4::_1
    }
}
#[doc = "Field `MB4` writer - mailbox 4 Interrupt Enable"]
pub type Mb4W<'a, REG> = crate::BitWriter<'a, REG, Mb4>;
impl<'a, REG> Mb4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb4::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb4::_1)
    }
}
#[doc = "mailbox 5 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb5 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb5> for bool {
    #[inline(always)]
    fn from(variant: Mb5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB5` reader - mailbox 5 Interrupt Enable"]
pub type Mb5R = crate::BitReader<Mb5>;
impl Mb5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb5 {
        match self.bits {
            false => Mb5::_0,
            true => Mb5::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb5::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb5::_1
    }
}
#[doc = "Field `MB5` writer - mailbox 5 Interrupt Enable"]
pub type Mb5W<'a, REG> = crate::BitWriter<'a, REG, Mb5>;
impl<'a, REG> Mb5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb5::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb5::_1)
    }
}
#[doc = "mailbox 6 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb6 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb6> for bool {
    #[inline(always)]
    fn from(variant: Mb6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB6` reader - mailbox 6 Interrupt Enable"]
pub type Mb6R = crate::BitReader<Mb6>;
impl Mb6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb6 {
        match self.bits {
            false => Mb6::_0,
            true => Mb6::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb6::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb6::_1
    }
}
#[doc = "Field `MB6` writer - mailbox 6 Interrupt Enable"]
pub type Mb6W<'a, REG> = crate::BitWriter<'a, REG, Mb6>;
impl<'a, REG> Mb6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb6::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb6::_1)
    }
}
#[doc = "mailbox 7 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb7 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb7> for bool {
    #[inline(always)]
    fn from(variant: Mb7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB7` reader - mailbox 7 Interrupt Enable"]
pub type Mb7R = crate::BitReader<Mb7>;
impl Mb7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb7 {
        match self.bits {
            false => Mb7::_0,
            true => Mb7::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb7::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb7::_1
    }
}
#[doc = "Field `MB7` writer - mailbox 7 Interrupt Enable"]
pub type Mb7W<'a, REG> = crate::BitWriter<'a, REG, Mb7>;
impl<'a, REG> Mb7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb7::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb7::_1)
    }
}
#[doc = "mailbox 8 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb8 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb8> for bool {
    #[inline(always)]
    fn from(variant: Mb8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB8` reader - mailbox 8 Interrupt Enable"]
pub type Mb8R = crate::BitReader<Mb8>;
impl Mb8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb8 {
        match self.bits {
            false => Mb8::_0,
            true => Mb8::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb8::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb8::_1
    }
}
#[doc = "Field `MB8` writer - mailbox 8 Interrupt Enable"]
pub type Mb8W<'a, REG> = crate::BitWriter<'a, REG, Mb8>;
impl<'a, REG> Mb8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb8::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb8::_1)
    }
}
#[doc = "mailbox 9 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb9 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb9> for bool {
    #[inline(always)]
    fn from(variant: Mb9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB9` reader - mailbox 9 Interrupt Enable"]
pub type Mb9R = crate::BitReader<Mb9>;
impl Mb9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb9 {
        match self.bits {
            false => Mb9::_0,
            true => Mb9::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb9::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb9::_1
    }
}
#[doc = "Field `MB9` writer - mailbox 9 Interrupt Enable"]
pub type Mb9W<'a, REG> = crate::BitWriter<'a, REG, Mb9>;
impl<'a, REG> Mb9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb9::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb9::_1)
    }
}
#[doc = "mailbox 10 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb10 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb10> for bool {
    #[inline(always)]
    fn from(variant: Mb10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB10` reader - mailbox 10 Interrupt Enable"]
pub type Mb10R = crate::BitReader<Mb10>;
impl Mb10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb10 {
        match self.bits {
            false => Mb10::_0,
            true => Mb10::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb10::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb10::_1
    }
}
#[doc = "Field `MB10` writer - mailbox 10 Interrupt Enable"]
pub type Mb10W<'a, REG> = crate::BitWriter<'a, REG, Mb10>;
impl<'a, REG> Mb10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb10::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb10::_1)
    }
}
#[doc = "mailbox 11 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb11 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb11> for bool {
    #[inline(always)]
    fn from(variant: Mb11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB11` reader - mailbox 11 Interrupt Enable"]
pub type Mb11R = crate::BitReader<Mb11>;
impl Mb11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb11 {
        match self.bits {
            false => Mb11::_0,
            true => Mb11::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb11::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb11::_1
    }
}
#[doc = "Field `MB11` writer - mailbox 11 Interrupt Enable"]
pub type Mb11W<'a, REG> = crate::BitWriter<'a, REG, Mb11>;
impl<'a, REG> Mb11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb11::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb11::_1)
    }
}
#[doc = "mailbox 12 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb12 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb12> for bool {
    #[inline(always)]
    fn from(variant: Mb12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB12` reader - mailbox 12 Interrupt Enable"]
pub type Mb12R = crate::BitReader<Mb12>;
impl Mb12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb12 {
        match self.bits {
            false => Mb12::_0,
            true => Mb12::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb12::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb12::_1
    }
}
#[doc = "Field `MB12` writer - mailbox 12 Interrupt Enable"]
pub type Mb12W<'a, REG> = crate::BitWriter<'a, REG, Mb12>;
impl<'a, REG> Mb12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb12::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb12::_1)
    }
}
#[doc = "mailbox 13 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb13 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb13> for bool {
    #[inline(always)]
    fn from(variant: Mb13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB13` reader - mailbox 13 Interrupt Enable"]
pub type Mb13R = crate::BitReader<Mb13>;
impl Mb13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb13 {
        match self.bits {
            false => Mb13::_0,
            true => Mb13::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb13::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb13::_1
    }
}
#[doc = "Field `MB13` writer - mailbox 13 Interrupt Enable"]
pub type Mb13W<'a, REG> = crate::BitWriter<'a, REG, Mb13>;
impl<'a, REG> Mb13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb13::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb13::_1)
    }
}
#[doc = "mailbox 14 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb14 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb14> for bool {
    #[inline(always)]
    fn from(variant: Mb14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB14` reader - mailbox 14 Interrupt Enable"]
pub type Mb14R = crate::BitReader<Mb14>;
impl Mb14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb14 {
        match self.bits {
            false => Mb14::_0,
            true => Mb14::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb14::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb14::_1
    }
}
#[doc = "Field `MB14` writer - mailbox 14 Interrupt Enable"]
pub type Mb14W<'a, REG> = crate::BitWriter<'a, REG, Mb14>;
impl<'a, REG> Mb14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb14::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb14::_1)
    }
}
#[doc = "mailbox 15 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb15 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb15> for bool {
    #[inline(always)]
    fn from(variant: Mb15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB15` reader - mailbox 15 Interrupt Enable"]
pub type Mb15R = crate::BitReader<Mb15>;
impl Mb15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb15 {
        match self.bits {
            false => Mb15::_0,
            true => Mb15::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb15::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb15::_1
    }
}
#[doc = "Field `MB15` writer - mailbox 15 Interrupt Enable"]
pub type Mb15W<'a, REG> = crate::BitWriter<'a, REG, Mb15>;
impl<'a, REG> Mb15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb15::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb15::_1)
    }
}
#[doc = "mailbox 16 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb16 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb16> for bool {
    #[inline(always)]
    fn from(variant: Mb16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB16` reader - mailbox 16 Interrupt Enable"]
pub type Mb16R = crate::BitReader<Mb16>;
impl Mb16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb16 {
        match self.bits {
            false => Mb16::_0,
            true => Mb16::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb16::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb16::_1
    }
}
#[doc = "Field `MB16` writer - mailbox 16 Interrupt Enable"]
pub type Mb16W<'a, REG> = crate::BitWriter<'a, REG, Mb16>;
impl<'a, REG> Mb16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb16::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb16::_1)
    }
}
#[doc = "mailbox 17 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb17 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb17> for bool {
    #[inline(always)]
    fn from(variant: Mb17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB17` reader - mailbox 17 Interrupt Enable"]
pub type Mb17R = crate::BitReader<Mb17>;
impl Mb17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb17 {
        match self.bits {
            false => Mb17::_0,
            true => Mb17::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb17::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb17::_1
    }
}
#[doc = "Field `MB17` writer - mailbox 17 Interrupt Enable"]
pub type Mb17W<'a, REG> = crate::BitWriter<'a, REG, Mb17>;
impl<'a, REG> Mb17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb17::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb17::_1)
    }
}
#[doc = "mailbox 18 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb18 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb18> for bool {
    #[inline(always)]
    fn from(variant: Mb18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB18` reader - mailbox 18 Interrupt Enable"]
pub type Mb18R = crate::BitReader<Mb18>;
impl Mb18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb18 {
        match self.bits {
            false => Mb18::_0,
            true => Mb18::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb18::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb18::_1
    }
}
#[doc = "Field `MB18` writer - mailbox 18 Interrupt Enable"]
pub type Mb18W<'a, REG> = crate::BitWriter<'a, REG, Mb18>;
impl<'a, REG> Mb18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb18::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb18::_1)
    }
}
#[doc = "mailbox 19 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb19 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb19> for bool {
    #[inline(always)]
    fn from(variant: Mb19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB19` reader - mailbox 19 Interrupt Enable"]
pub type Mb19R = crate::BitReader<Mb19>;
impl Mb19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb19 {
        match self.bits {
            false => Mb19::_0,
            true => Mb19::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb19::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb19::_1
    }
}
#[doc = "Field `MB19` writer - mailbox 19 Interrupt Enable"]
pub type Mb19W<'a, REG> = crate::BitWriter<'a, REG, Mb19>;
impl<'a, REG> Mb19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb19::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb19::_1)
    }
}
#[doc = "mailbox 20 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb20 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb20> for bool {
    #[inline(always)]
    fn from(variant: Mb20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB20` reader - mailbox 20 Interrupt Enable"]
pub type Mb20R = crate::BitReader<Mb20>;
impl Mb20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb20 {
        match self.bits {
            false => Mb20::_0,
            true => Mb20::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb20::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb20::_1
    }
}
#[doc = "Field `MB20` writer - mailbox 20 Interrupt Enable"]
pub type Mb20W<'a, REG> = crate::BitWriter<'a, REG, Mb20>;
impl<'a, REG> Mb20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb20::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb20::_1)
    }
}
#[doc = "mailbox 21 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb21 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb21> for bool {
    #[inline(always)]
    fn from(variant: Mb21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB21` reader - mailbox 21 Interrupt Enable"]
pub type Mb21R = crate::BitReader<Mb21>;
impl Mb21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb21 {
        match self.bits {
            false => Mb21::_0,
            true => Mb21::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb21::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb21::_1
    }
}
#[doc = "Field `MB21` writer - mailbox 21 Interrupt Enable"]
pub type Mb21W<'a, REG> = crate::BitWriter<'a, REG, Mb21>;
impl<'a, REG> Mb21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb21::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb21::_1)
    }
}
#[doc = "mailbox 22 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb22 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb22> for bool {
    #[inline(always)]
    fn from(variant: Mb22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB22` reader - mailbox 22 Interrupt Enable"]
pub type Mb22R = crate::BitReader<Mb22>;
impl Mb22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb22 {
        match self.bits {
            false => Mb22::_0,
            true => Mb22::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb22::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb22::_1
    }
}
#[doc = "Field `MB22` writer - mailbox 22 Interrupt Enable"]
pub type Mb22W<'a, REG> = crate::BitWriter<'a, REG, Mb22>;
impl<'a, REG> Mb22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb22::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb22::_1)
    }
}
#[doc = "mailbox 23 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb23 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb23> for bool {
    #[inline(always)]
    fn from(variant: Mb23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB23` reader - mailbox 23 Interrupt Enable"]
pub type Mb23R = crate::BitReader<Mb23>;
impl Mb23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb23 {
        match self.bits {
            false => Mb23::_0,
            true => Mb23::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb23::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb23::_1
    }
}
#[doc = "Field `MB23` writer - mailbox 23 Interrupt Enable"]
pub type Mb23W<'a, REG> = crate::BitWriter<'a, REG, Mb23>;
impl<'a, REG> Mb23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb23::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb23::_1)
    }
}
#[doc = "Transmit FIFO Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb24 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb24> for bool {
    #[inline(always)]
    fn from(variant: Mb24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB24` reader - Transmit FIFO Interrupt Enable"]
pub type Mb24R = crate::BitReader<Mb24>;
impl Mb24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb24 {
        match self.bits {
            false => Mb24::_0,
            true => Mb24::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb24::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb24::_1
    }
}
#[doc = "Field `MB24` writer - Transmit FIFO Interrupt Enable"]
pub type Mb24W<'a, REG> = crate::BitWriter<'a, REG, Mb24>;
impl<'a, REG> Mb24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb24::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb24::_1)
    }
}
#[doc = "Transmit FIFO Interrupt Generation Timing Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb25 {
    #[doc = "0: Every time transmission is completed"]
    _0 = 0,
    #[doc = "1: When the transmit FIFO becomes empty due to completion of transmission"]
    _1 = 1,
}
impl From<Mb25> for bool {
    #[inline(always)]
    fn from(variant: Mb25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB25` reader - Transmit FIFO Interrupt Generation Timing Control"]
pub type Mb25R = crate::BitReader<Mb25>;
impl Mb25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb25 {
        match self.bits {
            false => Mb25::_0,
            true => Mb25::_1,
        }
    }
    #[doc = "Every time transmission is completed"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb25::_0
    }
    #[doc = "When the transmit FIFO becomes empty due to completion of transmission"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb25::_1
    }
}
#[doc = "Field `MB25` writer - Transmit FIFO Interrupt Generation Timing Control"]
pub type Mb25W<'a, REG> = crate::BitWriter<'a, REG, Mb25>;
impl<'a, REG> Mb25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Every time transmission is completed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb25::_0)
    }
    #[doc = "When the transmit FIFO becomes empty due to completion of transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb25::_1)
    }
}
#[doc = "Receive FIFO Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb28 {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<Mb28> for bool {
    #[inline(always)]
    fn from(variant: Mb28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB28` reader - Receive FIFO Interrupt Enable"]
pub type Mb28R = crate::BitReader<Mb28>;
impl Mb28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb28 {
        match self.bits {
            false => Mb28::_0,
            true => Mb28::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb28::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb28::_1
    }
}
#[doc = "Field `MB28` writer - Receive FIFO Interrupt Enable"]
pub type Mb28W<'a, REG> = crate::BitWriter<'a, REG, Mb28>;
impl<'a, REG> Mb28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb28::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb28::_1)
    }
}
#[doc = "Receive FIFO Interrupt Generation Timing Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mb29 {
    #[doc = "0: Every time reception is completed"]
    _0 = 0,
    #[doc = "1: When the receive FIFO becomes buffer warning by completion of reception"]
    _1 = 1,
}
impl From<Mb29> for bool {
    #[inline(always)]
    fn from(variant: Mb29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB29` reader - Receive FIFO Interrupt Generation Timing Control"]
pub type Mb29R = crate::BitReader<Mb29>;
impl Mb29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mb29 {
        match self.bits {
            false => Mb29::_0,
            true => Mb29::_1,
        }
    }
    #[doc = "Every time reception is completed"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mb29::_0
    }
    #[doc = "When the receive FIFO becomes buffer warning by completion of reception"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mb29::_1
    }
}
#[doc = "Field `MB29` writer - Receive FIFO Interrupt Generation Timing Control"]
pub type Mb29W<'a, REG> = crate::BitWriter<'a, REG, Mb29>;
impl<'a, REG> Mb29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Every time reception is completed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mb29::_0)
    }
    #[doc = "When the receive FIFO becomes buffer warning by completion of reception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mb29::_1)
    }
}
impl R {
    #[doc = "Bit 0 - mailbox 0 Interrupt Enable"]
    #[inline(always)]
    pub fn mb0(&self) -> Mb0R {
        Mb0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - mailbox 1 Interrupt Enable"]
    #[inline(always)]
    pub fn mb1(&self) -> Mb1R {
        Mb1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - mailbox 2 Interrupt Enable"]
    #[inline(always)]
    pub fn mb2(&self) -> Mb2R {
        Mb2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - mailbox 3 Interrupt Enable"]
    #[inline(always)]
    pub fn mb3(&self) -> Mb3R {
        Mb3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - mailbox 4 Interrupt Enable"]
    #[inline(always)]
    pub fn mb4(&self) -> Mb4R {
        Mb4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - mailbox 5 Interrupt Enable"]
    #[inline(always)]
    pub fn mb5(&self) -> Mb5R {
        Mb5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - mailbox 6 Interrupt Enable"]
    #[inline(always)]
    pub fn mb6(&self) -> Mb6R {
        Mb6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - mailbox 7 Interrupt Enable"]
    #[inline(always)]
    pub fn mb7(&self) -> Mb7R {
        Mb7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - mailbox 8 Interrupt Enable"]
    #[inline(always)]
    pub fn mb8(&self) -> Mb8R {
        Mb8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - mailbox 9 Interrupt Enable"]
    #[inline(always)]
    pub fn mb9(&self) -> Mb9R {
        Mb9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - mailbox 10 Interrupt Enable"]
    #[inline(always)]
    pub fn mb10(&self) -> Mb10R {
        Mb10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - mailbox 11 Interrupt Enable"]
    #[inline(always)]
    pub fn mb11(&self) -> Mb11R {
        Mb11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - mailbox 12 Interrupt Enable"]
    #[inline(always)]
    pub fn mb12(&self) -> Mb12R {
        Mb12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - mailbox 13 Interrupt Enable"]
    #[inline(always)]
    pub fn mb13(&self) -> Mb13R {
        Mb13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - mailbox 14 Interrupt Enable"]
    #[inline(always)]
    pub fn mb14(&self) -> Mb14R {
        Mb14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - mailbox 15 Interrupt Enable"]
    #[inline(always)]
    pub fn mb15(&self) -> Mb15R {
        Mb15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - mailbox 16 Interrupt Enable"]
    #[inline(always)]
    pub fn mb16(&self) -> Mb16R {
        Mb16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - mailbox 17 Interrupt Enable"]
    #[inline(always)]
    pub fn mb17(&self) -> Mb17R {
        Mb17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - mailbox 18 Interrupt Enable"]
    #[inline(always)]
    pub fn mb18(&self) -> Mb18R {
        Mb18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - mailbox 19 Interrupt Enable"]
    #[inline(always)]
    pub fn mb19(&self) -> Mb19R {
        Mb19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - mailbox 20 Interrupt Enable"]
    #[inline(always)]
    pub fn mb20(&self) -> Mb20R {
        Mb20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - mailbox 21 Interrupt Enable"]
    #[inline(always)]
    pub fn mb21(&self) -> Mb21R {
        Mb21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - mailbox 22 Interrupt Enable"]
    #[inline(always)]
    pub fn mb22(&self) -> Mb22R {
        Mb22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - mailbox 23 Interrupt Enable"]
    #[inline(always)]
    pub fn mb23(&self) -> Mb23R {
        Mb23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmit FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn mb24(&self) -> Mb24R {
        Mb24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transmit FIFO Interrupt Generation Timing Control"]
    #[inline(always)]
    pub fn mb25(&self) -> Mb25R {
        Mb25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Receive FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn mb28(&self) -> Mb28R {
        Mb28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Receive FIFO Interrupt Generation Timing Control"]
    #[inline(always)]
    pub fn mb29(&self) -> Mb29R {
        Mb29R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MIER_FIFO")
            .field("mb29", &self.mb29())
            .field("mb28", &self.mb28())
            .field("mb25", &self.mb25())
            .field("mb24", &self.mb24())
            .field("mb23", &self.mb23())
            .field("mb22", &self.mb22())
            .field("mb21", &self.mb21())
            .field("mb20", &self.mb20())
            .field("mb19", &self.mb19())
            .field("mb18", &self.mb18())
            .field("mb17", &self.mb17())
            .field("mb16", &self.mb16())
            .field("mb15", &self.mb15())
            .field("mb14", &self.mb14())
            .field("mb13", &self.mb13())
            .field("mb12", &self.mb12())
            .field("mb11", &self.mb11())
            .field("mb10", &self.mb10())
            .field("mb9", &self.mb9())
            .field("mb8", &self.mb8())
            .field("mb7", &self.mb7())
            .field("mb6", &self.mb6())
            .field("mb5", &self.mb5())
            .field("mb4", &self.mb4())
            .field("mb3", &self.mb3())
            .field("mb2", &self.mb2())
            .field("mb1", &self.mb1())
            .field("mb0", &self.mb0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - mailbox 0 Interrupt Enable"]
    #[inline(always)]
    pub fn mb0(&mut self) -> Mb0W<MierFifoSpec> {
        Mb0W::new(self, 0)
    }
    #[doc = "Bit 1 - mailbox 1 Interrupt Enable"]
    #[inline(always)]
    pub fn mb1(&mut self) -> Mb1W<MierFifoSpec> {
        Mb1W::new(self, 1)
    }
    #[doc = "Bit 2 - mailbox 2 Interrupt Enable"]
    #[inline(always)]
    pub fn mb2(&mut self) -> Mb2W<MierFifoSpec> {
        Mb2W::new(self, 2)
    }
    #[doc = "Bit 3 - mailbox 3 Interrupt Enable"]
    #[inline(always)]
    pub fn mb3(&mut self) -> Mb3W<MierFifoSpec> {
        Mb3W::new(self, 3)
    }
    #[doc = "Bit 4 - mailbox 4 Interrupt Enable"]
    #[inline(always)]
    pub fn mb4(&mut self) -> Mb4W<MierFifoSpec> {
        Mb4W::new(self, 4)
    }
    #[doc = "Bit 5 - mailbox 5 Interrupt Enable"]
    #[inline(always)]
    pub fn mb5(&mut self) -> Mb5W<MierFifoSpec> {
        Mb5W::new(self, 5)
    }
    #[doc = "Bit 6 - mailbox 6 Interrupt Enable"]
    #[inline(always)]
    pub fn mb6(&mut self) -> Mb6W<MierFifoSpec> {
        Mb6W::new(self, 6)
    }
    #[doc = "Bit 7 - mailbox 7 Interrupt Enable"]
    #[inline(always)]
    pub fn mb7(&mut self) -> Mb7W<MierFifoSpec> {
        Mb7W::new(self, 7)
    }
    #[doc = "Bit 8 - mailbox 8 Interrupt Enable"]
    #[inline(always)]
    pub fn mb8(&mut self) -> Mb8W<MierFifoSpec> {
        Mb8W::new(self, 8)
    }
    #[doc = "Bit 9 - mailbox 9 Interrupt Enable"]
    #[inline(always)]
    pub fn mb9(&mut self) -> Mb9W<MierFifoSpec> {
        Mb9W::new(self, 9)
    }
    #[doc = "Bit 10 - mailbox 10 Interrupt Enable"]
    #[inline(always)]
    pub fn mb10(&mut self) -> Mb10W<MierFifoSpec> {
        Mb10W::new(self, 10)
    }
    #[doc = "Bit 11 - mailbox 11 Interrupt Enable"]
    #[inline(always)]
    pub fn mb11(&mut self) -> Mb11W<MierFifoSpec> {
        Mb11W::new(self, 11)
    }
    #[doc = "Bit 12 - mailbox 12 Interrupt Enable"]
    #[inline(always)]
    pub fn mb12(&mut self) -> Mb12W<MierFifoSpec> {
        Mb12W::new(self, 12)
    }
    #[doc = "Bit 13 - mailbox 13 Interrupt Enable"]
    #[inline(always)]
    pub fn mb13(&mut self) -> Mb13W<MierFifoSpec> {
        Mb13W::new(self, 13)
    }
    #[doc = "Bit 14 - mailbox 14 Interrupt Enable"]
    #[inline(always)]
    pub fn mb14(&mut self) -> Mb14W<MierFifoSpec> {
        Mb14W::new(self, 14)
    }
    #[doc = "Bit 15 - mailbox 15 Interrupt Enable"]
    #[inline(always)]
    pub fn mb15(&mut self) -> Mb15W<MierFifoSpec> {
        Mb15W::new(self, 15)
    }
    #[doc = "Bit 16 - mailbox 16 Interrupt Enable"]
    #[inline(always)]
    pub fn mb16(&mut self) -> Mb16W<MierFifoSpec> {
        Mb16W::new(self, 16)
    }
    #[doc = "Bit 17 - mailbox 17 Interrupt Enable"]
    #[inline(always)]
    pub fn mb17(&mut self) -> Mb17W<MierFifoSpec> {
        Mb17W::new(self, 17)
    }
    #[doc = "Bit 18 - mailbox 18 Interrupt Enable"]
    #[inline(always)]
    pub fn mb18(&mut self) -> Mb18W<MierFifoSpec> {
        Mb18W::new(self, 18)
    }
    #[doc = "Bit 19 - mailbox 19 Interrupt Enable"]
    #[inline(always)]
    pub fn mb19(&mut self) -> Mb19W<MierFifoSpec> {
        Mb19W::new(self, 19)
    }
    #[doc = "Bit 20 - mailbox 20 Interrupt Enable"]
    #[inline(always)]
    pub fn mb20(&mut self) -> Mb20W<MierFifoSpec> {
        Mb20W::new(self, 20)
    }
    #[doc = "Bit 21 - mailbox 21 Interrupt Enable"]
    #[inline(always)]
    pub fn mb21(&mut self) -> Mb21W<MierFifoSpec> {
        Mb21W::new(self, 21)
    }
    #[doc = "Bit 22 - mailbox 22 Interrupt Enable"]
    #[inline(always)]
    pub fn mb22(&mut self) -> Mb22W<MierFifoSpec> {
        Mb22W::new(self, 22)
    }
    #[doc = "Bit 23 - mailbox 23 Interrupt Enable"]
    #[inline(always)]
    pub fn mb23(&mut self) -> Mb23W<MierFifoSpec> {
        Mb23W::new(self, 23)
    }
    #[doc = "Bit 24 - Transmit FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn mb24(&mut self) -> Mb24W<MierFifoSpec> {
        Mb24W::new(self, 24)
    }
    #[doc = "Bit 25 - Transmit FIFO Interrupt Generation Timing Control"]
    #[inline(always)]
    pub fn mb25(&mut self) -> Mb25W<MierFifoSpec> {
        Mb25W::new(self, 25)
    }
    #[doc = "Bit 28 - Receive FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn mb28(&mut self) -> Mb28W<MierFifoSpec> {
        Mb28W::new(self, 28)
    }
    #[doc = "Bit 29 - Receive FIFO Interrupt Generation Timing Control"]
    #[inline(always)]
    pub fn mb29(&mut self) -> Mb29W<MierFifoSpec> {
        Mb29W::new(self, 29)
    }
}
#[doc = "Mailbox Interrupt Enable Register for FIFO Mailbox Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`mier_fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mier_fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MierFifoSpec;
impl crate::RegisterSpec for MierFifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mier_fifo::R`](R) reader structure"]
impl crate::Readable for MierFifoSpec {}
#[doc = "`write(|w| ..)` method takes [`mier_fifo::W`](W) writer structure"]
impl crate::Writable for MierFifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIER_FIFO to value 0"]
impl crate::Resettable for MierFifoSpec {}
