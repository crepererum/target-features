//! # Target features
//! Provides a database of target features available to the Rust compiler.
//!
#![doc = include_str!(concat!(env!("OUT_DIR"), "/features.md"))]

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

/// A target architecture.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Architecture {
    /// Arm
    Arm,
    /// AArch64
    AArch64,
    /// BPF
    Bpf,
    /// Hexagon
    Hexagon,
    /// MIPS,
    Mips,
    /// PowerPC
    PowerPC,
    /// RISC-V
    RiscV,
    /// WASM
    Wasm,
    /// x86 and x86-64
    X86,
    /// Another target, which doesn't have features
    Unsupported,
}

/// Returned by [`Feature::new`] when the requested feature can't be found.
pub struct UnknownFeature;

/// A target feature.
#[derive(PartialEq, Eq)]
pub struct Feature(usize);

impl Feature {
    /// Look up a feature.
    pub const fn new(architecture: Architecture, feature: &str) -> Result<Self, UnknownFeature> {
        const fn str_eq(a: &str, b: &str) -> bool {
            let a = a.as_bytes();
            let b = b.as_bytes();

            if a.len() != b.len() {
                return false;
            }

            let mut i = 0;
            while i < a.len() {
                if a[i] != b[i] {
                    return false;
                }
                i += 1;
            }
            true
        }

        let mut i = 0;
        while i < FEATURES.len() {
            if (architecture as u8) == (FEATURES[i].0 as u8) && str_eq(feature, FEATURES[i].1) {
                return Ok(Self(i));
            }
            i += 1;
        }

        Err(UnknownFeature)
    }

    /// Get the name of the feature.
    pub const fn name(&self) -> &'static str {
        FEATURES[self.0].1
    }

    /// Get the architecture this feature is for.
    pub const fn architecture(&self) -> Architecture {
        FEATURES[self.0].0
    }

    /// Get a human-readable description of the feature.
    pub const fn description(&self) -> &'static str {
        FEATURES[self.0].2
    }

    /// Return all features which are implied by the existence of this feature.
    ///
    /// For example, "avx2" implies the existence of "avx" on x86 architectures.
    pub const fn implies(&self) -> &'static [Feature] {
        FEATURES[self.0].3
    }
}

/// A target architecture with optional features.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Target {
    architecture: Architecture,
    features: [bool; FEATURES.len()],
}

impl Target {
    /// Create a target with no specified features.
    pub const fn new(architecture: Architecture) -> Self {
        Self {
            architecture,
            features: [false; FEATURES.len()],
        }
    }

    /// Returns the target architecture.
    pub const fn architecture(&self) -> Architecture {
        self.architecture
    }

    /// Returns whether the target supports the specified feature.
    pub const fn supports_feature(&self, feature: Feature) -> bool {
        // First check the target specifically
        if self.features[feature.0] {
            return true;
        }

        // Next check implied features
        let mut i = 0;
        while i < self.features.len() {
            if self.features[i] {
                let implies = Feature(i).implies();
                let mut j = 0;
                while j < implies.len() {
                    if feature.0 == implies[0].0 {
                        return true;
                    }
                    j += 1;
                }
            }
            i += 1;
        }

        // The feature is neither specified or implied
        false
    }

    /// Returns whether the target supports the specified feature.
    ///
    /// # Panics
    /// Panics if the feature doesn't belong to the target architecture.
    pub const fn supports_feature_str(&self, feature: &str) -> bool {
        if let Ok(feature) = Feature::new(self.architecture, feature) {
            self.supports_feature(feature)
        } else {
            panic!("unknown feature");
        }
    }

    /// Add a feature to the target.
    ///
    /// # Panics
    /// Panics if the feature doesn't belong to the target architecture.
    pub const fn with_feature(mut self, feature: Feature) -> Self {
        assert!(feature.architecture() as u8 == self.architecture as u8);
        self.features[feature.0] = true;
        self
    }

    /// Add a feature to the target.
    ///
    /// # Panics
    /// Panics if the requested feature name doesn't exist for the target architecture.
    pub const fn with_feature_str(self, feature: &str) -> Self {
        if let Ok(feature) = Feature::new(self.architecture, feature) {
            self.with_feature(feature)
        } else {
            panic!("unknown feature");
        }
    }

    /// Remove a feature from the target.
    ///
    /// # Panics
    /// Panics if the feature doesn't belong to the target architecture.
    pub const fn without_feature(mut self, feature: Feature) -> Self {
        assert!(feature.architecture() as u8 == self.architecture as u8);
        self.features[feature.0] = false;
        self
    }

    /// Remove a feature from the target.
    ///
    /// # Panics
    /// Panics if the requested feature name doesn't exist for the target architecture.
    pub const fn without_feature_str(self, feature: &str) -> Self {
        if let Ok(feature) = Feature::new(self.architecture, feature) {
            self.without_feature(feature)
        } else {
            panic!("unknown feature");
        }
    }
}