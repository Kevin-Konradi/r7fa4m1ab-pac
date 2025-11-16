#[doc = "Register `GTIOR` reader"]
pub type R = crate::R<GtiorSpec>;
#[doc = "Register `GTIOR` writer"]
pub type W = crate::W<GtiorSpec>;
#[doc = "GTIOCA Pin Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gtioa {
    #[doc = "0: Initial output is Low. Output retained at cycle end. Output retained at GTCCRA compare match."]
    _00000 = 0,
    #[doc = "1: Initial output is Low. Output retained at cycle end. Low output at GTCCRA compare match."]
    _00001 = 1,
    #[doc = "2: Initial output is Low. Output retained at cycle end. High output at GTCCRA compare match."]
    _00010 = 2,
    #[doc = "3: Initial output is Low. Output retained at cycle end. Output toggled at GTCCRA compare match."]
    _00011 = 3,
    #[doc = "4: Initial output is Low. Low output at cycle end. Output retained at GTCCRA compare match."]
    _00100 = 4,
    #[doc = "5: Initial output is Low. Low output at cycle end. Low output at GTCCRA compare match."]
    _00101 = 5,
    #[doc = "6: Initial output is Low. Low output at cycle end. High output at GTCCRA compare match."]
    _00110 = 6,
    #[doc = "7: Initial output is Low. Low output at cycle end. Output toggled at GTCCRA compare match."]
    _00111 = 7,
    #[doc = "8: Initial output is Low. High output at cycle end. Output retained at GTCCRA compare match."]
    _01000 = 8,
    #[doc = "9: Initial output is Low. High output at cycle end. Low output at GTCCRA compare match."]
    _01001 = 9,
    #[doc = "10: Initial output is Low. High output at cycle end. High output at GTCCRA compare match."]
    _01010 = 10,
    #[doc = "11: Initial output is Low. High output at cycle end. Output toggled at GTCCRA compare match."]
    _01011 = 11,
    #[doc = "12: Initial output is Low. Output toggled at cycle end. Output retained at GTCCRA compare match."]
    _01100 = 12,
    #[doc = "13: Initial output is Low. Output toggled at cycle end. Low output at GTCCRA compare match."]
    _01101 = 13,
    #[doc = "14: Initial output is Low. Output toggled at cycle end. High output at GTCCRA compare match."]
    _01110 = 14,
    #[doc = "15: Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRA compare match."]
    _01111 = 15,
    #[doc = "16: Initial output is High. Output retained at cycle end. Output retained at GTCCRA compare match."]
    _10000 = 16,
    #[doc = "17: Initial output is High. Output retained at cycle end. Low output at GTCCRA compare match."]
    _10001 = 17,
    #[doc = "18: Initial output is High. Output retained at cycle end. High output at GTCCRA compare match."]
    _10010 = 18,
    #[doc = "19: Initial output is High. Output retained at cycle end. Output toggled at GTCCRA compare match."]
    _10011 = 19,
    #[doc = "20: Initial output is High. Low output at cycle end. Output retained at GTCCRA compare match."]
    _10100 = 20,
    #[doc = "21: Initial output is High. Low output at cycle end. Low output at GTCCRA compare match."]
    _10101 = 21,
    #[doc = "22: Initial output is High. Low output at cycle end. High output at GTCCRA compare match."]
    _10110 = 22,
    #[doc = "23: Initial output is High. Low output at cycle end. Output toggled at GTCCRA compare match."]
    _10111 = 23,
    #[doc = "24: Initial output is High. High output at cycle end. Output retained at GTCCRA compare match."]
    _11000 = 24,
    #[doc = "25: Initial output is High. High output at cycle end. Low output at GTCCRA compare match."]
    _11001 = 25,
    #[doc = "26: Initial output is High. High output at cycle end. High output at GTCCRA compare match."]
    _11010 = 26,
    #[doc = "27: Initial output is High. High output at cycle end. Output toggled at GTCCRA compare match."]
    _11011 = 27,
    #[doc = "28: Initial output is High. Output toggled at cycle end. Output retained at GTCCRA compare match."]
    _11100 = 28,
    #[doc = "29: Initial output is High. Output toggled at cycle end. Low output at GTCCRA compare match."]
    _11101 = 29,
    #[doc = "30: Initial output is High. Output toggled at cycle end. High output at GTCCRA compare match."]
    _11110 = 30,
    #[doc = "31: Initial output is High. Output toggled at cycle end. Output toggled at GTCCRA compare match."]
    _11111 = 31,
}
impl From<Gtioa> for u8 {
    #[inline(always)]
    fn from(variant: Gtioa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gtioa {
    type Ux = u8;
}
impl crate::IsEnum for Gtioa {}
#[doc = "Field `GTIOA` reader - GTIOCA Pin Function Select"]
pub type GtioaR = crate::FieldReader<Gtioa>;
impl GtioaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gtioa {
        match self.bits {
            0 => Gtioa::_00000,
            1 => Gtioa::_00001,
            2 => Gtioa::_00010,
            3 => Gtioa::_00011,
            4 => Gtioa::_00100,
            5 => Gtioa::_00101,
            6 => Gtioa::_00110,
            7 => Gtioa::_00111,
            8 => Gtioa::_01000,
            9 => Gtioa::_01001,
            10 => Gtioa::_01010,
            11 => Gtioa::_01011,
            12 => Gtioa::_01100,
            13 => Gtioa::_01101,
            14 => Gtioa::_01110,
            15 => Gtioa::_01111,
            16 => Gtioa::_10000,
            17 => Gtioa::_10001,
            18 => Gtioa::_10010,
            19 => Gtioa::_10011,
            20 => Gtioa::_10100,
            21 => Gtioa::_10101,
            22 => Gtioa::_10110,
            23 => Gtioa::_10111,
            24 => Gtioa::_11000,
            25 => Gtioa::_11001,
            26 => Gtioa::_11010,
            27 => Gtioa::_11011,
            28 => Gtioa::_11100,
            29 => Gtioa::_11101,
            30 => Gtioa::_11110,
            31 => Gtioa::_11111,
            _ => unreachable!(),
        }
    }
    #[doc = "Initial output is Low. Output retained at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == Gtioa::_00000
    }
    #[doc = "Initial output is Low. Output retained at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_00001(&self) -> bool {
        *self == Gtioa::_00001
    }
    #[doc = "Initial output is Low. Output retained at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_00010(&self) -> bool {
        *self == Gtioa::_00010
    }
    #[doc = "Initial output is Low. Output retained at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_00011(&self) -> bool {
        *self == Gtioa::_00011
    }
    #[doc = "Initial output is Low. Low output at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        *self == Gtioa::_00100
    }
    #[doc = "Initial output is Low. Low output at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        *self == Gtioa::_00101
    }
    #[doc = "Initial output is Low. Low output at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        *self == Gtioa::_00110
    }
    #[doc = "Initial output is Low. Low output at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == Gtioa::_00111
    }
    #[doc = "Initial output is Low. High output at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        *self == Gtioa::_01000
    }
    #[doc = "Initial output is Low. High output at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        *self == Gtioa::_01001
    }
    #[doc = "Initial output is Low. High output at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        *self == Gtioa::_01010
    }
    #[doc = "Initial output is Low. High output at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        *self == Gtioa::_01011
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        *self == Gtioa::_01100
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        *self == Gtioa::_01101
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        *self == Gtioa::_01110
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == Gtioa::_01111
    }
    #[doc = "Initial output is High. Output retained at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == Gtioa::_10000
    }
    #[doc = "Initial output is High. Output retained at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == Gtioa::_10001
    }
    #[doc = "Initial output is High. Output retained at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == Gtioa::_10010
    }
    #[doc = "Initial output is High. Output retained at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        *self == Gtioa::_10011
    }
    #[doc = "Initial output is High. Low output at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_10100(&self) -> bool {
        *self == Gtioa::_10100
    }
    #[doc = "Initial output is High. Low output at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_10101(&self) -> bool {
        *self == Gtioa::_10101
    }
    #[doc = "Initial output is High. Low output at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_10110(&self) -> bool {
        *self == Gtioa::_10110
    }
    #[doc = "Initial output is High. Low output at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_10111(&self) -> bool {
        *self == Gtioa::_10111
    }
    #[doc = "Initial output is High. High output at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_11000(&self) -> bool {
        *self == Gtioa::_11000
    }
    #[doc = "Initial output is High. High output at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_11001(&self) -> bool {
        *self == Gtioa::_11001
    }
    #[doc = "Initial output is High. High output at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_11010(&self) -> bool {
        *self == Gtioa::_11010
    }
    #[doc = "Initial output is High. High output at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_11011(&self) -> bool {
        *self == Gtioa::_11011
    }
    #[doc = "Initial output is High. Output toggled at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_11100(&self) -> bool {
        *self == Gtioa::_11100
    }
    #[doc = "Initial output is High. Output toggled at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_11101(&self) -> bool {
        *self == Gtioa::_11101
    }
    #[doc = "Initial output is High. Output toggled at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_11110(&self) -> bool {
        *self == Gtioa::_11110
    }
    #[doc = "Initial output is High. Output toggled at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        *self == Gtioa::_11111
    }
}
#[doc = "Field `GTIOA` writer - GTIOCA Pin Function Select"]
pub type GtioaW<'a, REG> = crate::FieldWriter<'a, REG, 5, Gtioa, crate::Safe>;
impl<'a, REG> GtioaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Initial output is Low. Output retained at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_00000)
    }
    #[doc = "Initial output is Low. Output retained at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _00001(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_00001)
    }
    #[doc = "Initial output is Low. Output retained at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _00010(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_00010)
    }
    #[doc = "Initial output is Low. Output retained at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn _00011(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_00011)
    }
    #[doc = "Initial output is Low. Low output at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn _00100(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_00100)
    }
    #[doc = "Initial output is Low. Low output at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _00101(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_00101)
    }
    #[doc = "Initial output is Low. Low output at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _00110(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_00110)
    }
    #[doc = "Initial output is Low. Low output at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn _00111(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_00111)
    }
    #[doc = "Initial output is Low. High output at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn _01000(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_01000)
    }
    #[doc = "Initial output is Low. High output at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _01001(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_01001)
    }
    #[doc = "Initial output is Low. High output at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _01010(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_01010)
    }
    #[doc = "Initial output is Low. High output at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn _01011(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_01011)
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn _01100(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_01100)
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _01101(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_01101)
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _01110(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_01110)
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn _01111(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_01111)
    }
    #[doc = "Initial output is High. Output retained at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn _10000(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_10000)
    }
    #[doc = "Initial output is High. Output retained at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _10001(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_10001)
    }
    #[doc = "Initial output is High. Output retained at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _10010(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_10010)
    }
    #[doc = "Initial output is High. Output retained at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn _10011(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_10011)
    }
    #[doc = "Initial output is High. Low output at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn _10100(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_10100)
    }
    #[doc = "Initial output is High. Low output at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _10101(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_10101)
    }
    #[doc = "Initial output is High. Low output at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _10110(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_10110)
    }
    #[doc = "Initial output is High. Low output at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn _10111(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_10111)
    }
    #[doc = "Initial output is High. High output at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn _11000(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_11000)
    }
    #[doc = "Initial output is High. High output at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _11001(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_11001)
    }
    #[doc = "Initial output is High. High output at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _11010(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_11010)
    }
    #[doc = "Initial output is High. High output at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn _11011(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_11011)
    }
    #[doc = "Initial output is High. Output toggled at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn _11100(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_11100)
    }
    #[doc = "Initial output is High. Output toggled at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _11101(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_11101)
    }
    #[doc = "Initial output is High. Output toggled at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _11110(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_11110)
    }
    #[doc = "Initial output is High. Output toggled at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn _11111(self) -> &'a mut crate::W<REG> {
        self.variant(Gtioa::_11111)
    }
}
#[doc = "GTIOCA Pin Output Value Setting at the Count Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oadflt {
    #[doc = "0: The GTIOCA pin outputs low when counting is stopped."]
    _0 = 0,
    #[doc = "1: The GTIOCA pin outputs high when counting is stopped."]
    _1 = 1,
}
impl From<Oadflt> for bool {
    #[inline(always)]
    fn from(variant: Oadflt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OADFLT` reader - GTIOCA Pin Output Value Setting at the Count Stop"]
pub type OadfltR = crate::BitReader<Oadflt>;
impl OadfltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oadflt {
        match self.bits {
            false => Oadflt::_0,
            true => Oadflt::_1,
        }
    }
    #[doc = "The GTIOCA pin outputs low when counting is stopped."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oadflt::_0
    }
    #[doc = "The GTIOCA pin outputs high when counting is stopped."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oadflt::_1
    }
}
#[doc = "Field `OADFLT` writer - GTIOCA Pin Output Value Setting at the Count Stop"]
pub type OadfltW<'a, REG> = crate::BitWriter<'a, REG, Oadflt>;
impl<'a, REG> OadfltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The GTIOCA pin outputs low when counting is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Oadflt::_0)
    }
    #[doc = "The GTIOCA pin outputs high when counting is stopped."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Oadflt::_1)
    }
}
#[doc = "GTIOCA Pin Output Setting at the Start/Stop Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oahld {
    #[doc = "0: The GTIOCA pin output level at start/stop of counting depends on the register setting."]
    _0 = 0,
    #[doc = "1: The GTIOCA pin output level is retained at start/stop of counting."]
    _1 = 1,
}
impl From<Oahld> for bool {
    #[inline(always)]
    fn from(variant: Oahld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OAHLD` reader - GTIOCA Pin Output Setting at the Start/Stop Count"]
pub type OahldR = crate::BitReader<Oahld>;
impl OahldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oahld {
        match self.bits {
            false => Oahld::_0,
            true => Oahld::_1,
        }
    }
    #[doc = "The GTIOCA pin output level at start/stop of counting depends on the register setting."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oahld::_0
    }
    #[doc = "The GTIOCA pin output level is retained at start/stop of counting."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oahld::_1
    }
}
#[doc = "Field `OAHLD` writer - GTIOCA Pin Output Setting at the Start/Stop Count"]
pub type OahldW<'a, REG> = crate::BitWriter<'a, REG, Oahld>;
impl<'a, REG> OahldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The GTIOCA pin output level at start/stop of counting depends on the register setting."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Oahld::_0)
    }
    #[doc = "The GTIOCA pin output level is retained at start/stop of counting."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Oahld::_1)
    }
}
#[doc = "GTIOCA Pin Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oae {
    #[doc = "0: Output is disabled"]
    _0 = 0,
    #[doc = "1: Output is enabled"]
    _1 = 1,
}
impl From<Oae> for bool {
    #[inline(always)]
    fn from(variant: Oae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OAE` reader - GTIOCA Pin Output Enable"]
pub type OaeR = crate::BitReader<Oae>;
impl OaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oae {
        match self.bits {
            false => Oae::_0,
            true => Oae::_1,
        }
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oae::_0
    }
    #[doc = "Output is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oae::_1
    }
}
#[doc = "Field `OAE` writer - GTIOCA Pin Output Enable"]
pub type OaeW<'a, REG> = crate::BitWriter<'a, REG, Oae>;
impl<'a, REG> OaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Oae::_0)
    }
    #[doc = "Output is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Oae::_1)
    }
}
#[doc = "GTIOCA Pin Disable Value Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Oadf {
    #[doc = "0: Output disable is prohibited."]
    _00 = 0,
    #[doc = "1: GTIOCA pin is set to Hi-Z when output disable is performed."]
    _01 = 1,
    #[doc = "2: GTIOCA pin is set to 0 when output disable is performed."]
    _10 = 2,
    #[doc = "3: GTIOCA pin is set to 1 when output disable is performed."]
    _11 = 3,
}
impl From<Oadf> for u8 {
    #[inline(always)]
    fn from(variant: Oadf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Oadf {
    type Ux = u8;
}
impl crate::IsEnum for Oadf {}
#[doc = "Field `OADF` reader - GTIOCA Pin Disable Value Setting"]
pub type OadfR = crate::FieldReader<Oadf>;
impl OadfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oadf {
        match self.bits {
            0 => Oadf::_00,
            1 => Oadf::_01,
            2 => Oadf::_10,
            3 => Oadf::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Output disable is prohibited."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Oadf::_00
    }
    #[doc = "GTIOCA pin is set to Hi-Z when output disable is performed."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Oadf::_01
    }
    #[doc = "GTIOCA pin is set to 0 when output disable is performed."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Oadf::_10
    }
    #[doc = "GTIOCA pin is set to 1 when output disable is performed."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Oadf::_11
    }
}
#[doc = "Field `OADF` writer - GTIOCA Pin Disable Value Setting"]
pub type OadfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Oadf, crate::Safe>;
impl<'a, REG> OadfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output disable is prohibited."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Oadf::_00)
    }
    #[doc = "GTIOCA pin is set to Hi-Z when output disable is performed."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Oadf::_01)
    }
    #[doc = "GTIOCA pin is set to 0 when output disable is performed."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Oadf::_10)
    }
    #[doc = "GTIOCA pin is set to 1 when output disable is performed."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Oadf::_11)
    }
}
#[doc = "Noise Filter A Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nfaen {
    #[doc = "0: The noise filter for the GTIOCA pin is disabled."]
    _0 = 0,
    #[doc = "1: The noise filter for the GTIOCA pin is enabled."]
    _1 = 1,
}
impl From<Nfaen> for bool {
    #[inline(always)]
    fn from(variant: Nfaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NFAEN` reader - Noise Filter A Enable"]
pub type NfaenR = crate::BitReader<Nfaen>;
impl NfaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nfaen {
        match self.bits {
            false => Nfaen::_0,
            true => Nfaen::_1,
        }
    }
    #[doc = "The noise filter for the GTIOCA pin is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nfaen::_0
    }
    #[doc = "The noise filter for the GTIOCA pin is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nfaen::_1
    }
}
#[doc = "Field `NFAEN` writer - Noise Filter A Enable"]
pub type NfaenW<'a, REG> = crate::BitWriter<'a, REG, Nfaen>;
impl<'a, REG> NfaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The noise filter for the GTIOCA pin is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nfaen::_0)
    }
    #[doc = "The noise filter for the GTIOCA pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nfaen::_1)
    }
}
#[doc = "Noise Filter A Sampling Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nfcsa {
    #[doc = "0: PCLK/1"]
    _00 = 0,
    #[doc = "1: PCLK/4"]
    _01 = 1,
    #[doc = "2: PCLK/16"]
    _10 = 2,
    #[doc = "3: PCLK/64"]
    _11 = 3,
}
impl From<Nfcsa> for u8 {
    #[inline(always)]
    fn from(variant: Nfcsa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nfcsa {
    type Ux = u8;
}
impl crate::IsEnum for Nfcsa {}
#[doc = "Field `NFCSA` reader - Noise Filter A Sampling Clock Select"]
pub type NfcsaR = crate::FieldReader<Nfcsa>;
impl NfcsaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nfcsa {
        match self.bits {
            0 => Nfcsa::_00,
            1 => Nfcsa::_01,
            2 => Nfcsa::_10,
            3 => Nfcsa::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK/1"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Nfcsa::_00
    }
    #[doc = "PCLK/4"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Nfcsa::_01
    }
    #[doc = "PCLK/16"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Nfcsa::_10
    }
    #[doc = "PCLK/64"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Nfcsa::_11
    }
}
#[doc = "Field `NFCSA` writer - Noise Filter A Sampling Clock Select"]
pub type NfcsaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nfcsa, crate::Safe>;
impl<'a, REG> NfcsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK/1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcsa::_00)
    }
    #[doc = "PCLK/4"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcsa::_01)
    }
    #[doc = "PCLK/16"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcsa::_10)
    }
    #[doc = "PCLK/64"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcsa::_11)
    }
}
#[doc = "GTIOCB Pin Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gtiob {
    #[doc = "0: Initial output is Low. Output retained at cycle end. Output retained at GTCCRB compare match."]
    _00000 = 0,
    #[doc = "1: Initial output is Low. Output retained at cycle end. Low output at GTCCRB compare match."]
    _00001 = 1,
    #[doc = "2: Initial output is Low. Output retained at cycle end. High output at GTCCRB compare match."]
    _00010 = 2,
    #[doc = "3: Initial output is Low. Output retained at cycle end. Output toggled at GTCCRB compare match."]
    _00011 = 3,
    #[doc = "4: Initial output is Low. Low output at cycle end. Output retained at GTCCRB compare match."]
    _00100 = 4,
    #[doc = "5: Initial output is Low. Low output at cycle end. Low output at GTCCRB compare match."]
    _00101 = 5,
    #[doc = "6: Initial output is Low. Low output at cycle end. High output at GTCCRB compare match."]
    _00110 = 6,
    #[doc = "7: Initial output is Low. Low output at cycle end. Output toggled at GTCCRB compare match."]
    _00111 = 7,
    #[doc = "8: Initial output is Low. High output at cycle end. Output retained at GTCCRB compare match."]
    _01000 = 8,
    #[doc = "9: Initial output is Low. High output at cycle end. Low output at GTCCRB compare match."]
    _01001 = 9,
    #[doc = "10: Initial output is Low. High output at cycle end. High output at GTCCRB compare match."]
    _01010 = 10,
    #[doc = "11: Initial output is Low. High output at cycle end. Output toggled at GTCCRB compare match."]
    _01011 = 11,
    #[doc = "12: Initial output is Low. Output toggled at cycle end. Output retained at GTCCRB compare match."]
    _01100 = 12,
    #[doc = "13: Initial output is Low. Output toggled at cycle end. Low output at GTCCRB compare match."]
    _01101 = 13,
    #[doc = "14: Initial output is Low. Output toggled at cycle end. High output at GTCCRB compare match."]
    _01110 = 14,
    #[doc = "15: Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRB compare match."]
    _01111 = 15,
    #[doc = "16: Initial output is High. Output retained at cycle end. Output retained at GTCCRB compare match."]
    _10000 = 16,
    #[doc = "17: Initial output is High. Output retained at cycle end. Low output at GTCCRB compare match."]
    _10001 = 17,
    #[doc = "18: Initial output is High. Output retained at cycle end. High output at GTCCRB compare match."]
    _10010 = 18,
    #[doc = "19: Initial output is High. Output retained at cycle end. Output toggled at GTCCRB compare match."]
    _10011 = 19,
    #[doc = "20: Initial output is High. Low output at cycle end. Output retained at GTCCRB compare match."]
    _10100 = 20,
    #[doc = "21: Initial output is High. Low output at cycle end. Low output at GTCCRB compare match."]
    _10101 = 21,
    #[doc = "22: Initial output is High. Low output at cycle end. High output at GTCCRB compare match."]
    _10110 = 22,
    #[doc = "23: Initial output is High. Low output at cycle end. Output toggled at GTCCRB compare match."]
    _10111 = 23,
    #[doc = "24: Initial output is High. High output at cycle end. Output retained at GTCCRB compare match."]
    _11000 = 24,
    #[doc = "25: Initial output is High. High output at cycle end. Low output at GTCCRB compare match."]
    _11001 = 25,
    #[doc = "26: Initial output is High. High output at cycle end. High output at GTCCRB compare match."]
    _11010 = 26,
    #[doc = "27: Initial output is High. High output at cycle end. Output toggled at GTCCRB compare match."]
    _11011 = 27,
    #[doc = "28: Initial output is High. Output toggled at cycle end. Output retained at GTCCRB compare match."]
    _11100 = 28,
    #[doc = "29: Initial output is High. Output toggled at cycle end. Low output at GTCCRB compare match."]
    _11101 = 29,
    #[doc = "30: Initial output is High. Output toggled at cycle end. High output at GTCCRB compare match."]
    _11110 = 30,
    #[doc = "31: Initial output is High. Output toggled at cycle end. Output toggled at GTCCRB compare match."]
    _11111 = 31,
}
impl From<Gtiob> for u8 {
    #[inline(always)]
    fn from(variant: Gtiob) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gtiob {
    type Ux = u8;
}
impl crate::IsEnum for Gtiob {}
#[doc = "Field `GTIOB` reader - GTIOCB Pin Function Select"]
pub type GtiobR = crate::FieldReader<Gtiob>;
impl GtiobR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gtiob {
        match self.bits {
            0 => Gtiob::_00000,
            1 => Gtiob::_00001,
            2 => Gtiob::_00010,
            3 => Gtiob::_00011,
            4 => Gtiob::_00100,
            5 => Gtiob::_00101,
            6 => Gtiob::_00110,
            7 => Gtiob::_00111,
            8 => Gtiob::_01000,
            9 => Gtiob::_01001,
            10 => Gtiob::_01010,
            11 => Gtiob::_01011,
            12 => Gtiob::_01100,
            13 => Gtiob::_01101,
            14 => Gtiob::_01110,
            15 => Gtiob::_01111,
            16 => Gtiob::_10000,
            17 => Gtiob::_10001,
            18 => Gtiob::_10010,
            19 => Gtiob::_10011,
            20 => Gtiob::_10100,
            21 => Gtiob::_10101,
            22 => Gtiob::_10110,
            23 => Gtiob::_10111,
            24 => Gtiob::_11000,
            25 => Gtiob::_11001,
            26 => Gtiob::_11010,
            27 => Gtiob::_11011,
            28 => Gtiob::_11100,
            29 => Gtiob::_11101,
            30 => Gtiob::_11110,
            31 => Gtiob::_11111,
            _ => unreachable!(),
        }
    }
    #[doc = "Initial output is Low. Output retained at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == Gtiob::_00000
    }
    #[doc = "Initial output is Low. Output retained at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_00001(&self) -> bool {
        *self == Gtiob::_00001
    }
    #[doc = "Initial output is Low. Output retained at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_00010(&self) -> bool {
        *self == Gtiob::_00010
    }
    #[doc = "Initial output is Low. Output retained at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_00011(&self) -> bool {
        *self == Gtiob::_00011
    }
    #[doc = "Initial output is Low. Low output at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        *self == Gtiob::_00100
    }
    #[doc = "Initial output is Low. Low output at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        *self == Gtiob::_00101
    }
    #[doc = "Initial output is Low. Low output at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        *self == Gtiob::_00110
    }
    #[doc = "Initial output is Low. Low output at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == Gtiob::_00111
    }
    #[doc = "Initial output is Low. High output at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        *self == Gtiob::_01000
    }
    #[doc = "Initial output is Low. High output at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        *self == Gtiob::_01001
    }
    #[doc = "Initial output is Low. High output at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        *self == Gtiob::_01010
    }
    #[doc = "Initial output is Low. High output at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        *self == Gtiob::_01011
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        *self == Gtiob::_01100
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        *self == Gtiob::_01101
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        *self == Gtiob::_01110
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == Gtiob::_01111
    }
    #[doc = "Initial output is High. Output retained at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == Gtiob::_10000
    }
    #[doc = "Initial output is High. Output retained at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == Gtiob::_10001
    }
    #[doc = "Initial output is High. Output retained at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == Gtiob::_10010
    }
    #[doc = "Initial output is High. Output retained at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        *self == Gtiob::_10011
    }
    #[doc = "Initial output is High. Low output at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_10100(&self) -> bool {
        *self == Gtiob::_10100
    }
    #[doc = "Initial output is High. Low output at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_10101(&self) -> bool {
        *self == Gtiob::_10101
    }
    #[doc = "Initial output is High. Low output at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_10110(&self) -> bool {
        *self == Gtiob::_10110
    }
    #[doc = "Initial output is High. Low output at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_10111(&self) -> bool {
        *self == Gtiob::_10111
    }
    #[doc = "Initial output is High. High output at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_11000(&self) -> bool {
        *self == Gtiob::_11000
    }
    #[doc = "Initial output is High. High output at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_11001(&self) -> bool {
        *self == Gtiob::_11001
    }
    #[doc = "Initial output is High. High output at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_11010(&self) -> bool {
        *self == Gtiob::_11010
    }
    #[doc = "Initial output is High. High output at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_11011(&self) -> bool {
        *self == Gtiob::_11011
    }
    #[doc = "Initial output is High. Output toggled at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_11100(&self) -> bool {
        *self == Gtiob::_11100
    }
    #[doc = "Initial output is High. Output toggled at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_11101(&self) -> bool {
        *self == Gtiob::_11101
    }
    #[doc = "Initial output is High. Output toggled at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_11110(&self) -> bool {
        *self == Gtiob::_11110
    }
    #[doc = "Initial output is High. Output toggled at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        *self == Gtiob::_11111
    }
}
#[doc = "Field `GTIOB` writer - GTIOCB Pin Function Select"]
pub type GtiobW<'a, REG> = crate::FieldWriter<'a, REG, 5, Gtiob, crate::Safe>;
impl<'a, REG> GtiobW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Initial output is Low. Output retained at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_00000)
    }
    #[doc = "Initial output is Low. Output retained at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _00001(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_00001)
    }
    #[doc = "Initial output is Low. Output retained at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _00010(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_00010)
    }
    #[doc = "Initial output is Low. Output retained at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn _00011(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_00011)
    }
    #[doc = "Initial output is Low. Low output at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn _00100(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_00100)
    }
    #[doc = "Initial output is Low. Low output at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _00101(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_00101)
    }
    #[doc = "Initial output is Low. Low output at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _00110(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_00110)
    }
    #[doc = "Initial output is Low. Low output at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn _00111(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_00111)
    }
    #[doc = "Initial output is Low. High output at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn _01000(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_01000)
    }
    #[doc = "Initial output is Low. High output at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _01001(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_01001)
    }
    #[doc = "Initial output is Low. High output at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _01010(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_01010)
    }
    #[doc = "Initial output is Low. High output at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn _01011(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_01011)
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn _01100(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_01100)
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _01101(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_01101)
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _01110(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_01110)
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn _01111(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_01111)
    }
    #[doc = "Initial output is High. Output retained at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn _10000(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_10000)
    }
    #[doc = "Initial output is High. Output retained at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _10001(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_10001)
    }
    #[doc = "Initial output is High. Output retained at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _10010(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_10010)
    }
    #[doc = "Initial output is High. Output retained at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn _10011(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_10011)
    }
    #[doc = "Initial output is High. Low output at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn _10100(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_10100)
    }
    #[doc = "Initial output is High. Low output at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _10101(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_10101)
    }
    #[doc = "Initial output is High. Low output at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _10110(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_10110)
    }
    #[doc = "Initial output is High. Low output at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn _10111(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_10111)
    }
    #[doc = "Initial output is High. High output at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn _11000(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_11000)
    }
    #[doc = "Initial output is High. High output at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _11001(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_11001)
    }
    #[doc = "Initial output is High. High output at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _11010(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_11010)
    }
    #[doc = "Initial output is High. High output at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn _11011(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_11011)
    }
    #[doc = "Initial output is High. Output toggled at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn _11100(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_11100)
    }
    #[doc = "Initial output is High. Output toggled at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _11101(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_11101)
    }
    #[doc = "Initial output is High. Output toggled at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _11110(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_11110)
    }
    #[doc = "Initial output is High. Output toggled at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn _11111(self) -> &'a mut crate::W<REG> {
        self.variant(Gtiob::_11111)
    }
}
#[doc = "GTIOCB Pin Output Value Setting at the Count Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Obdflt {
    #[doc = "0: The GTIOCB pin outputs low when counting is stopped."]
    _0 = 0,
    #[doc = "1: The GTIOCB pin outputs high when counting is stopped."]
    _1 = 1,
}
impl From<Obdflt> for bool {
    #[inline(always)]
    fn from(variant: Obdflt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBDFLT` reader - GTIOCB Pin Output Value Setting at the Count Stop"]
pub type ObdfltR = crate::BitReader<Obdflt>;
impl ObdfltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Obdflt {
        match self.bits {
            false => Obdflt::_0,
            true => Obdflt::_1,
        }
    }
    #[doc = "The GTIOCB pin outputs low when counting is stopped."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Obdflt::_0
    }
    #[doc = "The GTIOCB pin outputs high when counting is stopped."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Obdflt::_1
    }
}
#[doc = "Field `OBDFLT` writer - GTIOCB Pin Output Value Setting at the Count Stop"]
pub type ObdfltW<'a, REG> = crate::BitWriter<'a, REG, Obdflt>;
impl<'a, REG> ObdfltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The GTIOCB pin outputs low when counting is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Obdflt::_0)
    }
    #[doc = "The GTIOCB pin outputs high when counting is stopped."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Obdflt::_1)
    }
}
#[doc = "GTIOCB Pin Output Setting at the Start/Stop Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Obhld {
    #[doc = "0: The GTIOCB pin output level at start/stop of counting depends on the register setting."]
    _0 = 0,
    #[doc = "1: The GTIOCB pin output level is retained at start/stop of counting."]
    _1 = 1,
}
impl From<Obhld> for bool {
    #[inline(always)]
    fn from(variant: Obhld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBHLD` reader - GTIOCB Pin Output Setting at the Start/Stop Count"]
pub type ObhldR = crate::BitReader<Obhld>;
impl ObhldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Obhld {
        match self.bits {
            false => Obhld::_0,
            true => Obhld::_1,
        }
    }
    #[doc = "The GTIOCB pin output level at start/stop of counting depends on the register setting."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Obhld::_0
    }
    #[doc = "The GTIOCB pin output level is retained at start/stop of counting."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Obhld::_1
    }
}
#[doc = "Field `OBHLD` writer - GTIOCB Pin Output Setting at the Start/Stop Count"]
pub type ObhldW<'a, REG> = crate::BitWriter<'a, REG, Obhld>;
impl<'a, REG> ObhldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The GTIOCB pin output level at start/stop of counting depends on the register setting."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Obhld::_0)
    }
    #[doc = "The GTIOCB pin output level is retained at start/stop of counting."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Obhld::_1)
    }
}
#[doc = "GTIOCB Pin Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Obe {
    #[doc = "0: Output is disabled"]
    _0 = 0,
    #[doc = "1: Output is enabled"]
    _1 = 1,
}
impl From<Obe> for bool {
    #[inline(always)]
    fn from(variant: Obe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBE` reader - GTIOCB Pin Output Enable"]
pub type ObeR = crate::BitReader<Obe>;
impl ObeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Obe {
        match self.bits {
            false => Obe::_0,
            true => Obe::_1,
        }
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Obe::_0
    }
    #[doc = "Output is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Obe::_1
    }
}
#[doc = "Field `OBE` writer - GTIOCB Pin Output Enable"]
pub type ObeW<'a, REG> = crate::BitWriter<'a, REG, Obe>;
impl<'a, REG> ObeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Obe::_0)
    }
    #[doc = "Output is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Obe::_1)
    }
}
#[doc = "GTIOCB Pin Disable Value Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Obdf {
    #[doc = "0: Output disable is prohibited."]
    _00 = 0,
    #[doc = "1: GTIOCB pin is set to Hi-Z when output disable is performed."]
    _01 = 1,
    #[doc = "2: GTIOCB pin is set to 0 when output disable is performed."]
    _10 = 2,
    #[doc = "3: GTIOCB pin is set to 1 when output disable is performed."]
    _11 = 3,
}
impl From<Obdf> for u8 {
    #[inline(always)]
    fn from(variant: Obdf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Obdf {
    type Ux = u8;
}
impl crate::IsEnum for Obdf {}
#[doc = "Field `OBDF` reader - GTIOCB Pin Disable Value Setting"]
pub type ObdfR = crate::FieldReader<Obdf>;
impl ObdfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Obdf {
        match self.bits {
            0 => Obdf::_00,
            1 => Obdf::_01,
            2 => Obdf::_10,
            3 => Obdf::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Output disable is prohibited."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Obdf::_00
    }
    #[doc = "GTIOCB pin is set to Hi-Z when output disable is performed."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Obdf::_01
    }
    #[doc = "GTIOCB pin is set to 0 when output disable is performed."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Obdf::_10
    }
    #[doc = "GTIOCB pin is set to 1 when output disable is performed."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Obdf::_11
    }
}
#[doc = "Field `OBDF` writer - GTIOCB Pin Disable Value Setting"]
pub type ObdfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Obdf, crate::Safe>;
impl<'a, REG> ObdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output disable is prohibited."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Obdf::_00)
    }
    #[doc = "GTIOCB pin is set to Hi-Z when output disable is performed."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Obdf::_01)
    }
    #[doc = "GTIOCB pin is set to 0 when output disable is performed."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Obdf::_10)
    }
    #[doc = "GTIOCB pin is set to 1 when output disable is performed."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Obdf::_11)
    }
}
#[doc = "Noise Filter B Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nfben {
    #[doc = "0: The noise filter for the GTIOCB pin is disabled."]
    _0 = 0,
    #[doc = "1: The noise filter for the GTIOCB pin is enabled."]
    _1 = 1,
}
impl From<Nfben> for bool {
    #[inline(always)]
    fn from(variant: Nfben) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NFBEN` reader - Noise Filter B Enable"]
pub type NfbenR = crate::BitReader<Nfben>;
impl NfbenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nfben {
        match self.bits {
            false => Nfben::_0,
            true => Nfben::_1,
        }
    }
    #[doc = "The noise filter for the GTIOCB pin is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nfben::_0
    }
    #[doc = "The noise filter for the GTIOCB pin is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nfben::_1
    }
}
#[doc = "Field `NFBEN` writer - Noise Filter B Enable"]
pub type NfbenW<'a, REG> = crate::BitWriter<'a, REG, Nfben>;
impl<'a, REG> NfbenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The noise filter for the GTIOCB pin is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nfben::_0)
    }
    #[doc = "The noise filter for the GTIOCB pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nfben::_1)
    }
}
#[doc = "Noise Filter B Sampling Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nfcsb {
    #[doc = "0: PCLK/1"]
    _00 = 0,
    #[doc = "1: PCLK/4"]
    _01 = 1,
    #[doc = "2: PCLK/16"]
    _10 = 2,
    #[doc = "3: PCLK/64"]
    _11 = 3,
}
impl From<Nfcsb> for u8 {
    #[inline(always)]
    fn from(variant: Nfcsb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nfcsb {
    type Ux = u8;
}
impl crate::IsEnum for Nfcsb {}
#[doc = "Field `NFCSB` reader - Noise Filter B Sampling Clock Select"]
pub type NfcsbR = crate::FieldReader<Nfcsb>;
impl NfcsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nfcsb {
        match self.bits {
            0 => Nfcsb::_00,
            1 => Nfcsb::_01,
            2 => Nfcsb::_10,
            3 => Nfcsb::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK/1"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Nfcsb::_00
    }
    #[doc = "PCLK/4"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Nfcsb::_01
    }
    #[doc = "PCLK/16"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Nfcsb::_10
    }
    #[doc = "PCLK/64"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Nfcsb::_11
    }
}
#[doc = "Field `NFCSB` writer - Noise Filter B Sampling Clock Select"]
pub type NfcsbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nfcsb, crate::Safe>;
impl<'a, REG> NfcsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK/1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcsb::_00)
    }
    #[doc = "PCLK/4"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcsb::_01)
    }
    #[doc = "PCLK/16"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcsb::_10)
    }
    #[doc = "PCLK/64"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcsb::_11)
    }
}
impl R {
    #[doc = "Bits 0:4 - GTIOCA Pin Function Select"]
    #[inline(always)]
    pub fn gtioa(&self) -> GtioaR {
        GtioaR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - GTIOCA Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    pub fn oadflt(&self) -> OadfltR {
        OadfltR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GTIOCA Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    pub fn oahld(&self) -> OahldR {
        OahldR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GTIOCA Pin Output Enable"]
    #[inline(always)]
    pub fn oae(&self) -> OaeR {
        OaeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - GTIOCA Pin Disable Value Setting"]
    #[inline(always)]
    pub fn oadf(&self) -> OadfR {
        OadfR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 13 - Noise Filter A Enable"]
    #[inline(always)]
    pub fn nfaen(&self) -> NfaenR {
        NfaenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Noise Filter A Sampling Clock Select"]
    #[inline(always)]
    pub fn nfcsa(&self) -> NfcsaR {
        NfcsaR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:20 - GTIOCB Pin Function Select"]
    #[inline(always)]
    pub fn gtiob(&self) -> GtiobR {
        GtiobR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - GTIOCB Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    pub fn obdflt(&self) -> ObdfltR {
        ObdfltR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GTIOCB Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    pub fn obhld(&self) -> ObhldR {
        ObhldR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GTIOCB Pin Output Enable"]
    #[inline(always)]
    pub fn obe(&self) -> ObeR {
        ObeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - GTIOCB Pin Disable Value Setting"]
    #[inline(always)]
    pub fn obdf(&self) -> ObdfR {
        ObdfR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 29 - Noise Filter B Enable"]
    #[inline(always)]
    pub fn nfben(&self) -> NfbenR {
        NfbenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Noise Filter B Sampling Clock Select"]
    #[inline(always)]
    pub fn nfcsb(&self) -> NfcsbR {
        NfcsbR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTIOR")
            .field("nfcsb", &self.nfcsb())
            .field("nfben", &self.nfben())
            .field("obdf", &self.obdf())
            .field("obe", &self.obe())
            .field("obhld", &self.obhld())
            .field("obdflt", &self.obdflt())
            .field("gtiob", &self.gtiob())
            .field("nfcsa", &self.nfcsa())
            .field("nfaen", &self.nfaen())
            .field("oadf", &self.oadf())
            .field("oae", &self.oae())
            .field("oahld", &self.oahld())
            .field("oadflt", &self.oadflt())
            .field("gtioa", &self.gtioa())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - GTIOCA Pin Function Select"]
    #[inline(always)]
    pub fn gtioa(&mut self) -> GtioaW<GtiorSpec> {
        GtioaW::new(self, 0)
    }
    #[doc = "Bit 6 - GTIOCA Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    pub fn oadflt(&mut self) -> OadfltW<GtiorSpec> {
        OadfltW::new(self, 6)
    }
    #[doc = "Bit 7 - GTIOCA Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    pub fn oahld(&mut self) -> OahldW<GtiorSpec> {
        OahldW::new(self, 7)
    }
    #[doc = "Bit 8 - GTIOCA Pin Output Enable"]
    #[inline(always)]
    pub fn oae(&mut self) -> OaeW<GtiorSpec> {
        OaeW::new(self, 8)
    }
    #[doc = "Bits 9:10 - GTIOCA Pin Disable Value Setting"]
    #[inline(always)]
    pub fn oadf(&mut self) -> OadfW<GtiorSpec> {
        OadfW::new(self, 9)
    }
    #[doc = "Bit 13 - Noise Filter A Enable"]
    #[inline(always)]
    pub fn nfaen(&mut self) -> NfaenW<GtiorSpec> {
        NfaenW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Noise Filter A Sampling Clock Select"]
    #[inline(always)]
    pub fn nfcsa(&mut self) -> NfcsaW<GtiorSpec> {
        NfcsaW::new(self, 14)
    }
    #[doc = "Bits 16:20 - GTIOCB Pin Function Select"]
    #[inline(always)]
    pub fn gtiob(&mut self) -> GtiobW<GtiorSpec> {
        GtiobW::new(self, 16)
    }
    #[doc = "Bit 22 - GTIOCB Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    pub fn obdflt(&mut self) -> ObdfltW<GtiorSpec> {
        ObdfltW::new(self, 22)
    }
    #[doc = "Bit 23 - GTIOCB Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    pub fn obhld(&mut self) -> ObhldW<GtiorSpec> {
        ObhldW::new(self, 23)
    }
    #[doc = "Bit 24 - GTIOCB Pin Output Enable"]
    #[inline(always)]
    pub fn obe(&mut self) -> ObeW<GtiorSpec> {
        ObeW::new(self, 24)
    }
    #[doc = "Bits 25:26 - GTIOCB Pin Disable Value Setting"]
    #[inline(always)]
    pub fn obdf(&mut self) -> ObdfW<GtiorSpec> {
        ObdfW::new(self, 25)
    }
    #[doc = "Bit 29 - Noise Filter B Enable"]
    #[inline(always)]
    pub fn nfben(&mut self) -> NfbenW<GtiorSpec> {
        NfbenW::new(self, 29)
    }
    #[doc = "Bits 30:31 - Noise Filter B Sampling Clock Select"]
    #[inline(always)]
    pub fn nfcsb(&mut self) -> NfcsbW<GtiorSpec> {
        NfcsbW::new(self, 30)
    }
}
#[doc = "General PWM Timer I/O Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtior::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtior::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtiorSpec;
impl crate::RegisterSpec for GtiorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtior::R`](R) reader structure"]
impl crate::Readable for GtiorSpec {}
#[doc = "`write(|w| ..)` method takes [`gtior::W`](W) writer structure"]
impl crate::Writable for GtiorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTIOR to value 0"]
impl crate::Resettable for GtiorSpec {}
