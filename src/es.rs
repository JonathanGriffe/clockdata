use elasticsearch::{Elasticsearch, SearchParts};
use serde_json::json;

pub async fn count_distinct_values(env: &str, client: &Elasticsearch) -> u64 {
    let body = json!({
        "size": 0,
        "query": {
            "range": {
                "@timestamp": {
                    "gte": "now-5m",
                    "lte": "now"
                }
            }
        },
        "aggs": {
            "distinct_count": {
                "cardinality": { "field": "user_id" }
            }
        }
    });

    // Send search request
    let response = client
        .search(SearchParts::Index(&[&format!("beatguessr-{}-*", env)]))
        .body(body)
        .send()
        .await
        .expect("Failed to send request")
        .json::<serde_json::Value>()
        .await
        .expect("Failed to parse response");

    let count = response["aggregations"]["distinct_count"]["value"]
        .as_u64()
        .expect("Failed to parse aggregation");

    count
}
