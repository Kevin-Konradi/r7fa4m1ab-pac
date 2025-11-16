#[doc = "Register `WDTCR` reader"]
pub type R = crate::R<WdtcrSpec>;
#[doc = "Register `WDTCR` writer"]
pub type W = crate::W<WdtcrSpec>;
#[doc = "Timeout Period Selection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tops {
    #[doc = "0: 1,024 cycles (03FFh)"]
    _00 = 0,
    #[doc = "1: 4,096 cycles (0FFFh)"]
    _01 = 1,
    #[doc = "2: 8,192 cycles (1FFFh)"]
    _10 = 2,
    #[doc = "3: 16,384 cycles (3FFFh)"]
    _11 = 3,
}
impl From<Tops> for u8 {
    #[inline(always)]
    fn from(variant: Tops) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tops {
    type Ux = u8;
}
impl crate::IsEnum for Tops {}
#[doc = "Field `TOPS` reader - Timeout Period Selection"]
pub type TopsR = crate::FieldReader<Tops>;
impl TopsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tops {
        match self.bits {
            0 => Tops::_00,
            1 => Tops::_01,
            2 => Tops::_10,
            3 => Tops::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "1,024 cycles (03FFh)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Tops::_00
    }
    #[doc = "4,096 cycles (0FFFh)"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Tops::_01
    }
    #[doc = "8,192 cycles (1FFFh)"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Tops::_10
    }
    #[doc = "16,384 cycles (3FFFh)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Tops::_11
    }
}
#[doc = "Field `TOPS` writer - Timeout Period Selection"]
pub type TopsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tops, crate::Safe>;
impl<'a, REG> TopsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1,024 cycles (03FFh)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Tops::_00)
    }
    #[doc = "4,096 cycles (0FFFh)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Tops::_01)
    }
    #[doc = "8,192 cycles (1FFFh)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Tops::_10)
    }
    #[doc = "16,384 cycles (3FFFh)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Tops::_11)
    }
}
#[doc = "Clock Division Ratio Selection\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cks {
    #[doc = "1: PCLK/4"]
    _0001 = 1,
    #[doc = "4: PCLK/64"]
    _0100 = 4,
    #[doc = "15: PCLK/128"]
    _1111 = 15,
    #[doc = "6: PCLK/512"]
    _0110 = 6,
    #[doc = "7: PCLK/2048"]
    _0111 = 7,
    #[doc = "8: PCLK/8192"]
    _1000 = 8,
    #[doc = "0: setting prohibited"]
    Others = 0,
}
impl From<Cks> for u8 {
    #[inline(always)]
    fn from(variant: Cks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cks {
    type Ux = u8;
}
impl crate::IsEnum for Cks {}
#[doc = "Field `CKS` reader - Clock Division Ratio Selection"]
pub type CksR = crate::FieldReader<Cks>;
impl CksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cks {
        match self.bits {
            1 => Cks::_0001,
            4 => Cks::_0100,
            15 => Cks::_1111,
            6 => Cks::_0110,
            7 => Cks::_0111,
            8 => Cks::_1000,
            _ => Cks::Others,
        }
    }
    #[doc = "PCLK/4"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == Cks::_0001
    }
    #[doc = "PCLK/64"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == Cks::_0100
    }
    #[doc = "PCLK/128"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == Cks::_1111
    }
    #[doc = "PCLK/512"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == Cks::_0110
    }
    #[doc = "PCLK/2048"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == Cks::_0111
    }
    #[doc = "PCLK/8192"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == Cks::_1000
    }
    #[doc = "setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Cks::Others)
    }
}
#[doc = "Field `CKS` writer - Clock Division Ratio Selection"]
pub type CksW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cks, crate::Safe>;
impl<'a, REG> CksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK/4"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_0001)
    }
    #[doc = "PCLK/64"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_0100)
    }
    #[doc = "PCLK/128"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_1111)
    }
    #[doc = "PCLK/512"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_0110)
    }
    #[doc = "PCLK/2048"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_0111)
    }
    #[doc = "PCLK/8192"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_1000)
    }
    #[doc = "setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::Others)
    }
}
#[doc = "Window End Position Selection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rpes {
    #[doc = "0: 75 percent"]
    _00 = 0,
    #[doc = "1: 50 percent"]
    _01 = 1,
    #[doc = "2: 25 percent"]
    _10 = 2,
    #[doc = "3: 0 percent (window end position is not specified)"]
    _11 = 3,
}
impl From<Rpes> for u8 {
    #[inline(always)]
    fn from(variant: Rpes) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rpes {
    type Ux = u8;
}
impl crate::IsEnum for Rpes {}
#[doc = "Field `RPES` reader - Window End Position Selection"]
pub type RpesR = crate::FieldReader<Rpes>;
impl RpesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpes {
        match self.bits {
            0 => Rpes::_00,
            1 => Rpes::_01,
            2 => Rpes::_10,
            3 => Rpes::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "75 percent"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Rpes::_00
    }
    #[doc = "50 percent"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Rpes::_01
    }
    #[doc = "25 percent"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Rpes::_10
    }
    #[doc = "0 percent (window end position is not specified)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Rpes::_11
    }
}
#[doc = "Field `RPES` writer - Window End Position Selection"]
pub type RpesW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rpes, crate::Safe>;
impl<'a, REG> RpesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "75 percent"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Rpes::_00)
    }
    #[doc = "50 percent"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Rpes::_01)
    }
    #[doc = "25 percent"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Rpes::_10)
    }
    #[doc = "0 percent (window end position is not specified)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Rpes::_11)
    }
}
#[doc = "Window Start Position Selection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rpss {
    #[doc = "0: 25 percent"]
    _00 = 0,
    #[doc = "1: 50 percent"]
    _01 = 1,
    #[doc = "2: 75 percent"]
    _10 = 2,
    #[doc = "3: 100 percent (window start position is not specified)"]
    _11 = 3,
}
impl From<Rpss> for u8 {
    #[inline(always)]
    fn from(variant: Rpss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rpss {
    type Ux = u8;
}
impl crate::IsEnum for Rpss {}
#[doc = "Field `RPSS` reader - Window Start Position Selection"]
pub type RpssR = crate::FieldReader<Rpss>;
impl RpssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpss {
        match self.bits {
            0 => Rpss::_00,
            1 => Rpss::_01,
            2 => Rpss::_10,
            3 => Rpss::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "25 percent"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Rpss::_00
    }
    #[doc = "50 percent"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Rpss::_01
    }
    #[doc = "75 percent"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Rpss::_10
    }
    #[doc = "100 percent (window start position is not specified)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Rpss::_11
    }
}
#[doc = "Field `RPSS` writer - Window Start Position Selection"]
pub type RpssW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rpss, crate::Safe>;
impl<'a, REG> RpssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "25 percent"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Rpss::_00)
    }
    #[doc = "50 percent"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Rpss::_01)
    }
    #[doc = "75 percent"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Rpss::_10)
    }
    #[doc = "100 percent (window start position is not specified)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Rpss::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Timeout Period Selection"]
    #[inline(always)]
    pub fn tops(&self) -> TopsR {
        TopsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - Clock Division Ratio Selection"]
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Window End Position Selection"]
    #[inline(always)]
    pub fn rpes(&self) -> RpesR {
        RpesR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Window Start Position Selection"]
    #[inline(always)]
    pub fn rpss(&self) -> RpssR {
        RpssR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCR")
            .field("rpss", &self.rpss())
            .field("rpes", &self.rpes())
            .field("cks", &self.cks())
            .field("tops", &self.tops())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Timeout Period Selection"]
    #[inline(always)]
    pub fn tops(&mut self) -> TopsW<WdtcrSpec> {
        TopsW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Clock Division Ratio Selection"]
    #[inline(always)]
    pub fn cks(&mut self) -> CksW<WdtcrSpec> {
        CksW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Window End Position Selection"]
    #[inline(always)]
    pub fn rpes(&mut self) -> RpesW<WdtcrSpec> {
        RpesW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Window Start Position Selection"]
    #[inline(always)]
    pub fn rpss(&mut self) -> RpssW<WdtcrSpec> {
        RpssW::new(self, 12)
    }
}
#[doc = "WDT Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtcrSpec;
impl crate::RegisterSpec for WdtcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wdtcr::R`](R) reader structure"]
impl crate::Readable for WdtcrSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtcr::W`](W) writer structure"]
impl crate::Writable for WdtcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDTCR to value 0x33f3"]
impl crate::Resettable for WdtcrSpec {
    const RESET_VALUE: u16 = 0x33f3;
}
