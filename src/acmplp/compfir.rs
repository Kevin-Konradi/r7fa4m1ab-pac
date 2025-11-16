#[doc = "Register `COMPFIR` reader"]
pub type R = crate::R<CompfirSpec>;
#[doc = "Register `COMPFIR` writer"]
pub type W = crate::W<CompfirSpec>;
#[doc = "ACMPLP0 Filter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C0fck {
    #[doc = "0: No Sampling (bypass)"]
    _00 = 0,
    #[doc = "1: Sampling at PCLK"]
    _01 = 1,
    #[doc = "2: Sampling at PCLK/8"]
    _10 = 2,
    #[doc = "3: Sampling at PCLK/32"]
    _11 = 3,
}
impl From<C0fck> for u8 {
    #[inline(always)]
    fn from(variant: C0fck) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C0fck {
    type Ux = u8;
}
impl crate::IsEnum for C0fck {}
#[doc = "Field `C0FCK` reader - ACMPLP0 Filter Select"]
pub type C0fckR = crate::FieldReader<C0fck>;
impl C0fckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0fck {
        match self.bits {
            0 => C0fck::_00,
            1 => C0fck::_01,
            2 => C0fck::_10,
            3 => C0fck::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "No Sampling (bypass)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == C0fck::_00
    }
    #[doc = "Sampling at PCLK"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == C0fck::_01
    }
    #[doc = "Sampling at PCLK/8"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == C0fck::_10
    }
    #[doc = "Sampling at PCLK/32"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == C0fck::_11
    }
}
#[doc = "Field `C0FCK` writer - ACMPLP0 Filter Select"]
pub type C0fckW<'a, REG> = crate::FieldWriter<'a, REG, 2, C0fck, crate::Safe>;
impl<'a, REG> C0fckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Sampling (bypass)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(C0fck::_00)
    }
    #[doc = "Sampling at PCLK"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(C0fck::_01)
    }
    #[doc = "Sampling at PCLK/8"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(C0fck::_10)
    }
    #[doc = "Sampling at PCLK/32"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(C0fck::_11)
    }
}
#[doc = "ACMPLP0 Edge Polarity Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0epo {
    #[doc = "0: Interrupt and ELC event request at rising edge"]
    _0 = 0,
    #[doc = "1: Interrupt and ELC event request at falling edge"]
    _1 = 1,
}
impl From<C0epo> for bool {
    #[inline(always)]
    fn from(variant: C0epo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0EPO` reader - ACMPLP0 Edge Polarity Switching"]
pub type C0epoR = crate::BitReader<C0epo>;
impl C0epoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0epo {
        match self.bits {
            false => C0epo::_0,
            true => C0epo::_1,
        }
    }
    #[doc = "Interrupt and ELC event request at rising edge"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0epo::_0
    }
    #[doc = "Interrupt and ELC event request at falling edge"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0epo::_1
    }
}
#[doc = "Field `C0EPO` writer - ACMPLP0 Edge Polarity Switching"]
pub type C0epoW<'a, REG> = crate::BitWriter<'a, REG, C0epo>;
impl<'a, REG> C0epoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt and ELC event request at rising edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0epo::_0)
    }
    #[doc = "Interrupt and ELC event request at falling edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0epo::_1)
    }
}
#[doc = "ACMPLP0 Edge Detection Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0edg {
    #[doc = "0: Interrupt and ELC event request by one-edge detection"]
    _0 = 0,
    #[doc = "1: Interrupt and ELC event request by both-edge detection"]
    _1 = 1,
}
impl From<C0edg> for bool {
    #[inline(always)]
    fn from(variant: C0edg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0EDG` reader - ACMPLP0 Edge Detection Selection"]
pub type C0edgR = crate::BitReader<C0edg>;
impl C0edgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0edg {
        match self.bits {
            false => C0edg::_0,
            true => C0edg::_1,
        }
    }
    #[doc = "Interrupt and ELC event request by one-edge detection"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0edg::_0
    }
    #[doc = "Interrupt and ELC event request by both-edge detection"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0edg::_1
    }
}
#[doc = "Field `C0EDG` writer - ACMPLP0 Edge Detection Selection"]
pub type C0edgW<'a, REG> = crate::BitWriter<'a, REG, C0edg>;
impl<'a, REG> C0edgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt and ELC event request by one-edge detection"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0edg::_0)
    }
    #[doc = "Interrupt and ELC event request by both-edge detection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0edg::_1)
    }
}
#[doc = "ACMPLP1 Filter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C1fck {
    #[doc = "0: No Sampling (bypass)"]
    _00 = 0,
    #[doc = "1: Sampling at PCLK"]
    _01 = 1,
    #[doc = "2: Sampling at PCLK/8"]
    _10 = 2,
    #[doc = "3: Sampling at PCLK/32"]
    _11 = 3,
}
impl From<C1fck> for u8 {
    #[inline(always)]
    fn from(variant: C1fck) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C1fck {
    type Ux = u8;
}
impl crate::IsEnum for C1fck {}
#[doc = "Field `C1FCK` reader - ACMPLP1 Filter Select"]
pub type C1fckR = crate::FieldReader<C1fck>;
impl C1fckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1fck {
        match self.bits {
            0 => C1fck::_00,
            1 => C1fck::_01,
            2 => C1fck::_10,
            3 => C1fck::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "No Sampling (bypass)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == C1fck::_00
    }
    #[doc = "Sampling at PCLK"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == C1fck::_01
    }
    #[doc = "Sampling at PCLK/8"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == C1fck::_10
    }
    #[doc = "Sampling at PCLK/32"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == C1fck::_11
    }
}
#[doc = "Field `C1FCK` writer - ACMPLP1 Filter Select"]
pub type C1fckW<'a, REG> = crate::FieldWriter<'a, REG, 2, C1fck, crate::Safe>;
impl<'a, REG> C1fckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Sampling (bypass)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(C1fck::_00)
    }
    #[doc = "Sampling at PCLK"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(C1fck::_01)
    }
    #[doc = "Sampling at PCLK/8"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(C1fck::_10)
    }
    #[doc = "Sampling at PCLK/32"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(C1fck::_11)
    }
}
#[doc = "ACMPLP1 Edge Polarity Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1epo {
    #[doc = "0: Interrupt and ELC event request at rising edge"]
    _0 = 0,
    #[doc = "1: Interrupt and ELC event request at falling edge"]
    _1 = 1,
}
impl From<C1epo> for bool {
    #[inline(always)]
    fn from(variant: C1epo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1EPO` reader - ACMPLP1 Edge Polarity Switching"]
pub type C1epoR = crate::BitReader<C1epo>;
impl C1epoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1epo {
        match self.bits {
            false => C1epo::_0,
            true => C1epo::_1,
        }
    }
    #[doc = "Interrupt and ELC event request at rising edge"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1epo::_0
    }
    #[doc = "Interrupt and ELC event request at falling edge"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1epo::_1
    }
}
#[doc = "Field `C1EPO` writer - ACMPLP1 Edge Polarity Switching"]
pub type C1epoW<'a, REG> = crate::BitWriter<'a, REG, C1epo>;
impl<'a, REG> C1epoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt and ELC event request at rising edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1epo::_0)
    }
    #[doc = "Interrupt and ELC event request at falling edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1epo::_1)
    }
}
#[doc = "ACMPLP1 Edge Detection Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1edg {
    #[doc = "0: Interrupt and ELC event request by one-edge detection"]
    _0 = 0,
    #[doc = "1: Interrupt and ELC event request by both-edge detection"]
    _1 = 1,
}
impl From<C1edg> for bool {
    #[inline(always)]
    fn from(variant: C1edg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1EDG` reader - ACMPLP1 Edge Detection Selection"]
pub type C1edgR = crate::BitReader<C1edg>;
impl C1edgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1edg {
        match self.bits {
            false => C1edg::_0,
            true => C1edg::_1,
        }
    }
    #[doc = "Interrupt and ELC event request by one-edge detection"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1edg::_0
    }
    #[doc = "Interrupt and ELC event request by both-edge detection"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1edg::_1
    }
}
#[doc = "Field `C1EDG` writer - ACMPLP1 Edge Detection Selection"]
pub type C1edgW<'a, REG> = crate::BitWriter<'a, REG, C1edg>;
impl<'a, REG> C1edgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt and ELC event request by one-edge detection"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1edg::_0)
    }
    #[doc = "Interrupt and ELC event request by both-edge detection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1edg::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - ACMPLP0 Filter Select"]
    #[inline(always)]
    pub fn c0fck(&self) -> C0fckR {
        C0fckR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - ACMPLP0 Edge Polarity Switching"]
    #[inline(always)]
    pub fn c0epo(&self) -> C0epoR {
        C0epoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ACMPLP0 Edge Detection Selection"]
    #[inline(always)]
    pub fn c0edg(&self) -> C0edgR {
        C0edgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - ACMPLP1 Filter Select"]
    #[inline(always)]
    pub fn c1fck(&self) -> C1fckR {
        C1fckR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - ACMPLP1 Edge Polarity Switching"]
    #[inline(always)]
    pub fn c1epo(&self) -> C1epoR {
        C1epoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ACMPLP1 Edge Detection Selection"]
    #[inline(always)]
    pub fn c1edg(&self) -> C1edgR {
        C1edgR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMPFIR")
            .field("c1edg", &self.c1edg())
            .field("c1epo", &self.c1epo())
            .field("c1fck", &self.c1fck())
            .field("c0edg", &self.c0edg())
            .field("c0epo", &self.c0epo())
            .field("c0fck", &self.c0fck())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - ACMPLP0 Filter Select"]
    #[inline(always)]
    pub fn c0fck(&mut self) -> C0fckW<CompfirSpec> {
        C0fckW::new(self, 0)
    }
    #[doc = "Bit 2 - ACMPLP0 Edge Polarity Switching"]
    #[inline(always)]
    pub fn c0epo(&mut self) -> C0epoW<CompfirSpec> {
        C0epoW::new(self, 2)
    }
    #[doc = "Bit 3 - ACMPLP0 Edge Detection Selection"]
    #[inline(always)]
    pub fn c0edg(&mut self) -> C0edgW<CompfirSpec> {
        C0edgW::new(self, 3)
    }
    #[doc = "Bits 4:5 - ACMPLP1 Filter Select"]
    #[inline(always)]
    pub fn c1fck(&mut self) -> C1fckW<CompfirSpec> {
        C1fckW::new(self, 4)
    }
    #[doc = "Bit 6 - ACMPLP1 Edge Polarity Switching"]
    #[inline(always)]
    pub fn c1epo(&mut self) -> C1epoW<CompfirSpec> {
        C1epoW::new(self, 6)
    }
    #[doc = "Bit 7 - ACMPLP1 Edge Detection Selection"]
    #[inline(always)]
    pub fn c1edg(&mut self) -> C1edgW<CompfirSpec> {
        C1edgW::new(self, 7)
    }
}
#[doc = "ACMPLP Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compfir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compfir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompfirSpec;
impl crate::RegisterSpec for CompfirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`compfir::R`](R) reader structure"]
impl crate::Readable for CompfirSpec {}
#[doc = "`write(|w| ..)` method takes [`compfir::W`](W) writer structure"]
impl crate::Writable for CompfirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMPFIR to value 0"]
impl crate::Resettable for CompfirSpec {}
