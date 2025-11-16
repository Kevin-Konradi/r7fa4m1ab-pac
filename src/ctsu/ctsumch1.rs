#[doc = "Register `CTSUMCH1` reader"]
pub type R = crate::R<Ctsumch1Spec>;
#[doc = "Register `CTSUMCH1` writer"]
pub type W = crate::W<Ctsumch1Spec>;
#[doc = "CTSU Measurement Channel 1 Note1: If the value of CTSUMCH1 was set to b'111111, the measurement is stopped.\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsumch1 {
    #[doc = "0: The value of CTSUMCH1 indicate to channel to be measured."]
    Ctsumch1 = 0,
}
impl From<Ctsumch1> for u8 {
    #[inline(always)]
    fn from(variant: Ctsumch1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsumch1 {
    type Ux = u8;
}
impl crate::IsEnum for Ctsumch1 {}
#[doc = "Field `CTSUMCH1` reader - CTSU Measurement Channel 1 Note1: If the value of CTSUMCH1 was set to b'111111, the measurement is stopped."]
pub type Ctsumch1R = crate::FieldReader<Ctsumch1>;
impl Ctsumch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsumch1 {
        match self.bits {
            _ => Ctsumch1::Ctsumch1,
        }
    }
    #[doc = "The value of CTSUMCH1 indicate to channel to be measured."]
    #[inline(always)]
    pub fn is_ctsumch1(&self) -> bool {
        matches!(self.variant(), Ctsumch1::Ctsumch1)
    }
}
impl R {
    #[doc = "Bits 0:5 - CTSU Measurement Channel 1 Note1: If the value of CTSUMCH1 was set to b'111111, the measurement is stopped."]
    #[inline(always)]
    pub fn ctsumch1(&self) -> Ctsumch1R {
        Ctsumch1R::new(self.bits & 0x3f)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUMCH1")
            .field("ctsumch1", &self.ctsumch1())
            .finish()
    }
}
impl W {}
#[doc = "CTSU Measurement Channel Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsumch1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsumch1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctsumch1Spec;
impl crate::RegisterSpec for Ctsumch1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsumch1::R`](R) reader structure"]
impl crate::Readable for Ctsumch1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctsumch1::W`](W) writer structure"]
impl crate::Writable for Ctsumch1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUMCH1 to value 0x3f"]
impl crate::Resettable for Ctsumch1Spec {
    const RESET_VALUE: u8 = 0x3f;
}
