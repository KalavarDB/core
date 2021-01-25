use crate::managers::config::post::PrivacyMode;
use raw_cpuid::ProcessorFrequencyInfo;

/// The manager which reports analytics information to the user (and back to the core developers when allowed to)
pub struct AnalyticsManager {
    /// The setting for privacy within the configuration
    /// `None` means that only the web ui will be able to receive data analytics
    /// anything else means both the web ui and devs will get analytics data
    pub mode: PrivacyMode,
}

/// A sub structure which contains all of the possible information which will be displayed to the user via the statistics viewer utility
/// as well as any information which MIGHT be shared with the developers based on privacy config
pub struct Analytics {
    /// The version of Kalavar, this is set at compile time and will not be altered except when a new release is ready for shipping.
    pub version: String,

    /// The information which we take regarding the CPU within your system
    pub cpu: CPUInfo,

    /// The information which we take regarding the OS you are using
    pub operating_system: OSInfo,

    /// The information which we take regarding the disks within your system
    pub disk_info: DiskInfo,

    /// The information which we take regarding the memory within your system
    pub mem_info: MemoryInfo,
}

/// An information storage structure designed to store information about the CPU
pub struct CPUInfo {
    /// The manufacturer of your CPU.
    /// E.G. "Intel", "AMD"
    pub manufacturer: String,

    /// The model of the the CPU.
    /// E.G. "Ryzen 5 3600x"
    pub model: String,

    /// The frequency of your CPU
    pub frequency: ProcessorFrequencyInfo,

    /// The number of logical cores within the CPU. This is normally around double your physical core count.
    pub logical_cores: u64,

    /// The number of physical cores within the CPU
    pub physical_cores: u64,
}

/// An information storage structure designed to store information about the OS
pub struct OSInfo {
    /// The name of the operating system which is running
    /// for Windows: "windows"
    /// for Linux: "linux"
    /// for MacOS: "darwin"
    pub os: String,

    /// The version of the operating system which is running
    pub version: String,

    /// The release ID for the operating system which is running
    pub release: String,
}

/// An information storage structure designed to store information about the disk
pub struct DiskInfo {
    /// The number of partitions across all the disks on the system
    pub partitions: usize,

    /// The amount of storage beiung
    pub usage_percent: f32,
}

/// An information storage structure designed to store information about the memory
pub struct MemoryInfo {
    /// The physical memory usage within the system
    pub physical: InnerMemoryInfo,

    /// The SWAPFILE usage within the system
    pub swap: InnerMemoryInfo,

    /// The memory that Kalavar is using within the system
    pub kalavar_specific: InnerMemoryInfo,
}

/// An information storage structure designed to store information about the memory
pub struct InnerMemoryInfo {
    /// The total available
    pub total: u64,

    /// The amount used
    pub used: u64,

    /// The amount free
    pub free: u64,
}