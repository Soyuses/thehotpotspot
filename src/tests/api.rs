use crate::*;

#[test]
fn api_request_response_serde_roundtrip() {
    let req = ApiRequest::GetMenu;
    let s = serde_json::to_string(&req).unwrap();
    let de: ApiRequest = serde_json::from_str(&s).unwrap();
    match de {
        ApiRequest::GetMenu => {}
        _ => panic!("mismatch"),
    }

    let resp = ApiResponse::Menu { items: vec![] };
    let s2 = serde_json::to_string(&resp).unwrap();
    let de2: ApiResponse = serde_json::from_str(&s2).unwrap();
    match de2 {
        ApiResponse::Menu { items } => assert!(items.is_empty()),
        _ => panic!("mismatch"),
    }
}


