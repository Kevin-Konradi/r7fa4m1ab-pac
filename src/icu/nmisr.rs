#[doc = "Register `NMISR` reader"]
pub type R = crate::R<NmisrSpec>;
#[doc = "IWDT Underflow/Refresh Error Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iwdtst {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested."]
    _1 = 1,
}
impl From<Iwdtst> for bool {
    #[inline(always)]
    fn from(variant: Iwdtst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDTST` reader - IWDT Underflow/Refresh Error Status Flag"]
pub type IwdtstR = crate::BitReader<Iwdtst>;
impl IwdtstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iwdtst {
        match self.bits {
            false => Iwdtst::_0,
            true => Iwdtst::_1,
        }
    }
    #[doc = "Interrupt not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iwdtst::_0
    }
    #[doc = "Interrupt requested."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iwdtst::_1
    }
}
#[doc = "WDT Underflow/Refresh Error Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtst {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested."]
    _1 = 1,
}
impl From<Wdtst> for bool {
    #[inline(always)]
    fn from(variant: Wdtst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTST` reader - WDT Underflow/Refresh Error Status Flag"]
pub type WdtstR = crate::BitReader<Wdtst>;
impl WdtstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtst {
        match self.bits {
            false => Wdtst::_0,
            true => Wdtst::_1,
        }
    }
    #[doc = "Interrupt not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wdtst::_0
    }
    #[doc = "Interrupt requested."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wdtst::_1
    }
}
#[doc = "Voltage-Monitoring 1 Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd1st {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested."]
    _1 = 1,
}
impl From<Lvd1st> for bool {
    #[inline(always)]
    fn from(variant: Lvd1st) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD1ST` reader - Voltage-Monitoring 1 Interrupt Status Flag"]
pub type Lvd1stR = crate::BitReader<Lvd1st>;
impl Lvd1stR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd1st {
        match self.bits {
            false => Lvd1st::_0,
            true => Lvd1st::_1,
        }
    }
    #[doc = "Interrupt not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd1st::_0
    }
    #[doc = "Interrupt requested."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd1st::_1
    }
}
#[doc = "Voltage-Monitoring 2 Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd2st {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested."]
    _1 = 1,
}
impl From<Lvd2st> for bool {
    #[inline(always)]
    fn from(variant: Lvd2st) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD2ST` reader - Voltage-Monitoring 2 Interrupt Status Flag"]
pub type Lvd2stR = crate::BitReader<Lvd2st>;
impl Lvd2stR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd2st {
        match self.bits {
            false => Lvd2st::_0,
            true => Lvd2st::_1,
        }
    }
    #[doc = "Interrupt not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd2st::_0
    }
    #[doc = "Interrupt requested."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd2st::_1
    }
}
#[doc = "VBATT monitor Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbattst {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested."]
    _1 = 1,
}
impl From<Vbattst> for bool {
    #[inline(always)]
    fn from(variant: Vbattst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATTST` reader - VBATT monitor Interrupt Status Flag"]
pub type VbattstR = crate::BitReader<Vbattst>;
impl VbattstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbattst {
        match self.bits {
            false => Vbattst::_0,
            true => Vbattst::_1,
        }
    }
    #[doc = "Interrupt not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbattst::_0
    }
    #[doc = "Interrupt requested."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbattst::_1
    }
}
#[doc = "Oscillation Stop Detection Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ostst {
    #[doc = "0: Interrupt not requested for main oscillation stop"]
    _0 = 0,
    #[doc = "1: Interrupt requested for main oscillation stop."]
    _1 = 1,
}
impl From<Ostst> for bool {
    #[inline(always)]
    fn from(variant: Ostst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSTST` reader - Oscillation Stop Detection Interrupt Status Flag"]
pub type OststR = crate::BitReader<Ostst>;
impl OststR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ostst {
        match self.bits {
            false => Ostst::_0,
            true => Ostst::_1,
        }
    }
    #[doc = "Interrupt not requested for main oscillation stop"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ostst::_0
    }
    #[doc = "Interrupt requested for main oscillation stop."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ostst::_1
    }
}
#[doc = "NMI Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nmist {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested."]
    _1 = 1,
}
impl From<Nmist> for bool {
    #[inline(always)]
    fn from(variant: Nmist) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIST` reader - NMI Status Flag"]
pub type NmistR = crate::BitReader<Nmist>;
impl NmistR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nmist {
        match self.bits {
            false => Nmist::_0,
            true => Nmist::_1,
        }
    }
    #[doc = "Interrupt not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nmist::_0
    }
    #[doc = "Interrupt requested."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nmist::_1
    }
}
#[doc = "RAM Parity Error Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpest {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested."]
    _1 = 1,
}
impl From<Rpest> for bool {
    #[inline(always)]
    fn from(variant: Rpest) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPEST` reader - RAM Parity Error Interrupt Status Flag"]
pub type RpestR = crate::BitReader<Rpest>;
impl RpestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpest {
        match self.bits {
            false => Rpest::_0,
            true => Rpest::_1,
        }
    }
    #[doc = "Interrupt not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rpest::_0
    }
    #[doc = "Interrupt requested."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rpest::_1
    }
}
#[doc = "RAM ECC Error Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reccst {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested."]
    _1 = 1,
}
impl From<Reccst> for bool {
    #[inline(always)]
    fn from(variant: Reccst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECCST` reader - RAM ECC Error Interrupt Status Flag"]
pub type ReccstR = crate::BitReader<Reccst>;
impl ReccstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reccst {
        match self.bits {
            false => Reccst::_0,
            true => Reccst::_1,
        }
    }
    #[doc = "Interrupt not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Reccst::_0
    }
    #[doc = "Interrupt requested."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Reccst::_1
    }
}
#[doc = "MPU Bus Slave Error Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bussst {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested."]
    _1 = 1,
}
impl From<Bussst> for bool {
    #[inline(always)]
    fn from(variant: Bussst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSSST` reader - MPU Bus Slave Error Interrupt Status Flag"]
pub type BussstR = crate::BitReader<Bussst>;
impl BussstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bussst {
        match self.bits {
            false => Bussst::_0,
            true => Bussst::_1,
        }
    }
    #[doc = "Interrupt not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bussst::_0
    }
    #[doc = "Interrupt requested."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bussst::_1
    }
}
#[doc = "MPU Bus Master Error Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busmst {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested."]
    _1 = 1,
}
impl From<Busmst> for bool {
    #[inline(always)]
    fn from(variant: Busmst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSMST` reader - MPU Bus Master Error Interrupt Status Flag"]
pub type BusmstR = crate::BitReader<Busmst>;
impl BusmstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busmst {
        match self.bits {
            false => Busmst::_0,
            true => Busmst::_1,
        }
    }
    #[doc = "Interrupt not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Busmst::_0
    }
    #[doc = "Interrupt requested."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Busmst::_1
    }
}
#[doc = "CPU Stack pointer monitor Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spest {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested."]
    _1 = 1,
}
impl From<Spest> for bool {
    #[inline(always)]
    fn from(variant: Spest) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPEST` reader - CPU Stack pointer monitor Interrupt Status Flag"]
pub type SpestR = crate::BitReader<Spest>;
impl SpestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spest {
        match self.bits {
            false => Spest::_0,
            true => Spest::_1,
        }
    }
    #[doc = "Interrupt not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spest::_0
    }
    #[doc = "Interrupt requested."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spest::_1
    }
}
impl R {
    #[doc = "Bit 0 - IWDT Underflow/Refresh Error Status Flag"]
    #[inline(always)]
    pub fn iwdtst(&self) -> IwdtstR {
        IwdtstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDT Underflow/Refresh Error Status Flag"]
    #[inline(always)]
    pub fn wdtst(&self) -> WdtstR {
        WdtstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage-Monitoring 1 Interrupt Status Flag"]
    #[inline(always)]
    pub fn lvd1st(&self) -> Lvd1stR {
        Lvd1stR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage-Monitoring 2 Interrupt Status Flag"]
    #[inline(always)]
    pub fn lvd2st(&self) -> Lvd2stR {
        Lvd2stR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VBATT monitor Interrupt Status Flag"]
    #[inline(always)]
    pub fn vbattst(&self) -> VbattstR {
        VbattstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Oscillation Stop Detection Interrupt Status Flag"]
    #[inline(always)]
    pub fn ostst(&self) -> OststR {
        OststR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NMI Status Flag"]
    #[inline(always)]
    pub fn nmist(&self) -> NmistR {
        NmistR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RAM Parity Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn rpest(&self) -> RpestR {
        RpestR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RAM ECC Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn reccst(&self) -> ReccstR {
        ReccstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MPU Bus Slave Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn bussst(&self) -> BussstR {
        BussstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MPU Bus Master Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn busmst(&self) -> BusmstR {
        BusmstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU Stack pointer monitor Interrupt Status Flag"]
    #[inline(always)]
    pub fn spest(&self) -> SpestR {
        SpestR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NMISR")
            .field("spest", &self.spest())
            .field("busmst", &self.busmst())
            .field("bussst", &self.bussst())
            .field("reccst", &self.reccst())
            .field("rpest", &self.rpest())
            .field("nmist", &self.nmist())
            .field("ostst", &self.ostst())
            .field("vbattst", &self.vbattst())
            .field("lvd2st", &self.lvd2st())
            .field("lvd1st", &self.lvd1st())
            .field("wdtst", &self.wdtst())
            .field("iwdtst", &self.iwdtst())
            .finish()
    }
}
#[doc = "Non-Maskable Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmisrSpec;
impl crate::RegisterSpec for NmisrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`nmisr::R`](R) reader structure"]
impl crate::Readable for NmisrSpec {}
#[doc = "`reset()` method sets NMISR to value 0"]
impl crate::Resettable for NmisrSpec {}
