#[doc = "Register `BUSSCNTP6B` reader"]
pub type R = crate::R<Busscntp6bSpec>;
#[doc = "Register `BUSSCNTP6B` writer"]
pub type W = crate::W<Busscntp6bSpec>;
#[doc = "Arbitration Method Specify the priority between groups\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Arbmet {
    #[doc = "0: fixed priority"]
    _00 = 0,
    #[doc = "1: round-robin"]
    _01 = 1,
    #[doc = "2: Setting prohibited"]
    Others = 2,
}
impl From<Arbmet> for u8 {
    #[inline(always)]
    fn from(variant: Arbmet) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Arbmet {
    type Ux = u8;
}
impl crate::IsEnum for Arbmet {}
#[doc = "Field `ARBMET` reader - Arbitration Method Specify the priority between groups"]
pub type ArbmetR = crate::FieldReader<Arbmet>;
impl ArbmetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arbmet {
        match self.bits {
            0 => Arbmet::_00,
            1 => Arbmet::_01,
            _ => Arbmet::Others,
        }
    }
    #[doc = "fixed priority"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Arbmet::_00
    }
    #[doc = "round-robin"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Arbmet::_01
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Arbmet::Others)
    }
}
#[doc = "Field `ARBMET` writer - Arbitration Method Specify the priority between groups"]
pub type ArbmetW<'a, REG> = crate::FieldWriter<'a, REG, 2, Arbmet, crate::Safe>;
impl<'a, REG> ArbmetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fixed priority"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Arbmet::_00)
    }
    #[doc = "round-robin"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Arbmet::_01)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Arbmet::Others)
    }
}
impl R {
    #[doc = "Bits 4:5 - Arbitration Method Specify the priority between groups"]
    #[inline(always)]
    pub fn arbmet(&self) -> ArbmetR {
        ArbmetR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSSCNTP6B")
            .field("arbmet", &self.arbmet())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:5 - Arbitration Method Specify the priority between groups"]
    #[inline(always)]
    pub fn arbmet(&mut self) -> ArbmetW<Busscntp6bSpec> {
        ArbmetW::new(self, 4)
    }
}
#[doc = "Slave Bus Control Register P6B\n\nYou can [`read`](crate::Reg::read) this register and get [`busscntp6b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscntp6b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Busscntp6bSpec;
impl crate::RegisterSpec for Busscntp6bSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`busscntp6b::R`](R) reader structure"]
impl crate::Readable for Busscntp6bSpec {}
#[doc = "`write(|w| ..)` method takes [`busscntp6b::W`](W) writer structure"]
impl crate::Writable for Busscntp6bSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUSSCNTP6B to value 0"]
impl crate::Resettable for Busscntp6bSpec {}
