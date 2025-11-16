#[doc = "Register `LVDLVLR` reader"]
pub type R = crate::R<LvdlvlrSpec>;
#[doc = "Register `LVDLVLR` writer"]
pub type W = crate::W<LvdlvlrSpec>;
#[doc = "Voltage Detection 1 Level Select (Standard voltage during drop in voltage)\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lvd1lvl {
    #[doc = "0: 4.29V (Vdet1_0)"]
    _00000 = 0,
    #[doc = "1: 4.14V (Vdet1_1)"]
    _00001 = 1,
    #[doc = "2: 4.02V (Vdet1_2)"]
    _00010 = 2,
    #[doc = "3: 3.84V (Vdet1_3)"]
    _00011 = 3,
    #[doc = "4: 3.10V (Vdet1_4)"]
    _00100 = 4,
    #[doc = "5: 3.00V (Vdet1_5)"]
    _00101 = 5,
    #[doc = "6: 2.90V (Vdet1_6)"]
    _00110 = 6,
    #[doc = "7: 2.79V (Vdet1_7)"]
    _00111 = 7,
    #[doc = "8: 2.68V (Vdet1_8)"]
    _01000 = 8,
    #[doc = "9: 2.58V (Vdet1_9)"]
    _01001 = 9,
    #[doc = "10: 2.48V (Vdet1_A)"]
    _01010 = 10,
    #[doc = "11: 2.20V (Vdet1_B)"]
    _01011 = 11,
    #[doc = "12: 1.96V (Vdet1_C)"]
    _01100 = 12,
    #[doc = "13: 1.86V (Vdet1_D)"]
    _01101 = 13,
    #[doc = "14: 1.75V (Vdet1_E)"]
    _01110 = 14,
    #[doc = "15: 1.65V (Vdet1_F)"]
    _01111 = 15,
    #[doc = "16: Setting prohibited"]
    Others = 16,
}
impl From<Lvd1lvl> for u8 {
    #[inline(always)]
    fn from(variant: Lvd1lvl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lvd1lvl {
    type Ux = u8;
}
impl crate::IsEnum for Lvd1lvl {}
#[doc = "Field `LVD1LVL` reader - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)"]
pub type Lvd1lvlR = crate::FieldReader<Lvd1lvl>;
impl Lvd1lvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd1lvl {
        match self.bits {
            0 => Lvd1lvl::_00000,
            1 => Lvd1lvl::_00001,
            2 => Lvd1lvl::_00010,
            3 => Lvd1lvl::_00011,
            4 => Lvd1lvl::_00100,
            5 => Lvd1lvl::_00101,
            6 => Lvd1lvl::_00110,
            7 => Lvd1lvl::_00111,
            8 => Lvd1lvl::_01000,
            9 => Lvd1lvl::_01001,
            10 => Lvd1lvl::_01010,
            11 => Lvd1lvl::_01011,
            12 => Lvd1lvl::_01100,
            13 => Lvd1lvl::_01101,
            14 => Lvd1lvl::_01110,
            15 => Lvd1lvl::_01111,
            _ => Lvd1lvl::Others,
        }
    }
    #[doc = "4.29V (Vdet1_0)"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == Lvd1lvl::_00000
    }
    #[doc = "4.14V (Vdet1_1)"]
    #[inline(always)]
    pub fn is_00001(&self) -> bool {
        *self == Lvd1lvl::_00001
    }
    #[doc = "4.02V (Vdet1_2)"]
    #[inline(always)]
    pub fn is_00010(&self) -> bool {
        *self == Lvd1lvl::_00010
    }
    #[doc = "3.84V (Vdet1_3)"]
    #[inline(always)]
    pub fn is_00011(&self) -> bool {
        *self == Lvd1lvl::_00011
    }
    #[doc = "3.10V (Vdet1_4)"]
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        *self == Lvd1lvl::_00100
    }
    #[doc = "3.00V (Vdet1_5)"]
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        *self == Lvd1lvl::_00101
    }
    #[doc = "2.90V (Vdet1_6)"]
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        *self == Lvd1lvl::_00110
    }
    #[doc = "2.79V (Vdet1_7)"]
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == Lvd1lvl::_00111
    }
    #[doc = "2.68V (Vdet1_8)"]
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        *self == Lvd1lvl::_01000
    }
    #[doc = "2.58V (Vdet1_9)"]
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        *self == Lvd1lvl::_01001
    }
    #[doc = "2.48V (Vdet1_A)"]
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        *self == Lvd1lvl::_01010
    }
    #[doc = "2.20V (Vdet1_B)"]
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        *self == Lvd1lvl::_01011
    }
    #[doc = "1.96V (Vdet1_C)"]
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        *self == Lvd1lvl::_01100
    }
    #[doc = "1.86V (Vdet1_D)"]
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        *self == Lvd1lvl::_01101
    }
    #[doc = "1.75V (Vdet1_E)"]
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        *self == Lvd1lvl::_01110
    }
    #[doc = "1.65V (Vdet1_F)"]
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == Lvd1lvl::_01111
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Lvd1lvl::Others)
    }
}
#[doc = "Field `LVD1LVL` writer - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)"]
pub type Lvd1lvlW<'a, REG> = crate::FieldWriter<'a, REG, 5, Lvd1lvl, crate::Safe>;
impl<'a, REG> Lvd1lvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4.29V (Vdet1_0)"]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_00000)
    }
    #[doc = "4.14V (Vdet1_1)"]
    #[inline(always)]
    pub fn _00001(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_00001)
    }
    #[doc = "4.02V (Vdet1_2)"]
    #[inline(always)]
    pub fn _00010(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_00010)
    }
    #[doc = "3.84V (Vdet1_3)"]
    #[inline(always)]
    pub fn _00011(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_00011)
    }
    #[doc = "3.10V (Vdet1_4)"]
    #[inline(always)]
    pub fn _00100(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_00100)
    }
    #[doc = "3.00V (Vdet1_5)"]
    #[inline(always)]
    pub fn _00101(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_00101)
    }
    #[doc = "2.90V (Vdet1_6)"]
    #[inline(always)]
    pub fn _00110(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_00110)
    }
    #[doc = "2.79V (Vdet1_7)"]
    #[inline(always)]
    pub fn _00111(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_00111)
    }
    #[doc = "2.68V (Vdet1_8)"]
    #[inline(always)]
    pub fn _01000(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_01000)
    }
    #[doc = "2.58V (Vdet1_9)"]
    #[inline(always)]
    pub fn _01001(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_01001)
    }
    #[doc = "2.48V (Vdet1_A)"]
    #[inline(always)]
    pub fn _01010(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_01010)
    }
    #[doc = "2.20V (Vdet1_B)"]
    #[inline(always)]
    pub fn _01011(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_01011)
    }
    #[doc = "1.96V (Vdet1_C)"]
    #[inline(always)]
    pub fn _01100(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_01100)
    }
    #[doc = "1.86V (Vdet1_D)"]
    #[inline(always)]
    pub fn _01101(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_01101)
    }
    #[doc = "1.75V (Vdet1_E)"]
    #[inline(always)]
    pub fn _01110(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_01110)
    }
    #[doc = "1.65V (Vdet1_F)"]
    #[inline(always)]
    pub fn _01111(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_01111)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::Others)
    }
}
#[doc = "Voltage Detection 2 Level Select (Standard voltage during drop in voltage)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lvd2lvl {
    #[doc = "0: 4.29V (Vdet2_0)"]
    _000 = 0,
    #[doc = "1: 4.14V (Vdet2_1)"]
    _001 = 1,
    #[doc = "2: 4.02V (Vdet2_2)"]
    _010 = 2,
    #[doc = "3: 3.84V (Vdet2_3)"]
    _011 = 3,
    #[doc = "4: Setting prohibited."]
    Others = 4,
}
impl From<Lvd2lvl> for u8 {
    #[inline(always)]
    fn from(variant: Lvd2lvl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lvd2lvl {
    type Ux = u8;
}
impl crate::IsEnum for Lvd2lvl {}
#[doc = "Field `LVD2LVL` reader - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)"]
pub type Lvd2lvlR = crate::FieldReader<Lvd2lvl>;
impl Lvd2lvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd2lvl {
        match self.bits {
            0 => Lvd2lvl::_000,
            1 => Lvd2lvl::_001,
            2 => Lvd2lvl::_010,
            3 => Lvd2lvl::_011,
            _ => Lvd2lvl::Others,
        }
    }
    #[doc = "4.29V (Vdet2_0)"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Lvd2lvl::_000
    }
    #[doc = "4.14V (Vdet2_1)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Lvd2lvl::_001
    }
    #[doc = "4.02V (Vdet2_2)"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Lvd2lvl::_010
    }
    #[doc = "3.84V (Vdet2_3)"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Lvd2lvl::_011
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Lvd2lvl::Others)
    }
}
#[doc = "Field `LVD2LVL` writer - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)"]
pub type Lvd2lvlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lvd2lvl, crate::Safe>;
impl<'a, REG> Lvd2lvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4.29V (Vdet2_0)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2lvl::_000)
    }
    #[doc = "4.14V (Vdet2_1)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2lvl::_001)
    }
    #[doc = "4.02V (Vdet2_2)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2lvl::_010)
    }
    #[doc = "3.84V (Vdet2_3)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2lvl::_011)
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2lvl::Others)
    }
}
impl R {
    #[doc = "Bits 0:4 - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)"]
    #[inline(always)]
    pub fn lvd1lvl(&self) -> Lvd1lvlR {
        Lvd1lvlR::new(self.bits & 0x1f)
    }
    #[doc = "Bits 5:7 - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)"]
    #[inline(always)]
    pub fn lvd2lvl(&self) -> Lvd2lvlR {
        Lvd2lvlR::new((self.bits >> 5) & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LVDLVLR")
            .field("lvd2lvl", &self.lvd2lvl())
            .field("lvd1lvl", &self.lvd1lvl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)"]
    #[inline(always)]
    pub fn lvd1lvl(&mut self) -> Lvd1lvlW<LvdlvlrSpec> {
        Lvd1lvlW::new(self, 0)
    }
    #[doc = "Bits 5:7 - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)"]
    #[inline(always)]
    pub fn lvd2lvl(&mut self) -> Lvd2lvlW<LvdlvlrSpec> {
        Lvd2lvlW::new(self, 5)
    }
}
#[doc = "Voltage Detection Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lvdlvlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdlvlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvdlvlrSpec;
impl crate::RegisterSpec for LvdlvlrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvdlvlr::R`](R) reader structure"]
impl crate::Readable for LvdlvlrSpec {}
#[doc = "`write(|w| ..)` method takes [`lvdlvlr::W`](W) writer structure"]
impl crate::Writable for LvdlvlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LVDLVLR to value 0x07"]
impl crate::Resettable for LvdlvlrSpec {
    const RESET_VALUE: u8 = 0x07;
}
