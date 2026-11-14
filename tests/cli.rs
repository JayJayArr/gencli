mod cli {
    use assert_cmd::cargo_bin_cmd;
    use predicates::str::contains;
    use rstest::rstest;

    #[tokio::test]
    async fn test_cli_aborts_without_keyfile() {
        let mut cmd = cargo_bin_cmd!();
        cmd.args(["list-inactive-identities"])
            .assert()
            .failure()
            .stderr(contains(
                "error: the following required arguments were not provided:\n  -k",
            ));
    }

    #[tokio::test]
    async fn test_cli_fails_with_dummy_keyfile() {
        let mut cmd = cargo_bin_cmd!();
        cmd.args(["list-inactive-identities", "-k", "tests/key-dummy.json"])
            .assert()
            .failure()
            .stderr(contains("tcp connect error"));
    }

    #[tokio::test]
    #[rstest]
    #[case("purge-inactive-identities")]
    #[case("purge-pictures")]
    async fn test_cli_emmits_delete_warning_on_flag(#[case] command: &str) {
        let mut cmd = cargo_bin_cmd!();
        cmd.args([command, "-k", "tests/key-dummy.json"])
            .assert()
            .failure()
            .stdout(contains("Are you sure you want to do this?"));
    }

    #[tokio::test]
    #[rstest]
    #[case("list-inactive-identities")]
    #[case("purge-inactive-identities")]
    #[case("purge-pictures")]
    async fn all_subcommands_are_accepted(#[case] command: &str) {
        let mut cmd = cargo_bin_cmd!();
        cmd.args([command]).assert().failure().stderr(contains(
            "the following required arguments were not provided:",
        ));
    }

    #[tokio::test]
    async fn test_cli_prints_correct_version_number() {
        let mut cmd = cargo_bin_cmd!();
        cmd.args(["--version"])
            .assert()
            .success()
            .stdout(contains("gencli"))
            .stdout(contains(env!("CARGO_PKG_VERSION")));
    }
}
