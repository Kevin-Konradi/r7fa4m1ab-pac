#[doc = "Register `ADANSA1` reader"]
pub type R = crate::R<Adansa1Spec>;
#[doc = "Register `ADANSA1` writer"]
pub type W = crate::W<Adansa1Spec>;
#[doc = "AN016 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa16 {
    #[doc = "0: AN016 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN016 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa16> for bool {
    #[inline(always)]
    fn from(variant: Ansa16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA16` reader - AN016 Select"]
pub type Ansa16R = crate::BitReader<Ansa16>;
impl Ansa16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa16 {
        match self.bits {
            false => Ansa16::_0,
            true => Ansa16::_1,
        }
    }
    #[doc = "AN016 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa16::_0
    }
    #[doc = "AN016 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa16::_1
    }
}
#[doc = "Field `ANSA16` writer - AN016 Select"]
pub type Ansa16W<'a, REG> = crate::BitWriter<'a, REG, Ansa16>;
impl<'a, REG> Ansa16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN016 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa16::_0)
    }
    #[doc = "AN016 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa16::_1)
    }
}
#[doc = "AN017 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa17 {
    #[doc = "0: AN017 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN017 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa17> for bool {
    #[inline(always)]
    fn from(variant: Ansa17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA17` reader - AN017 Select"]
pub type Ansa17R = crate::BitReader<Ansa17>;
impl Ansa17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa17 {
        match self.bits {
            false => Ansa17::_0,
            true => Ansa17::_1,
        }
    }
    #[doc = "AN017 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa17::_0
    }
    #[doc = "AN017 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa17::_1
    }
}
#[doc = "Field `ANSA17` writer - AN017 Select"]
pub type Ansa17W<'a, REG> = crate::BitWriter<'a, REG, Ansa17>;
impl<'a, REG> Ansa17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN017 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa17::_0)
    }
    #[doc = "AN017 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa17::_1)
    }
}
#[doc = "AN018 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa18 {
    #[doc = "0: AN018 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN018 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa18> for bool {
    #[inline(always)]
    fn from(variant: Ansa18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA18` reader - AN018 Select"]
pub type Ansa18R = crate::BitReader<Ansa18>;
impl Ansa18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa18 {
        match self.bits {
            false => Ansa18::_0,
            true => Ansa18::_1,
        }
    }
    #[doc = "AN018 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa18::_0
    }
    #[doc = "AN018 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa18::_1
    }
}
#[doc = "Field `ANSA18` writer - AN018 Select"]
pub type Ansa18W<'a, REG> = crate::BitWriter<'a, REG, Ansa18>;
impl<'a, REG> Ansa18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN018 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa18::_0)
    }
    #[doc = "AN018 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa18::_1)
    }
}
#[doc = "AN019 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa19 {
    #[doc = "0: AN019 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN019 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa19> for bool {
    #[inline(always)]
    fn from(variant: Ansa19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA19` reader - AN019 Select"]
pub type Ansa19R = crate::BitReader<Ansa19>;
impl Ansa19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa19 {
        match self.bits {
            false => Ansa19::_0,
            true => Ansa19::_1,
        }
    }
    #[doc = "AN019 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa19::_0
    }
    #[doc = "AN019 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa19::_1
    }
}
#[doc = "Field `ANSA19` writer - AN019 Select"]
pub type Ansa19W<'a, REG> = crate::BitWriter<'a, REG, Ansa19>;
impl<'a, REG> Ansa19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN019 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa19::_0)
    }
    #[doc = "AN019 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa19::_1)
    }
}
#[doc = "AN020 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa20 {
    #[doc = "0: AN020 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN020 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa20> for bool {
    #[inline(always)]
    fn from(variant: Ansa20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA20` reader - AN020 Select"]
pub type Ansa20R = crate::BitReader<Ansa20>;
impl Ansa20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa20 {
        match self.bits {
            false => Ansa20::_0,
            true => Ansa20::_1,
        }
    }
    #[doc = "AN020 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa20::_0
    }
    #[doc = "AN020 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa20::_1
    }
}
#[doc = "Field `ANSA20` writer - AN020 Select"]
pub type Ansa20W<'a, REG> = crate::BitWriter<'a, REG, Ansa20>;
impl<'a, REG> Ansa20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN020 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa20::_0)
    }
    #[doc = "AN020 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa20::_1)
    }
}
#[doc = "AN021 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa21 {
    #[doc = "0: AN021 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN021 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa21> for bool {
    #[inline(always)]
    fn from(variant: Ansa21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA21` reader - AN021 Select"]
pub type Ansa21R = crate::BitReader<Ansa21>;
impl Ansa21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa21 {
        match self.bits {
            false => Ansa21::_0,
            true => Ansa21::_1,
        }
    }
    #[doc = "AN021 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa21::_0
    }
    #[doc = "AN021 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa21::_1
    }
}
#[doc = "Field `ANSA21` writer - AN021 Select"]
pub type Ansa21W<'a, REG> = crate::BitWriter<'a, REG, Ansa21>;
impl<'a, REG> Ansa21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN021 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa21::_0)
    }
    #[doc = "AN021 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa21::_1)
    }
}
#[doc = "AN022 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa22 {
    #[doc = "0: AN022 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN022 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa22> for bool {
    #[inline(always)]
    fn from(variant: Ansa22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA22` reader - AN022 Select"]
pub type Ansa22R = crate::BitReader<Ansa22>;
impl Ansa22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa22 {
        match self.bits {
            false => Ansa22::_0,
            true => Ansa22::_1,
        }
    }
    #[doc = "AN022 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa22::_0
    }
    #[doc = "AN022 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa22::_1
    }
}
#[doc = "Field `ANSA22` writer - AN022 Select"]
pub type Ansa22W<'a, REG> = crate::BitWriter<'a, REG, Ansa22>;
impl<'a, REG> Ansa22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN022 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa22::_0)
    }
    #[doc = "AN022 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa22::_1)
    }
}
#[doc = "AN023 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa23 {
    #[doc = "0: AN023 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN023 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa23> for bool {
    #[inline(always)]
    fn from(variant: Ansa23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA23` reader - AN023 Select"]
pub type Ansa23R = crate::BitReader<Ansa23>;
impl Ansa23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa23 {
        match self.bits {
            false => Ansa23::_0,
            true => Ansa23::_1,
        }
    }
    #[doc = "AN023 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa23::_0
    }
    #[doc = "AN023 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa23::_1
    }
}
#[doc = "Field `ANSA23` writer - AN023 Select"]
pub type Ansa23W<'a, REG> = crate::BitWriter<'a, REG, Ansa23>;
impl<'a, REG> Ansa23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN023 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa23::_0)
    }
    #[doc = "AN023 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa23::_1)
    }
}
#[doc = "AN024 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa24 {
    #[doc = "0: AN024 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN024 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa24> for bool {
    #[inline(always)]
    fn from(variant: Ansa24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA24` reader - AN024 Select"]
pub type Ansa24R = crate::BitReader<Ansa24>;
impl Ansa24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa24 {
        match self.bits {
            false => Ansa24::_0,
            true => Ansa24::_1,
        }
    }
    #[doc = "AN024 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa24::_0
    }
    #[doc = "AN024 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa24::_1
    }
}
#[doc = "Field `ANSA24` writer - AN024 Select"]
pub type Ansa24W<'a, REG> = crate::BitWriter<'a, REG, Ansa24>;
impl<'a, REG> Ansa24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN024 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa24::_0)
    }
    #[doc = "AN024 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa24::_1)
    }
}
#[doc = "AN025 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa25 {
    #[doc = "0: AN025 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN025 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansa25> for bool {
    #[inline(always)]
    fn from(variant: Ansa25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSA25` reader - AN025 Select"]
pub type Ansa25R = crate::BitReader<Ansa25>;
impl Ansa25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansa25 {
        match self.bits {
            false => Ansa25::_0,
            true => Ansa25::_1,
        }
    }
    #[doc = "AN025 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa25::_0
    }
    #[doc = "AN025 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa25::_1
    }
}
#[doc = "Field `ANSA25` writer - AN025 Select"]
pub type Ansa25W<'a, REG> = crate::BitWriter<'a, REG, Ansa25>;
impl<'a, REG> Ansa25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN025 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa25::_0)
    }
    #[doc = "AN025 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa25::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AN016 Select"]
    #[inline(always)]
    pub fn ansa16(&self) -> Ansa16R {
        Ansa16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AN017 Select"]
    #[inline(always)]
    pub fn ansa17(&self) -> Ansa17R {
        Ansa17R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AN018 Select"]
    #[inline(always)]
    pub fn ansa18(&self) -> Ansa18R {
        Ansa18R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AN019 Select"]
    #[inline(always)]
    pub fn ansa19(&self) -> Ansa19R {
        Ansa19R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AN020 Select"]
    #[inline(always)]
    pub fn ansa20(&self) -> Ansa20R {
        Ansa20R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AN021 Select"]
    #[inline(always)]
    pub fn ansa21(&self) -> Ansa21R {
        Ansa21R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AN022 Select"]
    #[inline(always)]
    pub fn ansa22(&self) -> Ansa22R {
        Ansa22R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AN023 Select"]
    #[inline(always)]
    pub fn ansa23(&self) -> Ansa23R {
        Ansa23R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AN024 Select"]
    #[inline(always)]
    pub fn ansa24(&self) -> Ansa24R {
        Ansa24R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AN025 Select"]
    #[inline(always)]
    pub fn ansa25(&self) -> Ansa25R {
        Ansa25R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADANSA1")
            .field("ansa25", &self.ansa25())
            .field("ansa24", &self.ansa24())
            .field("ansa23", &self.ansa23())
            .field("ansa22", &self.ansa22())
            .field("ansa21", &self.ansa21())
            .field("ansa20", &self.ansa20())
            .field("ansa19", &self.ansa19())
            .field("ansa18", &self.ansa18())
            .field("ansa17", &self.ansa17())
            .field("ansa16", &self.ansa16())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - AN016 Select"]
    #[inline(always)]
    pub fn ansa16(&mut self) -> Ansa16W<Adansa1Spec> {
        Ansa16W::new(self, 0)
    }
    #[doc = "Bit 1 - AN017 Select"]
    #[inline(always)]
    pub fn ansa17(&mut self) -> Ansa17W<Adansa1Spec> {
        Ansa17W::new(self, 1)
    }
    #[doc = "Bit 2 - AN018 Select"]
    #[inline(always)]
    pub fn ansa18(&mut self) -> Ansa18W<Adansa1Spec> {
        Ansa18W::new(self, 2)
    }
    #[doc = "Bit 3 - AN019 Select"]
    #[inline(always)]
    pub fn ansa19(&mut self) -> Ansa19W<Adansa1Spec> {
        Ansa19W::new(self, 3)
    }
    #[doc = "Bit 4 - AN020 Select"]
    #[inline(always)]
    pub fn ansa20(&mut self) -> Ansa20W<Adansa1Spec> {
        Ansa20W::new(self, 4)
    }
    #[doc = "Bit 5 - AN021 Select"]
    #[inline(always)]
    pub fn ansa21(&mut self) -> Ansa21W<Adansa1Spec> {
        Ansa21W::new(self, 5)
    }
    #[doc = "Bit 6 - AN022 Select"]
    #[inline(always)]
    pub fn ansa22(&mut self) -> Ansa22W<Adansa1Spec> {
        Ansa22W::new(self, 6)
    }
    #[doc = "Bit 7 - AN023 Select"]
    #[inline(always)]
    pub fn ansa23(&mut self) -> Ansa23W<Adansa1Spec> {
        Ansa23W::new(self, 7)
    }
    #[doc = "Bit 8 - AN024 Select"]
    #[inline(always)]
    pub fn ansa24(&mut self) -> Ansa24W<Adansa1Spec> {
        Ansa24W::new(self, 8)
    }
    #[doc = "Bit 9 - AN025 Select"]
    #[inline(always)]
    pub fn ansa25(&mut self) -> Ansa25W<Adansa1Spec> {
        Ansa25W::new(self, 9)
    }
}
#[doc = "A/D Channel Select Register A1\n\nYou can [`read`](crate::Reg::read) this register and get [`adansa1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansa1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adansa1Spec;
impl crate::RegisterSpec for Adansa1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adansa1::R`](R) reader structure"]
impl crate::Readable for Adansa1Spec {}
#[doc = "`write(|w| ..)` method takes [`adansa1::W`](W) writer structure"]
impl crate::Writable for Adansa1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADANSA1 to value 0"]
impl crate::Resettable for Adansa1Spec {}
