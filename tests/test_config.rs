mod utils;

use hig::Config;
use utils::setup;

#[test]
fn new_branch_name_without_prefix() {
    setup::repo(|repo| {
        let config = Config::load_local(&repo);

        let name = config.new_branch_name("feature", hig::BranchType::Feature);
        assert_eq!(name, "feature");

        let name = config.new_branch_name("hotfix", hig::BranchType::Hotfix);
        assert_eq!(name, "hotfix");

        let name = config.new_branch_name("release", hig::BranchType::Release);
        assert_eq!(name, "release");
    });
}

#[test]
fn new_branch_name_with_prefix() {
    setup::repo(|repo| {
        let mut config = Config::load_local(&repo);
        config.data.feature.prefix = Some("feature/".to_string());
        config.data.hotfix.prefix = Some("hotfix/".to_string());
        config.data.release.prefix = Some("release/".to_string());

        let name = config.new_branch_name("feature", hig::BranchType::Feature);
        assert_eq!(name, "feature/feature");

        let name = config.new_branch_name("hotfix", hig::BranchType::Hotfix);
        assert_eq!(name, "hotfix/hotfix");

        let name = config.new_branch_name("release", hig::BranchType::Release);
        assert_eq!(name, "release/release");
    });
}

#[test]
fn save_config() {
    setup::repo(|repo| {
        let mut config = Config::load_local(&repo);
        config.data.feature.prefix = Some("feature/".to_string());
        config.data.hotfix.prefix = Some("hotfix/".to_string());
        config.data.release.prefix = Some("release/".to_string());
        config.save().unwrap();

        let config = Config::load_local(&repo);
        assert_eq!(config.data.feature.prefix, Some("feature/".to_string()));
        assert_eq!(config.data.hotfix.prefix, Some("hotfix/".to_string()));
        assert_eq!(config.data.release.prefix, Some("release/".to_string()));
    });
}
