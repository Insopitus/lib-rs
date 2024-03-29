use std::fmt::Display;

/// Semantic Versioning
///
/// features:
/// 1. convert strings to SemVer
/// 2. compare 2 versions' using >, <, or ==
/// 3. into string
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemVer {
    // ordering is important since Ord trait relies on this
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
impl SemVer {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        SemVer {
            major,
            minor,
            patch,
        }
    }
    /// create from a string
    pub fn parse(s: &str) -> Result<SemVer, SemVerErr> {
        let mut split = s.split('.');
        let major: u32;
        let minor: u32;
        let patch: u32;
        match split.next() {
            Some(m) => match m.parse() {
                Ok(v) => {
                    major = v;
                    match split.next() {
                        Some(m) => match m.parse() {
                            Ok(v) => {
                                minor = v;
                                match split.next() {
                                    Some(m) => match m.parse() {
                                        Ok(v) => {
                                            patch = v;
                                        }
                                        Err(_) => return Err(SemVerErr::InvalidVersion),
                                    },
                                    None => {
                                        return Err(SemVerErr::InvalidVersion);
                                    }
                                }
                            }
                            Err(_) => return Err(SemVerErr::InvalidVersion),
                        },
                        None => {
                            return Err(SemVerErr::InvalidVersion);
                        }
                    }
                }
                Err(_) => return Err(SemVerErr::InvalidVersion),
            },
            None => {
                return Err(SemVerErr::InvalidVersion);
            }
        }
        Ok(SemVer::new(major, minor, patch))
    }

    /// if the target version is compatible with self version
    pub fn is_compatible_with(&self, target: &SemVer) -> bool {
        if self.major != target.major {
            false
        } else if self.major == 0 {
            self.minor == target.minor
        } else {
            true
        }
    }
    pub fn is_stable(&self) -> bool {
        self.major > 0
    }
}

impl Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Default for SemVer {
    fn default() -> Self {
        Self {
            major: 0,
            minor: 1,
            patch: 0,
        }
    }
}

// impl Ord for SemVer {
//     fn cmp(&self, other: &Self) -> Ordering {
//         match self.major.cmp(&other.major) {
//             Ordering::Greater => Ordering::Greater,
//             Ordering::Less => Ordering::Less,
//             Ordering::Equal => match self.minor.cmp(&other.minor) {
//                 Ordering::Greater => Ordering::Greater,
//                 Ordering::Less => Ordering::Less,
//                 Ordering::Equal => match self.patch.cmp(&other.patch) {
//                     Ordering::Greater => Ordering::Greater,
//                     Ordering::Less => Ordering::Less,
//                     Ordering::Equal => Ordering::Equal,
//                 },
//             },
//         }
//     }
// }

#[derive(Debug)]
pub enum SemVerErr {
    InvalidVersion,
}

#[cfg(test)]
mod test {
    use super::SemVer;

    #[test]
    fn new() {
        assert_eq!(
            SemVer::new(1, 0, 0),
            SemVer {
                major: 1,
                minor: 0,
                patch: 0
            }
        );
    }
    #[test]
    fn from_string() {
        assert_eq!(SemVer::parse("2.1.2").unwrap(), SemVer::new(2, 1, 2));
    }
    #[test]
    fn compatible() {
        assert!(SemVer::parse("2.1.2")
            .unwrap()
            .is_compatible_with(&SemVer::new(2, 8, 145)));
        assert_eq!(
            SemVer::parse("0.1.5")
                .unwrap()
                .is_compatible_with(&SemVer::parse("0.3.0").unwrap()),
            false
        );
    }

    #[test]
    fn compare() {
        assert!(SemVer::new(0, 1, 2) > SemVer::new(0, 0, 1));
        assert!(SemVer::new(0, 1, 2) == SemVer::new(0, 1, 2));
        assert!(SemVer::new(0, 0, 2) > SemVer::new(0, 0, 1));
        assert!(SemVer::new(0, 1, 2) > SemVer::new(0, 0, 1));
        assert!(SemVer::new(1, 1, 2) > SemVer::new(0, 0, 1));
        assert!(SemVer::new(2, 0, 2) > SemVer::new(0, 1, 1));
    }
    #[test]
    fn to_string() {
        assert!(&SemVer::new(7, 14, 59).to_string() == "7.14.59");
    }
    #[test]
    fn sort() {
        let mut arr = [
            SemVer::new(0, 10, 0),
            SemVer::new(0, 1, 0),
            SemVer::new(1, 0, 0),
        ];
        arr.sort();
        assert!(
            arr == [
                SemVer::new(0, 1, 0),
                SemVer::new(0, 10, 0),
                SemVer::new(1, 0, 0)
            ]
        )
    }
}
