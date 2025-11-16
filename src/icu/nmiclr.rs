#[doc = "Register `NMICLR` reader"]
pub type R = crate::R<NmiclrSpec>;
#[doc = "Register `NMICLR` writer"]
pub type W = crate::W<NmiclrSpec>;
#[doc = "IWDT Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iwdtclr {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.IWDTST flag."]
    _1 = 1,
}
impl From<Iwdtclr> for bool {
    #[inline(always)]
    fn from(variant: Iwdtclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDTCLR` writer - IWDT Clear"]
pub type IwdtclrW<'a, REG> = crate::BitWriter<'a, REG, Iwdtclr>;
impl<'a, REG> IwdtclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtclr::_0)
    }
    #[doc = "Clear the NMISR.IWDTST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtclr::_1)
    }
}
#[doc = "WDT Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtclr {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.WDTST flag."]
    _1 = 1,
}
impl From<Wdtclr> for bool {
    #[inline(always)]
    fn from(variant: Wdtclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCLR` writer - WDT Clear"]
pub type WdtclrW<'a, REG> = crate::BitWriter<'a, REG, Wdtclr>;
impl<'a, REG> WdtclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtclr::_0)
    }
    #[doc = "Clear the NMISR.WDTST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtclr::_1)
    }
}
#[doc = "LVD1 Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd1clr {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.LVD1ST flag."]
    _1 = 1,
}
impl From<Lvd1clr> for bool {
    #[inline(always)]
    fn from(variant: Lvd1clr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD1CLR` writer - LVD1 Clear"]
pub type Lvd1clrW<'a, REG> = crate::BitWriter<'a, REG, Lvd1clr>;
impl<'a, REG> Lvd1clrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1clr::_0)
    }
    #[doc = "Clear the NMISR.LVD1ST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1clr::_1)
    }
}
#[doc = "LVD2 Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd2clr {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.LVD2ST flag."]
    _1 = 1,
}
impl From<Lvd2clr> for bool {
    #[inline(always)]
    fn from(variant: Lvd2clr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD2CLR` writer - LVD2 Clear"]
pub type Lvd2clrW<'a, REG> = crate::BitWriter<'a, REG, Lvd2clr>;
impl<'a, REG> Lvd2clrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2clr::_0)
    }
    #[doc = "Clear the NMISR.LVD2ST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2clr::_1)
    }
}
#[doc = "VBATT Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbattclr {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.VBATTST flag."]
    _1 = 1,
}
impl From<Vbattclr> for bool {
    #[inline(always)]
    fn from(variant: Vbattclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATTCLR` writer - VBATT Clear"]
pub type VbattclrW<'a, REG> = crate::BitWriter<'a, REG, Vbattclr>;
impl<'a, REG> VbattclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbattclr::_0)
    }
    #[doc = "Clear the NMISR.VBATTST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbattclr::_1)
    }
}
#[doc = "OST Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ostclr {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.OSTST flag."]
    _1 = 1,
}
impl From<Ostclr> for bool {
    #[inline(always)]
    fn from(variant: Ostclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSTCLR` writer - OST Clear"]
pub type OstclrW<'a, REG> = crate::BitWriter<'a, REG, Ostclr>;
impl<'a, REG> OstclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ostclr::_0)
    }
    #[doc = "Clear the NMISR.OSTST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ostclr::_1)
    }
}
#[doc = "NMI Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nmiclr {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.NMIST flag."]
    _1 = 1,
}
impl From<Nmiclr> for bool {
    #[inline(always)]
    fn from(variant: Nmiclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMICLR` writer - NMI Clear"]
pub type NmiclrW<'a, REG> = crate::BitWriter<'a, REG, Nmiclr>;
impl<'a, REG> NmiclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nmiclr::_0)
    }
    #[doc = "Clear the NMISR.NMIST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nmiclr::_1)
    }
}
#[doc = "SRAM Parity Error Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpeclr {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.RPEST flag."]
    _1 = 1,
}
impl From<Rpeclr> for bool {
    #[inline(always)]
    fn from(variant: Rpeclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPECLR` writer - SRAM Parity Error Clear"]
pub type RpeclrW<'a, REG> = crate::BitWriter<'a, REG, Rpeclr>;
impl<'a, REG> RpeclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rpeclr::_0)
    }
    #[doc = "Clear the NMISR.RPEST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rpeclr::_1)
    }
}
#[doc = "SRAM ECC Error Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reccclr {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.RECCST flag."]
    _1 = 1,
}
impl From<Reccclr> for bool {
    #[inline(always)]
    fn from(variant: Reccclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECCCLR` writer - SRAM ECC Error Clear"]
pub type ReccclrW<'a, REG> = crate::BitWriter<'a, REG, Reccclr>;
impl<'a, REG> ReccclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Reccclr::_0)
    }
    #[doc = "Clear the NMISR.RECCST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Reccclr::_1)
    }
}
#[doc = "Bus Slave Error Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bussclr {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.BUSSST flag."]
    _1 = 1,
}
impl From<Bussclr> for bool {
    #[inline(always)]
    fn from(variant: Bussclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSSCLR` writer - Bus Slave Error Clear"]
pub type BussclrW<'a, REG> = crate::BitWriter<'a, REG, Bussclr>;
impl<'a, REG> BussclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bussclr::_0)
    }
    #[doc = "Clear the NMISR.BUSSST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bussclr::_1)
    }
}
#[doc = "Bus Master Error Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busmclr {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.BUSMST flag."]
    _1 = 1,
}
impl From<Busmclr> for bool {
    #[inline(always)]
    fn from(variant: Busmclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSMCLR` writer - Bus Master Error Clear"]
pub type BusmclrW<'a, REG> = crate::BitWriter<'a, REG, Busmclr>;
impl<'a, REG> BusmclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Busmclr::_0)
    }
    #[doc = "Clear the NMISR.BUSMST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Busmclr::_1)
    }
}
#[doc = "CPU Stack Pointer Monitor Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Speclr {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.SPEST flag."]
    _1 = 1,
}
impl From<Speclr> for bool {
    #[inline(always)]
    fn from(variant: Speclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPECLR` writer - CPU Stack Pointer Monitor Interrupt Clear"]
pub type SpeclrW<'a, REG> = crate::BitWriter<'a, REG, Speclr>;
impl<'a, REG> SpeclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Speclr::_0)
    }
    #[doc = "Clear the NMISR.SPEST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Speclr::_1)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NMICLR").finish()
    }
}
impl W {
    #[doc = "Bit 0 - IWDT Clear"]
    #[inline(always)]
    pub fn iwdtclr(&mut self) -> IwdtclrW<NmiclrSpec> {
        IwdtclrW::new(self, 0)
    }
    #[doc = "Bit 1 - WDT Clear"]
    #[inline(always)]
    pub fn wdtclr(&mut self) -> WdtclrW<NmiclrSpec> {
        WdtclrW::new(self, 1)
    }
    #[doc = "Bit 2 - LVD1 Clear"]
    #[inline(always)]
    pub fn lvd1clr(&mut self) -> Lvd1clrW<NmiclrSpec> {
        Lvd1clrW::new(self, 2)
    }
    #[doc = "Bit 3 - LVD2 Clear"]
    #[inline(always)]
    pub fn lvd2clr(&mut self) -> Lvd2clrW<NmiclrSpec> {
        Lvd2clrW::new(self, 3)
    }
    #[doc = "Bit 4 - VBATT Clear"]
    #[inline(always)]
    pub fn vbattclr(&mut self) -> VbattclrW<NmiclrSpec> {
        VbattclrW::new(self, 4)
    }
    #[doc = "Bit 6 - OST Clear"]
    #[inline(always)]
    pub fn ostclr(&mut self) -> OstclrW<NmiclrSpec> {
        OstclrW::new(self, 6)
    }
    #[doc = "Bit 7 - NMI Clear"]
    #[inline(always)]
    pub fn nmiclr(&mut self) -> NmiclrW<NmiclrSpec> {
        NmiclrW::new(self, 7)
    }
    #[doc = "Bit 8 - SRAM Parity Error Clear"]
    #[inline(always)]
    pub fn rpeclr(&mut self) -> RpeclrW<NmiclrSpec> {
        RpeclrW::new(self, 8)
    }
    #[doc = "Bit 9 - SRAM ECC Error Clear"]
    #[inline(always)]
    pub fn reccclr(&mut self) -> ReccclrW<NmiclrSpec> {
        ReccclrW::new(self, 9)
    }
    #[doc = "Bit 10 - Bus Slave Error Clear"]
    #[inline(always)]
    pub fn bussclr(&mut self) -> BussclrW<NmiclrSpec> {
        BussclrW::new(self, 10)
    }
    #[doc = "Bit 11 - Bus Master Error Clear"]
    #[inline(always)]
    pub fn busmclr(&mut self) -> BusmclrW<NmiclrSpec> {
        BusmclrW::new(self, 11)
    }
    #[doc = "Bit 12 - CPU Stack Pointer Monitor Interrupt Clear"]
    #[inline(always)]
    pub fn speclr(&mut self) -> SpeclrW<NmiclrSpec> {
        SpeclrW::new(self, 12)
    }
}
#[doc = "Non-Maskable Interrupt Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmiclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmiclrSpec;
impl crate::RegisterSpec for NmiclrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`nmiclr::R`](R) reader structure"]
impl crate::Readable for NmiclrSpec {}
#[doc = "`write(|w| ..)` method takes [`nmiclr::W`](W) writer structure"]
impl crate::Writable for NmiclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NMICLR to value 0"]
impl crate::Resettable for NmiclrSpec {}
