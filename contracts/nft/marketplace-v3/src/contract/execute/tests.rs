use super::*;

#[test]
fn pasg_payment_attributes_reference_canonical_surface() {
    let response = add_pasg_payment_attributes(Response::new(), "upasg", "marketplace_sale");

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
    assert!(response
        .attributes
        .iter()
        .any(|attr| attr.key == "pasg_fee_flow" && attr.value == "marketplace_sale"));
}

#[test]
fn pasg_payment_attributes_preserve_non_native_collection_denoms() {
    let response = add_pasg_payment_attributes(Response::new(), "uion", "marketplace_sale");

    assert!(response
        .attributes
        .iter()
        .any(|attr| attr.key == "pasg_settlement_denom" && attr.value == "uion"));
    assert!(response
        .attributes
        .iter()
        .any(|attr| attr.key == "pasg_uses_native_utility" && attr.value == "false"));
}
