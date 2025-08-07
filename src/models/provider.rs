#[derive(Copy, Clone, Debug)]
pub enum PackageManager {
    Apt,
    Yum,
    Brew,
    Choco,
}
impl PackageManager {
    pub fn pkg_name(self) -> &'static str {
        match self {
            PackageManager::Apt => "apt-get",
            PackageManager::Yum => "yum",
            PackageManager::Brew => "brew",
            PackageManager::Choco => "choco",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum CloudProvider {
    Aws,
    Gcp,
    Azure,
}
impl CloudProvider {
    pub fn provider_name(&self) -> &'static str {
        match self {
            CloudProvider::Aws => "aws",
            CloudProvider::Azure => "azure",
            CloudProvider::Gcp => "gcp",
        }
    }

    pub fn cli_binary(&self) -> &'static str {
        match self {
            CloudProvider::Aws => "aws",
            CloudProvider::Azure => "az",
            CloudProvider::Gcp => "gcloud",
        }
    }

    pub fn cli_name(&self, pm: PackageManager) -> &'static str {
        use CloudProvider::*;
        match (self, pm) {
            (Aws, PackageManager::Apt) => "awscli",
            (Aws, PackageManager::Yum) => "awscli",
            (Aws, PackageManager::Brew) => "awscli",
            (Aws, PackageManager::Choco) => "awscli",

            (Gcp, PackageManager::Apt) => "google-cloud-cli",
            (Gcp, PackageManager::Yum) => "google-cloud-cli",
            (Gcp, PackageManager::Brew) => "google-cloud-cli",
            (Gcp, PackageManager::Choco) => "google-cloud-cli",

            (Azure, PackageManager::Apt) => "azure-cli",
            (Azure, PackageManager::Yum) => "azure-cli",
            (Azure, PackageManager::Brew) => "azure-cli",
            (Azure, PackageManager::Choco) => "azure-cli",
        }
    }
}