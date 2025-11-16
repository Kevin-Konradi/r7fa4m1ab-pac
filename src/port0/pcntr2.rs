#[doc = "Register `PCNTR2` reader"]
pub type R = crate::R<Pcntr2Spec>;
#[doc = "Pmn Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Pidr {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input."]
    _1 = 1,
}
impl From<Pidr> for u16 {
    #[inline(always)]
    fn from(variant: Pidr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pidr {
    type Ux = u16;
}
impl crate::IsEnum for Pidr {}
#[doc = "Field `PIDR` reader - Pmn Input Data"]
pub type PidrR = crate::FieldReader<Pidr>;
impl PidrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pidr> {
        match self.bits {
            0 => Some(Pidr::_0),
            1 => Some(Pidr::_1),
            _ => None,
        }
    }
    #[doc = "Low input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr::_0
    }
    #[doc = "High input."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr::_1
    }
}
impl R {
    #[doc = "Bits 0:15 - Pmn Input Data"]
    #[inline(always)]
    pub fn pidr(&self) -> PidrR {
        PidrR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCNTR2")
            .field("pidr", &self.pidr())
            .finish()
    }
}
#[doc = "Port Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pcntr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcntr2Spec;
impl crate::RegisterSpec for Pcntr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcntr2::R`](R) reader structure"]
impl crate::Readable for Pcntr2Spec {}
#[doc = "`reset()` method sets PCNTR2 to value 0"]
impl crate::Resettable for Pcntr2Spec {}
