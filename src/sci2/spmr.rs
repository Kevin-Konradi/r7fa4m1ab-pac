#[doc = "Register `SPMR` reader"]
pub type R = crate::R<SpmrSpec>;
#[doc = "Register `SPMR` writer"]
pub type W = crate::W<SpmrSpec>;
#[doc = "SSn Pin Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sse {
    #[doc = "0: SSn pin function is disabled."]
    _0 = 0,
    #[doc = "1: SSn pin function is enabled."]
    _1 = 1,
}
impl From<Sse> for bool {
    #[inline(always)]
    fn from(variant: Sse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSE` reader - SSn Pin Function Enable"]
pub type SseR = crate::BitReader<Sse>;
impl SseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sse {
        match self.bits {
            false => Sse::_0,
            true => Sse::_1,
        }
    }
    #[doc = "SSn pin function is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sse::_0
    }
    #[doc = "SSn pin function is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sse::_1
    }
}
#[doc = "Field `SSE` writer - SSn Pin Function Enable"]
pub type SseW<'a, REG> = crate::BitWriter<'a, REG, Sse>;
impl<'a, REG> SseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SSn pin function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sse::_0)
    }
    #[doc = "SSn pin function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sse::_1)
    }
}
#[doc = "CTS Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctse {
    #[doc = "0: CTS function is disabled (RTS output function is enabled)."]
    _0 = 0,
    #[doc = "1: CTS function is enabled."]
    _1 = 1,
}
impl From<Ctse> for bool {
    #[inline(always)]
    fn from(variant: Ctse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSE` reader - CTS Enable"]
pub type CtseR = crate::BitReader<Ctse>;
impl CtseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctse {
        match self.bits {
            false => Ctse::_0,
            true => Ctse::_1,
        }
    }
    #[doc = "CTS function is disabled (RTS output function is enabled)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctse::_0
    }
    #[doc = "CTS function is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctse::_1
    }
}
#[doc = "Field `CTSE` writer - CTS Enable"]
pub type CtseW<'a, REG> = crate::BitWriter<'a, REG, Ctse>;
impl<'a, REG> CtseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CTS function is disabled (RTS output function is enabled)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctse::_0)
    }
    #[doc = "CTS function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctse::_1)
    }
}
#[doc = "Master Slave Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mss {
    #[doc = "0: Transmission is through the TXDn pin and reception is through the RXDn pin (master mode)."]
    _0 = 0,
    #[doc = "1: Reception is through the TXDn pin and transmission is through the RXDn pin (slave mode)."]
    _1 = 1,
}
impl From<Mss> for bool {
    #[inline(always)]
    fn from(variant: Mss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSS` reader - Master Slave Select"]
pub type MssR = crate::BitReader<Mss>;
impl MssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mss {
        match self.bits {
            false => Mss::_0,
            true => Mss::_1,
        }
    }
    #[doc = "Transmission is through the TXDn pin and reception is through the RXDn pin (master mode)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mss::_0
    }
    #[doc = "Reception is through the TXDn pin and transmission is through the RXDn pin (slave mode)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mss::_1
    }
}
#[doc = "Field `MSS` writer - Master Slave Select"]
pub type MssW<'a, REG> = crate::BitWriter<'a, REG, Mss>;
impl<'a, REG> MssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmission is through the TXDn pin and reception is through the RXDn pin (master mode)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mss::_0)
    }
    #[doc = "Reception is through the TXDn pin and transmission is through the RXDn pin (slave mode)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mss::_1)
    }
}
#[doc = "Mode Fault Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mff {
    #[doc = "0: No mode fault error"]
    _0 = 0,
    #[doc = "1: Mode fault error"]
    _1 = 1,
}
impl From<Mff> for bool {
    #[inline(always)]
    fn from(variant: Mff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MFF` reader - Mode Fault Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type MffR = crate::BitReader<Mff>;
impl MffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mff {
        match self.bits {
            false => Mff::_0,
            true => Mff::_1,
        }
    }
    #[doc = "No mode fault error"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mff::_0
    }
    #[doc = "Mode fault error"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mff::_1
    }
}
#[doc = "Field `MFF` writer - Mode Fault Flag"]
pub type MffW<'a, REG> = crate::BitWriter0C<'a, REG, Mff>;
impl<'a, REG> MffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No mode fault error"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mff::_0)
    }
    #[doc = "Mode fault error"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mff::_1)
    }
}
#[doc = "Clock Polarity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckpol {
    #[doc = "0: Clock polarity is not inverted."]
    _0 = 0,
    #[doc = "1: Clock polarity is inverted"]
    _1 = 1,
}
impl From<Ckpol> for bool {
    #[inline(always)]
    fn from(variant: Ckpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKPOL` reader - Clock Polarity Select"]
pub type CkpolR = crate::BitReader<Ckpol>;
impl CkpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckpol {
        match self.bits {
            false => Ckpol::_0,
            true => Ckpol::_1,
        }
    }
    #[doc = "Clock polarity is not inverted."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ckpol::_0
    }
    #[doc = "Clock polarity is inverted"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ckpol::_1
    }
}
#[doc = "Field `CKPOL` writer - Clock Polarity Select"]
pub type CkpolW<'a, REG> = crate::BitWriter<'a, REG, Ckpol>;
impl<'a, REG> CkpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock polarity is not inverted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ckpol::_0)
    }
    #[doc = "Clock polarity is inverted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ckpol::_1)
    }
}
#[doc = "Clock Phase Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckph {
    #[doc = "0: Clock is not delayed."]
    _0 = 0,
    #[doc = "1: Clock is delayed."]
    _1 = 1,
}
impl From<Ckph> for bool {
    #[inline(always)]
    fn from(variant: Ckph) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKPH` reader - Clock Phase Select"]
pub type CkphR = crate::BitReader<Ckph>;
impl CkphR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckph {
        match self.bits {
            false => Ckph::_0,
            true => Ckph::_1,
        }
    }
    #[doc = "Clock is not delayed."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ckph::_0
    }
    #[doc = "Clock is delayed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ckph::_1
    }
}
#[doc = "Field `CKPH` writer - Clock Phase Select"]
pub type CkphW<'a, REG> = crate::BitWriter<'a, REG, Ckph>;
impl<'a, REG> CkphW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock is not delayed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ckph::_0)
    }
    #[doc = "Clock is delayed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ckph::_1)
    }
}
impl R {
    #[doc = "Bit 0 - SSn Pin Function Enable"]
    #[inline(always)]
    pub fn sse(&self) -> SseR {
        SseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTS Enable"]
    #[inline(always)]
    pub fn ctse(&self) -> CtseR {
        CtseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master Slave Select"]
    #[inline(always)]
    pub fn mss(&self) -> MssR {
        MssR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Mode Fault Flag"]
    #[inline(always)]
    pub fn mff(&self) -> MffR {
        MffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Clock Polarity Select"]
    #[inline(always)]
    pub fn ckpol(&self) -> CkpolR {
        CkpolR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock Phase Select"]
    #[inline(always)]
    pub fn ckph(&self) -> CkphR {
        CkphR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPMR")
            .field("ckph", &self.ckph())
            .field("ckpol", &self.ckpol())
            .field("mss", &self.mss())
            .field("ctse", &self.ctse())
            .field("sse", &self.sse())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SSn Pin Function Enable"]
    #[inline(always)]
    pub fn sse(&mut self) -> SseW<SpmrSpec> {
        SseW::new(self, 0)
    }
    #[doc = "Bit 1 - CTS Enable"]
    #[inline(always)]
    pub fn ctse(&mut self) -> CtseW<SpmrSpec> {
        CtseW::new(self, 1)
    }
    #[doc = "Bit 2 - Master Slave Select"]
    #[inline(always)]
    pub fn mss(&mut self) -> MssW<SpmrSpec> {
        MssW::new(self, 2)
    }
    #[doc = "Bit 4 - Mode Fault Flag"]
    #[inline(always)]
    pub fn mff(&mut self) -> MffW<SpmrSpec> {
        MffW::new(self, 4)
    }
    #[doc = "Bit 6 - Clock Polarity Select"]
    #[inline(always)]
    pub fn ckpol(&mut self) -> CkpolW<SpmrSpec> {
        CkpolW::new(self, 6)
    }
    #[doc = "Bit 7 - Clock Phase Select"]
    #[inline(always)]
    pub fn ckph(&mut self) -> CkphW<SpmrSpec> {
        CkphW::new(self, 7)
    }
}
#[doc = "SPI Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpmrSpec;
impl crate::RegisterSpec for SpmrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spmr::R`](R) reader structure"]
impl crate::Readable for SpmrSpec {}
#[doc = "`write(|w| ..)` method takes [`spmr::W`](W) writer structure"]
impl crate::Writable for SpmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x10;
}
#[doc = "`reset()` method sets SPMR to value 0"]
impl crate::Resettable for SpmrSpec {}
