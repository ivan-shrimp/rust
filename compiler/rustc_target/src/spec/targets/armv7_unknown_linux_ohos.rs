use crate::spec::{base, Target, TargetOptions};

// This target is for OpenHarmony on ARMv7 Linux with thumb-mode, but no NEON or
// hardfloat.

pub fn target() -> Target {
    // Most of these settings are copied from the armv7_unknown_linux_musleabi
    // target.
    Target {
        // LLVM 15 doesn't support OpenHarmony yet, use a linux target instead.
        llvm_target: "armv7-unknown-linux-gnueabi".into(),
        metadata: crate::spec::TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: None,
        },
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".into(),
        arch: "arm".into(),

        options: TargetOptions {
            abi: "eabi".into(),
            features: "+v7,+thumb2,+soft-float,-neon".into(),
            max_atomic_width: Some(64),
            mcount: "\u{1}mcount".into(),
            ..base::linux_ohos::opts()
        },
    }
}
