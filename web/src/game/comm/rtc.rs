use yew::prelude::*;
use web_sys::*;

use std::sync::{Arc, Mutex};

use std::vec::Vec;
use serde::*;
use wasm_bindgen_futures::{
    spawn_local,
    JsFuture,
};
use wasm_bindgen::{
    JsCast,
    JsValue,
    closure::Closure,
};

use js_sys::{Array, Reflect};


#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub enum RtcDataMsg {
    Blub,
    Name(String),
    Message(String),
}

#[derive(Serialize, Deserialize)]
pub enum RtcSignaling {
    Start,
    Offer(String),
    Answer(String),
    Ice(String, u16, String),
}

pub type SignalCallback = Callback<(RtcSignaling,usize)>;
pub type MessageCallback = Callback<(RtcDataMsg,usize)>;

#[derive(Clone)]
pub struct RTCConnection {
    pub conn: RtcPeerConnection,
    pub data: Arc<Mutex<RtcDataChannel>>,
    pub onsignal: SignalCallback,
    pub onmessage: MessageCallback,
    pub id: usize,
}

impl RTCConnection {
    pub fn get_ice_server(
        url: &'static str,
        username: &'static str,
        credential: &'static str
    ) -> RtcIceServer {
        let mut server = RtcIceServer::new();
        server.urls(&url.into());
        server.username(&username);
        server.credential(&credential);
        server
    }

    pub fn new(onsignal: SignalCallback, onmessage: MessageCallback, id: usize) -> Self
    {
        let username: &str = "6d3d463f278b172035926945";
        let credential: &str = "iSQuNExCmL18pvPU";

        // Stun server
        /*let mut stun_server = RtcIceServer::new();
        stun_server.url(&STUN_URL);*/

        // Turn server
        let turn_server1 = Self::get_ice_server("turn:a.relay.metered.ca:80", username, credential);
        let turn_server2 = Self::get_ice_server("turn:a.relay.metered.ca:443", username, credential);
        //let turn_server3 = Self::get_ice_server("turn:a.relay.metered.ca:443?transport=tcp", username, credential);

        let array_ice_servers = Array::of2(
            turn_server1.as_ref(),
            turn_server2.as_ref(),
            //turn_server3.as_ref(),
        );

        let rtc_config = {
            let mut cfg = RtcConfiguration::new();
            cfg.ice_servers(array_ice_servers.as_ref());
            cfg
        };

        let conn = RtcPeerConnection::new_with_configuration(&rtc_config).unwrap();

        let call = onsignal.clone();
        let onicecanditate = Closure::<dyn FnMut(_)>::new(move |ev: RtcPeerConnectionIceEvent| {
            if let Some(c) = ev.candidate() {
                let msg = RtcSignaling::Ice(c.candidate(), c.sdp_m_line_index().unwrap(),  c.sdp_mid().unwrap());
                call.emit( (msg, id) );
            }
        });
        conn.set_onicecandidate( Some(onicecanditate.as_ref().unchecked_ref()) );
        onicecanditate.forget();

        /*let onsig = onsignal.clone();
        let conc = conn.clone();
        let negneeded = Closure::<dyn FnMut(_)>::new(move |_ev: Event| {
            let call = onsig.clone();
            let con = conc.clone();

            spawn_local(async move {
                crate::utils::log("ON NEGOTIATION NEEDED!");
                let offer = JsFuture::from(con.create_offer()).await.unwrap();
                let _ = JsFuture::from(con.set_local_description(&offer.clone().into())).await;

                let offer_str = Reflect::get(&JsValue::from(offer), &JsValue::from("sdp")).unwrap()
                    .as_string()
                    .ok_or(JsValue::from_str("Failed to get SDP as string")).unwrap();

                call.emit( (RtcSignaling::RtcOffer(offer_str), id) );
            });
        });
        conn.set_onnegotiationneeded( Some(negneeded.as_ref().unchecked_ref()) );
        negneeded.forget();*/

        let dc = {
            let dc = conn.create_data_channel("data");
            add_data_channel_events(&dc, onmessage.clone(), id);
            Arc::new( Mutex::new(dc) )
        };

        let dc_clone = dc.clone();
        let onmsg = onmessage.clone();
        let ondatachannel = Closure::<dyn FnMut(_)>::new(move |ev: RtcDataChannelEvent| {
            let dc = ev.channel();
            add_data_channel_events(&dc, onmsg.clone(), id);
            let mut m = dc_clone.lock().unwrap();
            *m = dc;
        });
        conn.set_ondatachannel( Some(ondatachannel.as_ref().unchecked_ref()) );
        ondatachannel.forget();

        let con = conn.clone();
        let dc_clone = dc.clone();
        let onmsg = onmessage.clone();
        let onconnstatechange = Closure::<dyn FnMut(_)>::new(move |_: Event| {
            match con.connection_state() {
                RtcPeerConnectionState::Connected =>  {
                    let dc = con.create_data_channel("data");
                    add_data_channel_events(&dc, onmsg.clone(), id);
                    let mut m = dc_clone.lock().unwrap();
                    *m = dc;
                },
                RtcPeerConnectionState::Closed => {
                    con.close();
                },
                _ => {},
            }
        });
        conn.set_onconnectionstatechange( Some(onconnstatechange.as_ref().unchecked_ref()) );
        onconnstatechange.forget();

	Self {
            conn,
            data: dc,
            onsignal,
            onmessage,
            id,
        }
    }

