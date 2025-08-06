use directories::UserDirs;
use which::which;

// Check AWS settings:
pub fn detect_aws() -> bool {
    // check aws binary:
    if which("aws").is_ok() {
        return true;
    }

    if let Some(home) = UserDirs::new().and_then(|u| Some(u.home_dir().to_path_buf())) {
        let aws_dir = home.join(".aws");
        if aws_dir.join("config").exists() || aws_dir.join("credentials.json").exists() {
            return true;
        }
    }

    false
}

pub fn detect_gcp() -> bool {
    if which("gcloud").is_ok() {
        return true;
    }

    if let Some(home) = UserDirs::new().and_then(|u| Some(u.home_dir().to_path_buf())) {
        return home.join(".config").join("gcloud").exists();
    }

    false
}

pub fn detect_azure() -> bool {
    if which("az").is_ok() {
        return true;
    }
    if let Some(home) = UserDirs::new().and_then(|u| Some(u.home_dir().to_path_buf())) {
        return home.join(".azure").exists();
    }

    false
}
