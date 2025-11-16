#[doc = "Register `RSTSR1` reader"]
pub type R = crate::R<Rstsr1Spec>;
#[doc = "Register `RSTSR1` writer"]
pub type W = crate::W<Rstsr1Spec>;
#[doc = "Independent Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iwdtrf {
    #[doc = "0: Independent watchdog timer reset not detected."]
    _0 = 0,
    #[doc = "1: Independent watchdog timer reset detected."]
    _1 = 1,
}
impl From<Iwdtrf> for bool {
    #[inline(always)]
    fn from(variant: Iwdtrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDTRF` reader - Independent Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type IwdtrfR = crate::BitReader<Iwdtrf>;
impl IwdtrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iwdtrf {
        match self.bits {
            false => Iwdtrf::_0,
            true => Iwdtrf::_1,
        }
    }
    #[doc = "Independent watchdog timer reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iwdtrf::_0
    }
    #[doc = "Independent watchdog timer reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iwdtrf::_1
    }
}
#[doc = "Field `IWDTRF` writer - Independent Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
pub type IwdtrfW<'a, REG> = crate::BitWriter0C<'a, REG, Iwdtrf>;
impl<'a, REG> IwdtrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Independent watchdog timer reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtrf::_0)
    }
    #[doc = "Independent watchdog timer reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtrf::_1)
    }
}
#[doc = "Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtrf {
    #[doc = "0: Watchdog timer reset not detected."]
    _0 = 0,
    #[doc = "1: Watchdog timer reset detected."]
    _1 = 1,
}
impl From<Wdtrf> for bool {
    #[inline(always)]
    fn from(variant: Wdtrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTRF` reader - Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type WdtrfR = crate::BitReader<Wdtrf>;
impl WdtrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtrf {
        match self.bits {
            false => Wdtrf::_0,
            true => Wdtrf::_1,
        }
    }
    #[doc = "Watchdog timer reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wdtrf::_0
    }
    #[doc = "Watchdog timer reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wdtrf::_1
    }
}
#[doc = "Field `WDTRF` writer - Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
pub type WdtrfW<'a, REG> = crate::BitWriter0C<'a, REG, Wdtrf>;
impl<'a, REG> WdtrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Watchdog timer reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtrf::_0)
    }
    #[doc = "Watchdog timer reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtrf::_1)
    }
}
#[doc = "Software Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrf {
    #[doc = "0: Software reset not detected."]
    _0 = 0,
    #[doc = "1: Software reset detected."]
    _1 = 1,
}
impl From<Swrf> for bool {
    #[inline(always)]
    fn from(variant: Swrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRF` reader - Software Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type SwrfR = crate::BitReader<Swrf>;
impl SwrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swrf {
        match self.bits {
            false => Swrf::_0,
            true => Swrf::_1,
        }
    }
    #[doc = "Software reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Swrf::_0
    }
    #[doc = "Software reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Swrf::_1
    }
}
#[doc = "Field `SWRF` writer - Software Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
pub type SwrfW<'a, REG> = crate::BitWriter0C<'a, REG, Swrf>;
impl<'a, REG> SwrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Swrf::_0)
    }
    #[doc = "Software reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Swrf::_1)
    }
}
#[doc = "RAM Parity Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rperf {
    #[doc = "0: RAM parity error reset not detected."]
    _0 = 0,
    #[doc = "1: RAM parity error reset detected."]
    _1 = 1,
}
impl From<Rperf> for bool {
    #[inline(always)]
    fn from(variant: Rperf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPERF` reader - RAM Parity Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type RperfR = crate::BitReader<Rperf>;
impl RperfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rperf {
        match self.bits {
            false => Rperf::_0,
            true => Rperf::_1,
        }
    }
    #[doc = "RAM parity error reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rperf::_0
    }
    #[doc = "RAM parity error reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rperf::_1
    }
}
#[doc = "Field `RPERF` writer - RAM Parity Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
pub type RperfW<'a, REG> = crate::BitWriter0C<'a, REG, Rperf>;
impl<'a, REG> RperfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RAM parity error reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rperf::_0)
    }
    #[doc = "RAM parity error reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rperf::_1)
    }
}
#[doc = "RAM ECC Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reerf {
    #[doc = "0: RAM ECC error reset not detected."]
    _0 = 0,
    #[doc = "1: RAM ECC error reset detected."]
    _1 = 1,
}
impl From<Reerf> for bool {
    #[inline(always)]
    fn from(variant: Reerf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REERF` reader - RAM ECC Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type ReerfR = crate::BitReader<Reerf>;
impl ReerfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reerf {
        match self.bits {
            false => Reerf::_0,
            true => Reerf::_1,
        }
    }
    #[doc = "RAM ECC error reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Reerf::_0
    }
    #[doc = "RAM ECC error reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Reerf::_1
    }
}
#[doc = "Field `REERF` writer - RAM ECC Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
pub type ReerfW<'a, REG> = crate::BitWriter0C<'a, REG, Reerf>;
impl<'a, REG> ReerfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RAM ECC error reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Reerf::_0)
    }
    #[doc = "RAM ECC error reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Reerf::_1)
    }
}
#[doc = "Bus Slave MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bussrf {
    #[doc = "0: Bus Slave MPU reset not detected."]
    _0 = 0,
    #[doc = "1: Bus Slave MPU reset detected."]
    _1 = 1,
}
impl From<Bussrf> for bool {
    #[inline(always)]
    fn from(variant: Bussrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSSRF` reader - Bus Slave MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type BussrfR = crate::BitReader<Bussrf>;
impl BussrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bussrf {
        match self.bits {
            false => Bussrf::_0,
            true => Bussrf::_1,
        }
    }
    #[doc = "Bus Slave MPU reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bussrf::_0
    }
    #[doc = "Bus Slave MPU reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bussrf::_1
    }
}
#[doc = "Field `BUSSRF` writer - Bus Slave MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
pub type BussrfW<'a, REG> = crate::BitWriter0C<'a, REG, Bussrf>;
impl<'a, REG> BussrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus Slave MPU reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bussrf::_0)
    }
    #[doc = "Bus Slave MPU reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bussrf::_1)
    }
}
#[doc = "Bus Master MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busmrf {
    #[doc = "0: Bus Master MPU reset not detected."]
    _0 = 0,
    #[doc = "1: Bus Master MPU reset detected."]
    _1 = 1,
}
impl From<Busmrf> for bool {
    #[inline(always)]
    fn from(variant: Busmrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSMRF` reader - Bus Master MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type BusmrfR = crate::BitReader<Busmrf>;
impl BusmrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busmrf {
        match self.bits {
            false => Busmrf::_0,
            true => Busmrf::_1,
        }
    }
    #[doc = "Bus Master MPU reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Busmrf::_0
    }
    #[doc = "Bus Master MPU reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Busmrf::_1
    }
}
#[doc = "Field `BUSMRF` writer - Bus Master MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
pub type BusmrfW<'a, REG> = crate::BitWriter0C<'a, REG, Busmrf>;
impl<'a, REG> BusmrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus Master MPU reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Busmrf::_0)
    }
    #[doc = "Bus Master MPU reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Busmrf::_1)
    }
}
#[doc = "SP Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sperf {
    #[doc = "0: SP error reset not detected."]
    _0 = 0,
    #[doc = "1: SP error reset detected."]
    _1 = 1,
}
impl From<Sperf> for bool {
    #[inline(always)]
    fn from(variant: Sperf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPERF` reader - SP Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type SperfR = crate::BitReader<Sperf>;
impl SperfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sperf {
        match self.bits {
            false => Sperf::_0,
            true => Sperf::_1,
        }
    }
    #[doc = "SP error reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sperf::_0
    }
    #[doc = "SP error reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sperf::_1
    }
}
#[doc = "Field `SPERF` writer - SP Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
pub type SperfW<'a, REG> = crate::BitWriter0C<'a, REG, Sperf>;
impl<'a, REG> SperfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SP error reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sperf::_0)
    }
    #[doc = "SP error reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sperf::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Independent Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn iwdtrf(&self) -> IwdtrfR {
        IwdtrfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn wdtrf(&self) -> WdtrfR {
        WdtrfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn swrf(&self) -> SwrfR {
        SwrfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - RAM Parity Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn rperf(&self) -> RperfR {
        RperfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RAM ECC Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn reerf(&self) -> ReerfR {
        ReerfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus Slave MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn bussrf(&self) -> BussrfR {
        BussrfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus Master MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn busmrf(&self) -> BusmrfR {
        BusmrfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SP Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn sperf(&self) -> SperfR {
        SperfR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTSR1").finish()
    }
}
impl W {
    #[doc = "Bit 0 - Independent Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn iwdtrf(&mut self) -> IwdtrfW<Rstsr1Spec> {
        IwdtrfW::new(self, 0)
    }
    #[doc = "Bit 1 - Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn wdtrf(&mut self) -> WdtrfW<Rstsr1Spec> {
        WdtrfW::new(self, 1)
    }
    #[doc = "Bit 2 - Software Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn swrf(&mut self) -> SwrfW<Rstsr1Spec> {
        SwrfW::new(self, 2)
    }
    #[doc = "Bit 8 - RAM Parity Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn rperf(&mut self) -> RperfW<Rstsr1Spec> {
        RperfW::new(self, 8)
    }
    #[doc = "Bit 9 - RAM ECC Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn reerf(&mut self) -> ReerfW<Rstsr1Spec> {
        ReerfW::new(self, 9)
    }
    #[doc = "Bit 10 - Bus Slave MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn bussrf(&mut self) -> BussrfW<Rstsr1Spec> {
        BussrfW::new(self, 10)
    }
    #[doc = "Bit 11 - Bus Master MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn busmrf(&mut self) -> BusmrfW<Rstsr1Spec> {
        BusmrfW::new(self, 11)
    }
    #[doc = "Bit 12 - SP Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn sperf(&mut self) -> SperfW<Rstsr1Spec> {
        SperfW::new(self, 12)
    }
}
#[doc = "Reset Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rstsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rstsr1Spec;
impl crate::RegisterSpec for Rstsr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rstsr1::R`](R) reader structure"]
impl crate::Readable for Rstsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`rstsr1::W`](W) writer structure"]
impl crate::Writable for Rstsr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0x1f07;
}
#[doc = "`reset()` method sets RSTSR1 to value 0"]
impl crate::Resettable for Rstsr1Spec {}
