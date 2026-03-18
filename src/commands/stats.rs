use crate::client::EtherscanClient;
use crate::error::XplorerError;
use crate::handlers;

fn date_range_params<'a>(
    startdate: &'a str,
    enddate: &'a str,
    sort: &'a str,
) -> Vec<(&'a str, &'a str)> {
    vec![
        ("startdate", startdate),
        ("enddate", enddate),
        ("sort", sort),
    ]
}

pub async fn dailyavgblocksize(
    client: &EtherscanClient,
    startdate: &str,
    enddate: &str,
    sort: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = date_range_params(startdate, enddate, sort);
    if raw {
        super::print_raw_response(client, "stats", "dailyavgblocksize", &params).await
    } else {
        let formatted = handlers::stats::format_dailyavgblocksize(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn dailyblkcount(
    client: &EtherscanClient,
    startdate: &str,
    enddate: &str,
    sort: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = date_range_params(startdate, enddate, sort);
    if raw {
        super::print_raw_response(client, "stats", "dailyblkcount", &params).await
    } else {
        let formatted = handlers::stats::format_dailyblkcount(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn dailyblockrewards(
    client: &EtherscanClient,
    startdate: &str,
    enddate: &str,
    sort: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = date_range_params(startdate, enddate, sort);
    if raw {
        super::print_raw_response(client, "stats", "dailyblockrewards", &params).await
    } else {
        let formatted = handlers::stats::format_dailyblockrewards(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn dailyavgblocktime(
    client: &EtherscanClient,
    startdate: &str,
    enddate: &str,
    sort: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = date_range_params(startdate, enddate, sort);
    if raw {
        super::print_raw_response(client, "stats", "dailyavgblocktime", &params).await
    } else {
        let formatted = handlers::stats::format_dailyavgblocktime(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn dailyuncleblkcount(
    client: &EtherscanClient,
    startdate: &str,
    enddate: &str,
    sort: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = date_range_params(startdate, enddate, sort);
    if raw {
        super::print_raw_response(client, "stats", "dailyuncleblkcount", &params).await
    } else {
        let formatted = handlers::stats::format_dailyuncleblkcount(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn dailyavggaslimit(
    client: &EtherscanClient,
    startdate: &str,
    enddate: &str,
    sort: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = date_range_params(startdate, enddate, sort);
    if raw {
        super::print_raw_response(client, "stats", "dailyavggaslimit", &params).await
    } else {
        let formatted = handlers::stats::format_dailyavggaslimit(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn dailyavggasprice(
    client: &EtherscanClient,
    startdate: &str,
    enddate: &str,
    sort: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = date_range_params(startdate, enddate, sort);
    if raw {
        super::print_raw_response(client, "stats", "dailyavggasprice", &params).await
    } else {
        let formatted = handlers::stats::format_dailyavggasprice(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn dailygasused(
    client: &EtherscanClient,
    startdate: &str,
    enddate: &str,
    sort: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = date_range_params(startdate, enddate, sort);
    if raw {
        super::print_raw_response(client, "stats", "dailygasused", &params).await
    } else {
        let formatted = handlers::stats::format_dailygasused(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn ethsupply(client: &EtherscanClient, raw: bool) -> Result<(), XplorerError> {
    if raw {
        super::print_raw_response(client, "stats", "ethsupply", &[]).await
    } else {
        let formatted = handlers::stats::format_ethsupply(client).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn ethsupply2(client: &EtherscanClient, raw: bool) -> Result<(), XplorerError> {
    if raw {
        super::print_raw_response(client, "stats", "ethsupply2", &[]).await
    } else {
        let formatted = handlers::stats::format_ethsupply2(client).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn ethprice(client: &EtherscanClient, raw: bool) -> Result<(), XplorerError> {
    if raw {
        super::print_raw_response(client, "stats", "ethprice", &[]).await
    } else {
        let formatted = handlers::stats::format_ethprice(client).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn nodecount(client: &EtherscanClient, raw: bool) -> Result<(), XplorerError> {
    if raw {
        super::print_raw_response(client, "stats", "nodecount", &[]).await
    } else {
        let formatted = handlers::stats::format_nodecount(client).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn chainsize(
    client: &EtherscanClient,
    startdate: &str,
    enddate: &str,
    clienttype: &str,
    syncmode: &str,
    sort: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = vec![
        ("startdate", startdate),
        ("enddate", enddate),
        ("clienttype", clienttype),
        ("syncmode", syncmode),
        ("sort", sort),
    ];
    if raw {
        super::print_raw_response(client, "stats", "chainsize", &params).await
    } else {
        let formatted = handlers::stats::format_chainsize(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn ethdailyprice(
    client: &EtherscanClient,
    startdate: &str,
    enddate: &str,
    sort: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = date_range_params(startdate, enddate, sort);
    if raw {
        super::print_raw_response(client, "stats", "ethdailyprice", &params).await
    } else {
        let formatted = handlers::stats::format_ethdailyprice(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn dailyavghashrate(
    client: &EtherscanClient,
    startdate: &str,
    enddate: &str,
    sort: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = date_range_params(startdate, enddate, sort);
    if raw {
        super::print_raw_response(client, "stats", "dailyavghashrate", &params).await
    } else {
        let formatted = handlers::stats::format_dailyavghashrate(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn dailyavgnetdifficulty(
    client: &EtherscanClient,
    startdate: &str,
    enddate: &str,
    sort: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = date_range_params(startdate, enddate, sort);
    if raw {
        super::print_raw_response(client, "stats", "dailyavgnetdifficulty", &params).await
    } else {
        let formatted = handlers::stats::format_dailyavgnetdifficulty(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn dailynetutilization(
    client: &EtherscanClient,
    startdate: &str,
    enddate: &str,
    sort: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = date_range_params(startdate, enddate, sort);
    if raw {
        super::print_raw_response(client, "stats", "dailynetutilization", &params).await
    } else {
        let formatted = handlers::stats::format_dailynetutilization(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn dailynewaddress(
    client: &EtherscanClient,
    startdate: &str,
    enddate: &str,
    sort: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = date_range_params(startdate, enddate, sort);
    if raw {
        super::print_raw_response(client, "stats", "dailynewaddress", &params).await
    } else {
        let formatted = handlers::stats::format_dailynewaddress(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn dailytx(
    client: &EtherscanClient,
    startdate: &str,
    enddate: &str,
    sort: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = date_range_params(startdate, enddate, sort);
    if raw {
        super::print_raw_response(client, "stats", "dailytx", &params).await
    } else {
        let formatted = handlers::stats::format_dailytx(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn dailytxnfee(
    client: &EtherscanClient,
    startdate: &str,
    enddate: &str,
    sort: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = date_range_params(startdate, enddate, sort);
    if raw {
        super::print_raw_response(client, "stats", "dailytxnfee", &params).await
    } else {
        let formatted = handlers::stats::format_dailytxnfee(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::EtherscanClient;

    #[tokio::test]
    async fn test_dailyavgblocksize_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "dailyavgblocksize".into()),
            ]))
            .with_status(200)
            .with_body(r#"{"status":"1","message":"OK","result":[]}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        assert!(
            dailyavgblocksize(&client, "2019-02-01", "2019-02-28", "asc", true)
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn test_dailyavggasprice_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "dailyavggasprice".into()),
            ]))
            .with_status(200)
            .with_body(r#"{"status":"1","message":"OK","result":[]}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        assert!(
            dailyavggasprice(&client, "2019-02-01", "2019-02-28", "asc", true)
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn test_ethsupply_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "ethsupply".into()),
            ]))
            .with_status(200)
            .with_body(r#"{"status":"1","message":"OK","result":"122373866217800000000000000"}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        assert!(ethsupply(&client, true).await.is_ok());
    }

    #[tokio::test]
    async fn test_ethdailyprice_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "ethdailyprice".into()),
            ]))
            .with_status(200)
            .with_body(r#"{"status":"1","message":"OK","result":[]}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        assert!(
            ethdailyprice(&client, "2019-02-01", "2019-02-28", "asc", true)
                .await
                .is_ok()
        );
    }
}
