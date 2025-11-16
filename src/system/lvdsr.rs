#[doc = "Register `LVD%sSR` reader"]
pub type R = crate::R<LvdsrSpec>;
#[doc = "Register `LVD%sSR` writer"]
pub type W = crate::W<LvdsrSpec>;
#[doc = "Voltage Monitor Voltage Change Detection Flag NOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Det {
    #[doc = "0: Not detected"]
    _0 = 0,
    #[doc = "1: Vdet1 passage detection"]
    _1 = 1,
}
impl From<Det> for bool {
    #[inline(always)]
    fn from(variant: Det) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DET` reader - Voltage Monitor Voltage Change Detection Flag NOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0.\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type DetR = crate::BitReader<Det>;
impl DetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Det {
        match self.bits {
            false => Det::_0,
            true => Det::_1,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Det::_0
    }
    #[doc = "Vdet1 passage detection"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Det::_1
    }
}
#[doc = "Field `DET` writer - Voltage Monitor Voltage Change Detection Flag NOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0."]
pub type DetW<'a, REG> = crate::BitWriter0C<'a, REG, Det>;
impl<'a, REG> DetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Det::_0)
    }
    #[doc = "Vdet1 passage detection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Det::_1)
    }
}
#[doc = "Voltage Monitor 1 Signal Monitor Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mon {
    #[doc = "0: VCC < Vdet"]
    _0 = 0,
    #[doc = "1: VCC >= Vdet or MON bit is disabled"]
    _1 = 1,
}
impl From<Mon> for bool {
    #[inline(always)]
    fn from(variant: Mon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MON` reader - Voltage Monitor 1 Signal Monitor Flag"]
pub type MonR = crate::BitReader<Mon>;
impl MonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mon {
        match self.bits {
            false => Mon::_0,
            true => Mon::_1,
        }
    }
    #[doc = "VCC < Vdet"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mon::_0
    }
    #[doc = "VCC >= Vdet or MON bit is disabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mon::_1
    }
}
impl R {
    #[doc = "Bit 0 - Voltage Monitor Voltage Change Detection Flag NOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0."]
    #[inline(always)]
    pub fn det(&self) -> DetR {
        DetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Voltage Monitor 1 Signal Monitor Flag"]
    #[inline(always)]
    pub fn mon(&self) -> MonR {
        MonR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LVDSR").field("mon", &self.mon()).finish()
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Monitor Voltage Change Detection Flag NOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0."]
    #[inline(always)]
    pub fn det(&mut self) -> DetW<LvdsrSpec> {
        DetW::new(self, 0)
    }
}
#[doc = "Voltage Monitor %s Circuit Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lvdsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvdsrSpec;
impl crate::RegisterSpec for LvdsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvdsr::R`](R) reader structure"]
impl crate::Readable for LvdsrSpec {}
#[doc = "`write(|w| ..)` method takes [`lvdsr::W`](W) writer structure"]
impl crate::Writable for LvdsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x01;
}
#[doc = "`reset()` method sets LVD%sSR to value 0x02"]
impl crate::Resettable for LvdsrSpec {
    const RESET_VALUE: u8 = 0x02;
}
