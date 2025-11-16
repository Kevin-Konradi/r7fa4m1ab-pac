#[doc = "Register `CTSUMCH0` reader"]
pub type R = crate::R<Ctsumch0Spec>;
#[doc = "Register `CTSUMCH0` writer"]
pub type W = crate::W<Ctsumch0Spec>;
#[doc = "CTSU Measurement Channel 0. Note1: Writing to these bits is only enabled in self-capacitance single-scan mode (CTSUCR1.CTSUMD\\[1:0\\] bits = 00b). Note2: If the value of CTSUMCH0 was set to b'111111 in mode other than self-capacitor single scan mode, the measurement is stopped.\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsumch0 {
    #[doc = "0: The value of CTSUMCH0 indicate to channel to be measured."]
    Ctsumch0 = 0,
}
impl From<Ctsumch0> for u8 {
    #[inline(always)]
    fn from(variant: Ctsumch0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsumch0 {
    type Ux = u8;
}
impl crate::IsEnum for Ctsumch0 {}
#[doc = "Field `CTSUMCH0` reader - CTSU Measurement Channel 0. Note1: Writing to these bits is only enabled in self-capacitance single-scan mode (CTSUCR1.CTSUMD\\[1:0\\] bits = 00b). Note2: If the value of CTSUMCH0 was set to b'111111 in mode other than self-capacitor single scan mode, the measurement is stopped."]
pub type Ctsumch0R = crate::FieldReader<Ctsumch0>;
impl Ctsumch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsumch0 {
        match self.bits {
            _ => Ctsumch0::Ctsumch0,
        }
    }
    #[doc = "The value of CTSUMCH0 indicate to channel to be measured."]
    #[inline(always)]
    pub fn is_ctsumch0(&self) -> bool {
        matches!(self.variant(), Ctsumch0::Ctsumch0)
    }
}
#[doc = "Field `CTSUMCH0` writer - CTSU Measurement Channel 0. Note1: Writing to these bits is only enabled in self-capacitance single-scan mode (CTSUCR1.CTSUMD\\[1:0\\] bits = 00b). Note2: If the value of CTSUMCH0 was set to b'111111 in mode other than self-capacitor single scan mode, the measurement is stopped."]
pub type Ctsumch0W<'a, REG> = crate::FieldWriter<'a, REG, 6, Ctsumch0, crate::Safe>;
impl<'a, REG> Ctsumch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The value of CTSUMCH0 indicate to channel to be measured."]
    #[inline(always)]
    pub fn ctsumch0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsumch0::Ctsumch0)
    }
}
impl R {
    #[doc = "Bits 0:5 - CTSU Measurement Channel 0. Note1: Writing to these bits is only enabled in self-capacitance single-scan mode (CTSUCR1.CTSUMD\\[1:0\\] bits = 00b). Note2: If the value of CTSUMCH0 was set to b'111111 in mode other than self-capacitor single scan mode, the measurement is stopped."]
    #[inline(always)]
    pub fn ctsumch0(&self) -> Ctsumch0R {
        Ctsumch0R::new(self.bits & 0x3f)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUMCH0")
            .field("ctsumch0", &self.ctsumch0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - CTSU Measurement Channel 0. Note1: Writing to these bits is only enabled in self-capacitance single-scan mode (CTSUCR1.CTSUMD\\[1:0\\] bits = 00b). Note2: If the value of CTSUMCH0 was set to b'111111 in mode other than self-capacitor single scan mode, the measurement is stopped."]
    #[inline(always)]
    pub fn ctsumch0(&mut self) -> Ctsumch0W<Ctsumch0Spec> {
        Ctsumch0W::new(self, 0)
    }
}
#[doc = "CTSU Measurement Channel Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsumch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsumch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctsumch0Spec;
impl crate::RegisterSpec for Ctsumch0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsumch0::R`](R) reader structure"]
impl crate::Readable for Ctsumch0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctsumch0::W`](W) writer structure"]
impl crate::Writable for Ctsumch0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUMCH0 to value 0x3f"]
impl crate::Resettable for Ctsumch0Spec {
    const RESET_VALUE: u8 = 0x3f;
}