    pub fn send_offer(&mut self) {
        let con = self.conn.clone();
        let id = self.id;
        let call = self.onsignal.clone();

        spawn_local(async move {
            let offer = JsFuture::from(con.create_offer()).await.unwrap();
            let _ = JsFuture::from(con.set_local_description(&offer.clone().into())).await;

            let offer_str = Reflect::get(&JsValue::from(offer), &JsValue::from("sdp")).unwrap()
                .as_string()
                .ok_or(JsValue::from_str("Failed to get SDP as string")).unwrap();

            call.emit( (RtcSignaling::Offer(offer_str), id) );
        });
    }

    pub fn on_offer(&mut self, offer: String) {
        let con = self.conn.clone();
        let id = self.id;
        let call = self.onsignal.clone();

        spawn_local(async move {
            let mut desc = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
            desc.sdp(&offer);
            let _ = JsFuture::from(con.set_remote_description(&desc)).await;

            let ans = JsFuture::from(con.create_answer()).await.expect("Could not create answer");
            let _ = JsFuture::from(con.set_local_description(&ans.clone().into())).await;

            let ans_str = Reflect::get(&JsValue::from(ans), &JsValue::from("sdp")).unwrap()
                .as_string()
                .ok_or(JsValue::from_str("Failed to get SDP as string")).unwrap();

            call.emit( (RtcSignaling::Answer(ans_str), id) );
        });
    }

    pub fn on_answer(&mut self, answer: String) {
        let con = self.conn.clone();

        spawn_local(async move {
            let mut desc = RtcSessionDescriptionInit::new(RtcSdpType::Answer);
            desc.sdp(&answer);
            let _ = JsFuture::from(con.set_remote_description(&desc)).await;
        });
    }

    pub fn on_ice_candidate(&mut self, candidate: String, sdp_m_line_index: u16, sdp_mid: String) {
        let con = self.conn.clone();
        let mut init = RtcIceCandidateInit::new(candidate.as_str());
        init.sdp_m_line_index(Some(sdp_m_line_index));
        init.sdp_mid(Some(sdp_mid.as_str()));

        let candidate = RtcIceCandidate::new(&init).expect("Could not recreate candidate!");

        spawn_local(async move {
            let fut = con.add_ice_candidate_with_opt_rtc_ice_candidate(Some(&candidate));
            JsFuture::from(fut).await.expect("Could not add ice candidate");
        });
    }
}

#[derive(Clone)]
pub struct RtcHandler {
    pub id: usize,
    pub pc: Vec<RTCConnection>,
}

