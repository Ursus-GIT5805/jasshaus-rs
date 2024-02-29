pub mod rtc;
pub mod chat;
pub mod player;

/*pub enum Msg {
    OnRtcSignal(rtc::RtcSignaling, usize),
    OnRtcData(rtc::RtcDataMsg, usize),
    SendRtcData(rtc::RtcDataMsg, usize),
    AddRtcAudiostream(web_sys::MediaStream),

    AddPlayer(usize),
    AddMsg(String, chat::ChatMessageType),

    OnChatlink(Scope<chat::Chat>),
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub onsignal: rtc::SignalCallback,
    pub onlink: Callback< Scope<CommComponent> >,
    pub maxplayers: usize,
}

pub struct CommComponent {
    pub rtc: rtc::RtcHandler,
    chatlink: Option< Scope<chat::Chat> >,
}

impl Component for CommComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props().onlink.emit( ctx.link().clone() );
        let onmessage = ctx.link().callback( move |(s, id): (rtc::RtcDataMsg, usize)| Msg::OnRtcData(s, id) );

        Self {
            rtc: rtc::RtcHandler::new(ctx.props().onsignal.clone(), onmessage, ctx.props().maxplayers),
            chatlink: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnChatlink(l) => self.chatlink = Some(l),
            Msg::OnRtcSignal(signal, sender) => self.rtc.handle_signaling(signal, sender),
            Msg::AddRtcAudiostream(stream) => self.rtc.init_audio(stream),
            Msg::OnRtcData(s, id) => {
                if let Some(link) = &self.chatlink {
                    if let rtc::RtcDataMsg::Message(t) = s {
                        crate::utils::log(t.as_str());
                            link.send_message( chat::Msg::Message(t, chat::ChatMessageType::Normal) );
                    }
                }
            },
            Msg::AddPlayer(id) => {
                self.rtc.pc[id].send_offer();
            },
            Msg::AddMsg(text, msgtype) => {
                if let Some(link) = &self.chatlink {
                    link.send_message( chat::Msg::Message(text, msgtype) );
                }
            }
            Msg::SendRtcData(msg, _peer) => {
                self.rtc.send_to_all(msg);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onlink = ctx.link().callback(move |l: Scope<chat::Chat> | Msg::OnChatlink(l));
        let l = self.chatlink.clone();
        let ontext = ctx.link().callback(move |text: String| {
            if let Some(link) = &l {
                link.send_message( chat::Msg::Message(text.clone(), chat::ChatMessageType::Yours) );
            }
            Msg::SendRtcData( rtc::RtcDataMsg::Message(text), 0 )
        });

        html ! {
            <chat::Chat {ontext} {onlink} />
        }
    }
}*/
