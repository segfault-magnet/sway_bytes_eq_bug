#[cfg(test)]
mod tests {
    use fuels::prelude::*;

    #[tokio::test]
    async fn it_works() {
        setup_program_test!(
            Wallets("wallet"),
            Abigen(Script(name = "MyScript", project = "script")),
            LoadScript(
                name = "script_instance",
                script = "MyScript",
                wallet = "wallet"
            )
        );

        script_instance
            .main()
            .call()
            .await
            .expect_err("Should have reverted because the strings are not the same!");
    }
}
