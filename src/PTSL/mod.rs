pub mod SDK {
    tonic::include_proto!("ptsl");
}

use tonic::transport::Channel;
use SDK::ptsl_client::PtslClient;
use SDK::{Request, RequestHeader, Response};

use crate::PTSL::SDK::{ResponseHeader, TaskStatus};

use self::SDK::{CommandId, RegisterConnectionRequestBody, RegisterConnectionResponseBody};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    CommandNotFound,
    SerdeErr(serde_json::Error),
    NetErr(tonic::Status),
    TransportErr(tonic::transport::Error),
    PTStatus(TaskStatus),
    PTError,
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
            header: Some(RequestHeader {
                task_id: String::new(),
                command: CommandId::RegisterConnection.into(),
                version: 3,
                session_id: String::new(),
            }),
            request_body_json: serde_json::to_string(&body).map_err(|err| Error::SerdeErr(err))?,
        });

        println!("Send request {:?}", request);
        let response = client
            .send_grpc_request(request)
            .await
            .map_err(|status| Error::NetErr(status))?;

        let body = response.into_inner();

        match body.header {
            Some(ResponseHeader { status, .. }) => match status {
                3 => {} // TaskStatus::
                _ => return Err(Error::PTError),
            },
            _ => return Err(Error::PTError),
        }

        // println!("meta: {:?}, body: {:?}, ext: {:?}", meta, body, ext);
        let response: RegisterConnectionResponseBody =
            serde_json::from_str(&body.response_body_json).map_err(|err| Error::SerdeErr(err))?;
        Ok(Self {
            client,
            session_id: response.session_id,
        })
    }

    pub async fn request<Req: Serialize, Resp>(
        &mut self,
        command_id: CommandId,
        payload: Req,
    ) -> Result<Resp, Error> {
        let header = RequestHeader {
            command: command_id.into(),
            task_id: String::new(),
            version: 3,
            session_id: self.session_id.clone(),
        };

        let request_body_json =
            serde_json::to_string(&payload).map_err(|err| Error::SerdeErr(err))?;

        println!("Send request {:?}", request_body_json);
        let resp = self
            .client
            .send_grpc_request(Request {
                header: Some(header),
                request_body_json,
            })
            .await
            .map_err(|status| Error::NetErr(status))?
            .into_inner();
        if let Some(header) = resp.header {
            match header.status {
                3 => serde_json::from_str(&resp.response_body_json)
                    .map_err(|err| Error::SerdeErr(err))?,
                _ => serde_json::from_str(&resp.response_error_json)
                    .map_err(|err| Error::SerdeErr(err))?,
            }
        }
        Err(Error::PTError)
    }
}
