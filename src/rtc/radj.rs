#[doc = "Register `RADJ` reader"]
pub type R = crate::R<RadjSpec>;
#[doc = "Register `RADJ` writer"]
pub type W = crate::W<RadjSpec>;
#[doc = "Field `ADJ` reader - Adjustment Value These bits specify the adjustment value from the prescaler."]
pub type AdjR = crate::FieldReader;
#[doc = "Field `ADJ` writer - Adjustment Value These bits specify the adjustment value from the prescaler."]
pub type AdjW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Plus-Minus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pmadj {
    #[doc = "0: Adjustment is not performed."]
    _00 = 0,
    #[doc = "1: Adjustment is performed by the addition to the prescaler."]
    _01 = 1,
    #[doc = "2: Adjustment is performed by the subtraction from the prescaler."]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<Pmadj> for u8 {
    #[inline(always)]
    fn from(variant: Pmadj) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pmadj {
    type Ux = u8;
}
impl crate::IsEnum for Pmadj {}
#[doc = "Field `PMADJ` reader - Plus-Minus"]
pub type PmadjR = crate::FieldReader<Pmadj>;
impl PmadjR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmadj {
        match self.bits {
            0 => Pmadj::_00,
            1 => Pmadj::_01,
            2 => Pmadj::_10,
            3 => Pmadj::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Adjustment is not performed."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Pmadj::_00
    }
    #[doc = "Adjustment is performed by the addition to the prescaler."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Pmadj::_01
    }
    #[doc = "Adjustment is performed by the subtraction from the prescaler."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Pmadj::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Pmadj::_11
    }
}
#[doc = "Field `PMADJ` writer - Plus-Minus"]
pub type PmadjW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pmadj, crate::Safe>;
impl<'a, REG> PmadjW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Adjustment is not performed."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Pmadj::_00)
    }
    #[doc = "Adjustment is performed by the addition to the prescaler."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Pmadj::_01)
    }
    #[doc = "Adjustment is performed by the subtraction from the prescaler."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Pmadj::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Pmadj::_11)
    }
}
impl R {
    #[doc = "Bits 0:5 - Adjustment Value These bits specify the adjustment value from the prescaler."]
    #[inline(always)]
    pub fn adj(&self) -> AdjR {
        AdjR::new(self.bits & 0x3f)
    }
    #[doc = "Bits 6:7 - Plus-Minus"]
    #[inline(always)]
    pub fn pmadj(&self) -> PmadjR {
        PmadjR::new((self.bits >> 6) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RADJ")
            .field("pmadj", &self.pmadj())
            .field("adj", &self.adj())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Adjustment Value These bits specify the adjustment value from the prescaler."]
    #[inline(always)]
    pub fn adj(&mut self) -> AdjW<RadjSpec> {
        AdjW::new(self, 0)
    }
    #[doc = "Bits 6:7 - Plus-Minus"]
    #[inline(always)]
    pub fn pmadj(&mut self) -> PmadjW<RadjSpec> {
        PmadjW::new(self, 6)
    }
}
#[doc = "Time Error Adjustment Register\n\nYou can [`read`](crate::Reg::read) this register and get [`radj::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`radj::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RadjSpec;
impl crate::RegisterSpec for RadjSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`radj::R`](R) reader structure"]
impl crate::Readable for RadjSpec {}
#[doc = "`write(|w| ..)` method takes [`radj::W`](W) writer structure"]
impl crate::Writable for RadjSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RADJ to value 0"]
impl crate::Resettable for RadjSpec {}
