#[doc = "Register `SPDCR` reader"]
pub type R = crate::R<SpdcrSpec>;
#[doc = "Register `SPDCR` writer"]
pub type W = crate::W<SpdcrSpec>;
#[doc = "RSPI Receive/Transmit Data Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sprdtd {
    #[doc = "0: SPDR values are read from the receive buffer"]
    _0 = 0,
    #[doc = "1: SPDR values are read from the transmit buffer (but only if the transmit buffer is empty)"]
    _1 = 1,
}
impl From<Sprdtd> for bool {
    #[inline(always)]
    fn from(variant: Sprdtd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPRDTD` reader - RSPI Receive/Transmit Data Selection"]
pub type SprdtdR = crate::BitReader<Sprdtd>;
impl SprdtdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sprdtd {
        match self.bits {
            false => Sprdtd::_0,
            true => Sprdtd::_1,
        }
    }
    #[doc = "SPDR values are read from the receive buffer"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sprdtd::_0
    }
    #[doc = "SPDR values are read from the transmit buffer (but only if the transmit buffer is empty)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sprdtd::_1
    }
}
#[doc = "Field `SPRDTD` writer - RSPI Receive/Transmit Data Selection"]
pub type SprdtdW<'a, REG> = crate::BitWriter<'a, REG, Sprdtd>;
impl<'a, REG> SprdtdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPDR values are read from the receive buffer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sprdtd::_0)
    }
    #[doc = "SPDR values are read from the transmit buffer (but only if the transmit buffer is empty)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sprdtd::_1)
    }
}
#[doc = "SPI Word Access/Halfword Access Specification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Splw {
    #[doc = "0: SPDR_HA is valid to access in halfwords"]
    _0 = 0,
    #[doc = "1: SPDR is valid (to access in words)."]
    _1 = 1,
}
impl From<Splw> for bool {
    #[inline(always)]
    fn from(variant: Splw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLW` reader - SPI Word Access/Halfword Access Specification"]
pub type SplwR = crate::BitReader<Splw>;
impl SplwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Splw {
        match self.bits {
            false => Splw::_0,
            true => Splw::_1,
        }
    }
    #[doc = "SPDR_HA is valid to access in halfwords"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Splw::_0
    }
    #[doc = "SPDR is valid (to access in words)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Splw::_1
    }
}
#[doc = "Field `SPLW` writer - SPI Word Access/Halfword Access Specification"]
pub type SplwW<'a, REG> = crate::BitWriter<'a, REG, Splw>;
impl<'a, REG> SplwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPDR_HA is valid to access in halfwords"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Splw::_0)
    }
    #[doc = "SPDR is valid (to access in words)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Splw::_1)
    }
}
impl R {
    #[doc = "Bit 4 - RSPI Receive/Transmit Data Selection"]
    #[inline(always)]
    pub fn sprdtd(&self) -> SprdtdR {
        SprdtdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI Word Access/Halfword Access Specification"]
    #[inline(always)]
    pub fn splw(&self) -> SplwR {
        SplwR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPDCR")
            .field("splw", &self.splw())
            .field("sprdtd", &self.sprdtd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - RSPI Receive/Transmit Data Selection"]
    #[inline(always)]
    pub fn sprdtd(&mut self) -> SprdtdW<SpdcrSpec> {
        SprdtdW::new(self, 4)
    }
    #[doc = "Bit 5 - SPI Word Access/Halfword Access Specification"]
    #[inline(always)]
    pub fn splw(&mut self) -> SplwW<SpdcrSpec> {
        SplwW::new(self, 5)
    }
}
#[doc = "SPI Data Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdcrSpec;
impl crate::RegisterSpec for SpdcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spdcr::R`](R) reader structure"]
impl crate::Readable for SpdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`spdcr::W`](W) writer structure"]
impl crate::Writable for SpdcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDCR to value 0"]
impl crate::Resettable for SpdcrSpec {}
