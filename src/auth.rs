use solana_keypair::{Keypair, Signer};
use tonic::transport::Channel;
use crate::errors::BlockEngineConnectionError;
use crate::grpc::auth::auth_service_client::AuthServiceClient;
use crate::grpc::auth::{GenerateAuthChallengeRequest, GenerateAuthTokensRequest, Role, Token};
pub type BlockEngineConnectionResult<T> = Result<T, BlockEngineConnectionError>;

pub async fn auth(
  auth_service_client: &mut AuthServiceClient<Channel>,
  keypair: &Keypair,
  role: Role,
) -> BlockEngineConnectionResult<(Token, Token)> {
  let challenge_resp = auth_service_client
      .generate_auth_challenge(GenerateAuthChallengeRequest {
        role: role as i32,
        pubkey: keypair.pubkey().as_ref().to_vec(),
      })
      .await?
      .into_inner();
  let challenge = format!("{}-{}", keypair.pubkey(), challenge_resp.challenge);
  let signed_challenge = keypair.sign_message(challenge.as_bytes()).as_ref().to_vec();

  let tokens = auth_service_client
      .generate_auth_tokens(GenerateAuthTokensRequest {
        challenge,
        client_pubkey: keypair.pubkey().as_ref().to_vec(),
        signed_challenge,
      })
      .await?
      .into_inner();

  Ok((tokens.access_token.unwrap(), tokens.refresh_token.unwrap()))
}