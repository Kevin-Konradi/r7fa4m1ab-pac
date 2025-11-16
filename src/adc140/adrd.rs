#[doc = "Register `ADRD` reader"]
pub type R = crate::R<AdrdSpec>;
#[doc = "Field `AD` reader - A/D-converted value (right-justified) The format for data determine ADCER.ADRFMT and ADCER.ADPRC."]
pub type AdR = crate::FieldReader<u16>;
#[doc = "Self-Diagnosis Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Diagst {
    #[doc = "0: Self-diagnosis has never been executed since power-on."]
    _00 = 0,
    #[doc = "1: Self-diagnosis using the voltage of 0 V has been executed."]
    _01 = 1,
    #[doc = "2: Self-diagnosis using the voltage of reference power supply(VREFH) x 1/2 has been executed."]
    _10 = 2,
    #[doc = "3: Self-diagnosis using the voltage of reference power supply(VREFH) has been executed."]
    _11 = 3,
}
impl From<Diagst> for u8 {
    #[inline(always)]
    fn from(variant: Diagst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Diagst {
    type Ux = u8;
}
impl crate::IsEnum for Diagst {}
#[doc = "Field `DIAGST` reader - Self-Diagnosis Status"]
pub type DiagstR = crate::FieldReader<Diagst>;
impl DiagstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Diagst {
        match self.bits {
            0 => Diagst::_00,
            1 => Diagst::_01,
            2 => Diagst::_10,
            3 => Diagst::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Self-diagnosis has never been executed since power-on."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Diagst::_00
    }
    #[doc = "Self-diagnosis using the voltage of 0 V has been executed."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Diagst::_01
    }
    #[doc = "Self-diagnosis using the voltage of reference power supply(VREFH) x 1/2 has been executed."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Diagst::_10
    }
    #[doc = "Self-diagnosis using the voltage of reference power supply(VREFH) has been executed."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Diagst::_11
    }
}
impl R {
    #[doc = "Bits 0:13 - A/D-converted value (right-justified) The format for data determine ADCER.ADRFMT and ADCER.ADPRC."]
    #[inline(always)]
    pub fn ad(&self) -> AdR {
        AdR::new(self.bits & 0x3fff)
    }
    #[doc = "Bits 14:15 - Self-Diagnosis Status"]
    #[inline(always)]
    pub fn diagst(&self) -> DiagstR {
        DiagstR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADRD")
            .field("diagst", &self.diagst())
            .field("ad", &self.ad())
            .finish()
    }
}
#[doc = "A/D Self-Diagnosis Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adrd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdrdSpec;
impl crate::RegisterSpec for AdrdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adrd::R`](R) reader structure"]
impl crate::Readable for AdrdSpec {}
#[doc = "`reset()` method sets ADRD to value 0"]
impl crate::Resettable for AdrdSpec {}
