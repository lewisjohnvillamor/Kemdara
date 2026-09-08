use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct MachineMetadata {
    pub os: &'static str,
    pub architecture: &'static str,
    pub logical_cores: usize,
    pub target_features: Vec<&'static str>,
}

pub fn current_machine() -> MachineMetadata {
    MachineMetadata {
        os: std::env::consts::OS,
        architecture: std::env::consts::ARCH,
        logical_cores: std::thread::available_parallelism()
            .map(usize::from)
            .unwrap_or(1),
        target_features: detected_features(),
    }
}

fn detected_features() -> Vec<&'static str> {
    let mut features = Vec::new();

    #[cfg(target_arch = "x86_64")]
    {
        if std::arch::is_x86_feature_detected!("sse2") {
            features.push("sse2");
        }
        if std::arch::is_x86_feature_detected!("avx2") {
            features.push("avx2");
        }
        if std::arch::is_x86_feature_detected!("aes") {
            features.push("aes-ni");
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        if std::arch::is_aarch64_feature_detected!("neon") {
            features.push("neon");
        }
        if std::arch::is_aarch64_feature_detected!("aes") {
            features.push("aes");
        }
    }

    features
}
