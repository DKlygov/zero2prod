use validator::validate_email;

#[derive(Debug, PartialEq)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if validate_email(&s) {
            return Ok(Self(s));
        }

        Err(format!("{} is not a valid subscriber email.", s))
    }
}
impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use fake::faker::internet::en::SafeEmail;
    use rand::{rngs::StdRng, SeedableRng};

    use fake::Fake;
    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            // API quickcheck поломали в версии 1.0.0
            // поменяв трейт на структуру
            // теперь надо импортировать генератор и создать генератор
            // вручную, используя сторонний пакет, например rand
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let email = SafeEmail().fake_with_rng(&mut rng);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_ara_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        SubscriberEmail::parse(valid_email.0).is_ok()
    }

    #[test]
    fn empty_string_is_rejected() {
        let email = SafeEmail().fake();
        assert_ne!(
            SubscriberEmail::parse(email),
            Ok(SubscriberEmail("".to_string()))
        );
    }
}
