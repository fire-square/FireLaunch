//! Version manifest structures.

use std::collections::HashMap;

use super::asset_index::AssetIndex;
use crate::storage::{Storage, StorageError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors, which can occur in this module.
#[derive(Error, Debug)]
pub enum VersionManifestError {
	/// Failed to store version manifest.
	#[error("Failed to store version manifest: {0}")]
	StorageError(#[from] StorageError),
	/// Failed to parse version manifest.
	#[error("Failed to parse version manifest: {0}")]
	ParseError(#[from] serde_json::Error),
	/// IO error.
	#[error("IO error: {0}")]
	IOError(#[from] std::io::Error),
}

fn default_libraries() -> Vec<Library> {
	Vec::new()
}

fn default_requires() -> Vec<Requirement> {
	Vec::new()
}

/// Artifact.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Artifact {
	/// Artifact sha1.
	pub sha1: String,
	/// Artifact size.
	pub size: u64,
	/// Artifact IPFS path.
	pub path: String,
}

impl Artifact {
	/// Get the artifact and store it.
	pub async fn get_artifact(&self, storage: &Storage) -> Result<(), VersionManifestError> {
		storage
			.download_asset_if_not_exists(&self.sha1, &self.path)
			.await?;
		Ok(())
	}

	/// Get the artifact and store it if it's invalid.
	pub async fn get_artifact_if_invalid(
		&self,
		storage: &Storage,
	) -> Result<(), VersionManifestError> {
		storage
			.download_asset_if_invalid(&self.sha1, &self.path)
			.await?;
		Ok(())
	}
}

/// Asset index artifact.
///
/// Basically, it's just an artifact with total size of all assets.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndexArtifact {
	/// Artifact sha1.
	pub sha1: String,
	/// Artifact size.
	pub size: u64,
	/// Artifact IPFS path.
	pub path: String,
	/// Total size of all assets.
	pub total_size: u64,
	/// ID of the asset index.
	pub id: String,
}

impl AssetIndexArtifact {
	/// Get the asset index artifact and parse it.
	pub async fn get_asset_index(
		&self,
		storage: &Storage,
	) -> Result<AssetIndex, VersionManifestError> {
		let asset_index_path = storage
			.download_asset_if_invalid(&self.sha1, &self.path)
			.await?;
		let asset_index_data = tokio::fs::read_to_string(asset_index_path).await?;
		let asset_index = serde_json::from_str(&asset_index_data)?;
		Ok(asset_index)
	}
}

/// Artifact downloads.
#[derive(Debug, Deserialize, Serialize)]
pub struct ArtifactDownloads {
	/// Artifact.
	pub artifact: Option<Artifact>,
	/// Classifiers.
	///
	/// This is a map of classifier name to artifact.
	/// For example, `natives-windows` to `Artifact`.
	///
	/// Classifiers defined in `natives` field of `Library` are not included here.
	pub classifiers: Option<HashMap<String, Artifact>>,
}

/// Main jar artifact.
#[derive(Debug, Deserialize, Serialize)]
pub struct MainJar {
	/// Downloads.
	pub downloads: ArtifactDownloads,
	/// Name.
	pub name: String,
}

/// Extract.
#[derive(Debug, Deserialize, Serialize)]
pub struct Extract {
	/// Exclude paths.
	pub exclude: Vec<String>,
}

/// Rule.
#[derive(Debug, Deserialize, Serialize)]
pub struct Rule {
	/// Action.
	pub action: String,
	/// OS.
	pub os: Option<RuleOS>,
}

/// Helper function to get the current OS name.
///
/// Supported OS names: `windows`, `osx` and `linux`.
fn get_os_name() -> String {
	#[cfg(target_os = "windows")]
	{
		"windows".to_string()
	}
	#[cfg(target_os = "macos")]
	{
		"osx".to_string()
	}
	#[cfg(target_os = "linux")]
	{
		"linux".to_string()
	}
}

impl Rule {
	fn action_to_bool(&self) -> bool {
		match self.action.as_str() {
			"allow" => true,
			"disallow" => false,
			_ => panic!("Invalid action"),
		}
	}

	fn invert(&self, b: bool) -> bool {
		match self.action_to_bool() {
			true => b,
			false => !b,
		}
	}

	/// Check if the rule is satisfied.
	///
	/// # Examples
	///
	/// current OS is windows:
	/// os.name = "windows"
	/// action = "allow"
	/// result = true
	///
	/// current OS is windows:
	/// os.name = "windows"
	/// action = "disallow"
	/// result = false
	///
	/// current OS is windows:
	/// os.name = "osx"
	/// action = "allow"
	/// result = false
	///
	/// current OS is windows:
	/// os.name = "osx"
	/// action = "disallow"
	/// result = true
	pub fn is_satisfied(&self) -> bool {
		match &self.os {
			Some(os) => self.invert(os.name == get_os_name()),
			None => true,
		}
	}
}

/// Rule OS.
#[derive(Debug, Deserialize, Serialize)]
pub struct RuleOS {
	/// Name.
	pub name: String,
}

/// Library artifact.
#[derive(Debug, Deserialize, Serialize)]
pub struct Library {
	/// Downloads.
	///
	/// This is the downloads information for the artifact. It includes the artifact itself and its classifiers.
	pub downloads: ArtifactDownloads,
	/// Name of the library.
	///
	/// For example, `org.lwjgl.lwjgl:lwjgl:3.2.2`.
	pub name: String,
	/// Extract.
	///
	/// This is the extract information for the artifact.
	/// If this is `None`, the artifact is not extracted.
	///
	/// Usually used for native libraries.
	pub extract: Option<Extract>,
	/// Rules.
	///
	/// This is a list of rules that must be satisfied for this library to be used.
	/// If this is empty, the library is always used.
	pub rules: Option<Vec<Rule>>,
	/// Natives.
	///
	/// This is a map of OS name to native classifier.
	/// For example, `windows` to `natives-windows`.
	pub natives: Option<HashMap<String, String>>,
}

impl Library {
	/// Check if the rules are satisfied.
	pub fn is_rules_satisfied(&self) -> bool {
		match &self.rules {
			Some(rules) => rules.iter().all(|rule| rule.is_satisfied()),
			None => true,
		}
	}

	/// Get Vec of artifacts that should be downloaded.
	pub fn get_artifacts(&self) -> Vec<Artifact> {
		let mut artifacts: Vec<Artifact> = Vec::new();
		if let Some(artifact) = &self.downloads.artifact {
			if self.is_rules_satisfied() {
				artifacts.push(artifact.clone());
			}
		}
		if let Some(classifiers) = &self.downloads.classifiers {
			if let Some(natives) = &self.natives {
				if let Some(native) = natives.get(&get_os_name()) {
					if let Some(classifier) = classifiers.get(native) {
						artifacts.push(classifier.clone());
					}
				}
			}
		}
		artifacts
	}
}

/// Requirement.
///
/// This is used to specify package requirements.
#[derive(Debug, Deserialize, Serialize)]
pub struct Requirement {
	/// Version of requirement.
	#[serde(rename = "suggests")]
	pub version: String,
	/// Version UID.
	pub uid: String,
}

/// Version manifest.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionManifest {
	/// Version traits.
	#[serde(rename = "+traits")]
	pub traits: Vec<String>,
	/// Asset index.
	///
	/// This is the asset index artifact of the Minecraft version.
	pub asset_index: Option<AssetIndexArtifact>,
	/// Compatible Java major versions.
	///
	/// This is a list of Java major versions that are compatible with this version.
	pub compatible_java_majors: Option<Vec<u8>>,
	/// Format version.
	///
	/// This is the version of the version manifest format.
	/// Currently, it's always `1`.
	pub format_version: u8,
	/// Library artifacts.
	#[serde(default = "default_libraries")]
	pub libraries: Vec<Library>,
	/// Main jar artifact.
	///
	/// This is the main jar artifact of the Minecraft version.
	pub main_jar: Option<MainJar>,
	/// Minecraft arguments.
	///
	/// This is the arguments to pass to the Minecraft launcher.
	pub minecraft_arguments: Option<String>,
	/// Main class.
	///
	/// This is the main class of the Minecraft version.
	pub main_class: Option<String>,
	/// Package version.
	pub version: String,
	/// Release type.
	///
	/// Can be `release`, `snapshot`, `old_beta` or `old_alpha`.
	#[serde(rename = "type")]
	pub release_type: String,
	/// Release time.
	pub release_time: String,
	/// Release name.
	pub name: String,
	/// Product UID.
	///
	/// Example: `org.lwjgl.lwjgl:lwjgl:3.2.2`.
	pub product_uid: String,
	/// Requirements.
	///
	/// This is a list of requirements that must be satisfied for this version to be used.
	#[serde(default = "default_requires")]
	pub requires: Vec<Requirement>,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rule_is_satisfied() {
		let rule = Rule {
			action: "allow".to_string(),
			os: None,
		};
		assert!(rule.is_satisfied());

		let rule = Rule {
			action: "allow".to_string(),
			os: Some(RuleOS {
				name: "windows".to_string(),
			}),
		};
		#[cfg(target_os = "windows")]
		{
			assert!(rule.is_satisfied());
		}
		#[cfg(not(target_os = "windows"))]
		{
			assert!(!rule.is_satisfied());
		}

		let rule = Rule {
			action: "disallow".to_string(),
			os: Some(RuleOS {
				name: "windows".to_string(),
			}),
		};
		#[cfg(target_os = "windows")]
		{
			assert!(!rule.is_satisfied());
		}
		#[cfg(not(target_os = "windows"))]
		{
			assert!(rule.is_satisfied());
		}
	}
}
