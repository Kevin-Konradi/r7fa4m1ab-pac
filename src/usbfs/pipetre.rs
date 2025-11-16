#[doc = "Register `PIPE%sTRE` reader"]
pub type R = crate::R<PipetreSpec>;
#[doc = "Register `PIPE%sTRE` writer"]
pub type W = crate::W<PipetreSpec>;
#[doc = "Transaction Counter Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trclr {
    #[doc = "0: Invalid"]
    _0 = 0,
    #[doc = "1: The current counter value is cleared."]
    _1 = 1,
}
impl From<Trclr> for bool {
    #[inline(always)]
    fn from(variant: Trclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRCLR` reader - Transaction Counter Clear"]
pub type TrclrR = crate::BitReader<Trclr>;
impl TrclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trclr {
        match self.bits {
            false => Trclr::_0,
            true => Trclr::_1,
        }
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Trclr::_0
    }
    #[doc = "The current counter value is cleared."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Trclr::_1
    }
}
#[doc = "Field `TRCLR` writer - Transaction Counter Clear"]
pub type TrclrW<'a, REG> = crate::BitWriter<'a, REG, Trclr>;
impl<'a, REG> TrclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Trclr::_0)
    }
    #[doc = "The current counter value is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Trclr::_1)
    }
}
#[doc = "Transaction Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trenb {
    #[doc = "0: Transaction counter is disabled."]
    _0 = 0,
    #[doc = "1: Transaction counter is enabled."]
    _1 = 1,
}
impl From<Trenb> for bool {
    #[inline(always)]
    fn from(variant: Trenb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRENB` reader - Transaction Counter Enable"]
pub type TrenbR = crate::BitReader<Trenb>;
impl TrenbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trenb {
        match self.bits {
            false => Trenb::_0,
            true => Trenb::_1,
        }
    }
    #[doc = "Transaction counter is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Trenb::_0
    }
    #[doc = "Transaction counter is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Trenb::_1
    }
}
#[doc = "Field `TRENB` writer - Transaction Counter Enable"]
pub type TrenbW<'a, REG> = crate::BitWriter<'a, REG, Trenb>;
impl<'a, REG> TrenbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transaction counter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Trenb::_0)
    }
    #[doc = "Transaction counter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Trenb::_1)
    }
}
impl R {
    #[doc = "Bit 8 - Transaction Counter Clear"]
    #[inline(always)]
    pub fn trclr(&self) -> TrclrR {
        TrclrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transaction Counter Enable"]
    #[inline(always)]
    pub fn trenb(&self) -> TrenbR {
        TrenbR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIPETRE")
            .field("trenb", &self.trenb())
            .field("trclr", &self.trclr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - Transaction Counter Clear"]
    #[inline(always)]
    pub fn trclr(&mut self) -> TrclrW<PipetreSpec> {
        TrclrW::new(self, 8)
    }
    #[doc = "Bit 9 - Transaction Counter Enable"]
    #[inline(always)]
    pub fn trenb(&mut self) -> TrenbW<PipetreSpec> {
        TrenbW::new(self, 9)
    }
}
#[doc = "Pipe %s Transaction Counter Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pipetre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipetre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PipetreSpec;
impl crate::RegisterSpec for PipetreSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pipetre::R`](R) reader structure"]
impl crate::Readable for PipetreSpec {}
#[doc = "`write(|w| ..)` method takes [`pipetre::W`](W) writer structure"]
impl crate::Writable for PipetreSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIPE%sTRE to value 0"]
impl crate::Resettable for PipetreSpec {}
