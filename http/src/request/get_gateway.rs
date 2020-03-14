use super::{prelude::*, GetGatewayAuthed, GetUserGatewayAuthed};
use dawn_model::gateway::connection_info::ConnectionInfo;

pub struct GetGateway<'a> {
    fut: Option<Pending<'a, ConnectionInfo>>,
    http: &'a Client,
}

impl<'a> GetGateway<'a> {
    pub(crate) fn new(http: &'a Client) -> Self {
        Self { fut: None, http }
    }

    pub fn authed(self) -> GetGatewayAuthed<'a> {
        GetGatewayAuthed::new(self.http)
    }

    pub fn user_authed(self) -> GetUserGatewayAuthed<'a> {
        GetUserGatewayAuthed::new(self.http)
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(
            self.http.request(Request::from(Route::GetGateway)),
        ));

        Ok(())
    }
}

poll_req!(GetGateway<'_>, ConnectionInfo);
