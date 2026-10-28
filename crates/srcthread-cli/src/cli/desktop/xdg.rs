//! FreeDesktop `.desktop` + icon install under `$XDG_DATA_HOME`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const SHIP_ID: &str = "dev.kbroom.srcthread";
const DEV_ID: &str = "dev.kbroom.srcthread-dev";

const SHIP_DESKTOP: &str =
    include_str!("../../../../../packaging/linux/dev.kbroom.srcthread.desktop");
const SHIP_ICON: &str = include_str!("../../../../../packaging/linux/dev.kbroom.srcthread.svg");
const DEV_DESKTOP: &str =
    include_str!("../../../../../packaging/linux/dev.kbroom.srcthread-dev.desktop");
const DEV_ICON: &str = include_str!("../../../../../packaging/linux/dev.kbroom.srcthread-dev.svg");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Flavor {
    Ship,
    Dev,
}

impl Flavor {
    pub(crate) fn from_dev_flag(dev: bool) -> Self {
        if dev { Self::Dev } else { Self::Ship }
    }

    pub(crate) fn id(self) -> &'static str {
        match self {
            Self::Ship => SHIP_ID,
            Self::Dev => DEV_ID,
        }
    }

    fn desktop_template(self) -> &'static str {
        match self {
            Self::Ship => SHIP_DESKTOP,
            Self::Dev => DEV_DESKTOP,
        }
    }

    fn icon_svg(self) -> &'static str {
        match self {
            Self::Ship => SHIP_ICON,
            Self::Dev => DEV_ICON,
        }
    }

    fn display_name(self) -> &'static str {
        match self {
            Self::Ship => "srcthread",
            Self::Dev => "srcthread (dev)",
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct XdgRoots {
    pub(crate) data_home: PathBuf,
    pub(crate) exe: PathBuf,
}

impl XdgRoots {
    pub(crate) fn from_env() -> Result<Self, String> {
        let dirs = directories::BaseDirs::new()
            .ok_or_else(|| "could not resolve XDG data directory".to_string())?;
        let exe = std::env::current_exe().map_err(|e| format!("current executable: {e}"))?;
        Ok(Self {
            data_home: dirs.data_dir().to_path_buf(),
            exe,
        })
    }

    #[cfg(test)]
    pub(crate) fn for_test(data_home: PathBuf, exe: PathBuf) -> Self {
        Self { data_home, exe }
    }

    pub(crate) fn applications_dir(&self) -> PathBuf {
        self.data_home.join("applications")
    }

    pub(crate) fn icon_dir(&self) -> PathBuf {
        self.data_home.join("icons/hicolor/scalable/apps")
    }

    pub(crate) fn desktop_path(&self, flavor: Flavor) -> PathBuf {
        self.applications_dir()
            .join(format!("{}.desktop", flavor.id()))
    }

    pub(crate) fn icon_path(&self, flavor: Flavor) -> PathBuf {
        self.icon_dir().join(format!("{}.svg", flavor.id()))
    }
}

pub(crate) fn require_linux() -> Result<(), String> {
    if cfg!(target_os = "linux") {
        Ok(())
    } else {
        Err("desktop XDG helpers are only supported on Linux".into())
    }
}

pub(crate) fn install(roots: &XdgRoots, flavor: Flavor) -> Result<(), String> {
    require_linux()?;
    let apps = roots.applications_dir();
    let icons = roots.icon_dir();
    fs::create_dir_all(&apps).map_err(|e| format!("mkdir {}: {e}", apps.display()))?;
    fs::create_dir_all(&icons).map_err(|e| format!("mkdir {}: {e}", icons.display()))?;

    let exe = roots.exe.display().to_string();
    let desktop = rewrite_desktop(flavor.desktop_template(), flavor, &exe);
    let desktop_path = roots.desktop_path(flavor);
    fs::write(&desktop_path, desktop)
        .map_err(|e| format!("write {}: {e}", desktop_path.display()))?;

    let icon_path = roots.icon_path(flavor);
    fs::write(&icon_path, flavor.icon_svg())
        .map_err(|e| format!("write {}: {e}", icon_path.display()))?;

    refresh_caches(&roots.data_home, &apps);
    println!(
        "installed FreeDesktop entry {} ({})",
        desktop_path.display(),
        flavor.id()
    );
    Ok(())
}

pub(crate) fn uninstall(roots: &XdgRoots, flavor: Flavor) -> Result<(), String> {
    require_linux()?;
    let desktop_path = roots.desktop_path(flavor);
    let icon_path = roots.icon_path(flavor);
    let mut removed = false;
    if desktop_path.is_file() {
        fs::remove_file(&desktop_path)
            .map_err(|e| format!("remove {}: {e}", desktop_path.display()))?;
        removed = true;
    }
    if icon_path.is_file() {
        fs::remove_file(&icon_path).map_err(|e| format!("remove {}: {e}", icon_path.display()))?;
        removed = true;
    }
    refresh_caches(&roots.data_home, &roots.applications_dir());
    if removed {
        println!("removed FreeDesktop entry for {}", flavor.id());
    } else {
        println!("no FreeDesktop entry for {} found", flavor.id());
    }
    Ok(())
}

pub(crate) fn status(roots: &XdgRoots, flavor: Flavor) -> Result<(), String> {
    require_linux()?;
    let desktop_path = roots.desktop_path(flavor);
    let icon_path = roots.icon_path(flavor);
    let desktop_state = if desktop_path.is_file() {
        "present"
    } else {
        "missing"
    };
    let icon_state = if icon_path.is_file() {
        "present"
    } else {
        "missing"
    };
    println!(
        "{}: desktop {} ({}), icon {} ({})",
        flavor.id(),
        desktop_state,
        desktop_path.display(),
        icon_state,
        icon_path.display()
    );
    Ok(())
}

fn rewrite_desktop(template: &str, flavor: Flavor, exe: &str) -> String {
    let mut out = String::new();
    for line in template.lines() {
        if line.starts_with("Name=") {
            out.push_str("Name=");
            out.push_str(flavor.display_name());
        } else if line.starts_with("Exec=") {
            out.push_str("Exec=");
            out.push_str(exe);
            out.push_str(" desktop");
        } else if line.starts_with("TryExec=") {
            out.push_str("TryExec=");
            out.push_str(exe);
        } else if line.starts_with("Icon=") {
            out.push_str("Icon=");
            out.push_str(flavor.id());
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    out
}

fn refresh_caches(data_home: &Path, applications: &Path) {
    let _ = Command::new("update-desktop-database")
        .arg(applications)
        .status();
    let _ = Command::new("gtk-update-icon-cache")
        .args(["-f", "-t"])
        .arg(data_home.join("icons/hicolor"))
        .status();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrite_sets_absolute_exec_and_dev_name() {
        let text = rewrite_desktop(DEV_DESKTOP, Flavor::Dev, "/tmp/shim/srcthread");
        assert!(text.contains("Name=srcthread (dev)\n"));
        assert!(text.contains("Exec=/tmp/shim/srcthread desktop\n"));
        assert!(text.contains("TryExec=/tmp/shim/srcthread\n"));
        assert!(text.contains("Icon=dev.kbroom.srcthread-dev\n"));
    }

    #[test]
    fn install_uninstall_round_trip() {
        if !cfg!(target_os = "linux") {
            return;
        }
        let tmp = tempfile::tempdir().unwrap();
        let roots = XdgRoots::for_test(
            tmp.path().to_path_buf(),
            PathBuf::from("/usr/bin/srcthread"),
        );
        install(&roots, Flavor::Ship).unwrap();
        assert!(roots.desktop_path(Flavor::Ship).is_file());
        assert!(roots.icon_path(Flavor::Ship).is_file());
        let desktop = fs::read_to_string(roots.desktop_path(Flavor::Ship)).unwrap();
        assert!(desktop.contains("Exec=/usr/bin/srcthread desktop"));
        status(&roots, Flavor::Ship).unwrap();
        uninstall(&roots, Flavor::Ship).unwrap();
        assert!(!roots.desktop_path(Flavor::Ship).is_file());
    }
}
