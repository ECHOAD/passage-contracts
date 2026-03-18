use super::*;
use cosmwasm_std::{testing::mock_dependencies, Addr, Coin, Decimal};

use crate::msg::{RoutingExecuteRoute, RoutingPreviewRoute};

#[test]
fn preview_split_handles_multiple_denoms() {
    let mut deps = mock_dependencies();

    SPLIT_CONFIG
        .save(
            deps.as_mut().storage,
            &SplitConfig {
                recipients: vec![
                    Recipient {
                        address: Addr::unchecked("platform"),
                        share: Decimal::percent(25),
                        label: None,
                    },
                    Recipient {
                        address: Addr::unchecked("creator"),
                        share: Decimal::percent(75),
                        label: None,
                    },
                ],
                active: true,
                created_at: 0,
                updated_at: 0,
            },
        )
        .unwrap();

    let response = query_preview_split(
        deps.as_ref(),
        vec![Coin::new(101u128, "upasg"), Coin::new(7u128, "uatom")],
    )
    .unwrap();

    assert_eq!(
        response.total_funds,
        vec![Coin::new(101u128, "upasg"), Coin::new(7u128, "uatom")]
    );
    assert_eq!(
        response.recipient_amounts[0],
        (
            "platform".to_string(),
            vec![Coin::new(25u128, "upasg"), Coin::new(1u128, "uatom")]
        )
    );
    assert_eq!(
        response.recipient_amounts[1],
        (
            "creator".to_string(),
            vec![Coin::new(76u128, "upasg"), Coin::new(6u128, "uatom")]
        )
    );
}


#[test]
fn routing_metadata_reports_generic_passthrough_surface() {
    let response = query_routing_metadata();

    assert!(response.forwards_attached_funds);
    assert!(response.preserves_input_denoms);
    assert_eq!(response.preview_query, RoutingPreviewRoute::PreviewSplit);
    assert_eq!(
        response.execute_routes,
        vec![
            RoutingExecuteRoute::Split,
            RoutingExecuteRoute::RouteWorldRevenue,
        ]
    );
}
