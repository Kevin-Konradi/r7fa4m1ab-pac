#[doc = "Register `PIPEPERI` reader"]
pub type R = crate::R<PipeperiSpec>;
#[doc = "Register `PIPEPERI` writer"]
pub type W = crate::W<PipeperiSpec>;
#[doc = "Field `IITV` reader - Interval Error Detection Interval Specifies the interval error detection timing for the selected pipe in terms of frames, which is expressed as nth power of 2."]
pub type IitvR = crate::FieldReader;
#[doc = "Field `IITV` writer - Interval Error Detection Interval Specifies the interval error detection timing for the selected pipe in terms of frames, which is expressed as nth power of 2."]
pub type IitvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Isochronous IN Buffer Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ifis {
    #[doc = "0: The buffer is not flushed."]
    _0 = 0,
    #[doc = "1: The buffer is flushed."]
    _1 = 1,
}
impl From<Ifis> for bool {
    #[inline(always)]
    fn from(variant: Ifis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFIS` reader - Isochronous IN Buffer Flush"]
pub type IfisR = crate::BitReader<Ifis>;
impl IfisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ifis {
        match self.bits {
            false => Ifis::_0,
            true => Ifis::_1,
        }
    }
    #[doc = "The buffer is not flushed."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ifis::_0
    }
    #[doc = "The buffer is flushed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ifis::_1
    }
}
#[doc = "Field `IFIS` writer - Isochronous IN Buffer Flush"]
pub type IfisW<'a, REG> = crate::BitWriter<'a, REG, Ifis>;
impl<'a, REG> IfisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The buffer is not flushed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ifis::_0)
    }
    #[doc = "The buffer is flushed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ifis::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Interval Error Detection Interval Specifies the interval error detection timing for the selected pipe in terms of frames, which is expressed as nth power of 2."]
    #[inline(always)]
    pub fn iitv(&self) -> IitvR {
        IitvR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 12 - Isochronous IN Buffer Flush"]
    #[inline(always)]
    pub fn ifis(&self) -> IfisR {
        IfisR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIPEPERI")
            .field("ifis", &self.ifis())
            .field("iitv", &self.iitv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Interval Error Detection Interval Specifies the interval error detection timing for the selected pipe in terms of frames, which is expressed as nth power of 2."]
    #[inline(always)]
    pub fn iitv(&mut self) -> IitvW<PipeperiSpec> {
        IitvW::new(self, 0)
    }
    #[doc = "Bit 12 - Isochronous IN Buffer Flush"]
    #[inline(always)]
    pub fn ifis(&mut self) -> IfisW<PipeperiSpec> {
        IfisW::new(self, 12)
    }
}
#[doc = "Pipe Cycle Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pipeperi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipeperi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PipeperiSpec;
impl crate::RegisterSpec for PipeperiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pipeperi::R`](R) reader structure"]
impl crate::Readable for PipeperiSpec {}
#[doc = "`write(|w| ..)` method takes [`pipeperi::W`](W) writer structure"]
impl crate::Writable for PipeperiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIPEPERI to value 0"]
impl crate::Resettable for PipeperiSpec {}
