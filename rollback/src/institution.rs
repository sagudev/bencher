use std::collections::BTreeMap;
use std::fmt;

use crate::account::{Account, AccountId};
use crate::total::Total;
use url::Url;

/// An iterable map of all institutions
pub type Institutions = BTreeMap<Institution, InstitutionAccounts>;

impl Total for Institutions {
    fn total(&self) -> u64 {
        self.iter()
            .fold(0, |acc, (_, inst_accs)| acc + inst_accs.total())
    }
}

/// An invesmtment institution
#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Institution {
    name: String,
    url: Url,
}

impl fmt::Display for Institution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Institution {
    pub fn new(name: String, url: Url) -> Self {
        Self { name, url }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn url(&self) -> &Url {
        &self.url
    }
}

/// All accounts, stored by `AccountId`, at a particular institution
pub type InstitutionAccounts = BTreeMap<AccountId, Account>;

impl Total for InstitutionAccounts {
    fn total(&self) -> u64 {
        self.iter()
            .fold(0, |acc, (_, account)| acc + account.total())
    }
}
