#[doc = "Register `ADANSB1` reader"]
pub type R = crate::R<Adansb1Spec>;
#[doc = "Register `ADANSB1` writer"]
pub type W = crate::W<Adansb1Spec>;
#[doc = "AN016 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb16 {
    #[doc = "0: AN016 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN016 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb16> for bool {
    #[inline(always)]
    fn from(variant: Ansb16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB16` reader - AN016 Select"]
pub type Ansb16R = crate::BitReader<Ansb16>;
impl Ansb16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb16 {
        match self.bits {
            false => Ansb16::_0,
            true => Ansb16::_1,
        }
    }
    #[doc = "AN016 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb16::_0
    }
    #[doc = "AN016 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb16::_1
    }
}
#[doc = "Field `ANSB16` writer - AN016 Select"]
pub type Ansb16W<'a, REG> = crate::BitWriter<'a, REG, Ansb16>;
impl<'a, REG> Ansb16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN016 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb16::_0)
    }
    #[doc = "AN016 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb16::_1)
    }
}
#[doc = "AN017 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb17 {
    #[doc = "0: AN017 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN017 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb17> for bool {
    #[inline(always)]
    fn from(variant: Ansb17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB17` reader - AN017 Select"]
pub type Ansb17R = crate::BitReader<Ansb17>;
impl Ansb17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb17 {
        match self.bits {
            false => Ansb17::_0,
            true => Ansb17::_1,
        }
    }
    #[doc = "AN017 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb17::_0
    }
    #[doc = "AN017 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb17::_1
    }
}
#[doc = "Field `ANSB17` writer - AN017 Select"]
pub type Ansb17W<'a, REG> = crate::BitWriter<'a, REG, Ansb17>;
impl<'a, REG> Ansb17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN017 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb17::_0)
    }
    #[doc = "AN017 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb17::_1)
    }
}
#[doc = "AN018 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb18 {
    #[doc = "0: AN018 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN018 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb18> for bool {
    #[inline(always)]
    fn from(variant: Ansb18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB18` reader - AN018 Select"]
pub type Ansb18R = crate::BitReader<Ansb18>;
impl Ansb18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb18 {
        match self.bits {
            false => Ansb18::_0,
            true => Ansb18::_1,
        }
    }
    #[doc = "AN018 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb18::_0
    }
    #[doc = "AN018 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb18::_1
    }
}
#[doc = "Field `ANSB18` writer - AN018 Select"]
pub type Ansb18W<'a, REG> = crate::BitWriter<'a, REG, Ansb18>;
impl<'a, REG> Ansb18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN018 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb18::_0)
    }
    #[doc = "AN018 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb18::_1)
    }
}
#[doc = "AN019 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb19 {
    #[doc = "0: AN019 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN019 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb19> for bool {
    #[inline(always)]
    fn from(variant: Ansb19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB19` reader - AN019 Select"]
pub type Ansb19R = crate::BitReader<Ansb19>;
impl Ansb19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb19 {
        match self.bits {
            false => Ansb19::_0,
            true => Ansb19::_1,
        }
    }
    #[doc = "AN019 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb19::_0
    }
    #[doc = "AN019 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb19::_1
    }
}
#[doc = "Field `ANSB19` writer - AN019 Select"]
pub type Ansb19W<'a, REG> = crate::BitWriter<'a, REG, Ansb19>;
impl<'a, REG> Ansb19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN019 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb19::_0)
    }
    #[doc = "AN019 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb19::_1)
    }
}
#[doc = "AN020 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb20 {
    #[doc = "0: AN020 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN020 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb20> for bool {
    #[inline(always)]
    fn from(variant: Ansb20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB20` reader - AN020 Select"]
pub type Ansb20R = crate::BitReader<Ansb20>;
impl Ansb20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb20 {
        match self.bits {
            false => Ansb20::_0,
            true => Ansb20::_1,
        }
    }
    #[doc = "AN020 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb20::_0
    }
    #[doc = "AN020 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb20::_1
    }
}
#[doc = "Field `ANSB20` writer - AN020 Select"]
pub type Ansb20W<'a, REG> = crate::BitWriter<'a, REG, Ansb20>;
impl<'a, REG> Ansb20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN020 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb20::_0)
    }
    #[doc = "AN020 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb20::_1)
    }
}
#[doc = "AN021 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb21 {
    #[doc = "0: AN021 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN021 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb21> for bool {
    #[inline(always)]
    fn from(variant: Ansb21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB21` reader - AN021 Select"]
pub type Ansb21R = crate::BitReader<Ansb21>;
impl Ansb21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb21 {
        match self.bits {
            false => Ansb21::_0,
            true => Ansb21::_1,
        }
    }
    #[doc = "AN021 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb21::_0
    }
    #[doc = "AN021 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb21::_1
    }
}
#[doc = "Field `ANSB21` writer - AN021 Select"]
pub type Ansb21W<'a, REG> = crate::BitWriter<'a, REG, Ansb21>;
impl<'a, REG> Ansb21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN021 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb21::_0)
    }
    #[doc = "AN021 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb21::_1)
    }
}
#[doc = "AN022 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb22 {
    #[doc = "0: AN022 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN022 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb22> for bool {
    #[inline(always)]
    fn from(variant: Ansb22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB22` reader - AN022 Select"]
pub type Ansb22R = crate::BitReader<Ansb22>;
impl Ansb22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb22 {
        match self.bits {
            false => Ansb22::_0,
            true => Ansb22::_1,
        }
    }
    #[doc = "AN022 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb22::_0
    }
    #[doc = "AN022 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb22::_1
    }
}
#[doc = "Field `ANSB22` writer - AN022 Select"]
pub type Ansb22W<'a, REG> = crate::BitWriter<'a, REG, Ansb22>;
impl<'a, REG> Ansb22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN022 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb22::_0)
    }
    #[doc = "AN022 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb22::_1)
    }
}
#[doc = "AN023 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb23 {
    #[doc = "0: AN023 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN023 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb23> for bool {
    #[inline(always)]
    fn from(variant: Ansb23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB23` reader - AN023 Select"]
pub type Ansb23R = crate::BitReader<Ansb23>;
impl Ansb23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb23 {
        match self.bits {
            false => Ansb23::_0,
            true => Ansb23::_1,
        }
    }
    #[doc = "AN023 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb23::_0
    }
    #[doc = "AN023 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb23::_1
    }
}
#[doc = "Field `ANSB23` writer - AN023 Select"]
pub type Ansb23W<'a, REG> = crate::BitWriter<'a, REG, Ansb23>;
impl<'a, REG> Ansb23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN023 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb23::_0)
    }
    #[doc = "AN023 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb23::_1)
    }
}
#[doc = "AN024 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb24 {
    #[doc = "0: AN024 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN024 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb24> for bool {
    #[inline(always)]
    fn from(variant: Ansb24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB24` reader - AN024 Select"]
pub type Ansb24R = crate::BitReader<Ansb24>;
impl Ansb24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb24 {
        match self.bits {
            false => Ansb24::_0,
            true => Ansb24::_1,
        }
    }
    #[doc = "AN024 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb24::_0
    }
    #[doc = "AN024 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb24::_1
    }
}
#[doc = "Field `ANSB24` writer - AN024 Select"]
pub type Ansb24W<'a, REG> = crate::BitWriter<'a, REG, Ansb24>;
impl<'a, REG> Ansb24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN024 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb24::_0)
    }
    #[doc = "AN024 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb24::_1)
    }
}
#[doc = "AN025 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb25 {
    #[doc = "0: AN025 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN025 is subjected to conversion."]
    _1 = 1,
}
impl From<Ansb25> for bool {
    #[inline(always)]
    fn from(variant: Ansb25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANSB25` reader - AN025 Select"]
pub type Ansb25R = crate::BitReader<Ansb25>;
impl Ansb25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ansb25 {
        match self.bits {
            false => Ansb25::_0,
            true => Ansb25::_1,
        }
    }
    #[doc = "AN025 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb25::_0
    }
    #[doc = "AN025 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb25::_1
    }
}
#[doc = "Field `ANSB25` writer - AN025 Select"]
pub type Ansb25W<'a, REG> = crate::BitWriter<'a, REG, Ansb25>;
impl<'a, REG> Ansb25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN025 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb25::_0)
    }
    #[doc = "AN025 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb25::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AN016 Select"]
    #[inline(always)]
    pub fn ansb16(&self) -> Ansb16R {
        Ansb16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AN017 Select"]
    #[inline(always)]
    pub fn ansb17(&self) -> Ansb17R {
        Ansb17R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AN018 Select"]
    #[inline(always)]
    pub fn ansb18(&self) -> Ansb18R {
        Ansb18R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AN019 Select"]
    #[inline(always)]
    pub fn ansb19(&self) -> Ansb19R {
        Ansb19R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AN020 Select"]
    #[inline(always)]
    pub fn ansb20(&self) -> Ansb20R {
        Ansb20R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AN021 Select"]
    #[inline(always)]
    pub fn ansb21(&self) -> Ansb21R {
        Ansb21R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AN022 Select"]
    #[inline(always)]
    pub fn ansb22(&self) -> Ansb22R {
        Ansb22R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AN023 Select"]
    #[inline(always)]
    pub fn ansb23(&self) -> Ansb23R {
        Ansb23R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AN024 Select"]
    #[inline(always)]
    pub fn ansb24(&self) -> Ansb24R {
        Ansb24R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AN025 Select"]
    #[inline(always)]
    pub fn ansb25(&self) -> Ansb25R {
        Ansb25R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADANSB1")
            .field("ansb25", &self.ansb25())
            .field("ansb24", &self.ansb24())
            .field("ansb23", &self.ansb23())
            .field("ansb22", &self.ansb22())
            .field("ansb21", &self.ansb21())
            .field("ansb20", &self.ansb20())
            .field("ansb19", &self.ansb19())
            .field("ansb18", &self.ansb18())
            .field("ansb17", &self.ansb17())
            .field("ansb16", &self.ansb16())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - AN016 Select"]
    #[inline(always)]
    pub fn ansb16(&mut self) -> Ansb16W<Adansb1Spec> {
        Ansb16W::new(self, 0)
    }
    #[doc = "Bit 1 - AN017 Select"]
    #[inline(always)]
    pub fn ansb17(&mut self) -> Ansb17W<Adansb1Spec> {
        Ansb17W::new(self, 1)
    }
    #[doc = "Bit 2 - AN018 Select"]
    #[inline(always)]
    pub fn ansb18(&mut self) -> Ansb18W<Adansb1Spec> {
        Ansb18W::new(self, 2)
    }
    #[doc = "Bit 3 - AN019 Select"]
    #[inline(always)]
    pub fn ansb19(&mut self) -> Ansb19W<Adansb1Spec> {
        Ansb19W::new(self, 3)
    }
    #[doc = "Bit 4 - AN020 Select"]
    #[inline(always)]
    pub fn ansb20(&mut self) -> Ansb20W<Adansb1Spec> {
        Ansb20W::new(self, 4)
    }
    #[doc = "Bit 5 - AN021 Select"]
    #[inline(always)]
    pub fn ansb21(&mut self) -> Ansb21W<Adansb1Spec> {
        Ansb21W::new(self, 5)
    }
    #[doc = "Bit 6 - AN022 Select"]
    #[inline(always)]
    pub fn ansb22(&mut self) -> Ansb22W<Adansb1Spec> {
        Ansb22W::new(self, 6)
    }
    #[doc = "Bit 7 - AN023 Select"]
    #[inline(always)]
    pub fn ansb23(&mut self) -> Ansb23W<Adansb1Spec> {
        Ansb23W::new(self, 7)
    }
    #[doc = "Bit 8 - AN024 Select"]
    #[inline(always)]
    pub fn ansb24(&mut self) -> Ansb24W<Adansb1Spec> {
        Ansb24W::new(self, 8)
    }
    #[doc = "Bit 9 - AN025 Select"]
    #[inline(always)]
    pub fn ansb25(&mut self) -> Ansb25W<Adansb1Spec> {
        Ansb25W::new(self, 9)
    }
}
#[doc = "A/D Channel Select Register B1\n\nYou can [`read`](crate::Reg::read) this register and get [`adansb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adansb1Spec;
impl crate::RegisterSpec for Adansb1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adansb1::R`](R) reader structure"]
impl crate::Readable for Adansb1Spec {}
#[doc = "`write(|w| ..)` method takes [`adansb1::W`](W) writer structure"]
impl crate::Writable for Adansb1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADANSB1 to value 0"]
impl crate::Resettable for Adansb1Spec {}
