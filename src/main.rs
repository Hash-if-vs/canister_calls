use anyhow::{Result, anyhow};
use candid::{CandidType, Decode, Encode, Principal};
use ic_agent::Agent;
use ic_agent::identity::{AnonymousIdentity, BasicIdentity, Secp256k1Identity};
pub use ic_transport_types::{CallResponse, signed};
use serde::Deserialize;
use std::fs;
pub mod jwt_token;
pub use jwt_token::generate_jwt_token;
#[derive(Debug, Deserialize, Clone, CandidType)]
#[serde(rename_all = "camelCase")]
pub struct WizardDetailsBasic {
    pub id: String,
    pub name: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub biography: String,
    pub description: String,
    pub avatar: String,
    #[serde(rename = "isPublished")]
    pub is_published: bool,
}

#[derive(Debug, Deserialize, Clone, CandidType)]
#[serde(rename_all = "camelCase")]
pub enum WizardVisibility {
    #[serde(rename = "publicVisibility")]
    PublicVisibility,
    #[serde(rename = "privateVisibility")]
    PrivateVisibility,
    #[serde(rename = "unlistedVisibility")]
    UnlistedVisibility,
}

#[derive(Debug, Deserialize, Clone, CandidType)]
pub struct WizardDetailsV3 {
    pub id: String,
    pub name: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub biography: String,
    pub description: String,
    pub avatar: String,
    #[serde(rename = "isPublished")]
    pub is_published: bool,
    pub greeting: String,
    pub summary: Option<String>,
    pub visibility: WizardVisibility,
    #[serde(rename = "poolAddress")]
    pub pool_address: Option<String>,
    #[serde(rename = "tokenAddress")]
    pub token_address: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let generated_token = generate_user_token().await?;
    println!("Generated Token: {}", generated_token);

    let user_principal =
        Principal::from_text("7asb6-fxesw-xu7xz-aqxjl-kxxa4-jdnrt-opepl-hhznk-4eekr-2bwm3-wqe")
            .unwrap();
    println!("{}", user_principal); // Replace with actual user principal
    let retrieved_token = get_user_token(user_principal).await?;
    println!("Retrieved Token: {}", retrieved_token);
    let jwt_token = generate_jwt_token(&retrieved_token).unwrap();
    println!("JWT Token: {}", jwt_token);
    Ok(())
}

async fn create_agent() -> Result<Agent, anyhow::Error> {
    // Create a BasicIdentity from the PEM file
    let identity = Secp256k1Identity::from_pem_file("testplug.pem")?;
    // let identity = AnonymousIdentity;
    let agent = Agent::builder()
        .with_url("https://ic0.app")
        .with_identity(identity)
        .build()?;

    agent.fetch_root_key().await?;
    Ok(agent)
}
pub async fn get_user_token(user_principal: Principal) -> Result<String, anyhow::Error> {
    let agent = create_agent().await?;

    let canister_id = "6qy4q-5aaaa-aaaah-adwma-cai";
    let response = agent
        .update(&Principal::from_text(canister_id)?, "getUserToken")
        .with_arg(Encode!(&user_principal)?)
        .call_and_wait()
        .await?;

    let token_result: Result<String, _> = Decode!(&response, String);

    match token_result {
        Ok(token) => Ok(token),
        Err(err) => Err(anyhow!("Failed to get user token: {:?}", err)),
    }
}

pub async fn generate_user_token() -> Result<String, anyhow::Error> {
    let agent = create_agent().await?;

    let canister_id = "6qy4q-5aaaa-aaaah-adwma-cai";
    let response = agent
        .update(&Principal::from_text(canister_id)?, "generateUserToken") // Corrected method name
        .with_arg(Encode!()?)
        .call_and_wait()
        .await?;

    // Decode the response bytes into a (String,)
    let token_result: Result<String, _> = Decode!(&response, String);

    match token_result {
        Ok(token) => Ok(token),
        Err(err) => Err(anyhow!("Failed to generate user token: {:?}", err)),
    }
}

pub async fn get_wizard_details(wizard_id: &str) -> Result<WizardDetailsV3, anyhow::Error> {
    let agent = create_agent().await?;

    let canister_id = "gichg-2iaaa-aaaah-adtia-cai";

    let response: Vec<u8> = agent
        .query(&Principal::from_text(canister_id)?, "getWizard")
        .with_arg(Encode!(&wizard_id)?)
        .call()
        .await?;

    // Handle possible version variations
    let wizard_v3: Result<Option<WizardDetailsV3>, _> = Decode!(&response, Option<WizardDetailsV3>);

    match wizard_v3 {
        Ok(Some(wizard)) => Ok(wizard),
        Ok(None) => Err(anyhow!("Wizard not found")),
        Err(err) => Err(anyhow!("Failed to decode wizard details: {}", err)),
    }
}
