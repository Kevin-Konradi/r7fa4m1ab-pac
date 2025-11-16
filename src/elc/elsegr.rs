#[doc = "Register `ELSEGR%s` reader"]
pub type R = crate::R<ElsegrSpec>;
#[doc = "Register `ELSEGR%s` writer"]
pub type W = crate::W<ElsegrSpec>;
#[doc = "Software Event Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seg {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Software event is generated."]
    _1 = 1,
}
impl From<Seg> for bool {
    #[inline(always)]
    fn from(variant: Seg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEG` writer - Software Event Generation"]
pub type SegW<'a, REG> = crate::BitWriter<'a, REG, Seg>;
impl<'a, REG> SegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Seg::_0)
    }
    #[doc = "Software event is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Seg::_1)
    }
}
#[doc = "SEG Bit Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum We {
    #[doc = "0: Write to SEG bit is disabled."]
    _0 = 0,
    #[doc = "1: Write to SEG bit is enabled."]
    _1 = 1,
}
impl From<We> for bool {
    #[inline(always)]
    fn from(variant: We) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE` reader - SEG Bit Write Enable"]
pub type WeR = crate::BitReader<We>;
impl WeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> We {
        match self.bits {
            false => We::_0,
            true => We::_1,
        }
    }
    #[doc = "Write to SEG bit is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == We::_0
    }
    #[doc = "Write to SEG bit is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == We::_1
    }
}
#[doc = "Field `WE` writer - SEG Bit Write Enable"]
pub type WeW<'a, REG> = crate::BitWriter<'a, REG, We>;
impl<'a, REG> WeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write to SEG bit is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(We::_0)
    }
    #[doc = "Write to SEG bit is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(We::_1)
    }
}
#[doc = "ELSEGR Register Write Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wi {
    #[doc = "0: Write to ELSEGR register is enabled."]
    _0 = 0,
    #[doc = "1: Write to ELSEGR register is disabled."]
    _1 = 1,
}
impl From<Wi> for bool {
    #[inline(always)]
    fn from(variant: Wi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WI` writer - ELSEGR Register Write Disable"]
pub type WiW<'a, REG> = crate::BitWriter<'a, REG, Wi>;
impl<'a, REG> WiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write to ELSEGR register is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wi::_0)
    }
    #[doc = "Write to ELSEGR register is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wi::_1)
    }
}
impl R {
    #[doc = "Bit 6 - SEG Bit Write Enable"]
    #[inline(always)]
    pub fn we(&self) -> WeR {
        WeR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ELSEGR").field("we", &self.we()).finish()
    }
}
impl W {
    #[doc = "Bit 0 - Software Event Generation"]
    #[inline(always)]
    pub fn seg(&mut self) -> SegW<ElsegrSpec> {
        SegW::new(self, 0)
    }
    #[doc = "Bit 6 - SEG Bit Write Enable"]
    #[inline(always)]
    pub fn we(&mut self) -> WeW<ElsegrSpec> {
        WeW::new(self, 6)
    }
    #[doc = "Bit 7 - ELSEGR Register Write Disable"]
    #[inline(always)]
    pub fn wi(&mut self) -> WiW<ElsegrSpec> {
        WiW::new(self, 7)
    }
}
#[doc = "Event Link Software Event Generation Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`elsegr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elsegr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ElsegrSpec;
impl crate::RegisterSpec for ElsegrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`elsegr::R`](R) reader structure"]
impl crate::Readable for ElsegrSpec {}
#[doc = "`write(|w| ..)` method takes [`elsegr::W`](W) writer structure"]
impl crate::Writable for ElsegrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ELSEGR%s to value 0x80"]
impl crate::Resettable for ElsegrSpec {
    const RESET_VALUE: u8 = 0x80;
}