impl RtcHandler {
    pub fn new(self_id: usize, onsignal: SignalCallback, onmessage: MessageCallback, num_players: usize) -> Self
    {
        let pc = (0..num_players).map(|i| RTCConnection::new(onsignal.clone(), onmessage.clone(), i)).collect();
        Self {
	    id: self_id,
            pc,
        }
    }

    pub fn send_to(&mut self, receiver: usize, msg: RtcDataMsg) {
        let data = self.pc[receiver].data.clone();
        let dc = data.lock().expect("Could not get datachannel unlocked!");
        let s = serde_json::to_string(&msg).expect("Could not deserialize RtcDataMsg!");
        let _ = dc.send_with_str(&s);
    }

    pub fn send_to_all(&mut self, msg: RtcDataMsg) {
        for i in 0..self.pc.len() {
            if self.pc[i].conn.connection_state() != RtcPeerConnectionState::Connected { continue; }
            self.send_to(i, msg.clone());
        }
    }

    pub fn handle_signaling(&mut self, packet: RtcSignaling, sender: usize) {
        let pc = &mut self.pc[sender];

        crate::utils::log("handle signaling");

        match packet {
            RtcSignaling::Start => pc.send_offer(),
            RtcSignaling::Offer(s) => pc.on_offer(s),
            RtcSignaling::Answer(s) => pc.on_answer(s),
            RtcSignaling::Ice(a,b,c) => pc.on_ice_candidate(a,b,c), //TODO make code cleaner
        }
    }

    pub fn init_rtc(&mut self, stream_opt: Option<web_sys::MediaStream>) {
	for pc in &mut self.pc {
	    if pc.id == self.id { continue; }

	    let remotestream = MediaStream::new().unwrap();

	    {
		let doc = web_sys::window().unwrap().document().unwrap();
		let ele = doc.get_element_by_id( format!("audio{}", pc.id).as_str() ).unwrap();
		let media: HtmlMediaElement = ele.dyn_into().unwrap();
		media.set_src_object(Some(&remotestream));
	    }

            let ontrack = Closure::<dyn FnMut(_)>::new(move |e: RtcTrackEvent| {
		for stream_weird in e.streams() {
		    let stream: MediaStream = stream_weird.into();
		    for track in stream.get_tracks() { remotestream.add_track(&track.into()); }
		}
            });
            pc.conn.set_ontrack( Some(ontrack.as_ref().unchecked_ref()) );
            ontrack.forget();

	    if let Some(stream) = stream_opt.clone() {
		for track in stream.get_tracks() {
                    pc.conn.add_track(&track.into(), &stream, &js_sys::Array::new());
		}
	    }

            pc.onsignal.emit( (RtcSignaling::Start, pc.id) );
        }
    }
}

pub fn setup_mic(onstream: Callback< Option<web_sys::MediaStream>  >) {
    spawn_local(async move {
        let window = web_sys::window().expect("Could not get window!");

	let md = window.navigator().media_devices().unwrap();

        let mut constr = web_sys::MediaStreamConstraints::new();
        constr.audio( &JsValue::TRUE );

        let localsteam = JsFuture::from( md.get_user_media_with_constraints(&constr).unwrap() ).await;

        match localsteam {
            Ok(stream) => onstream.emit( Some(stream.into()) ), // got mic
            Err(_) => onstream.emit( None ), // user disagreed
        }
    });
}

pub fn add_data_channel_events(dc: &web_sys::RtcDataChannel, onmessage: MessageCallback, id: usize) {
    let call = onmessage.clone();
    let onmsg = Closure::<dyn FnMut(_)>::new(move |ev: MessageEvent| match ev.data().as_string() {
        Some(s) => {
            let msg: RtcDataMsg = serde_json::from_str(&s).unwrap();
            call.emit( (msg, id) );
        },
        None => {},
    });
    dc.set_onmessage(Some(onmsg.as_ref().unchecked_ref()));
    onmsg.forget();
}
