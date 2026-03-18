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
}
