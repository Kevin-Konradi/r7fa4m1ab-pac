#[doc = "Register `ECSR` reader"]
pub type R = crate::R<EcsrSpec>;
#[doc = "Register `ECSR` writer"]
pub type W = crate::W<EcsrSpec>;
#[doc = "Stuff Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sef {
    #[doc = "0: No stuff error detected"]
    _0 = 0,
    #[doc = "1: Stuff error detected"]
    _1 = 1,
}
impl From<Sef> for bool {
    #[inline(always)]
    fn from(variant: Sef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEF` reader - Stuff Error Flag"]
pub type SefR = crate::BitReader<Sef>;
impl SefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sef {
        match self.bits {
            false => Sef::_0,
            true => Sef::_1,
        }
    }
    #[doc = "No stuff error detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sef::_0
    }
    #[doc = "Stuff error detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sef::_1
    }
}
#[doc = "Field `SEF` writer - Stuff Error Flag"]
pub type SefW<'a, REG> = crate::BitWriter<'a, REG, Sef>;
impl<'a, REG> SefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No stuff error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sef::_0)
    }
    #[doc = "Stuff error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sef::_1)
    }
}
#[doc = "Form Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fef {
    #[doc = "0: No form error detected"]
    _0 = 0,
    #[doc = "1: Form error detected"]
    _1 = 1,
}
impl From<Fef> for bool {
    #[inline(always)]
    fn from(variant: Fef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEF` reader - Form Error Flag"]
pub type FefR = crate::BitReader<Fef>;
impl FefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fef {
        match self.bits {
            false => Fef::_0,
            true => Fef::_1,
        }
    }
    #[doc = "No form error detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fef::_0
    }
    #[doc = "Form error detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fef::_1
    }
}
#[doc = "Field `FEF` writer - Form Error Flag"]
pub type FefW<'a, REG> = crate::BitWriter<'a, REG, Fef>;
impl<'a, REG> FefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No form error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fef::_0)
    }
    #[doc = "Form error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fef::_1)
    }
}
#[doc = "ACK Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aef {
    #[doc = "0: No ACK error detected"]
    _0 = 0,
    #[doc = "1: ACK error detected"]
    _1 = 1,
}
impl From<Aef> for bool {
    #[inline(always)]
    fn from(variant: Aef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AEF` reader - ACK Error Flag"]
pub type AefR = crate::BitReader<Aef>;
impl AefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aef {
        match self.bits {
            false => Aef::_0,
            true => Aef::_1,
        }
    }
    #[doc = "No ACK error detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aef::_0
    }
    #[doc = "ACK error detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aef::_1
    }
}
#[doc = "Field `AEF` writer - ACK Error Flag"]
pub type AefW<'a, REG> = crate::BitWriter<'a, REG, Aef>;
impl<'a, REG> AefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ACK error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aef::_0)
    }
    #[doc = "ACK error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aef::_1)
    }
}
#[doc = "CRC Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cef {
    #[doc = "0: No CRC error detected"]
    _0 = 0,
    #[doc = "1: CRC error detected"]
    _1 = 1,
}
impl From<Cef> for bool {
    #[inline(always)]
    fn from(variant: Cef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEF` reader - CRC Error Flag"]
pub type CefR = crate::BitReader<Cef>;
impl CefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cef {
        match self.bits {
            false => Cef::_0,
            true => Cef::_1,
        }
    }
    #[doc = "No CRC error detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cef::_0
    }
    #[doc = "CRC error detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cef::_1
    }
}
#[doc = "Field `CEF` writer - CRC Error Flag"]
pub type CefW<'a, REG> = crate::BitWriter<'a, REG, Cef>;
impl<'a, REG> CefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No CRC error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cef::_0)
    }
    #[doc = "CRC error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cef::_1)
    }
}
#[doc = "Bit Error (recessive) Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Be1f {
    #[doc = "0: No bit error (recessive) detected"]
    _0 = 0,
    #[doc = "1: Bit error (recessive) detected"]
    _1 = 1,
}
impl From<Be1f> for bool {
    #[inline(always)]
    fn from(variant: Be1f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BE1F` reader - Bit Error (recessive) Flag"]
pub type Be1fR = crate::BitReader<Be1f>;
impl Be1fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Be1f {
        match self.bits {
            false => Be1f::_0,
            true => Be1f::_1,
        }
    }
    #[doc = "No bit error (recessive) detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Be1f::_0
    }
    #[doc = "Bit error (recessive) detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Be1f::_1
    }
}
#[doc = "Field `BE1F` writer - Bit Error (recessive) Flag"]
pub type Be1fW<'a, REG> = crate::BitWriter<'a, REG, Be1f>;
impl<'a, REG> Be1fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No bit error (recessive) detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Be1f::_0)
    }
    #[doc = "Bit error (recessive) detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Be1f::_1)
    }
}
#[doc = "Bit Error (dominant) Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Be0f {
    #[doc = "0: No bit error (dominant) detected"]
    _0 = 0,
    #[doc = "1: Bit error (dominant) detected"]
    _1 = 1,
}
impl From<Be0f> for bool {
    #[inline(always)]
    fn from(variant: Be0f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BE0F` reader - Bit Error (dominant) Flag"]
pub type Be0fR = crate::BitReader<Be0f>;
impl Be0fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Be0f {
        match self.bits {
            false => Be0f::_0,
            true => Be0f::_1,
        }
    }
    #[doc = "No bit error (dominant) detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Be0f::_0
    }
    #[doc = "Bit error (dominant) detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Be0f::_1
    }
}
#[doc = "Field `BE0F` writer - Bit Error (dominant) Flag"]
pub type Be0fW<'a, REG> = crate::BitWriter<'a, REG, Be0f>;
impl<'a, REG> Be0fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No bit error (dominant) detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Be0f::_0)
    }
    #[doc = "Bit error (dominant) detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Be0f::_1)
    }
}
#[doc = "ACK Delimiter Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adef {
    #[doc = "0: No ACK delimiter error detected"]
    _0 = 0,
    #[doc = "1: ACK delimiter error detected"]
    _1 = 1,
}
impl From<Adef> for bool {
    #[inline(always)]
    fn from(variant: Adef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADEF` reader - ACK Delimiter Error Flag"]
pub type AdefR = crate::BitReader<Adef>;
impl AdefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adef {
        match self.bits {
            false => Adef::_0,
            true => Adef::_1,
        }
    }
    #[doc = "No ACK delimiter error detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adef::_0
    }
    #[doc = "ACK delimiter error detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adef::_1
    }
}
#[doc = "Field `ADEF` writer - ACK Delimiter Error Flag"]
pub type AdefW<'a, REG> = crate::BitWriter<'a, REG, Adef>;
impl<'a, REG> AdefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ACK delimiter error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adef::_0)
    }
    #[doc = "ACK delimiter error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adef::_1)
    }
}
#[doc = "Error Display Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edpm {
    #[doc = "0: Output of first detected error code"]
    _0 = 0,
    #[doc = "1: Output of accumulated error code"]
    _1 = 1,
}
impl From<Edpm> for bool {
    #[inline(always)]
    fn from(variant: Edpm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDPM` reader - Error Display Mode Select"]
pub type EdpmR = crate::BitReader<Edpm>;
impl EdpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edpm {
        match self.bits {
            false => Edpm::_0,
            true => Edpm::_1,
        }
    }
    #[doc = "Output of first detected error code"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Edpm::_0
    }
    #[doc = "Output of accumulated error code"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Edpm::_1
    }
}
#[doc = "Field `EDPM` writer - Error Display Mode Select"]
pub type EdpmW<'a, REG> = crate::BitWriter<'a, REG, Edpm>;
impl<'a, REG> EdpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output of first detected error code"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Edpm::_0)
    }
    #[doc = "Output of accumulated error code"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Edpm::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Stuff Error Flag"]
    #[inline(always)]
    pub fn sef(&self) -> SefR {
        SefR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Form Error Flag"]
    #[inline(always)]
    pub fn fef(&self) -> FefR {
        FefR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ACK Error Flag"]
    #[inline(always)]
    pub fn aef(&self) -> AefR {
        AefR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CRC Error Flag"]
    #[inline(always)]
    pub fn cef(&self) -> CefR {
        CefR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit Error (recessive) Flag"]
    #[inline(always)]
    pub fn be1f(&self) -> Be1fR {
        Be1fR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit Error (dominant) Flag"]
    #[inline(always)]
    pub fn be0f(&self) -> Be0fR {
        Be0fR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ACK Delimiter Error Flag"]
    #[inline(always)]
    pub fn adef(&self) -> AdefR {
        AdefR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error Display Mode Select"]
    #[inline(always)]
    pub fn edpm(&self) -> EdpmR {
        EdpmR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECSR")
            .field("edpm", &self.edpm())
            .field("adef", &self.adef())
            .field("be0f", &self.be0f())
            .field("be1f", &self.be1f())
            .field("cef", &self.cef())
            .field("aef", &self.aef())
            .field("fef", &self.fef())
            .field("sef", &self.sef())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Stuff Error Flag"]
    #[inline(always)]
    pub fn sef(&mut self) -> SefW<EcsrSpec> {
        SefW::new(self, 0)
    }
    #[doc = "Bit 1 - Form Error Flag"]
    #[inline(always)]
    pub fn fef(&mut self) -> FefW<EcsrSpec> {
        FefW::new(self, 1)
    }
    #[doc = "Bit 2 - ACK Error Flag"]
    #[inline(always)]
    pub fn aef(&mut self) -> AefW<EcsrSpec> {
        AefW::new(self, 2)
    }
    #[doc = "Bit 3 - CRC Error Flag"]
    #[inline(always)]
    pub fn cef(&mut self) -> CefW<EcsrSpec> {
        CefW::new(self, 3)
    }
    #[doc = "Bit 4 - Bit Error (recessive) Flag"]
    #[inline(always)]
    pub fn be1f(&mut self) -> Be1fW<EcsrSpec> {
        Be1fW::new(self, 4)
    }
    #[doc = "Bit 5 - Bit Error (dominant) Flag"]
    #[inline(always)]
    pub fn be0f(&mut self) -> Be0fW<EcsrSpec> {
        Be0fW::new(self, 5)
    }
    #[doc = "Bit 6 - ACK Delimiter Error Flag"]
    #[inline(always)]
    pub fn adef(&mut self) -> AdefW<EcsrSpec> {
        AdefW::new(self, 6)
    }
    #[doc = "Bit 7 - Error Display Mode Select"]
    #[inline(always)]
    pub fn edpm(&mut self) -> EdpmW<EcsrSpec> {
        EdpmW::new(self, 7)
    }
}
#[doc = "Error Code Store Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcsrSpec;
impl crate::RegisterSpec for EcsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ecsr::R`](R) reader structure"]
impl crate::Readable for EcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ecsr::W`](W) writer structure"]
impl crate::Writable for EcsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECSR to value 0"]
impl crate::Resettable for EcsrSpec {}
