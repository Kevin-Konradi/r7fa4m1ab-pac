#[doc = "Register `FLSTOP` reader"]
pub type R = crate::R<FlstopSpec>;
#[doc = "Register `FLSTOP` writer"]
pub type W = crate::W<FlstopSpec>;
#[doc = "Selecting ON/OFF of the Flash Memory Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flstop {
    #[doc = "0: Code flash and data flash memory operates"]
    _0 = 0,
    #[doc = "1: Code flash and data flash memory stops."]
    _1 = 1,
}
impl From<Flstop> for bool {
    #[inline(always)]
    fn from(variant: Flstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLSTOP` reader - Selecting ON/OFF of the Flash Memory Operation"]
pub type FlstopR = crate::BitReader<Flstop>;
impl FlstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flstop {
        match self.bits {
            false => Flstop::_0,
            true => Flstop::_1,
        }
    }
    #[doc = "Code flash and data flash memory operates"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Flstop::_0
    }
    #[doc = "Code flash and data flash memory stops."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Flstop::_1
    }
}
#[doc = "Field `FLSTOP` writer - Selecting ON/OFF of the Flash Memory Operation"]
pub type FlstopW<'a, REG> = crate::BitWriter<'a, REG, Flstop>;
impl<'a, REG> FlstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Code flash and data flash memory operates"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Flstop::_0)
    }
    #[doc = "Code flash and data flash memory stops."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Flstop::_1)
    }
}
#[doc = "Flash Memory Operation Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flstpf {
    #[doc = "0: Transition completed"]
    _0 = 0,
    #[doc = "1: During transition (from the flash-stop-status to flash-operating-status or vice versa)"]
    _1 = 1,
}
impl From<Flstpf> for bool {
    #[inline(always)]
    fn from(variant: Flstpf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLSTPF` reader - Flash Memory Operation Status Flag"]
pub type FlstpfR = crate::BitReader<Flstpf>;
impl FlstpfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flstpf {
        match self.bits {
            false => Flstpf::_0,
            true => Flstpf::_1,
        }
    }
    #[doc = "Transition completed"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Flstpf::_0
    }
    #[doc = "During transition (from the flash-stop-status to flash-operating-status or vice versa)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Flstpf::_1
    }
}
impl R {
    #[doc = "Bit 0 - Selecting ON/OFF of the Flash Memory Operation"]
    #[inline(always)]
    pub fn flstop(&self) -> FlstopR {
        FlstopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Flash Memory Operation Status Flag"]
    #[inline(always)]
    pub fn flstpf(&self) -> FlstpfR {
        FlstpfR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLSTOP")
            .field("flstpf", &self.flstpf())
            .field("flstop", &self.flstop())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Selecting ON/OFF of the Flash Memory Operation"]
    #[inline(always)]
    pub fn flstop(&mut self) -> FlstopW<FlstopSpec> {
        FlstopW::new(self, 0)
    }
}
#[doc = "Flash Operation Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flstop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flstop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlstopSpec;
impl crate::RegisterSpec for FlstopSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`flstop::R`](R) reader structure"]
impl crate::Readable for FlstopSpec {}
#[doc = "`write(|w| ..)` method takes [`flstop::W`](W) writer structure"]
impl crate::Writable for FlstopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLSTOP to value 0"]
impl crate::Resettable for FlstopSpec {}
