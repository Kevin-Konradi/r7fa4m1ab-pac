#[doc = "Register `LCDC0` reader"]
pub type R = crate::R<Lcdc0Spec>;
#[doc = "Register `LCDC0` writer"]
pub type W = crate::W<Lcdc0Spec>;
#[doc = "LCD clock (LCDCL)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcdc {
    #[doc = "1: (Sub clock)/22 or (LOCO clock)/22"]
    _000001 = 1,
    #[doc = "2: (Sub clock)/23 or (LOCO clock)/23"]
    _000010 = 2,
    #[doc = "3: (Sub clock)/24 or (LOCO clock)/24"]
    _000011 = 3,
    #[doc = "4: (Sub clock)/25 or (LOCO clock)/25"]
    _000100 = 4,
    #[doc = "5: (Sub clock)/26 or (LOCO clock)/26"]
    _000101 = 5,
    #[doc = "6: (Sub clock)/27 or (LOCO clock)/27"]
    _000110 = 6,
    #[doc = "7: (Sub clock)/28 or (LOCO clock)/28"]
    _000111 = 7,
    #[doc = "8: (Sub clock)/29 or (LOCO clock)/29"]
    _001000 = 8,
    #[doc = "9: (Sub clock)/210 or (LOCO clock)/210"]
    _001001 = 9,
    #[doc = "17: (Main clock)/28 or (HOCO clock)/28"]
    _010001 = 17,
    #[doc = "18: (Main clock)/29 or (HOCO clock)/29"]
    _010010 = 18,
    #[doc = "19: (Main clock)/210 or (HOCO clock)/210"]
    _010011 = 19,
    #[doc = "20: (Main clock)/211 or (HOCO clock)/211"]
    _010100 = 20,
    #[doc = "21: (Main clock)/212 or (HOCO clock)/212"]
    _010101 = 21,
    #[doc = "22: (Main clock)/213 or (HOCO clock)/213"]
    _010110 = 22,
    #[doc = "23: (Main clock)/214 or (HOCO clock)/214"]
    _010111 = 23,
    #[doc = "24: (Main clock)/215 or (HOCO clock)/215"]
    _011000 = 24,
    #[doc = "25: (Main clock)/216 or (HOCO clock)/216"]
    _011001 = 25,
    #[doc = "26: (Main clock)/217 or (HOCO clock)/217"]
    _011010 = 26,
    #[doc = "27: (Main clock)/218 or (HOCO clock)/218"]
    _011011 = 27,
    #[doc = "43: (Main clock)/219 or (HOCO clock)/219"]
    _101011 = 43,
    #[doc = "0: Other than above Setting prohibited"]
    Others = 0,
}
impl From<Lcdc> for u8 {
    #[inline(always)]
    fn from(variant: Lcdc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcdc {
    type Ux = u8;
}
impl crate::IsEnum for Lcdc {}
#[doc = "Field `LCDC` reader - LCD clock (LCDCL)"]
pub type LcdcR = crate::FieldReader<Lcdc>;
impl LcdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdc {
        match self.bits {
            1 => Lcdc::_000001,
            2 => Lcdc::_000010,
            3 => Lcdc::_000011,
            4 => Lcdc::_000100,
            5 => Lcdc::_000101,
            6 => Lcdc::_000110,
            7 => Lcdc::_000111,
            8 => Lcdc::_001000,
            9 => Lcdc::_001001,
            17 => Lcdc::_010001,
            18 => Lcdc::_010010,
            19 => Lcdc::_010011,
            20 => Lcdc::_010100,
            21 => Lcdc::_010101,
            22 => Lcdc::_010110,
            23 => Lcdc::_010111,
            24 => Lcdc::_011000,
            25 => Lcdc::_011001,
            26 => Lcdc::_011010,
            27 => Lcdc::_011011,
            43 => Lcdc::_101011,
            _ => Lcdc::Others,
        }
    }
    #[doc = "(Sub clock)/22 or (LOCO clock)/22"]
    #[inline(always)]
    pub fn is_000001(&self) -> bool {
        *self == Lcdc::_000001
    }
    #[doc = "(Sub clock)/23 or (LOCO clock)/23"]
    #[inline(always)]
    pub fn is_000010(&self) -> bool {
        *self == Lcdc::_000010
    }
    #[doc = "(Sub clock)/24 or (LOCO clock)/24"]
    #[inline(always)]
    pub fn is_000011(&self) -> bool {
        *self == Lcdc::_000011
    }
    #[doc = "(Sub clock)/25 or (LOCO clock)/25"]
    #[inline(always)]
    pub fn is_000100(&self) -> bool {
        *self == Lcdc::_000100
    }
    #[doc = "(Sub clock)/26 or (LOCO clock)/26"]
    #[inline(always)]
    pub fn is_000101(&self) -> bool {
        *self == Lcdc::_000101
    }
    #[doc = "(Sub clock)/27 or (LOCO clock)/27"]
    #[inline(always)]
    pub fn is_000110(&self) -> bool {
        *self == Lcdc::_000110
    }
    #[doc = "(Sub clock)/28 or (LOCO clock)/28"]
    #[inline(always)]
    pub fn is_000111(&self) -> bool {
        *self == Lcdc::_000111
    }
    #[doc = "(Sub clock)/29 or (LOCO clock)/29"]
    #[inline(always)]
    pub fn is_001000(&self) -> bool {
        *self == Lcdc::_001000
    }
    #[doc = "(Sub clock)/210 or (LOCO clock)/210"]
    #[inline(always)]
    pub fn is_001001(&self) -> bool {
        *self == Lcdc::_001001
    }
    #[doc = "(Main clock)/28 or (HOCO clock)/28"]
    #[inline(always)]
    pub fn is_010001(&self) -> bool {
        *self == Lcdc::_010001
    }
    #[doc = "(Main clock)/29 or (HOCO clock)/29"]
    #[inline(always)]
    pub fn is_010010(&self) -> bool {
        *self == Lcdc::_010010
    }
    #[doc = "(Main clock)/210 or (HOCO clock)/210"]
    #[inline(always)]
    pub fn is_010011(&self) -> bool {
        *self == Lcdc::_010011
    }
    #[doc = "(Main clock)/211 or (HOCO clock)/211"]
    #[inline(always)]
    pub fn is_010100(&self) -> bool {
        *self == Lcdc::_010100
    }
    #[doc = "(Main clock)/212 or (HOCO clock)/212"]
    #[inline(always)]
    pub fn is_010101(&self) -> bool {
        *self == Lcdc::_010101
    }
    #[doc = "(Main clock)/213 or (HOCO clock)/213"]
    #[inline(always)]
    pub fn is_010110(&self) -> bool {
        *self == Lcdc::_010110
    }
    #[doc = "(Main clock)/214 or (HOCO clock)/214"]
    #[inline(always)]
    pub fn is_010111(&self) -> bool {
        *self == Lcdc::_010111
    }
    #[doc = "(Main clock)/215 or (HOCO clock)/215"]
    #[inline(always)]
    pub fn is_011000(&self) -> bool {
        *self == Lcdc::_011000
    }
    #[doc = "(Main clock)/216 or (HOCO clock)/216"]
    #[inline(always)]
    pub fn is_011001(&self) -> bool {
        *self == Lcdc::_011001
    }
    #[doc = "(Main clock)/217 or (HOCO clock)/217"]
    #[inline(always)]
    pub fn is_011010(&self) -> bool {
        *self == Lcdc::_011010
    }
    #[doc = "(Main clock)/218 or (HOCO clock)/218"]
    #[inline(always)]
    pub fn is_011011(&self) -> bool {
        *self == Lcdc::_011011
    }
    #[doc = "(Main clock)/219 or (HOCO clock)/219"]
    #[inline(always)]
    pub fn is_101011(&self) -> bool {
        *self == Lcdc::_101011
    }
    #[doc = "Other than above Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Lcdc::Others)
    }
}
#[doc = "Field `LCDC` writer - LCD clock (LCDCL)"]
pub type LcdcW<'a, REG> = crate::FieldWriter<'a, REG, 6, Lcdc, crate::Safe>;
impl<'a, REG> LcdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "(Sub clock)/22 or (LOCO clock)/22"]
    #[inline(always)]
    pub fn _000001(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_000001)
    }
    #[doc = "(Sub clock)/23 or (LOCO clock)/23"]
    #[inline(always)]
    pub fn _000010(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_000010)
    }
    #[doc = "(Sub clock)/24 or (LOCO clock)/24"]
    #[inline(always)]
    pub fn _000011(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_000011)
    }
    #[doc = "(Sub clock)/25 or (LOCO clock)/25"]
    #[inline(always)]
    pub fn _000100(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_000100)
    }
    #[doc = "(Sub clock)/26 or (LOCO clock)/26"]
    #[inline(always)]
    pub fn _000101(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_000101)
    }
    #[doc = "(Sub clock)/27 or (LOCO clock)/27"]
    #[inline(always)]
    pub fn _000110(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_000110)
    }
    #[doc = "(Sub clock)/28 or (LOCO clock)/28"]
    #[inline(always)]
    pub fn _000111(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_000111)
    }
    #[doc = "(Sub clock)/29 or (LOCO clock)/29"]
    #[inline(always)]
    pub fn _001000(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_001000)
    }
    #[doc = "(Sub clock)/210 or (LOCO clock)/210"]
    #[inline(always)]
    pub fn _001001(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_001001)
    }
    #[doc = "(Main clock)/28 or (HOCO clock)/28"]
    #[inline(always)]
    pub fn _010001(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_010001)
    }
    #[doc = "(Main clock)/29 or (HOCO clock)/29"]
    #[inline(always)]
    pub fn _010010(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_010010)
    }
    #[doc = "(Main clock)/210 or (HOCO clock)/210"]
    #[inline(always)]
    pub fn _010011(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_010011)
    }
    #[doc = "(Main clock)/211 or (HOCO clock)/211"]
    #[inline(always)]
    pub fn _010100(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_010100)
    }
    #[doc = "(Main clock)/212 or (HOCO clock)/212"]
    #[inline(always)]
    pub fn _010101(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_010101)
    }
    #[doc = "(Main clock)/213 or (HOCO clock)/213"]
    #[inline(always)]
    pub fn _010110(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_010110)
    }
    #[doc = "(Main clock)/214 or (HOCO clock)/214"]
    #[inline(always)]
    pub fn _010111(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_010111)
    }
    #[doc = "(Main clock)/215 or (HOCO clock)/215"]
    #[inline(always)]
    pub fn _011000(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_011000)
    }
    #[doc = "(Main clock)/216 or (HOCO clock)/216"]
    #[inline(always)]
    pub fn _011001(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_011001)
    }
    #[doc = "(Main clock)/217 or (HOCO clock)/217"]
    #[inline(always)]
    pub fn _011010(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_011010)
    }
    #[doc = "(Main clock)/218 or (HOCO clock)/218"]
    #[inline(always)]
    pub fn _011011(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_011011)
    }
    #[doc = "(Main clock)/219 or (HOCO clock)/219"]
    #[inline(always)]
    pub fn _101011(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::_101011)
    }
    #[doc = "Other than above Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdc::Others)
    }
}
impl R {
    #[doc = "Bits 0:5 - LCD clock (LCDCL)"]
    #[inline(always)]
    pub fn lcdc(&self) -> LcdcR {
        LcdcR::new(self.bits & 0x3f)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCDC0").field("lcdc", &self.lcdc()).finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - LCD clock (LCDCL)"]
    #[inline(always)]
    pub fn lcdc(&mut self) -> LcdcW<Lcdc0Spec> {
        LcdcW::new(self, 0)
    }
}
#[doc = "LCD Clock Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdc0Spec;
impl crate::RegisterSpec for Lcdc0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdc0::R`](R) reader structure"]
impl crate::Readable for Lcdc0Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdc0::W`](W) writer structure"]
impl crate::Writable for Lcdc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC0 to value 0"]
impl crate::Resettable for Lcdc0Spec {}
