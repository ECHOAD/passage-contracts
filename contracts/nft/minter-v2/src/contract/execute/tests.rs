use super::*;

#[test]
fn pasg_payment_attributes_reference_canonical_surface() {
    let response = add_pasg_payment_attributes(Response::new(), "upasg", "mint_payment");

    assert!(response
        .attributes
        .iter()
        .any(|attr| attr.key == "pasg_utility_query"
            && attr.value == crate::msg::CANONICAL_PASG_UTILITY_QUERY));
    assert!(response
        .attributes
        .iter()
        .any(|attr| attr.key == "pasg_native_denom"
            && attr.value == crate::msg::CANONICAL_PASG_NATIVE_DENOM));
    assert!(response
        .attributes
        .iter()
        .any(|attr| attr.key == "pasg_uses_native_utility" && attr.value == "true"));
}

#[test]
fn pasg_payment_attributes_mark_non_native_withdrawals() {
    let response = add_pasg_payment_attributes(Response::new(), "uatom", "mint_withdrawal");

    assert!(response
        .attributes
        .iter()
        .any(|attr| attr.key == "pasg_settlement_denom" && attr.value == "uatom"));
    assert!(response
        .attributes
        .iter()
        .any(|attr| attr.key == "pasg_uses_native_utility" && attr.value == "false"));
    assert!(response
        .attributes
        .iter()
        .any(|attr| attr.key == "pasg_fee_flow" && attr.value == "mint_withdrawal"));
}
