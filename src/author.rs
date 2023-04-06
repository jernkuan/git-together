use std::fmt;

use crate::errors::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub gpg: String,
    pub ssh: String,
}

pub struct AuthorParser {
    pub domain: Option<String>,
}

impl AuthorParser {
    pub fn parse(&self, raw: &str) -> Result<Author> {
        let mut split = raw.split(';').map(str::trim);

        let name = match split.next() {
            Some(name) if !name.is_empty() => name,
            _ => {
                return Err("missing name".into());
            }
        };

        let email_seed = match split.next() {
            Some(email_seed) if !email_seed.is_empty() => email_seed,
            _ => {
                return Err("missing email seed".into());
            }
        };

        let email = if email_seed.contains('@') {
            email_seed.into()
        } else {
            let domain = match self.domain {
                Some(ref domain) => domain,
                None => {
                    return Err("missing domain".into());
                }
            };
            format!("{}@{}", email_seed, domain)
        };

        let gpg = match split.next() {
            Some(gpg) if !gpg.is_empty() => gpg,
            _ => ""
        };

        let ssh= match split.next() {
            Some(ssh) if !ssh.is_empty() => ssh,
            _ => ""
        };

        Ok(Author {
            name: name.into(),
            email,
            gpg: gpg.into(),
            ssh: ssh.into(),
        })
    }
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <{}>", self.name, self.email).unwrap();
        if self.gpg != "" {
            write!(f, " gpg:{}", self.gpg).unwrap();
        }
        if self.ssh!= "" {
            write!(f, " ssh:{}", self.ssh).unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let author_parser = AuthorParser {
            domain: Some("example.com".into()),
        };

        let author = author_parser.parse("Jane Doe; jdoe").unwrap();
        assert_eq!(author.name, "Jane Doe");
        assert_eq!(author.email, "jdoe@example.com");

        let author = author_parser.parse("");
        assert!(author.is_err());

        let author = author_parser.parse("Jane Doe");
        assert!(author.is_err());

        let author = author_parser.parse("Jane Doe; ");
        assert!(author.is_err());

        let author = author_parser
            .parse("Jane Doe; jane.doe@example.edu")
            .unwrap();
        assert_eq!(author.name, "Jane Doe");
        assert_eq!(author.email, "jane.doe@example.edu");

        let author = author_parser
            .parse("Jane Doe; jane.doe@example.edu; B87A4EFD")
            .unwrap();
        assert_eq!(author.name, "Jane Doe");
        assert_eq!(author.email, "jane.doe@example.edu");
        assert_eq!(author.gpg, "B87A4EFD");

        let author = author_parser
            .parse("Jane Doe; jane.doe@example.edu; ;")
            .unwrap();
        assert_eq!(author.name, "Jane Doe");
        assert_eq!(author.email, "jane.doe@example.edu");
        assert_eq!(author.gpg, "");

        let author = author_parser
            .parse("Jane Doe; jane.doe@example.edu; ; id_rsa")
            .unwrap();
        assert_eq!(author.name, "Jane Doe");
        assert_eq!(author.email, "jane.doe@example.edu");
        assert_eq!(author.gpg, "");
        assert_eq!(author.ssh, "id_rsa");
    }
}
