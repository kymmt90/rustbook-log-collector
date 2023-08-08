use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Log {
    pub user_agent: String,
    pub response_time: i32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct DateTimeRange {
    pub from: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
}

pub mod logs {
    pub mod get {
        use crate::{DateTimeRange, Log};
        use serde::{Deserialize, Serialize};

        pub type Query = DateTimeRange;

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
        pub struct Response(pub Vec<Log>);
    }

    pub mod post {
        use chrono::{DateTime, Utc};
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
        pub struct Request {
            pub user_agent: String,
            pub response_time: i32,
            pub timestamp: Option<DateTime<Utc>>,
        }
    }
}
