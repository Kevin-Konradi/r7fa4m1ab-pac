#[doc = "Register `MEMWAIT` reader"]
pub type R = crate::R<MemwaitSpec>;
#[doc = "Register `MEMWAIT` writer"]
pub type W = crate::W<MemwaitSpec>;
#[doc = "Memory Wait Cycle Select Note: Writing 0 to the MEMWAIT is prohibited when SCKDIVCR.ICK selects division by 1 and SCKSCR.CKSEL\\[2:0\\] bits select the system clock source that is faster than 32 MHz (ICLK > 32 MHz).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memwait {
    #[doc = "0: no wait"]
    _0 = 0,
    #[doc = "1: wait"]
    _1 = 1,
}
impl From<Memwait> for bool {
    #[inline(always)]
    fn from(variant: Memwait) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMWAIT` reader - Memory Wait Cycle Select Note: Writing 0 to the MEMWAIT is prohibited when SCKDIVCR.ICK selects division by 1 and SCKSCR.CKSEL\\[2:0\\] bits select the system clock source that is faster than 32 MHz (ICLK > 32 MHz)."]
pub type MemwaitR = crate::BitReader<Memwait>;
impl MemwaitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memwait {
        match self.bits {
            false => Memwait::_0,
            true => Memwait::_1,
        }
    }
    #[doc = "no wait"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Memwait::_0
    }
    #[doc = "wait"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Memwait::_1
    }
}
#[doc = "Field `MEMWAIT` writer - Memory Wait Cycle Select Note: Writing 0 to the MEMWAIT is prohibited when SCKDIVCR.ICK selects division by 1 and SCKSCR.CKSEL\\[2:0\\] bits select the system clock source that is faster than 32 MHz (ICLK > 32 MHz)."]
pub type MemwaitW<'a, REG> = crate::BitWriter<'a, REG, Memwait>;
impl<'a, REG> MemwaitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no wait"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Memwait::_0)
    }
    #[doc = "wait"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Memwait::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Memory Wait Cycle Select Note: Writing 0 to the MEMWAIT is prohibited when SCKDIVCR.ICK selects division by 1 and SCKSCR.CKSEL\\[2:0\\] bits select the system clock source that is faster than 32 MHz (ICLK > 32 MHz)."]
    #[inline(always)]
    pub fn memwait(&self) -> MemwaitR {
        MemwaitR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEMWAIT")
            .field("memwait", &self.memwait())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Memory Wait Cycle Select Note: Writing 0 to the MEMWAIT is prohibited when SCKDIVCR.ICK selects division by 1 and SCKSCR.CKSEL\\[2:0\\] bits select the system clock source that is faster than 32 MHz (ICLK > 32 MHz)."]
    #[inline(always)]
    pub fn memwait(&mut self) -> MemwaitW<MemwaitSpec> {
        MemwaitW::new(self, 0)
    }
}
#[doc = "Memory Wait Cycle Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`memwait::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memwait::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemwaitSpec;
impl crate::RegisterSpec for MemwaitSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`memwait::R`](R) reader structure"]
impl crate::Readable for MemwaitSpec {}
#[doc = "`write(|w| ..)` method takes [`memwait::W`](W) writer structure"]
impl crate::Writable for MemwaitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEMWAIT to value 0"]
impl crate::Resettable for MemwaitSpec {}
