use std::borrow::Cow;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use ref_cast::RefCast;
use tracing::debug;
use uv_fs::Simplified;

use uv_cache::Cache;
use uv_cache_key::{CacheKey, CacheKeyHasher};
use uv_pep440::{Prerelease, Version};
use uv_platform::{Arch, Libc, Os, Platform};

use crate::discovery::{
    EnvironmentPreference, PythonRequest, VersionRequest, find_python_installation,
};
use crate::implementation::LenientImplementationName;
use crate::{
    Error, ImplementationName, Interpreter, PythonPreference, PythonSource, PythonVariant,
    PythonVersion,
};

/// A Python interpreter and accompanying tools.
#[derive(Clone, Debug)]
pub struct PythonInstallation {
    // Public in the crate for test assertions
    pub(crate) source: PythonSource,
    pub(crate) interpreter: Interpreter,
}

impl PythonInstallation {
    /// Create a new [`PythonInstallation`] from a source and interpreter.
    pub fn new(source: PythonSource, interpreter: Interpreter) -> Self {
        Self {
            source,
            interpreter,
        }
    }

    /// Check whether this installation satisfies the standard post-query discovery filters:
    /// environment preference, version request, and Python preference.
    pub(crate) fn satisfies_preferences(
        &self,
        version: &VersionRequest,
        environments: EnvironmentPreference,
        preference: PythonPreference,
    ) -> bool {
        if !environments.allows_installation(self) {
            return false;
        }
        if !version.matches_installation(self) {
            debug!(
                "Skipping interpreter at `{}` from {}: does not satisfy request `{version}`",
                self.interpreter.sys_executable().user_display(),
                self.source,
            );
            return false;
        }
        if !preference.allows_installation(self) {
            return false;
        }
        true
    }

    /// Find an installed [`PythonInstallation`].
    ///
    /// This is the standard interface for discovering a Python installation for creating
    /// an environment. If interested in finding an existing environment, see
    /// [`PythonEnvironment::find`] instead.
    ///
    /// Note we still require an [`EnvironmentPreference`] as this can either bypass virtual environments
    /// or prefer them. In most cases, this should be [`EnvironmentPreference::OnlySystem`]
    /// but if you want to allow an interpreter from a virtual environment if it satisfies the request,
    /// then use [`EnvironmentPreference::Any`].
    ///
    /// See [`find_installation`] for implementation details.
    pub fn find(
        request: &PythonRequest,
        environments: EnvironmentPreference,
        preference: PythonPreference,
        cache: &Cache,
    ) -> Result<Self, Error> {
        Self::find_existing(request, environments, preference, cache)
    }

    /// Find an existing [`PythonInstallation`].
    pub fn find_existing(
        request: &PythonRequest,
        environments: EnvironmentPreference,
        preference: PythonPreference,
        cache: &Cache,
    ) -> Result<Self, Error> {
        Ok(find_python_installation(
            request,
            environments,
            preference,
            cache,
        )??)
    }

    /// Return the [`PythonSource`] of the Python installation, indicating where it was found.
    pub fn source(&self) -> &PythonSource {
        &self.source
    }

    pub fn key(&self) -> PythonInstallationKey {
        self.interpreter.key()
    }

    /// Return the Python [`Version`] of the Python installation as reported by its interpreter.
    pub fn python_version(&self) -> &Version {
        self.interpreter.python_version()
    }

    /// Return the [`LenientImplementationName`] of the Python installation as reported by its interpreter.
    pub fn implementation(&self) -> LenientImplementationName {
        LenientImplementationName::from(self.interpreter.implementation_name())
    }

    /// Whether this is a CPython installation.
    ///
    /// Returns false if it is an alternative implementation, e.g., PyPy.
    pub(crate) fn is_alternative_implementation(&self) -> bool {
        !matches!(
            self.implementation(),
            LenientImplementationName::Known(ImplementationName::CPython)
        ) || self.os().is_emscripten()
    }

    /// Return the [`Arch`] of the Python installation as reported by its interpreter.
    pub fn arch(&self) -> Arch {
        self.interpreter.arch()
    }

    /// Return the [`Libc`] of the Python installation as reported by its interpreter.
    pub fn libc(&self) -> Libc {
        self.interpreter.libc()
    }

