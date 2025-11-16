#[doc = "Register `DMAMD` reader"]
pub type R = crate::R<DmamdSpec>;
#[doc = "Register `DMAMD` writer"]
pub type W = crate::W<DmamdSpec>;
#[doc = "Field `DARA` reader - Destination Address Extended Repeat Area Specifies the extended repeat area on the destination address. For details on the settings."]
pub type DaraR = crate::FieldReader;
#[doc = "Field `DARA` writer - Destination Address Extended Repeat Area Specifies the extended repeat area on the destination address. For details on the settings."]
pub type DaraW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Destination Address Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dm {
    #[doc = "0: Fixed address"]
    _00 = 0,
    #[doc = "1: Offset addition"]
    _01 = 1,
    #[doc = "2: Incremented address"]
    _10 = 2,
    #[doc = "3: Decremented address."]
    _11 = 3,
}
impl From<Dm> for u8 {
    #[inline(always)]
    fn from(variant: Dm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dm {
    type Ux = u8;
}
impl crate::IsEnum for Dm {}
#[doc = "Field `DM` reader - Destination Address Update Mode"]
pub type DmR = crate::FieldReader<Dm>;
impl DmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dm {
        match self.bits {
            0 => Dm::_00,
            1 => Dm::_01,
            2 => Dm::_10,
            3 => Dm::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Fixed address"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dm::_00
    }
    #[doc = "Offset addition"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Dm::_01
    }
    #[doc = "Incremented address"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dm::_10
    }
    #[doc = "Decremented address."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dm::_11
    }
}
#[doc = "Field `DM` writer - Destination Address Update Mode"]
pub type DmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dm, crate::Safe>;
impl<'a, REG> DmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Fixed address"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dm::_00)
    }
    #[doc = "Offset addition"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Dm::_01)
    }
    #[doc = "Incremented address"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dm::_10)
    }
    #[doc = "Decremented address."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dm::_11)
    }
}
#[doc = "Field `SARA` reader - Source Address Extended Repeat Area Specifies the extended repeat area on the source address. For details on the settings."]
pub type SaraR = crate::FieldReader;
#[doc = "Field `SARA` writer - Source Address Extended Repeat Area Specifies the extended repeat area on the source address. For details on the settings."]
pub type SaraW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Source Address Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sm {
    #[doc = "0: Fixed address"]
    _00 = 0,
    #[doc = "1: Offset addition"]
    _01 = 1,
    #[doc = "2: Incremented address"]
    _10 = 2,
    #[doc = "3: Decremented address."]
    _11 = 3,
}
impl From<Sm> for u8 {
    #[inline(always)]
    fn from(variant: Sm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sm {
    type Ux = u8;
}
impl crate::IsEnum for Sm {}
#[doc = "Field `SM` reader - Source Address Update Mode"]
pub type SmR = crate::FieldReader<Sm>;
impl SmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sm {
        match self.bits {
            0 => Sm::_00,
            1 => Sm::_01,
            2 => Sm::_10,
            3 => Sm::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Fixed address"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Sm::_00
    }
    #[doc = "Offset addition"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Sm::_01
    }
    #[doc = "Incremented address"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Sm::_10
    }
    #[doc = "Decremented address."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Sm::_11
    }
}
#[doc = "Field `SM` writer - Source Address Update Mode"]
pub type SmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sm, crate::Safe>;
impl<'a, REG> SmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Fixed address"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Sm::_00)
    }
    #[doc = "Offset addition"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Sm::_01)
    }
    #[doc = "Incremented address"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Sm::_10)
    }
    #[doc = "Decremented address."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Sm::_11)
    }
}
impl R {
    #[doc = "Bits 0:4 - Destination Address Extended Repeat Area Specifies the extended repeat area on the destination address. For details on the settings."]
    #[inline(always)]
    pub fn dara(&self) -> DaraR {
        DaraR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - Destination Address Update Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DmR {
        DmR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Source Address Extended Repeat Area Specifies the extended repeat area on the source address. For details on the settings."]
    #[inline(always)]
    pub fn sara(&self) -> SaraR {
        SaraR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 14:15 - Source Address Update Mode"]
    #[inline(always)]
    pub fn sm(&self) -> SmR {
        SmR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAMD")
            .field("sm", &self.sm())
            .field("sara", &self.sara())
            .field("dm", &self.dm())
            .field("dara", &self.dara())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Destination Address Extended Repeat Area Specifies the extended repeat area on the destination address. For details on the settings."]
    #[inline(always)]
    pub fn dara(&mut self) -> DaraW<DmamdSpec> {
        DaraW::new(self, 0)
    }
    #[doc = "Bits 6:7 - Destination Address Update Mode"]
    #[inline(always)]
    pub fn dm(&mut self) -> DmW<DmamdSpec> {
        DmW::new(self, 6)
    }
    #[doc = "Bits 8:12 - Source Address Extended Repeat Area Specifies the extended repeat area on the source address. For details on the settings."]
    #[inline(always)]
    pub fn sara(&mut self) -> SaraW<DmamdSpec> {
        SaraW::new(self, 8)
    }
    #[doc = "Bits 14:15 - Source Address Update Mode"]
    #[inline(always)]
    pub fn sm(&mut self) -> SmW<DmamdSpec> {
        SmW::new(self, 14)
    }
}
#[doc = "DMA Address Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmamd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmamdSpec;
impl crate::RegisterSpec for DmamdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmamd::R`](R) reader structure"]
impl crate::Readable for DmamdSpec {}
#[doc = "`write(|w| ..)` method takes [`dmamd::W`](W) writer structure"]
impl crate::Writable for DmamdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMAMD to value 0"]
impl crate::Resettable for DmamdSpec {}
