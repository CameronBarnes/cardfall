use std::{convert::Infallible, str::FromStr, sync::LazyLock};

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use string_interner::{symbol::SymbolU32, DefaultStringInterner};

static INTERNER: LazyLock<Mutex<DefaultStringInterner>> =
    LazyLock::new(|| Mutex::new(DefaultStringInterner::new()));

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(from = "String")]
#[serde(into = "String")]
pub struct SharedStr {
    /// The Symbol to refer to the source String in the String Cache
    symbol: SymbolU32,
}

impl SharedStr {
    /// The ``SymbolU32`` to refer to the source ``String`` in the ``DefaultStringInterner`` Cache
    #[must_use] pub const fn symbol(self) -> SymbolU32 {
        self.symbol
    }

    /// Returns the ``String`` stored in the Cache for this `SharedStr` instance
    ///
    /// # Panics
    /// Should never panic. The panic occurs if it attempts to resolve the symbol and fails, but
    /// that shouldnt ever happen because the symbol can only be created by an interner in the
    /// static context.
    #[must_use] pub fn string(self) -> String {
        INTERNER
            .lock()
            .resolve(self.symbol)
            .map(std::string::ToString::to_string)
            .expect("Resolving should never fail, INTERNER is static and SharedStr can only be created after interning the String")
    }
}

impl From<SharedStr> for String {
    fn from(val: SharedStr) -> Self {
        val.string()
    }
}

impl From<String> for SharedStr {
    fn from(value: String) -> Self {
        Self::from_str(&value).expect("Infallible")
    }
}

impl FromStr for SharedStr {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            symbol: INTERNER.lock().get_or_intern(s),
        })
    }
}
