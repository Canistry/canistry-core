use candid::{CandidType, Deserialize};
use ic_cdk::api::management_canister::{
    main::{self, CanisterIdRecord, CreateCanisterArgument, InstallCodeArgument, CanisterInstallMode},
};
use ic_cdk_macros::*;
use serde::Serialize;

#[derive(CandidType, Deserialize, Serialize)]
struct CanisterCode {
    code: Vec<u8>,
}

#[update]
async fn deploy_canister(canister_code: CanisterCode) -> String {
    ic_cdk::println!("Deploying canister...");

    // Create a new canister with cycles (None specifies default amount)
    let create_canister_args = CreateCanisterArgument { settings: None };
    let cycles: u128 = 0; // Adjust cycles as needed

    let (canister_id_record,): (CanisterIdRecord,) = main::create_canister(create_canister_args, cycles).await.expect("Failed to create canister");

    let canister_id = canister_id_record.canister_id;

    // Install the code into the new canister
    let install_code_args = InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id,
        wasm_module: canister_code.code,
        arg: vec![],
    };

    main::install_code(install_code_args).await.expect("Failed to install canister code");

    ic_cdk::println!("Canister deployed successfully: {:?}", canister_id);
    format!("Canister deployed successfully: {:?}", canister_id)
}

#[query]
fn get_canister_metrics(canister_id: String) -> String {
    ic_cdk::println!("Fetching metrics for canister: {}", canister_id);
    // Add monitoring logic here
    "Canister metrics".to_string()
}

#[update]
fn set_alert(canister_id: String, threshold: u32) -> String {
    ic_cdk::println!("Setting alert for canister: {} with threshold: {}", canister_id, threshold);
    // Add alerting logic here
    "Alert set successfully".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::{Decode, Encode};

    #[test]
    fn test_deploy_canister() {
        let canister_code = CanisterCode {
            code: vec![0, 1, 2, 3],
        };
        let encoded = Encode!(&canister_code).unwrap();
        let decoded: CanisterCode = Decode!(&encoded).unwrap();
        assert_eq!(canister_code.code, decoded.code);
    }
}
