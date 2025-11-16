#[doc = "Register `ADDISCR` reader"]
pub type R = crate::R<AddiscrSpec>;
#[doc = "Register `ADDISCR` writer"]
pub type W = crate::W<AddiscrSpec>;
#[doc = "The charging time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adndis {
    #[doc = "0: Disconnection detection is disabled"]
    _0000 = 0,
    #[doc = "1: Setting prohibited"]
    _0001 = 1,
    #[doc = "2: ( 1 / ADCLK ) x ADNDIS"]
    Others = 2,
}
impl From<Adndis> for u8 {
    #[inline(always)]
    fn from(variant: Adndis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adndis {
    type Ux = u8;
}
impl crate::IsEnum for Adndis {}
#[doc = "Field `ADNDIS` reader - The charging time"]
pub type AdndisR = crate::FieldReader<Adndis>;
impl AdndisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adndis {
        match self.bits {
            0 => Adndis::_0000,
            1 => Adndis::_0001,
            _ => Adndis::Others,
        }
    }
    #[doc = "Disconnection detection is disabled"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Adndis::_0000
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == Adndis::_0001
    }
    #[doc = "( 1 / ADCLK ) x ADNDIS"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Adndis::Others)
    }
}
#[doc = "Field `ADNDIS` writer - The charging time"]
pub type AdndisW<'a, REG> = crate::FieldWriter<'a, REG, 4, Adndis, crate::Safe>;
impl<'a, REG> AdndisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disconnection detection is disabled"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Adndis::_0000)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(Adndis::_0001)
    }
    #[doc = "( 1 / ADCLK ) x ADNDIS"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Adndis::Others)
    }
}
#[doc = "Selection of Precharge or Discharge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pchg {
    #[doc = "0: Discharge"]
    _0 = 0,
    #[doc = "1: Precharge"]
    _1 = 1,
}
impl From<Pchg> for bool {
    #[inline(always)]
    fn from(variant: Pchg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCHG` reader - Selection of Precharge or Discharge"]
pub type PchgR = crate::BitReader<Pchg>;
impl PchgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pchg {
        match self.bits {
            false => Pchg::_0,
            true => Pchg::_1,
        }
    }
    #[doc = "Discharge"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pchg::_0
    }
    #[doc = "Precharge"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pchg::_1
    }
}
#[doc = "Field `PCHG` writer - Selection of Precharge or Discharge"]
pub type PchgW<'a, REG> = crate::BitWriter<'a, REG, Pchg>;
impl<'a, REG> PchgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Discharge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pchg::_0)
    }
    #[doc = "Precharge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pchg::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - The charging time"]
    #[inline(always)]
    pub fn adndis(&self) -> AdndisR {
        AdndisR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Selection of Precharge or Discharge"]
    #[inline(always)]
    pub fn pchg(&self) -> PchgR {
        PchgR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDISCR")
            .field("pchg", &self.pchg())
            .field("adndis", &self.adndis())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - The charging time"]
    #[inline(always)]
    pub fn adndis(&mut self) -> AdndisW<AddiscrSpec> {
        AdndisW::new(self, 0)
    }
    #[doc = "Bit 4 - Selection of Precharge or Discharge"]
    #[inline(always)]
    pub fn pchg(&mut self) -> PchgW<AddiscrSpec> {
        PchgW::new(self, 4)
    }
}
#[doc = "A/D Disconnection Detection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addiscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addiscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddiscrSpec;
impl crate::RegisterSpec for AddiscrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`addiscr::R`](R) reader structure"]
impl crate::Readable for AddiscrSpec {}
#[doc = "`write(|w| ..)` method takes [`addiscr::W`](W) writer structure"]
impl crate::Writable for AddiscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDISCR to value 0"]
impl crate::Resettable for AddiscrSpec {}
