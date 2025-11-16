#[doc = "Register `SSIFSR` reader"]
pub type R = crate::R<SsifsrSpec>;
#[doc = "Register `SSIFSR` writer"]
pub type W = crate::W<SsifsrSpec>;
#[doc = "Receive Data Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdf {
    #[doc = "0: The size of received data in SSIFRDR is not more than the value of SSISCR.RDFS"]
    _0 = 0,
    #[doc = "1: The size of received data in SSIFRDR is not less than the value of SSISCR.RDFS plus one."]
    _1 = 1,
}
impl From<Rdf> for bool {
    #[inline(always)]
    fn from(variant: Rdf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDF` reader - Receive Data Full Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type RdfR = crate::BitReader<Rdf>;
impl RdfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdf {
        match self.bits {
            false => Rdf::_0,
            true => Rdf::_1,
        }
    }
    #[doc = "The size of received data in SSIFRDR is not more than the value of SSISCR.RDFS"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rdf::_0
    }
    #[doc = "The size of received data in SSIFRDR is not less than the value of SSISCR.RDFS plus one."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rdf::_1
    }
}
#[doc = "Field `RDF` writer - Receive Data Full Flag"]
pub type RdfW<'a, REG> = crate::BitWriter0C<'a, REG, Rdf>;
impl<'a, REG> RdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The size of received data in SSIFRDR is not more than the value of SSISCR.RDFS"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdf::_0)
    }
    #[doc = "The size of received data in SSIFRDR is not less than the value of SSISCR.RDFS plus one."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdf::_1)
    }
}
#[doc = "Field `RDC` reader - Number of Receive FIFO Data Indication Flag Number of receive FIFO data indication flag."]
pub type RdcR = crate::FieldReader;
#[doc = "Transmit Data Empty Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tde {
    #[doc = "0: The free space of SSIFTDR is not more than the value of SSISCR.TDES"]
    _0 = 0,
    #[doc = "1: The free space of SSIFTDR is not less than the value of SSISCR.TDES plus one."]
    _1 = 1,
}
impl From<Tde> for bool {
    #[inline(always)]
    fn from(variant: Tde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDE` reader - Transmit Data Empty Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type TdeR = crate::BitReader<Tde>;
impl TdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tde {
        match self.bits {
            false => Tde::_0,
            true => Tde::_1,
        }
    }
    #[doc = "The free space of SSIFTDR is not more than the value of SSISCR.TDES"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tde::_0
    }
    #[doc = "The free space of SSIFTDR is not less than the value of SSISCR.TDES plus one."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tde::_1
    }
}
#[doc = "Field `TDE` writer - Transmit Data Empty Flag"]
pub type TdeW<'a, REG> = crate::BitWriter0C<'a, REG, Tde>;
impl<'a, REG> TdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The free space of SSIFTDR is not more than the value of SSISCR.TDES"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tde::_0)
    }
    #[doc = "The free space of SSIFTDR is not less than the value of SSISCR.TDES plus one."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tde::_1)
    }
}
#[doc = "Field `TDC` reader - Number of Transmit FIFO Data Indication Flag Number of transmit FIFO data indication flag."]
pub type TdcR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Receive Data Full Flag"]
    #[inline(always)]
    pub fn rdf(&self) -> RdfR {
        RdfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of Receive FIFO Data Indication Flag Number of receive FIFO data indication flag."]
    #[inline(always)]
    pub fn rdc(&self) -> RdcR {
        RdcR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Transmit Data Empty Flag"]
    #[inline(always)]
    pub fn tde(&self) -> TdeR {
        TdeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Number of Transmit FIFO Data Indication Flag Number of transmit FIFO data indication flag."]
    #[inline(always)]
    pub fn tdc(&self) -> TdcR {
        TdcR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSIFSR")
            .field("tdc", &self.tdc())
            .field("rdc", &self.rdc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Receive Data Full Flag"]
    #[inline(always)]
    pub fn rdf(&mut self) -> RdfW<SsifsrSpec> {
        RdfW::new(self, 0)
    }
    #[doc = "Bit 16 - Transmit Data Empty Flag"]
    #[inline(always)]
    pub fn tde(&mut self) -> TdeW<SsifsrSpec> {
        TdeW::new(self, 16)
    }
}
#[doc = "FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssifsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssifsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsifsrSpec;
impl crate::RegisterSpec for SsifsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssifsr::R`](R) reader structure"]
impl crate::Readable for SsifsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssifsr::W`](W) writer structure"]
impl crate::Writable for SsifsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_0001;
}
#[doc = "`reset()` method sets SSIFSR to value 0x0001_0000"]
impl crate::Resettable for SsifsrSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
