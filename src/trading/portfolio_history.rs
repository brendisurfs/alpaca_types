use serde::{Deserialize, Serialize};

#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Deserialize, Default, Debug)]
#[cfg_attr(feature = "ts", derive(TS), ts(export))]
pub struct PortfolioHistory {
    pub timestamp: Vec<i64>,
    pub equity: Vec<f64>,
    pub profit_loss: Vec<f64>,
    pub profit_loss_pct: Vec<f64>,
    pub base_value: f64,
    pub base_value_asof: String,
    pub timeframe: String,
}
impl PortfolioHistory {
    /// converts PortfolioHistory into a Vec of HistoryFrame.
    /// Errors if the timestamp cannot be parsed.
    pub fn to_frames(self) -> Result<Vec<HistoryFrame>, time::error::ComponentRange> {
        let mut collection = vec![];
        for (idx, timestamp) in self.timestamp.iter().enumerate() {
            let report_time = time::OffsetDateTime::from_unix_timestamp(*timestamp)?;

            let Some(equity) = self.equity.get(idx) else {
                continue;
            };
            let Some(profit_loss) = self.profit_loss.get(idx) else {
                continue;
            };

            let Some(profit_loss_pct) = self.profit_loss_pct.get(idx) else {
                continue;
            };

            let base_value = self.base_value;
            let base_value_as_of = self.base_value_asof.clone();

            let frame = HistoryFrame {
                base_value,
                equity: *equity,
                base_value_as_of,
                profit_loss: *profit_loss,
                time: report_time.to_string(),
                profit_loss_percent: *profit_loss_pct,
            };
            collection.push(frame);
        }

        Ok(collection)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "ts", derive(TS), ts(export))]
///  An alternative structure to convert Portfolio History into frames.
pub struct HistoryFrame {
    // The time of the report
    pub time: String,
    // equity value of the account in dollar amount as of the end of each time window
    pub equity: f64,
    // The Dollar amount of Profit or Loss
    pub profit_loss: f64,
    // The Profit Loss Percent compared to the basis
    pub profit_loss_percent: f64,
    // basis in dollar of the profit loss calculation
    pub base_value: f64,
    // If included, then it indicates that the base_value is the account's closing
    // equity value at this trading date
    pub base_value_as_of: String,
}

#[cfg(test)]
mod portfolio_history_test {
    use crate::trading::portfolio_history::PortfolioHistory;
    const JSON_DATA: &str = r#"
        {
            "timestamp": [ 1722000600, 1722004200, 1722007800,1722011400, 1722015000, 1722018600, 1722022200],
            "equity": [100129.67, 100129.67,100129.67, 100129.67, 100129.67, 100129.67, 100129.67],
            "profit_loss": [ 0,0,0,0,0,0,0],
            "profit_loss_pct": [0,0,0,0,0,0,0],
            "base_value": 100129.67,
            "base_value_asof": "2024-07-25",
            "timeframe": "1H"
        }
        "#;

    #[test]
    fn position_history_parses() {
        let res = serde_json::from_str::<PortfolioHistory>(JSON_DATA);
        assert!(res.is_ok());
    }

    #[test]
    fn history_converts_to_frames() {
        let res = serde_json::from_str::<PortfolioHistory>(JSON_DATA)
            .expect("failed to parse portfolio history");

        let frames_result = res.to_frames();
        assert!(frames_result.is_ok());
    }
}
