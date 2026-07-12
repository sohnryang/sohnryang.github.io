use serde::{Deserialize, Serialize};

use crate::item::{CareerItem, Link};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Section {
    pub name: String,
    pub items: Vec<CareerItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Resume {
    pub links: Vec<Link>,
    pub sections: Section,
}

#[cfg(test)]
mod tests {
    use super::{Resume, Section};
    use crate::item::{CareerItem, CareerRange, Date, Link};
    use chrono::Month;

    #[test]
    fn deserializes_full_resume() {
        let yaml = r#"
links:
  - name: GitHub
    url: https://github.com/sohnryang
  - name: Email
    url: mailto:loop.infinitely@gmail.com
sections:
  name: Experience
  items:
    - title: Software Engineer
      subtitle: Acme Corp
      description: Wrote a lot of Rust.
      range: Apr 2025 - Jan 2027
      links:
        - name: Project
          url: https://example.com
    - title: Student
      subtitle: Some University
      description: Studied things.
      range: Jul 2026 - Ongoing
      links: []
    - title: Intern
      subtitle: Another Company
      description: A summer internship.
      range: Aug 2024
      links: []
"#;

        let resume: Resume = yaml_serde::from_str(yaml).expect("resume should deserialize");

        let expected = Resume {
            links: vec![
                Link {
                    name: "GitHub".to_string(),
                    url: "https://github.com/sohnryang".to_string(),
                },
                Link {
                    name: "Email".to_string(),
                    url: "mailto:loop.infinitely@gmail.com".to_string(),
                },
            ],
            sections: Section {
                name: "Experience".to_string(),
                items: vec![
                    CareerItem {
                        title: "Software Engineer".to_string(),
                        subtitle: "Acme Corp".to_string(),
                        description: "Wrote a lot of Rust.".to_string(),
                        range: CareerRange::Range(
                            Date {
                                year: 2025,
                                month: Month::April,
                            },
                            Date {
                                year: 2027,
                                month: Month::January,
                            },
                        ),
                        links: vec![Link {
                            name: "Project".to_string(),
                            url: "https://example.com".to_string(),
                        }],
                    },
                    CareerItem {
                        title: "Student".to_string(),
                        subtitle: "Some University".to_string(),
                        description: "Studied things.".to_string(),
                        range: CareerRange::Ongoing(Date {
                            year: 2026,
                            month: Month::July,
                        }),
                        links: vec![],
                    },
                    CareerItem {
                        title: "Intern".to_string(),
                        subtitle: "Another Company".to_string(),
                        description: "A summer internship.".to_string(),
                        range: CareerRange::Single(Date {
                            year: 2024,
                            month: Month::August,
                        }),
                        links: vec![],
                    },
                ],
            },
        };

        assert_eq!(resume, expected);
    }
}
