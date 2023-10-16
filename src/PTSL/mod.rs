pub mod SDK {
    tonic::include_proto!("ptsl");
}
use tonic::transport::Channel;
use SDK::ptsl_client::PtslClient;
use SDK::{Request, RequestHeader, Response};

use self::SDK::{CommandId, RegisterConnectionRequestBody, RegisterConnectionResponseBody};
use serde::Serialize;

#[derive(Debug)]
pub enum Error {
    CommandNotFound,
    SerdeErr(serde_json::Error),
    NetErr(tonic::Status),
    TransportErr(tonic::transport::Error),
}

pub struct Client {
    client: PtslClient<Channel>,
    session_id: String,
}

impl Client {
    pub async fn connect(
        addr: String,
        company_name: String,
        application_name: String,
    ) -> Result<Self, Error> {
        let mut client = PtslClient::connect(addr)
            .await
            .map_err(|err| Error::TransportErr(err))?;

        let body: RegisterConnectionRequestBody = RegisterConnectionRequestBody {
            company_name,
            application_name,
        };

        let request = tonic::Request::new(Request {
            header: None,
            request_body_json: serde_json::to_string(&body).map_err(|err| Error::SerdeErr(err))?,
        });
        let response = client
            .send_grpc_request(request)
            .await
            .map_err(|status| Error::NetErr(status))?;

        let body_response = response.into_inner().response_body_json;

        let response: RegisterConnectionResponseBody =
            serde_json::from_str(&body_response).map_err(|err| Error::SerdeErr(err))?;
        Ok(Self {
            client,
            session_id: response.session_id,
        })
    }

    pub async fn request<T: Serialize>(
        &mut self,
        command: &str,
        payload: T,
    ) -> Result<tonic::Response<Response>, Error> {
        match CommandId::from_str_name(command) {
            Some(command_id) => {
                let header = RequestHeader {
                    command: command_id.into(),
                    task_id: String::new(),
                    version: 3,
                    session_id: self.session_id.clone(),
                };

                let request_body_json =
                    serde_json::to_string(&payload).map_err(|err| Error::SerdeErr(err))?;
                self.client
                    .send_grpc_request(Request {
                        header: Some(header),
                        request_body_json,
                    })
                    .await
                    .map_err(|status| Error::NetErr(status))
            }
            None => Err(Error::CommandNotFound),
        }
    }
}
