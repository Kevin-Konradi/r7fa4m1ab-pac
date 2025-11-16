#[doc = "Register `DBGSTOPCR` reader"]
pub type R = crate::R<DbgstopcrSpec>;
#[doc = "Register `DBGSTOPCR` writer"]
pub type W = crate::W<DbgstopcrSpec>;
#[doc = "Mask bit for IWDT reset/interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopIwdt {
    #[doc = "0: Mask IWDT reset/interrupt"]
    _0 = 0,
    #[doc = "1: Enable IWDT reset"]
    _1 = 1,
}
impl From<DbgstopIwdt> for bool {
    #[inline(always)]
    fn from(variant: DbgstopIwdt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGSTOP_IWDT` reader - Mask bit for IWDT reset/interrupt"]
pub type DbgstopIwdtR = crate::BitReader<DbgstopIwdt>;
impl DbgstopIwdtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopIwdt {
        match self.bits {
            false => DbgstopIwdt::_0,
            true => DbgstopIwdt::_1,
        }
    }
    #[doc = "Mask IWDT reset/interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopIwdt::_0
    }
    #[doc = "Enable IWDT reset"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopIwdt::_1
    }
}
#[doc = "Field `DBGSTOP_IWDT` writer - Mask bit for IWDT reset/interrupt"]
pub type DbgstopIwdtW<'a, REG> = crate::BitWriter<'a, REG, DbgstopIwdt>;
impl<'a, REG> DbgstopIwdtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mask IWDT reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopIwdt::_0)
    }
    #[doc = "Enable IWDT reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopIwdt::_1)
    }
}
#[doc = "Mask bit for WDT reset/interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopWdt {
    #[doc = "0: Mask WDT reset/interrupt"]
    _0 = 0,
    #[doc = "1: Enable WDT reset"]
    _1 = 1,
}
impl From<DbgstopWdt> for bool {
    #[inline(always)]
    fn from(variant: DbgstopWdt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGSTOP_WDT` reader - Mask bit for WDT reset/interrupt"]
pub type DbgstopWdtR = crate::BitReader<DbgstopWdt>;
impl DbgstopWdtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopWdt {
        match self.bits {
            false => DbgstopWdt::_0,
            true => DbgstopWdt::_1,
        }
    }
    #[doc = "Mask WDT reset/interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopWdt::_0
    }
    #[doc = "Enable WDT reset"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopWdt::_1
    }
}
#[doc = "Field `DBGSTOP_WDT` writer - Mask bit for WDT reset/interrupt"]
pub type DbgstopWdtW<'a, REG> = crate::BitWriter<'a, REG, DbgstopWdt>;
impl<'a, REG> DbgstopWdtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mask WDT reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopWdt::_0)
    }
    #[doc = "Enable WDT reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopWdt::_1)
    }
}
#[doc = "Field `DBGSTOP_LVD` reader - b18: Mask bit for LVD2 reset/interrupt (0:enable / 1:Mask) b17: Mask bit for LVD1 reset/interrupt (0:enable / 1:Mask) b16: Mask bit for LVD0 reset (0:enable / 1:Mask)"]
pub type DbgstopLvdR = crate::FieldReader;
#[doc = "Field `DBGSTOP_LVD` writer - b18: Mask bit for LVD2 reset/interrupt (0:enable / 1:Mask) b17: Mask bit for LVD1 reset/interrupt (0:enable / 1:Mask) b16: Mask bit for LVD0 reset (0:enable / 1:Mask)"]
pub type DbgstopLvdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Mask bit for RAM parity error reset/interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopRper {
    #[doc = "0: Enable RAM parity error reset/interrupt"]
    _0 = 0,
    #[doc = "1: Mask RAM parity error reset/interrupt"]
    _1 = 1,
}
impl From<DbgstopRper> for bool {
    #[inline(always)]
    fn from(variant: DbgstopRper) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGSTOP_RPER` reader - Mask bit for RAM parity error reset/interrupt"]
pub type DbgstopRperR = crate::BitReader<DbgstopRper>;
impl DbgstopRperR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopRper {
        match self.bits {
            false => DbgstopRper::_0,
            true => DbgstopRper::_1,
        }
    }
    #[doc = "Enable RAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopRper::_0
    }
    #[doc = "Mask RAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopRper::_1
    }
}
#[doc = "Field `DBGSTOP_RPER` writer - Mask bit for RAM parity error reset/interrupt"]
pub type DbgstopRperW<'a, REG> = crate::BitWriter<'a, REG, DbgstopRper>;
impl<'a, REG> DbgstopRperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable RAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopRper::_0)
    }
    #[doc = "Mask RAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopRper::_1)
    }
}
#[doc = "Mask bit for RAM ECC error reset/interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopReccr {
    #[doc = "0: Enable RAM ECC error reset/interrupt"]
    _0 = 0,
    #[doc = "1: Mask RAM ECC error reset/interrupt"]
    _1 = 1,
}
impl From<DbgstopReccr> for bool {
    #[inline(always)]
    fn from(variant: DbgstopReccr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGSTOP_RECCR` reader - Mask bit for RAM ECC error reset/interrupt"]
pub type DbgstopReccrR = crate::BitReader<DbgstopReccr>;
impl DbgstopReccrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopReccr {
        match self.bits {
            false => DbgstopReccr::_0,
            true => DbgstopReccr::_1,
        }
    }
    #[doc = "Enable RAM ECC error reset/interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopReccr::_0
    }
    #[doc = "Mask RAM ECC error reset/interrupt"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopReccr::_1
    }
}
#[doc = "Field `DBGSTOP_RECCR` writer - Mask bit for RAM ECC error reset/interrupt"]
pub type DbgstopReccrW<'a, REG> = crate::BitWriter<'a, REG, DbgstopReccr>;
impl<'a, REG> DbgstopReccrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable RAM ECC error reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopReccr::_0)
    }
    #[doc = "Mask RAM ECC error reset/interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopReccr::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Mask bit for IWDT reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_iwdt(&self) -> DbgstopIwdtR {
        DbgstopIwdtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for WDT reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_wdt(&self) -> DbgstopWdtR {
        DbgstopWdtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:18 - b18: Mask bit for LVD2 reset/interrupt (0:enable / 1:Mask) b17: Mask bit for LVD1 reset/interrupt (0:enable / 1:Mask) b16: Mask bit for LVD0 reset (0:enable / 1:Mask)"]
    #[inline(always)]
    pub fn dbgstop_lvd(&self) -> DbgstopLvdR {
        DbgstopLvdR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - Mask bit for RAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_rper(&self) -> DbgstopRperR {
        DbgstopRperR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Mask bit for RAM ECC error reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_reccr(&self) -> DbgstopReccrR {
        DbgstopReccrR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGSTOPCR")
            .field("dbgstop_reccr", &self.dbgstop_reccr())
            .field("dbgstop_rper", &self.dbgstop_rper())
            .field("dbgstop_lvd", &self.dbgstop_lvd())
            .field("dbgstop_wdt", &self.dbgstop_wdt())
            .field("dbgstop_iwdt", &self.dbgstop_iwdt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for IWDT reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_iwdt(&mut self) -> DbgstopIwdtW<DbgstopcrSpec> {
        DbgstopIwdtW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask bit for WDT reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_wdt(&mut self) -> DbgstopWdtW<DbgstopcrSpec> {
        DbgstopWdtW::new(self, 1)
    }
    #[doc = "Bits 16:18 - b18: Mask bit for LVD2 reset/interrupt (0:enable / 1:Mask) b17: Mask bit for LVD1 reset/interrupt (0:enable / 1:Mask) b16: Mask bit for LVD0 reset (0:enable / 1:Mask)"]
    #[inline(always)]
    pub fn dbgstop_lvd(&mut self) -> DbgstopLvdW<DbgstopcrSpec> {
        DbgstopLvdW::new(self, 16)
    }
    #[doc = "Bit 24 - Mask bit for RAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_rper(&mut self) -> DbgstopRperW<DbgstopcrSpec> {
        DbgstopRperW::new(self, 24)
    }
    #[doc = "Bit 25 - Mask bit for RAM ECC error reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_reccr(&mut self) -> DbgstopReccrW<DbgstopcrSpec> {
        DbgstopReccrW::new(self, 25)
    }
}
#[doc = "Debug Stop Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgstopcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgstopcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgstopcrSpec;
impl crate::RegisterSpec for DbgstopcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgstopcr::R`](R) reader structure"]
impl crate::Readable for DbgstopcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgstopcr::W`](W) writer structure"]
impl crate::Writable for DbgstopcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBGSTOPCR to value 0x03"]
impl crate::Resettable for DbgstopcrSpec {
    const RESET_VALUE: u32 = 0x03;
}
