#[doc = "Register `CTSUSO1` reader"]
pub type R = crate::R<Ctsuso1Spec>;
#[doc = "Register `CTSUSO1` writer"]
pub type W = crate::W<Ctsuso1Spec>;
#[doc = "Field `CTSURICOA` reader - CTSU Reference ICO Current Adjustment Current offset amount is CTSUSO ( 0 to 255 )"]
pub type CtsuricoaR = crate::FieldReader;
#[doc = "Field `CTSURICOA` writer - CTSU Reference ICO Current Adjustment Current offset amount is CTSUSO ( 0 to 255 )"]
pub type CtsuricoaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CTSUSDPA` reader - CTSU Base Clock Setting Operating clock divided by ( CTSUSDPA + 1 ) x 2"]
pub type CtsusdpaR = crate::FieldReader;
#[doc = "Field `CTSUSDPA` writer - CTSU Base Clock Setting Operating clock divided by ( CTSUSDPA + 1 ) x 2"]
pub type CtsusdpaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "CTSU ICO Gain Adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsuicog {
    #[doc = "0: 100 percent gain"]
    _00 = 0,
    #[doc = "1: 66 percent gain"]
    _01 = 1,
    #[doc = "2: 50 percent gain"]
    _10 = 2,
    #[doc = "3: 40 percent gain"]
    _11 = 3,
}
impl From<Ctsuicog> for u8 {
    #[inline(always)]
    fn from(variant: Ctsuicog) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsuicog {
    type Ux = u8;
}
impl crate::IsEnum for Ctsuicog {}
#[doc = "Field `CTSUICOG` reader - CTSU ICO Gain Adjustment"]
pub type CtsuicogR = crate::FieldReader<Ctsuicog>;
impl CtsuicogR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsuicog {
        match self.bits {
            0 => Ctsuicog::_00,
            1 => Ctsuicog::_01,
            2 => Ctsuicog::_10,
            3 => Ctsuicog::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "100 percent gain"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Ctsuicog::_00
    }
    #[doc = "66 percent gain"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Ctsuicog::_01
    }
    #[doc = "50 percent gain"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Ctsuicog::_10
    }
    #[doc = "40 percent gain"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Ctsuicog::_11
    }
}
#[doc = "Field `CTSUICOG` writer - CTSU ICO Gain Adjustment"]
pub type CtsuicogW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctsuicog, crate::Safe>;
impl<'a, REG> CtsuicogW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "100 percent gain"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuicog::_00)
    }
    #[doc = "66 percent gain"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuicog::_01)
    }
    #[doc = "50 percent gain"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuicog::_10)
    }
    #[doc = "40 percent gain"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuicog::_11)
    }
}
impl R {
    #[doc = "Bits 0:7 - CTSU Reference ICO Current Adjustment Current offset amount is CTSUSO ( 0 to 255 )"]
    #[inline(always)]
    pub fn ctsuricoa(&self) -> CtsuricoaR {
        CtsuricoaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - CTSU Base Clock Setting Operating clock divided by ( CTSUSDPA + 1 ) x 2"]
    #[inline(always)]
    pub fn ctsusdpa(&self) -> CtsusdpaR {
        CtsusdpaR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - CTSU ICO Gain Adjustment"]
    #[inline(always)]
    pub fn ctsuicog(&self) -> CtsuicogR {
        CtsuicogR::new(((self.bits >> 13) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUSO1")
            .field("ctsuicog", &self.ctsuicog())
            .field("ctsusdpa", &self.ctsusdpa())
            .field("ctsuricoa", &self.ctsuricoa())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Reference ICO Current Adjustment Current offset amount is CTSUSO ( 0 to 255 )"]
    #[inline(always)]
    pub fn ctsuricoa(&mut self) -> CtsuricoaW<Ctsuso1Spec> {
        CtsuricoaW::new(self, 0)
    }
    #[doc = "Bits 8:12 - CTSU Base Clock Setting Operating clock divided by ( CTSUSDPA + 1 ) x 2"]
    #[inline(always)]
    pub fn ctsusdpa(&mut self) -> CtsusdpaW<Ctsuso1Spec> {
        CtsusdpaW::new(self, 8)
    }
    #[doc = "Bits 13:14 - CTSU ICO Gain Adjustment"]
    #[inline(always)]
    pub fn ctsuicog(&mut self) -> CtsuicogW<Ctsuso1Spec> {
        CtsuicogW::new(self, 13)
    }
}
#[doc = "CTSU Sensor Offset Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsuso1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuso1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctsuso1Spec;
impl crate::RegisterSpec for Ctsuso1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctsuso1::R`](R) reader structure"]
impl crate::Readable for Ctsuso1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctsuso1::W`](W) writer structure"]
impl crate::Writable for Ctsuso1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUSO1 to value 0"]
impl crate::Resettable for Ctsuso1Spec {}
