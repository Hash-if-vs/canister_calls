use anyhow::{Result, anyhow};
use candid::{CandidType, Decode, Encode, Principal};
use ic_agent::Agent;
use ic_agent::identity::AnonymousIdentity;
use serde::Deserialize;
use std::time::SystemTime;

#[derive(Debug, Deserialize, Clone, CandidType)]
pub struct WizardDetailsBasicWithCreatorName {
    pub id: String,
    pub name: String,
    // TODO: Change to Principal
    pub user_id: String,
    pub biography: String,
    pub description: String,
    pub avatar: String,
    pub is_published: bool,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub creator_name: String,
    pub token_address: Option<String>,
    pub pool_address: Option<String>,
}
#[derive(Debug, Deserialize, Clone, CandidType)]

pub enum WizardVisibility {
    PublicVisibility,
    PrivateVisibility,
    UnlistedVisibility,
}
#[derive(Debug, Deserialize, Clone, CandidType)]
pub struct WizardDetailsV3 {
    pub id: String,
    pub name: String,
    // TODO: Change to Principal
    pub user_id: String,
    pub biography: String,
    pub description: String,
    pub avatar: String,
    pub is_published: bool,
    pub greeting: String,
    pub summary: Option<String>,
    pub visibility: WizardVisibility,
    pub pool_address: Option<String>,
    pub token_address: Option<String>,
}
#[tokio::main]
async fn main() -> Result<()> {
    let identity = AnonymousIdentity;

    let agent = Agent::builder()
        .with_url("https://ic0.app")
        .with_identity(identity)
        .build()
        .map_err(|e| anyhow!("Failed to create agent: {}", e))?;

    agent
        .fetch_root_key()
        .await
        .map_err(|e| anyhow!("Failed to fetch root key: {}", e))?;

    let canister_id = "gichg-2iaaa-aaaah-adtia-cai";
    let wizard_id = "0";

    let response: Vec<u8> = agent
        .query(&Principal::from_text(canister_id).unwrap(), "getWizard")
        .with_arg(Encode!(&wizard_id)?)
        .call()
        .await?;

    let wizard: Option<WizardDetailsV3> = Decode!(&response, Option<WizardDetailsV3>)?;

    match wizard {
        Some(wizard) => println!("Found wizard: {:#?}", wizard),
        None => println!("Wizard not found"),
    }

    // let response: Vec<u8> = agent
    //     .query(&Principal::from_text(canister_id).unwrap(), "getWizards")
    //     .with_arg(Encode!()?) // No arguments needed for this method
    //     .call()
    //     .await
    //     .map_err(|e| anyhow!("Failed to call canister method: {}", e))?;

    // // Step 5: Decode the response
    // let wizards: Vec<WizardDetailsBasicWithCreatorName> =
    //     Decode!(&response, Vec<WizardDetailsBasicWithCreatorName>)
    //         .map_err(|e| anyhow!("Failed to decode response: {}", e))?;

    // // Step 6: Print the result
    // println!("Wizards: {:?}", wizards);

    // let arg =
    //     Encode!(&"World".to_string()).map_err(|e| anyhow!("Failed to encode argument: {}", e))?;

    // let response: Vec<u8> = agent
    //     .query(&Principal::from_text(canister_id).unwrap(), "greet")
    //     .with_arg(arg)
    //     .call()
    //     .await
    //     .map_err(|e| anyhow!("Failed to call canister method: {}", e))?;

    // let response: String =
    //     Decode!(&response, String).map_err(|e| anyhow!("Failed to decode response: {}", e))?;

    // println!("Response: {}", response);

    Ok(())
}
