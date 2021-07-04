use anyhow::Error;

use yew::prelude::*;
use yew::format::Json;
use yew::services::ConsoleService;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

struct Model {
    ws: Option<WebSocketTask>,
    link: ComponentLink<Self>,
    text: String,
    server_data: String,
}

enum Msg {
    Connect,
    Disconnected,
    Ignore,
    TextInput(String),
    SendText,
    Received(Result<String, Error>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            ws: None,
            link: link,
            text: String::new(),
            server_data: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Connect => {
                ConsoleService::log("Connecting");
                let cbout = self.link.callback(|Json(data)| Msg::Received(data));
                let cbnot = self.link.callback(|input| {
                    ConsoleService::log(&format!("Notification: {:?}", input));
                    match input {
                        WebSocketStatus::Closed | WebSocketStatus::Error => {
                            Msg::Disconnected
                        }
                        _ => Msg::Ignore,
                    }
                });
                if self.ws.is_none() {
                    let task = WebSocketService::connect_text("ws://127.0.0.1:2794", cbout, cbnot);
                    self.ws = Some(task.unwrap());
                }
                true
            }
            Msg::Disconnected => {
                self.ws = None;
                true
            }
            Msg::Ignore => {
                false
            }
            Msg::TextInput(e) => {
                self.text = e;
                true
            }
            Msg::SendText => {
                match self.ws {
                    Some(ref mut task) => {
                        task.send(Json(&self.text));
                        true
                    }
                    None => {
                        false
                    }
                }
            }
            Msg::Received(Ok(s)) => {
                self.server_data.push_str(&format!("{}\n", &s));
                true
            }
            Msg::Received(Err(s)) => {
                self.server_data.push_str(&format!("Error when reading from server: {}\n", &s.to_string()));
                true
            }
        }
    }

    fn change(&mut self, _prop: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                // connect button
                <p><button onclick=self.link.callback(|_| Msg::Connect)>{ "Connect" }</button></p><br/>
                // text showing whether we're connected or not
                <p>{ "Connected: "}{ !self.ws.is_none() } </p><br/>
                // input box for sending text
                <p><input type="text" value=self.text.clone() oninput=self.link.callback(|e: InputData| Msg::TextInput(e.value))/></p><br/>
                // button for sending text
                <p><button onclick=self.link.callback(|_| Msg::SendText)>{ "Send" }</button></p><br/>
                // text area for showing data from the server
                <p><textarea value=self.server_data.clone()></textarea></p><br/>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
