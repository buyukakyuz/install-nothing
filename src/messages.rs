/// Easter egg messages that appear randomly during installation
pub const EASTER_EGGS: &[&str] = &[
    "Reticulating splines...",
    "Calibrating flux capacitor...",
    "Reversing polarity of neutron flow...",
    "Initializing holodeck subroutines...",
    "Downloading more RAM...",
    "Dividing by zero... Just kidding!",
    "Asking politely for system resources...",
    "Convincing AI not to take over...",
];

/// Warning messages that may appear during installation
pub const WARNINGS: &[&str] = &[
    "WARNING: Package version mismatch, attempting compatibility mode...",
    "WARNING: Checksum verification skipped (--force flag detected)",
    "WARNING: Deprecated dependency detected, adding to legacy support list",
    "WARNING: Mirror responded slowly, may switch to backup",
];

/// Retry messages for simulated connection issues
pub const RETRY_MESSAGES: &[&str] = &[
    "Connection timeout... Retrying (Attempt 2/5)",
    "Network error... Attempting reconnection",
    "Mirror unresponsive, trying alternate server",
];

/// System components to initialize
pub const SYSTEM_COMPONENTS: &[&str] = &[
    "Loading kernel modules",
    "Mounting root filesystem (ext3)",
    "Initializing network interfaces",
    "Starting system daemons",
    "Configuring system clock",
];

/// Hardware drivers to install
pub const DRIVERS: &[(&str, &str)] = &[
    ("VGA Graphics Adapter", "VESA 2.0 Compatible"),
    ("Sound Blaster 16", "Creative Labs"),
    ("Intel 82540EM Gigabit Ethernet", "e1000"),
    ("PS/2 Mouse", "Generic"),
    ("USB 1.1 UHCI Controller", "usb-uhci"),
];

/// Packages to install
pub const PACKAGES: &[&str] = &[
    "libc6:amd64",
    "libgtk-2.0-0",
    "python2.7-minimal",
    "perl-base",
    "gcc-4.8",
    "make",
    "binutils",
    "libjpeg62",
    "libpng12-0",
    "libssl1.0.0",
];

/// Retro software to install (name, version, size in KB)
pub const RETRO_SOFTWARE: &[(&str, &str, u32)] = &[
    ("Netscape Navigator", "4.79", 15234),
    ("WinAmp", "2.95", 3421),
    ("mIRC", "6.35", 1876),
    ("RealPlayer", "8.0", 8932),
    ("Adobe Flash Player", "7.0", 2341),
];

/// Optimization tasks
pub const OPTIMIZATION_TASKS: &[&str] = &[
    "Defragmenting installation cache",
    "Rebuilding font cache",
    "Updating shared library cache",
    "Optimizing package database",
    "Generating manual page index",
];
