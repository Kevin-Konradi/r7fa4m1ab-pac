#[doc = "Register `SCKDIVCR` reader"]
pub type R = crate::R<SckdivcrSpec>;
#[doc = "Register `SCKDIVCR` writer"]
pub type W = crate::W<SckdivcrSpec>;
#[doc = "Peripheral Module Clock D (PCLKD) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pckd {
    #[doc = "0: /1"]
    _000 = 0,
    #[doc = "1: /2"]
    _001 = 1,
    #[doc = "2: /4"]
    _010 = 2,
    #[doc = "3: /8"]
    _011 = 3,
    #[doc = "4: /16"]
    _100 = 4,
    #[doc = "5: /32"]
    _101 = 5,
    #[doc = "6: /64"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    Others = 7,
}
impl From<Pckd> for u8 {
    #[inline(always)]
    fn from(variant: Pckd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pckd {
    type Ux = u8;
}
impl crate::IsEnum for Pckd {}
#[doc = "Field `PCKD` reader - Peripheral Module Clock D (PCLKD) Select"]
pub type PckdR = crate::FieldReader<Pckd>;
impl PckdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pckd {
        match self.bits {
            0 => Pckd::_000,
            1 => Pckd::_001,
            2 => Pckd::_010,
            3 => Pckd::_011,
            4 => Pckd::_100,
            5 => Pckd::_101,
            6 => Pckd::_110,
            7 => Pckd::Others,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Pckd::_000
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Pckd::_001
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Pckd::_010
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Pckd::_011
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Pckd::_100
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Pckd::_101
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Pckd::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Pckd::Others
    }
}
#[doc = "Field `PCKD` writer - Peripheral Module Clock D (PCLKD) Select"]
pub type PckdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pckd, crate::Safe>;
impl<'a, REG> PckdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Pckd::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Pckd::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Pckd::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Pckd::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Pckd::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Pckd::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Pckd::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Pckd::Others)
    }
}
#[doc = "Peripheral Module Clock C (PCLKC) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pckc {
    #[doc = "0: /1"]
    _000 = 0,
    #[doc = "1: /2"]
    _001 = 1,
    #[doc = "2: /4"]
    _010 = 2,
    #[doc = "3: /8"]
    _011 = 3,
    #[doc = "4: /16"]
    _100 = 4,
    #[doc = "5: /32"]
    _101 = 5,
    #[doc = "6: /64"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    Others = 7,
}
impl From<Pckc> for u8 {
    #[inline(always)]
    fn from(variant: Pckc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pckc {
    type Ux = u8;
}
impl crate::IsEnum for Pckc {}
#[doc = "Field `PCKC` reader - Peripheral Module Clock C (PCLKC) Select"]
pub type PckcR = crate::FieldReader<Pckc>;
impl PckcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pckc {
        match self.bits {
            0 => Pckc::_000,
            1 => Pckc::_001,
            2 => Pckc::_010,
            3 => Pckc::_011,
            4 => Pckc::_100,
            5 => Pckc::_101,
            6 => Pckc::_110,
            7 => Pckc::Others,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Pckc::_000
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Pckc::_001
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Pckc::_010
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Pckc::_011
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Pckc::_100
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Pckc::_101
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Pckc::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Pckc::Others
    }
}
#[doc = "Field `PCKC` writer - Peripheral Module Clock C (PCLKC) Select"]
pub type PckcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pckc, crate::Safe>;
impl<'a, REG> PckcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Pckc::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Pckc::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Pckc::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Pckc::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Pckc::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Pckc::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Pckc::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Pckc::Others)
    }
}
#[doc = "Peripheral Module Clock B (PCLKB) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pckb {
    #[doc = "0: /1"]
    _000 = 0,
    #[doc = "1: /2"]
    _001 = 1,
    #[doc = "2: /4"]
    _010 = 2,
    #[doc = "3: /8"]
    _011 = 3,
    #[doc = "4: /16"]
    _100 = 4,
    #[doc = "5: /32"]
    _101 = 5,
    #[doc = "6: /64"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    Others = 7,
}
impl From<Pckb> for u8 {
    #[inline(always)]
    fn from(variant: Pckb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pckb {
    type Ux = u8;
}
impl crate::IsEnum for Pckb {}
#[doc = "Field `PCKB` reader - Peripheral Module Clock B (PCLKB) Select"]
pub type PckbR = crate::FieldReader<Pckb>;
impl PckbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pckb {
        match self.bits {
            0 => Pckb::_000,
            1 => Pckb::_001,
            2 => Pckb::_010,
            3 => Pckb::_011,
            4 => Pckb::_100,
            5 => Pckb::_101,
            6 => Pckb::_110,
            7 => Pckb::Others,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Pckb::_000
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Pckb::_001
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Pckb::_010
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Pckb::_011
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Pckb::_100
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Pckb::_101
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Pckb::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Pckb::Others
    }
}
#[doc = "Field `PCKB` writer - Peripheral Module Clock B (PCLKB) Select"]
pub type PckbW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pckb, crate::Safe>;
impl<'a, REG> PckbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Pckb::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Pckb::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Pckb::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Pckb::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Pckb::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Pckb::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Pckb::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Pckb::Others)
    }
}
#[doc = "Peripheral Module Clock A (PCLKA) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pcka {
    #[doc = "0: /1"]
    _000 = 0,
    #[doc = "1: /2"]
    _001 = 1,
    #[doc = "2: /4"]
    _010 = 2,
    #[doc = "3: /8"]
    _011 = 3,
    #[doc = "4: /16"]
    _100 = 4,
    #[doc = "5: /32"]
    _101 = 5,
    #[doc = "6: /64"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    Others = 7,
}
impl From<Pcka> for u8 {
    #[inline(always)]
    fn from(variant: Pcka) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pcka {
    type Ux = u8;
}
impl crate::IsEnum for Pcka {}
#[doc = "Field `PCKA` reader - Peripheral Module Clock A (PCLKA) Select"]
pub type PckaR = crate::FieldReader<Pcka>;
impl PckaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcka {
        match self.bits {
            0 => Pcka::_000,
            1 => Pcka::_001,
            2 => Pcka::_010,
            3 => Pcka::_011,
            4 => Pcka::_100,
            5 => Pcka::_101,
            6 => Pcka::_110,
            7 => Pcka::Others,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Pcka::_000
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Pcka::_001
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Pcka::_010
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Pcka::_011
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Pcka::_100
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Pcka::_101
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Pcka::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Pcka::Others
    }
}
#[doc = "Field `PCKA` writer - Peripheral Module Clock A (PCLKA) Select"]
pub type PckaW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pcka, crate::Safe>;
impl<'a, REG> PckaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Pcka::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Pcka::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Pcka::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Pcka::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Pcka::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Pcka::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Pcka::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Pcka::Others)
    }
}
#[doc = "System Clock (ICLK) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ick {
    #[doc = "0: /1"]
    _000 = 0,
    #[doc = "1: /2"]
    _001 = 1,
    #[doc = "2: /4"]
    _010 = 2,
    #[doc = "3: /8"]
    _011 = 3,
    #[doc = "4: /16"]
    _100 = 4,
    #[doc = "5: /32"]
    _101 = 5,
    #[doc = "6: /64"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    Others = 7,
}
impl From<Ick> for u8 {
    #[inline(always)]
    fn from(variant: Ick) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ick {
    type Ux = u8;
}
impl crate::IsEnum for Ick {}
#[doc = "Field `ICK` reader - System Clock (ICLK) Select"]
pub type IckR = crate::FieldReader<Ick>;
impl IckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ick {
        match self.bits {
            0 => Ick::_000,
            1 => Ick::_001,
            2 => Ick::_010,
            3 => Ick::_011,
            4 => Ick::_100,
            5 => Ick::_101,
            6 => Ick::_110,
            7 => Ick::Others,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Ick::_000
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Ick::_001
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Ick::_010
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Ick::_011
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Ick::_100
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Ick::_101
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Ick::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Ick::Others
    }
}
#[doc = "Field `ICK` writer - System Clock (ICLK) Select"]
pub type IckW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ick, crate::Safe>;
impl<'a, REG> IckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Ick::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Ick::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Ick::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Ick::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Ick::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Ick::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Ick::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Ick::Others)
    }
}
#[doc = "Flash IF Clock (FCLK) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fck {
    #[doc = "0: /1"]
    _000 = 0,
    #[doc = "1: /2"]
    _001 = 1,
    #[doc = "2: /4"]
    _010 = 2,
    #[doc = "3: /8"]
    _011 = 3,
    #[doc = "4: /16"]
    _100 = 4,
    #[doc = "5: /32"]
    _101 = 5,
    #[doc = "6: /64"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    Others = 7,
}
impl From<Fck> for u8 {
    #[inline(always)]
    fn from(variant: Fck) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fck {
    type Ux = u8;
}
impl crate::IsEnum for Fck {}
#[doc = "Field `FCK` reader - Flash IF Clock (FCLK) Select"]
pub type FckR = crate::FieldReader<Fck>;
impl FckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fck {
        match self.bits {
            0 => Fck::_000,
            1 => Fck::_001,
            2 => Fck::_010,
            3 => Fck::_011,
            4 => Fck::_100,
            5 => Fck::_101,
            6 => Fck::_110,
            7 => Fck::Others,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Fck::_000
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Fck::_001
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Fck::_010
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Fck::_011
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Fck::_100
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Fck::_101
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Fck::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Fck::Others
    }
}
#[doc = "Field `FCK` writer - Flash IF Clock (FCLK) Select"]
pub type FckW<'a, REG> = crate::FieldWriter<'a, REG, 3, Fck, crate::Safe>;
impl<'a, REG> FckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Fck::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Fck::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Fck::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Fck::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Fck::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Fck::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Fck::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Fck::Others)
    }
}
impl R {
    #[doc = "Bits 0:2 - Peripheral Module Clock D (PCLKD) Select"]
    #[inline(always)]
    pub fn pckd(&self) -> PckdR {
        PckdR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Peripheral Module Clock C (PCLKC) Select"]
    #[inline(always)]
    pub fn pckc(&self) -> PckcR {
        PckcR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Peripheral Module Clock B (PCLKB) Select"]
    #[inline(always)]
    pub fn pckb(&self) -> PckbR {
        PckbR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Peripheral Module Clock A (PCLKA) Select"]
    #[inline(always)]
    pub fn pcka(&self) -> PckaR {
        PckaR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 24:26 - System Clock (ICLK) Select"]
    #[inline(always)]
    pub fn ick(&self) -> IckR {
        IckR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Flash IF Clock (FCLK) Select"]
    #[inline(always)]
    pub fn fck(&self) -> FckR {
        FckR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCKDIVCR")
            .field("fck", &self.fck())
            .field("ick", &self.ick())
            .field("pcka", &self.pcka())
            .field("pckb", &self.pckb())
            .field("pckc", &self.pckc())
            .field("pckd", &self.pckd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Peripheral Module Clock D (PCLKD) Select"]
    #[inline(always)]
    pub fn pckd(&mut self) -> PckdW<SckdivcrSpec> {
        PckdW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Peripheral Module Clock C (PCLKC) Select"]
    #[inline(always)]
    pub fn pckc(&mut self) -> PckcW<SckdivcrSpec> {
        PckcW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Peripheral Module Clock B (PCLKB) Select"]
    #[inline(always)]
    pub fn pckb(&mut self) -> PckbW<SckdivcrSpec> {
        PckbW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Peripheral Module Clock A (PCLKA) Select"]
    #[inline(always)]
    pub fn pcka(&mut self) -> PckaW<SckdivcrSpec> {
        PckaW::new(self, 12)
    }
    #[doc = "Bits 24:26 - System Clock (ICLK) Select"]
    #[inline(always)]
    pub fn ick(&mut self) -> IckW<SckdivcrSpec> {
        IckW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Flash IF Clock (FCLK) Select"]
    #[inline(always)]
    pub fn fck(&mut self) -> FckW<SckdivcrSpec> {
        FckW::new(self, 28)
    }
}
#[doc = "System Clock Division Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sckdivcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckdivcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SckdivcrSpec;
impl crate::RegisterSpec for SckdivcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sckdivcr::R`](R) reader structure"]
impl crate::Readable for SckdivcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sckdivcr::W`](W) writer structure"]
impl crate::Writable for SckdivcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCKDIVCR to value 0x4404_4444"]
impl crate::Resettable for SckdivcrSpec {
    const RESET_VALUE: u32 = 0x4404_4444;
}
