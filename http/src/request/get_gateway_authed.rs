use crate::request::prelude::*;
use dawn_model::gateway::connection_info::BotConnectionInfo;

pub struct GetGatewayAuthed<'a> {
    fut: Option<Pending<'a, BotConnectionInfo>>,
    http: &'a Client,
}

impl<'a> GetGatewayAuthed<'a> {
    pub fn new(http: &'a Client) -> Self {
        Self { fut: None, http }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(
            self.http.request(Request::from(Route::GetGatewayBot)),
        ));

        Ok(())
    }
}

poll_req!(GetGatewayAuthed<'_>, BotConnectionInfo);

pub struct GetUserGatewayAuthed<'a> {
    fut: Option<Pending<'a, BotConnectionInfo>>,
    http: &'a Client,
}

impl<'a> GetUserGatewayAuthed<'a> {
    pub fn new(http: &'a Client) -> Self {
        Self { fut: None, http }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(
            self.http.request(Request::from(Route::GetGateway)),
        ));

        Ok(())
    }
}
poll_req!(GetUserGatewayAuthed<'_>, BotConnectionInfo);
