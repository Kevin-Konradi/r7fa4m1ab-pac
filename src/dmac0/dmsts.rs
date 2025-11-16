#[doc = "Register `DMSTS` reader"]
pub type R = crate::R<DmstsSpec>;
#[doc = "Register `DMSTS` writer"]
pub type W = crate::W<DmstsSpec>;
#[doc = "Transfer Escape End Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Esif {
    #[doc = "0: No interrupt"]
    _0 = 0,
    #[doc = "1: Interrupt occurred."]
    _1 = 1,
}
impl From<Esif> for bool {
    #[inline(always)]
    fn from(variant: Esif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESIF` reader - Transfer Escape End Interrupt Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EsifR = crate::BitReader<Esif>;
impl EsifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Esif {
        match self.bits {
            false => Esif::_0,
            true => Esif::_1,
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Esif::_0
    }
    #[doc = "Interrupt occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Esif::_1
    }
}
#[doc = "Field `ESIF` writer - Transfer Escape End Interrupt Flag"]
pub type EsifW<'a, REG> = crate::BitWriter0C<'a, REG, Esif>;
impl<'a, REG> EsifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Esif::_0)
    }
    #[doc = "Interrupt occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Esif::_1)
    }
}
#[doc = "Transfer End Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtif {
    #[doc = "0: No interrupt"]
    _0 = 0,
    #[doc = "1: Interrupt occurred."]
    _1 = 1,
}
impl From<Dtif> for bool {
    #[inline(always)]
    fn from(variant: Dtif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTIF` reader - Transfer End Interrupt Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type DtifR = crate::BitReader<Dtif>;
impl DtifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtif {
        match self.bits {
            false => Dtif::_0,
            true => Dtif::_1,
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dtif::_0
    }
    #[doc = "Interrupt occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtif::_1
    }
}
#[doc = "Field `DTIF` writer - Transfer End Interrupt Flag"]
pub type DtifW<'a, REG> = crate::BitWriter0C<'a, REG, Dtif>;
impl<'a, REG> DtifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtif::_0)
    }
    #[doc = "Interrupt occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtif::_1)
    }
}
#[doc = "DMA Active Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Act {
    #[doc = "0: DMAC operation suspended"]
    _0 = 0,
    #[doc = "1: DMAC operating."]
    _1 = 1,
}
impl From<Act> for bool {
    #[inline(always)]
    fn from(variant: Act) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACT` reader - DMA Active Flag"]
pub type ActR = crate::BitReader<Act>;
impl ActR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Act {
        match self.bits {
            false => Act::_0,
            true => Act::_1,
        }
    }
    #[doc = "DMAC operation suspended"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Act::_0
    }
    #[doc = "DMAC operating."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Act::_1
    }
}
impl R {
    #[doc = "Bit 0 - Transfer Escape End Interrupt Flag"]
    #[inline(always)]
    pub fn esif(&self) -> EsifR {
        EsifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Transfer End Interrupt Flag"]
    #[inline(always)]
    pub fn dtif(&self) -> DtifR {
        DtifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Active Flag"]
    #[inline(always)]
    pub fn act(&self) -> ActR {
        ActR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMSTS").field("act", &self.act()).finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Escape End Interrupt Flag"]
    #[inline(always)]
    pub fn esif(&mut self) -> EsifW<DmstsSpec> {
        EsifW::new(self, 0)
    }
    #[doc = "Bit 4 - Transfer End Interrupt Flag"]
    #[inline(always)]
    pub fn dtif(&mut self) -> DtifW<DmstsSpec> {
        DtifW::new(self, 4)
    }
}
#[doc = "DMA Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmstsSpec;
impl crate::RegisterSpec for DmstsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dmsts::R`](R) reader structure"]
impl crate::Readable for DmstsSpec {}
#[doc = "`write(|w| ..)` method takes [`dmsts::W`](W) writer structure"]
impl crate::Writable for DmstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x11;
}
#[doc = "`reset()` method sets DMSTS to value 0"]
impl crate::Resettable for DmstsSpec {}
