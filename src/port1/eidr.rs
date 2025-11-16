#[doc = "Register `EIDR` reader"]
pub type R = crate::R<EidrSpec>;
#[doc = "Pmn Event Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Eidr {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input."]
    _1 = 1,
}
impl From<Eidr> for u16 {
    #[inline(always)]
    fn from(variant: Eidr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eidr {
    type Ux = u16;
}
impl crate::IsEnum for Eidr {}
#[doc = "Field `EIDR` reader - Pmn Event Input Data"]
pub type EidrR = crate::FieldReader<Eidr>;
impl EidrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Eidr> {
        match self.bits {
            0 => Some(Eidr::_0),
            1 => Some(Eidr::_1),
            _ => None,
        }
    }
    #[doc = "Low input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eidr::_0
    }
    #[doc = "High input."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eidr::_1
    }
}
impl R {
    #[doc = "Bits 0:15 - Pmn Event Input Data"]
    #[inline(always)]
    pub fn eidr(&self) -> EidrR {
        EidrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EIDR").field("eidr", &self.eidr()).finish()
    }
}
#[doc = "Event input data register\n\nYou can [`read`](crate::Reg::read) this register and get [`eidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EidrSpec;
impl crate::RegisterSpec for EidrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`eidr::R`](R) reader structure"]
impl crate::Readable for EidrSpec {}
#[doc = "`reset()` method sets EIDR to value 0"]
impl crate::Resettable for EidrSpec {}
