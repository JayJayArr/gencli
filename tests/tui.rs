mod tui {
    use assert_cmd::cargo_bin_cmd;
    use predicates::str::contains;

    #[tokio::test]
    async fn test_tui_start_with_keyfile() {
        let mut cmd = cargo_bin_cmd!();
        cmd.args(["tui"]).assert().failure().stderr(contains(
            "error: the following required arguments were not provided:\n  -k",
        ));
    }
}
