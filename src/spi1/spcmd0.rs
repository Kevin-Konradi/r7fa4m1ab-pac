#[doc = "Register `SPCMD0` reader"]
pub type R = crate::R<Spcmd0Spec>;
#[doc = "Register `SPCMD0` writer"]
pub type W = crate::W<Spcmd0Spec>;
#[doc = "RSPCK Phase Setting\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpha {
    #[doc = "0: Data sampling on odd edge, data variation on even edge"]
    _0 = 0,
    #[doc = "1: Data variation on odd edge, data sampling on even edge"]
    _1 = 1,
}
impl From<Cpha> for bool {
    #[inline(always)]
    fn from(variant: Cpha) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - RSPCK Phase Setting"]
pub type CphaR = crate::BitReader<Cpha>;
impl CphaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpha {
        match self.bits {
            false => Cpha::_0,
            true => Cpha::_1,
        }
    }
    #[doc = "Data sampling on odd edge, data variation on even edge"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cpha::_0
    }
    #[doc = "Data variation on odd edge, data sampling on even edge"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cpha::_1
    }
}
#[doc = "Field `CPHA` writer - RSPCK Phase Setting"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG, Cpha>;
impl<'a, REG> CphaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data sampling on odd edge, data variation on even edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::_0)
    }
    #[doc = "Data variation on odd edge, data sampling on even edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::_1)
    }
}
#[doc = "RSPCK Polarity Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpol {
    #[doc = "0: RSPCK is low when idle"]
    _0 = 0,
    #[doc = "1: RSPCK is high when idle"]
    _1 = 1,
}
impl From<Cpol> for bool {
    #[inline(always)]
    fn from(variant: Cpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - RSPCK Polarity Setting"]
pub type CpolR = crate::BitReader<Cpol>;
impl CpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpol {
        match self.bits {
            false => Cpol::_0,
            true => Cpol::_1,
        }
    }
    #[doc = "RSPCK is low when idle"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cpol::_0
    }
    #[doc = "RSPCK is high when idle"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cpol::_1
    }
}
#[doc = "Field `CPOL` writer - RSPCK Polarity Setting"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG, Cpol>;
impl<'a, REG> CpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RSPCK is low when idle"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::_0)
    }
    #[doc = "RSPCK is high when idle"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::_1)
    }
}
#[doc = "Bit Rate Division Setting\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Brdv {
    #[doc = "0: These bits select the base bit rate"]
    _00 = 0,
    #[doc = "1: These bits select the base bit rate divided by 2"]
    _01 = 1,
    #[doc = "2: These bits select the base bit rate divided by 4"]
    _10 = 2,
    #[doc = "3: These bits select the base bit rate divided by 8"]
    _11 = 3,
}
impl From<Brdv> for u8 {
    #[inline(always)]
    fn from(variant: Brdv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Brdv {
    type Ux = u8;
}
impl crate::IsEnum for Brdv {}
#[doc = "Field `BRDV` reader - Bit Rate Division Setting"]
pub type BrdvR = crate::FieldReader<Brdv>;
impl BrdvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brdv {
        match self.bits {
            0 => Brdv::_00,
            1 => Brdv::_01,
            2 => Brdv::_10,
            3 => Brdv::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "These bits select the base bit rate"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Brdv::_00
    }
    #[doc = "These bits select the base bit rate divided by 2"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Brdv::_01
    }
    #[doc = "These bits select the base bit rate divided by 4"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Brdv::_10
    }
    #[doc = "These bits select the base bit rate divided by 8"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Brdv::_11
    }
}
#[doc = "Field `BRDV` writer - Bit Rate Division Setting"]
pub type BrdvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Brdv, crate::Safe>;
impl<'a, REG> BrdvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "These bits select the base bit rate"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Brdv::_00)
    }
    #[doc = "These bits select the base bit rate divided by 2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Brdv::_01)
    }
    #[doc = "These bits select the base bit rate divided by 4"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Brdv::_10)
    }
    #[doc = "These bits select the base bit rate divided by 8"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Brdv::_11)
    }
}
#[doc = "SSL Signal Assertion Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ssla {
    #[doc = "0: SSL0"]
    _000 = 0,
    #[doc = "1: SSL1"]
    _001 = 1,
    #[doc = "2: SSL2"]
    _010 = 2,
    #[doc = "3: SSL3"]
    _011 = 3,
    #[doc = "4: Setting prohibited"]
    Others = 4,
}
impl From<Ssla> for u8 {
    #[inline(always)]
    fn from(variant: Ssla) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ssla {
    type Ux = u8;
}
impl crate::IsEnum for Ssla {}
#[doc = "Field `SSLA` reader - SSL Signal Assertion Setting"]
pub type SslaR = crate::FieldReader<Ssla>;
impl SslaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssla {
        match self.bits {
            0 => Ssla::_000,
            1 => Ssla::_001,
            2 => Ssla::_010,
            3 => Ssla::_011,
            _ => Ssla::Others,
        }
    }
    #[doc = "SSL0"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Ssla::_000
    }
    #[doc = "SSL1"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Ssla::_001
    }
    #[doc = "SSL2"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Ssla::_010
    }
    #[doc = "SSL3"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Ssla::_011
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Ssla::Others)
    }
}
#[doc = "Field `SSLA` writer - SSL Signal Assertion Setting"]
pub type SslaW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ssla, crate::Safe>;
impl<'a, REG> SslaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SSL0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Ssla::_000)
    }
    #[doc = "SSL1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Ssla::_001)
    }
    #[doc = "SSL2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Ssla::_010)
    }
    #[doc = "SSL3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Ssla::_011)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Ssla::Others)
    }
}
#[doc = "RSPI Data Length Setting\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spb {
    #[doc = "0: 20 bits"]
    _0000 = 0,
    #[doc = "1: 24 bits"]
    _0001 = 1,
    #[doc = "2: 32 bits"]
    _0010 = 2,
    #[doc = "3: 32 bits"]
    _0011 = 3,
    #[doc = "8: 9 bits"]
    _1000 = 8,
    #[doc = "9: 10 bits"]
    _1001 = 9,
    #[doc = "10: 11 bits"]
    _1010 = 10,
    #[doc = "11: 12 bits"]
    _1011 = 11,
    #[doc = "12: 13 bits"]
    _1100 = 12,
    #[doc = "13: 14 bits"]
    _1101 = 13,
    #[doc = "14: 15 bits"]
    _1110 = 14,
    #[doc = "15: 16 bits"]
    _1111 = 15,
    #[doc = "4: 8bits"]
    Others = 4,
}
impl From<Spb> for u8 {
    #[inline(always)]
    fn from(variant: Spb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spb {
    type Ux = u8;
}
impl crate::IsEnum for Spb {}
#[doc = "Field `SPB` reader - RSPI Data Length Setting"]
pub type SpbR = crate::FieldReader<Spb>;
impl SpbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spb {
        match self.bits {
            0 => Spb::_0000,
            1 => Spb::_0001,
            2 => Spb::_0010,
            3 => Spb::_0011,
            8 => Spb::_1000,
            9 => Spb::_1001,
            10 => Spb::_1010,
            11 => Spb::_1011,
            12 => Spb::_1100,
            13 => Spb::_1101,
            14 => Spb::_1110,
            15 => Spb::_1111,
            _ => Spb::Others,
        }
    }
    #[doc = "20 bits"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Spb::_0000
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == Spb::_0001
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == Spb::_0010
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == Spb::_0011
    }
    #[doc = "9 bits"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == Spb::_1000
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == Spb::_1001
    }
    #[doc = "11 bits"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == Spb::_1010
    }
    #[doc = "12 bits"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == Spb::_1011
    }
    #[doc = "13 bits"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == Spb::_1100
    }
    #[doc = "14 bits"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == Spb::_1101
    }
    #[doc = "15 bits"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == Spb::_1110
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == Spb::_1111
    }
    #[doc = "8bits"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Spb::Others)
    }
}
#[doc = "Field `SPB` writer - RSPI Data Length Setting"]
pub type SpbW<'a, REG> = crate::FieldWriter<'a, REG, 4, Spb, crate::Safe>;
impl<'a, REG> SpbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "20 bits"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_0000)
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_0001)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_0010)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_0011)
    }
    #[doc = "9 bits"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_1000)
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_1001)
    }
    #[doc = "11 bits"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_1010)
    }
    #[doc = "12 bits"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_1011)
    }
    #[doc = "13 bits"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_1100)
    }
    #[doc = "14 bits"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_1101)
    }
    #[doc = "15 bits"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_1110)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_1111)
    }
    #[doc = "8bits"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::Others)
    }
}
#[doc = "RSPI LSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsbf {
    #[doc = "0: MSB first"]
    _0 = 0,
    #[doc = "1: LSB first"]
    _1 = 1,
}
impl From<Lsbf> for bool {
    #[inline(always)]
    fn from(variant: Lsbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSBF` reader - RSPI LSB First"]
pub type LsbfR = crate::BitReader<Lsbf>;
impl LsbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsbf {
        match self.bits {
            false => Lsbf::_0,
            true => Lsbf::_1,
        }
    }
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lsbf::_0
    }
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lsbf::_1
    }
}
#[doc = "Field `LSBF` writer - RSPI LSB First"]
pub type LsbfW<'a, REG> = crate::BitWriter<'a, REG, Lsbf>;
impl<'a, REG> LsbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lsbf::_0)
    }
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsbf::_1)
    }
}
#[doc = "RSPI Next-Access Delay Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spnden {
    #[doc = "0: A next-access delay of 1 RSPCK + 2 PCLK"]
    _0 = 0,
    #[doc = "1: A next-access delay is equal to the setting of the RSPI next-access delay register (SPND)"]
    _1 = 1,
}
impl From<Spnden> for bool {
    #[inline(always)]
    fn from(variant: Spnden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPNDEN` reader - RSPI Next-Access Delay Enable"]
pub type SpndenR = crate::BitReader<Spnden>;
impl SpndenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spnden {
        match self.bits {
            false => Spnden::_0,
            true => Spnden::_1,
        }
    }
    #[doc = "A next-access delay of 1 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spnden::_0
    }
    #[doc = "A next-access delay is equal to the setting of the RSPI next-access delay register (SPND)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spnden::_1
    }
}
#[doc = "Field `SPNDEN` writer - RSPI Next-Access Delay Enable"]
pub type SpndenW<'a, REG> = crate::BitWriter<'a, REG, Spnden>;
impl<'a, REG> SpndenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A next-access delay of 1 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Spnden::_0)
    }
    #[doc = "A next-access delay is equal to the setting of the RSPI next-access delay register (SPND)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Spnden::_1)
    }
}
#[doc = "SSL Negation Delay Setting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slnden {
    #[doc = "0: An SSL negation delay of 1 RSPCK"]
    _0 = 0,
    #[doc = "1: An SSL negation delay is equal to the setting of the RSPI slave select negation delay register (SSLND)"]
    _1 = 1,
}
impl From<Slnden> for bool {
    #[inline(always)]
    fn from(variant: Slnden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLNDEN` reader - SSL Negation Delay Setting Enable"]
pub type SlndenR = crate::BitReader<Slnden>;
impl SlndenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slnden {
        match self.bits {
            false => Slnden::_0,
            true => Slnden::_1,
        }
    }
    #[doc = "An SSL negation delay of 1 RSPCK"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Slnden::_0
    }
    #[doc = "An SSL negation delay is equal to the setting of the RSPI slave select negation delay register (SSLND)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Slnden::_1
    }
}
#[doc = "Field `SLNDEN` writer - SSL Negation Delay Setting Enable"]
pub type SlndenW<'a, REG> = crate::BitWriter<'a, REG, Slnden>;
impl<'a, REG> SlndenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An SSL negation delay of 1 RSPCK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Slnden::_0)
    }
    #[doc = "An SSL negation delay is equal to the setting of the RSPI slave select negation delay register (SSLND)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Slnden::_1)
    }
}
#[doc = "RSPCK Delay Setting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sckden {
    #[doc = "0: An RSPCK delay of 1 RSPCK"]
    _0 = 0,
    #[doc = "1: An RSPCK delay is equal to the setting of the RSPI clock delay register (SPCKD)"]
    _1 = 1,
}
impl From<Sckden> for bool {
    #[inline(always)]
    fn from(variant: Sckden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCKDEN` reader - RSPCK Delay Setting Enable"]
pub type SckdenR = crate::BitReader<Sckden>;
impl SckdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sckden {
        match self.bits {
            false => Sckden::_0,
            true => Sckden::_1,
        }
    }
    #[doc = "An RSPCK delay of 1 RSPCK"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sckden::_0
    }
    #[doc = "An RSPCK delay is equal to the setting of the RSPI clock delay register (SPCKD)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sckden::_1
    }
}
#[doc = "Field `SCKDEN` writer - RSPCK Delay Setting Enable"]
pub type SckdenW<'a, REG> = crate::BitWriter<'a, REG, Sckden>;
impl<'a, REG> SckdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An RSPCK delay of 1 RSPCK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sckden::_0)
    }
    #[doc = "An RSPCK delay is equal to the setting of the RSPI clock delay register (SPCKD)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sckden::_1)
    }
}
impl R {
    #[doc = "Bit 0 - RSPCK Phase Setting"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RSPCK Polarity Setting"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Bit Rate Division Setting"]
    #[inline(always)]
    pub fn brdv(&self) -> BrdvR {
        BrdvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - SSL Signal Assertion Setting"]
    #[inline(always)]
    pub fn ssla(&self) -> SslaR {
        SslaR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - RSPI Data Length Setting"]
    #[inline(always)]
    pub fn spb(&self) -> SpbR {
        SpbR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - RSPI LSB First"]
    #[inline(always)]
    pub fn lsbf(&self) -> LsbfR {
        LsbfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RSPI Next-Access Delay Enable"]
    #[inline(always)]
    pub fn spnden(&self) -> SpndenR {
        SpndenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SSL Negation Delay Setting Enable"]
    #[inline(always)]
    pub fn slnden(&self) -> SlndenR {
        SlndenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RSPCK Delay Setting Enable"]
    #[inline(always)]
    pub fn sckden(&self) -> SckdenR {
        SckdenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPCMD0")
            .field("sckden", &self.sckden())
            .field("slnden", &self.slnden())
            .field("spnden", &self.spnden())
            .field("lsbf", &self.lsbf())
            .field("spb", &self.spb())
            .field("ssla", &self.ssla())
            .field("brdv", &self.brdv())
            .field("cpol", &self.cpol())
            .field("cpha", &self.cpha())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RSPCK Phase Setting"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<Spcmd0Spec> {
        CphaW::new(self, 0)
    }
    #[doc = "Bit 1 - RSPCK Polarity Setting"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<Spcmd0Spec> {
        CpolW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Bit Rate Division Setting"]
    #[inline(always)]
    pub fn brdv(&mut self) -> BrdvW<Spcmd0Spec> {
        BrdvW::new(self, 2)
    }
    #[doc = "Bits 4:6 - SSL Signal Assertion Setting"]
    #[inline(always)]
    pub fn ssla(&mut self) -> SslaW<Spcmd0Spec> {
        SslaW::new(self, 4)
    }
    #[doc = "Bits 8:11 - RSPI Data Length Setting"]
    #[inline(always)]
    pub fn spb(&mut self) -> SpbW<Spcmd0Spec> {
        SpbW::new(self, 8)
    }
    #[doc = "Bit 12 - RSPI LSB First"]
    #[inline(always)]
    pub fn lsbf(&mut self) -> LsbfW<Spcmd0Spec> {
        LsbfW::new(self, 12)
    }
    #[doc = "Bit 13 - RSPI Next-Access Delay Enable"]
    #[inline(always)]
    pub fn spnden(&mut self) -> SpndenW<Spcmd0Spec> {
        SpndenW::new(self, 13)
    }
    #[doc = "Bit 14 - SSL Negation Delay Setting Enable"]
    #[inline(always)]
    pub fn slnden(&mut self) -> SlndenW<Spcmd0Spec> {
        SlndenW::new(self, 14)
    }
    #[doc = "Bit 15 - RSPCK Delay Setting Enable"]
    #[inline(always)]
    pub fn sckden(&mut self) -> SckdenW<Spcmd0Spec> {
        SckdenW::new(self, 15)
    }
}
#[doc = "SPI Command Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`spcmd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcmd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spcmd0Spec;
impl crate::RegisterSpec for Spcmd0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spcmd0::R`](R) reader structure"]
impl crate::Readable for Spcmd0Spec {}
#[doc = "`write(|w| ..)` method takes [`spcmd0::W`](W) writer structure"]
impl crate::Writable for Spcmd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPCMD0 to value 0x070d"]
impl crate::Resettable for Spcmd0Spec {
    const RESET_VALUE: u16 = 0x070d;
}