    /// Return the [`Os`] of the Python installation as reported by its interpreter.
    pub fn os(&self) -> Os {
        self.interpreter.os()
    }

    /// Return the [`Interpreter`] for the Python installation.
    pub fn interpreter(&self) -> &Interpreter {
        &self.interpreter
    }

    /// Consume the [`PythonInstallation`] and return the [`Interpreter`].
    pub fn into_interpreter(self) -> Interpreter {
        self.interpreter
    }
}

#[derive(Error, Debug)]
pub enum PythonInstallationKeyError {
    #[error("Failed to parse Python installation key `{0}`: {1}")]
    ParseError(String, String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PythonInstallationKey {
    pub(super) implementation: LenientImplementationName,
    pub(super) major: u8,
    pub(super) minor: u8,
    pub(super) patch: u8,
    pub(super) prerelease: Option<Prerelease>,
    pub(super) platform: Platform,
    pub(super) variant: PythonVariant,
}

impl PythonInstallationKey {
    pub(crate) fn new(
        implementation: LenientImplementationName,
        major: u8,
        minor: u8,
        patch: u8,
        prerelease: Option<Prerelease>,
        platform: Platform,
        variant: PythonVariant,
    ) -> Self {
        Self {
            implementation,
            major,
            minor,
            patch,
            prerelease,
            platform,
            variant,
        }
    }

    pub fn implementation(&self) -> Cow<'_, LenientImplementationName> {
        if self.os().is_emscripten() {
            Cow::Owned(LenientImplementationName::from(ImplementationName::Pyodide))
        } else {
            Cow::Borrowed(&self.implementation)
        }
    }

    pub fn version(&self) -> PythonVersion {
        PythonVersion::from_str(&format!(
            "{}.{}.{}{}",
            self.major,
            self.minor,
            self.patch,
            self.prerelease
                .map(|pre| pre.to_string())
                .unwrap_or_default()
        ))
        .expect("Python installation keys must have valid Python versions")
    }

    pub fn major(&self) -> u8 {
        self.major
    }

    pub fn minor(&self) -> u8 {
        self.minor
    }

    pub fn arch(&self) -> &Arch {
        &self.platform.arch
    }

    pub fn os(&self) -> &Os {
        &self.platform.os
    }

    pub fn libc(&self) -> &Libc {
        &self.platform.libc
    }

    pub fn variant(&self) -> &PythonVariant {
        &self.variant
    }

    /// Return a canonical name for a minor versioned executable.
    pub fn executable_name_minor(&self) -> String {
        format!(
            "{name}{maj}.{min}{var}{exe}",
            name = self.implementation().executable_install_name(),
            maj = self.major,
            min = self.minor,
            var = self.variant.executable_suffix(),
            exe = std::env::consts::EXE_SUFFIX
        )
    }

    /// Return a canonical name for a major versioned executable.
    pub fn executable_name_major(&self) -> String {
        format!(
            "{name}{maj}{var}{exe}",
            name = self.implementation().executable_install_name(),
            maj = self.major,
            var = self.variant.executable_suffix(),
            exe = std::env::consts::EXE_SUFFIX
        )
    }

    /// Return a canonical name for an un-versioned executable.
    pub fn executable_name(&self) -> String {
        format!(
            "{name}{var}{exe}",
            name = self.implementation().executable_install_name(),
            var = self.variant.executable_suffix(),
            exe = std::env::consts::EXE_SUFFIX
        )
    }
}

impl fmt::Display for PythonInstallationKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant = match self.variant {
            PythonVariant::Default => String::new(),
            _ => format!("+{}", self.variant),
        };
        write!(
            f,
            "{}-{}.{}.{}{}{}-{}",
            self.implementation(),
            self.major,
            self.minor,
            self.patch,
            self.prerelease
                .map(|pre| pre.to_string())
                .unwrap_or_default(),
            variant,
            self.platform
        )
    }
}

impl CacheKey for PythonInstallationKey {
    fn cache_key(&self, state: &mut CacheKeyHasher) {
        self.hash(state);
    }
}

