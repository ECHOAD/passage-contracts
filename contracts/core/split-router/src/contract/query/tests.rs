use super::*;
use cosmwasm_std::{testing::mock_dependencies, Coin};

#[test]
fn preview_split_handles_multiple_denoms() {
    let mut deps = mock_dependencies();

    SPLIT_RULES
        .save(
            deps.as_mut().storage,
            "preview-key",
            &SplitRule {
                key: "preview-key".to_string(),
                owner: Addr::unchecked("owner"),
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
        "preview-key".to_string(),
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
