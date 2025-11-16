#[doc = "Register `CFIFOCTR` reader"]
pub type R = crate::R<CfifoctrSpec>;
#[doc = "Register `CFIFOCTR` writer"]
pub type W = crate::W<CfifoctrSpec>;
#[doc = "Field `DTLN` reader - Receive Data Length Indicates the length of the receive data."]
pub type DtlnR = crate::FieldReader<u16>;
#[doc = "FIFO Port Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frdy {
    #[doc = "0: FIFO port access is disabled."]
    _0 = 0,
    #[doc = "1: FIFO port access is enabled."]
    _1 = 1,
}
impl From<Frdy> for bool {
    #[inline(always)]
    fn from(variant: Frdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRDY` reader - FIFO Port Ready"]
pub type FrdyR = crate::BitReader<Frdy>;
impl FrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frdy {
        match self.bits {
            false => Frdy::_0,
            true => Frdy::_1,
        }
    }
    #[doc = "FIFO port access is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Frdy::_0
    }
    #[doc = "FIFO port access is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Frdy::_1
    }
}
#[doc = "CPU Buffer Clear Note: Only 0 can be read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bclr {
    #[doc = "0: Does not operate"]
    _0 = 0,
    #[doc = "1: FIFO buffer cleared on the CPU side."]
    _1 = 1,
}
impl From<Bclr> for bool {
    #[inline(always)]
    fn from(variant: Bclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCLR` writer - CPU Buffer Clear Note: Only 0 can be read."]
pub type BclrW<'a, REG> = crate::BitWriter<'a, REG, Bclr>;
impl<'a, REG> BclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not operate"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bclr::_0)
    }
    #[doc = "FIFO buffer cleared on the CPU side."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bclr::_1)
    }
}
#[doc = "Buffer Memory Valid Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bval {
    #[doc = "0: Invalid"]
    _0 = 0,
    #[doc = "1: Writing ended"]
    _1 = 1,
}
impl From<Bval> for bool {
    #[inline(always)]
    fn from(variant: Bval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BVAL` reader - Buffer Memory Valid Flag"]
pub type BvalR = crate::BitReader<Bval>;
impl BvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bval {
        match self.bits {
            false => Bval::_0,
            true => Bval::_1,
        }
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bval::_0
    }
    #[doc = "Writing ended"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bval::_1
    }
}
#[doc = "Field `BVAL` writer - Buffer Memory Valid Flag"]
pub type BvalW<'a, REG> = crate::BitWriter<'a, REG, Bval>;
impl<'a, REG> BvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bval::_0)
    }
    #[doc = "Writing ended"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bval::_1)
    }
}
impl R {
    #[doc = "Bits 0:8 - Receive Data Length Indicates the length of the receive data."]
    #[inline(always)]
    pub fn dtln(&self) -> DtlnR {
        DtlnR::new(self.bits & 0x01ff)
    }
    #[doc = "Bit 13 - FIFO Port Ready"]
    #[inline(always)]
    pub fn frdy(&self) -> FrdyR {
        FrdyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Buffer Memory Valid Flag"]
    #[inline(always)]
    pub fn bval(&self) -> BvalR {
        BvalR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFIFOCTR")
            .field("bval", &self.bval())
            .field("frdy", &self.frdy())
            .field("dtln", &self.dtln())
            .finish()
    }
}
impl W {
    #[doc = "Bit 14 - CPU Buffer Clear Note: Only 0 can be read."]
    #[inline(always)]
    pub fn bclr(&mut self) -> BclrW<CfifoctrSpec> {
        BclrW::new(self, 14)
    }
    #[doc = "Bit 15 - Buffer Memory Valid Flag"]
    #[inline(always)]
    pub fn bval(&mut self) -> BvalW<CfifoctrSpec> {
        BvalW::new(self, 15)
    }
}
#[doc = "CFIFO Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfifoctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifoctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfifoctrSpec;
impl crate::RegisterSpec for CfifoctrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfifoctr::R`](R) reader structure"]
impl crate::Readable for CfifoctrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfifoctr::W`](W) writer structure"]
impl crate::Writable for CfifoctrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFIFOCTR to value 0"]
impl crate::Resettable for CfifoctrSpec {}