impl FromStr for PythonInstallationKey {
    type Err = PythonInstallationKeyError;

    fn from_str(key: &str) -> Result<Self, Self::Err> {
        let parts = key.split('-').collect::<Vec<_>>();

        // We need exactly implementation-version-os-arch-libc
        if parts.len() != 5 {
            return Err(PythonInstallationKeyError::ParseError(
                key.to_string(),
                format!(
                    "expected exactly 5 `-`-separated values, got {}",
                    parts.len()
                ),
            ));
        }

        let [implementation_str, version_str, os, arch, libc] = parts.as_slice() else {
            unreachable!()
        };

        let implementation = LenientImplementationName::from(*implementation_str);

        let (version, variant) = match version_str.split_once('+') {
            Some((version, variant)) => {
                let variant = PythonVariant::from_str(variant).map_err(|()| {
                    PythonInstallationKeyError::ParseError(
                        key.to_string(),
                        format!("invalid Python variant: {variant}"),
                    )
                })?;
                (version, variant)
            }
            None => (*version_str, PythonVariant::Default),
        };

        let version = PythonVersion::from_str(version).map_err(|err| {
            PythonInstallationKeyError::ParseError(
                key.to_string(),
                format!("invalid Python version: {err}"),
            )
        })?;

        let platform = Platform::from_parts(os, arch, libc).map_err(|err| {
            PythonInstallationKeyError::ParseError(
                key.to_string(),
                format!("invalid platform: {err}"),
            )
        })?;

        Ok(Self {
            implementation,
            major: version.major(),
            minor: version.minor(),
            patch: version.patch().unwrap_or_default(),
            prerelease: version.pre(),
            platform,
            variant,
        })
    }
}

impl PartialOrd for PythonInstallationKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PythonInstallationKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.implementation
            .cmp(&other.implementation)
            .then_with(|| self.version().cmp(&other.version()))
            // Platforms are sorted in preferred order for the target
            .then_with(|| self.platform.cmp(&other.platform).reverse())
            // Python variants are sorted in preferred order, with `Default` first
            .then_with(|| self.variant.cmp(&other.variant).reverse())
    }
}

/// A view into a [`PythonInstallationKey`] that excludes the patch and prerelease versions.
#[derive(Clone, Eq, Ord, PartialOrd, RefCast)]
#[repr(transparent)]
pub struct PythonInstallationMinorVersionKey(PythonInstallationKey);

impl PythonInstallationMinorVersionKey {
    /// Cast a `&PythonInstallationKey` to a `&PythonInstallationMinorVersionKey` using ref-cast.
    #[inline]
    pub fn ref_cast(key: &PythonInstallationKey) -> &Self {
        RefCast::ref_cast(key)
    }
}

impl fmt::Display for PythonInstallationMinorVersionKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display every field on the wrapped key except the patch
        // and prerelease (with special formatting for the variant).
        let variant = match self.0.variant {
            PythonVariant::Default => String::new(),
            _ => format!("+{}", self.0.variant),
        };
        write!(
            f,
            "{}-{}.{}{}-{}",
            self.0.implementation, self.0.major, self.0.minor, variant, self.0.platform,
        )
    }
}

impl fmt::Debug for PythonInstallationMinorVersionKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display every field on the wrapped key except the patch
        // and prerelease.
        f.debug_struct("PythonInstallationMinorVersionKey")
            .field("implementation", &self.0.implementation)
            .field("major", &self.0.major)
            .field("minor", &self.0.minor)
            .field("variant", &self.0.variant)
            .field("os", &self.0.platform.os)
            .field("arch", &self.0.platform.arch)
            .field("libc", &self.0.platform.libc)
            .finish()
    }
}

impl PartialEq for PythonInstallationMinorVersionKey {
    fn eq(&self, other: &Self) -> bool {
        // Compare every field on the wrapped key except the patch
        // and prerelease.
        self.0.implementation == other.0.implementation
            && self.0.major == other.0.major
            && self.0.minor == other.0.minor
            && self.0.platform == other.0.platform
            && self.0.variant == other.0.variant
    }
}

impl Hash for PythonInstallationMinorVersionKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash every field on the wrapped key except the patch
        // and prerelease.
        self.0.implementation.hash(state);
        self.0.major.hash(state);
        self.0.minor.hash(state);
        self.0.platform.hash(state);
        self.0.variant.hash(state);
    }
}

impl CacheKey for PythonInstallationMinorVersionKey {
    fn cache_key(&self, state: &mut CacheKeyHasher) {
        self.hash(state);
    }
}

impl From<PythonInstallationKey> for PythonInstallationMinorVersionKey {
    fn from(key: PythonInstallationKey) -> Self {
        Self(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uv_platform::ArchVariant;

    #[test]
    fn test_python_installation_key_from_str() {
        // Test basic parsing
        let key = PythonInstallationKey::from_str("cpython-3.12.0-linux-x86_64-gnu").unwrap();
        assert_eq!(
            key.implementation,
            LenientImplementationName::Known(ImplementationName::CPython)
        );
        assert_eq!(key.major, 3);
        assert_eq!(key.minor, 12);
        assert_eq!(key.patch, 0);
        assert_eq!(
            key.platform.os,
            Os::new(target_lexicon::OperatingSystem::Linux)
        );
        assert_eq!(
            key.platform.arch,
            Arch::new(target_lexicon::Architecture::X86_64, None)
        );
        assert_eq!(
            key.platform.libc,
            Libc::Some(target_lexicon::Environment::Gnu)
        );

        // Test with architecture variant
        let key = PythonInstallationKey::from_str("cpython-3.11.2-linux-x86_64_v3-musl").unwrap();
        assert_eq!(
            key.implementation,
            LenientImplementationName::Known(ImplementationName::CPython)
        );
        assert_eq!(key.major, 3);
        assert_eq!(key.minor, 11);
        assert_eq!(key.patch, 2);
        assert_eq!(
            key.platform.os,
            Os::new(target_lexicon::OperatingSystem::Linux)
        );
        assert_eq!(
            key.platform.arch,
            Arch::new(target_lexicon::Architecture::X86_64, Some(ArchVariant::V3))
        );
        assert_eq!(
            key.platform.libc,
            Libc::Some(target_lexicon::Environment::Musl)
        );

        // Test with Python variant (freethreaded)
        let key = PythonInstallationKey::from_str("cpython-3.13.0+freethreaded-macos-aarch64-none")
            .unwrap();
        assert_eq!(
            key.implementation,
            LenientImplementationName::Known(ImplementationName::CPython)
        );
        assert_eq!(key.major, 3);
        assert_eq!(key.minor, 13);
        assert_eq!(key.patch, 0);
        assert_eq!(key.variant, PythonVariant::Freethreaded);
        assert_eq!(
            key.platform.os,
            Os::new(target_lexicon::OperatingSystem::Darwin(None))
        );
        assert_eq!(
            key.platform.arch,
            Arch::new(
                target_lexicon::Architecture::Aarch64(target_lexicon::Aarch64Architecture::Aarch64),
                None
            )
        );
        assert_eq!(key.platform.libc, Libc::None);

        // Test error cases
        assert!(PythonInstallationKey::from_str("cpython-3.12.0-linux-x86_64").is_err());
        assert!(PythonInstallationKey::from_str("cpython-3.12.0").is_err());
        assert!(PythonInstallationKey::from_str("cpython").is_err());
    }

    #[test]
    fn test_python_installation_key_display() {
        let key = PythonInstallationKey {
            implementation: LenientImplementationName::from("cpython"),
            major: 3,
            minor: 12,
            patch: 0,
            prerelease: None,
            platform: Platform::from_str("linux-x86_64-gnu").unwrap(),
            variant: PythonVariant::Default,
        };
        assert_eq!(key.to_string(), "cpython-3.12.0-linux-x86_64-gnu");

        let key_with_variant = PythonInstallationKey {
            implementation: LenientImplementationName::from("cpython"),
            major: 3,
            minor: 13,
            patch: 0,
            prerelease: None,
            platform: Platform::from_str("macos-aarch64-none").unwrap(),
            variant: PythonVariant::Freethreaded,
        };
        assert_eq!(
            key_with_variant.to_string(),
            "cpython-3.13.0+freethreaded-macos-aarch64-none"
        );
    }
}
