// Application constants and configuration

pub static APP_ID: &str = "org.gnome.UpdateCenter";
pub static APP_NAME: &str = "GNOME Update Center";
pub static APP_VERSION: &str = "0.1.0";

// Update settings defaults
pub static DEFAULT_AUTO_CHECK: bool = true;
pub static DEFAULT_AUTO_DOWNLOAD: bool = false;
pub static DEFAULT_SECURITY_ONLY: bool = true;
pub static DEFAULT_CHECK_INTERVAL_DAYS: u32 = 1;

// For detecting distribution
pub enum DistroFamily {
    Debian,    // Debian, Ubuntu, Linux Mint, etc.
    Fedora,    // Fedora, RHEL, CentOS
    Arch,      // Arch Linux, Manjaro
    Suse,      // openSUSE, SUSE
    Other,
}

pub fn detect_distro() -> DistroFamily {
    // In a real app, this would check /etc/os-release
    // For now we'll just return Debian/Ubuntu as the default
    
    DistroFamily::Debian
}