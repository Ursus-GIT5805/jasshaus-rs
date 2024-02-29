use std::sync::{
    Arc,
    Mutex,
};
use yew::prelude::*;
use wasm_bindgen::JsCast;

//type OnChangeCallback = Callback<Event>;

pub trait YewSetting: Clone + PartialEq + Default {
    fn setting_html(arc: Arc< Mutex<Self> >, onchange: Callback<Self>) -> Html;
}

//---

#[derive(PartialEq, Properties)]
pub struct YewSettingsFormProps<T>
where T: PartialEq {
    pub onchange: Callback<T>,

    #[prop_or_default]
    pub title: Option<String>,
}

pub struct YewSettingsForm<T>
where T: YewSetting {
    pub data: Arc< Mutex< T > >
}

impl<T> Component for YewSettingsForm<T>
where T: YewSetting + 'static {
    type Message = ();
    type Properties = YewSettingsFormProps<T>;

    fn create(_ctx: &Context<Self>) -> Self {
	let mutex = Mutex::new( T::default() );
	let arc = Arc::new( mutex );

	Self {
	    data: arc,
	}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
	let title = match ctx.props().title.clone() {
	    Some(s) => html! { <h3>{s}</h3> },
	    None => html! {},
	};

	let onchange = ctx.props().onchange.clone();

	html! {
	    <div>
	     {title}
	     {T::setting_html( self.data.clone(), onchange )}
	    </div>
	}
    }
}

// ----

#[derive(PartialEq, Properties)]
pub struct NumberInputProps<T>
where T: PartialEq + Copy {
    pub min: T,
    pub max: T,

    pub label: String,
    pub onchange: Callback<T>,

    #[prop_or_default]
    pub tooltip: Option<String>,
    #[prop_or_default]
    pub placeholder: Option<T>,
}

pub struct NumberInput<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Component for NumberInput<T>
where T: PartialEq + ToString + std::str::FromStr + std::fmt::Debug + Copy + 'static {
    type Message = ();
    type Properties = NumberInputProps<T>;

    fn create(_ctx: &Context<Self>) -> Self {
	Self {
	    _phantom: std::marker::PhantomData::<T>::default()
	}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
	let label = format!("{}: ", ctx.props().label.clone());

	let min = ctx.props().min.to_string();
	let max = ctx.props().max.to_string();

	let callback = ctx.props().onchange.clone();
	let onfocusout = Callback::from(move |e: web_sys::FocusEvent| {
	    if let Some(target) = e.target() {
		if let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>() {
		    if let Ok(v) = input.value().parse::<T>() {
			callback.emit(v);
		    }
		}
	    }
	});

	html! {
	    <p>
		<label>{label}</label>
		<input type="number" {min} {max} {onfocusout}/>
		</p>
	}
    }
}

// ---

#[derive(PartialEq, Properties)]
pub struct BooleanInputProps {
    pub label: String,
    pub onchange: Callback<bool>,

    #[prop_or_default]
    pub tooltip: Option<String>,
}

pub struct BooleanInput {}
impl Component for BooleanInput {
    type Message = ();
    type Properties = BooleanInputProps;

    fn create(_ctx: &Context<Self>) -> Self {
	Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
	let label = ctx.props().label.clone();

	let callback = ctx.props().onchange.clone();
	let onchange = Callback::from(move |e: web_sys::Event| {
	    if let Some(target) = e.target() {
		if let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>() {
		    callback.emit( input.checked() );
		}
	    }
	});

	html! {
	    <p>
		<label>{label}</label>
		<input {onchange} type="checkbox"/>
		</p>
	}
    }
}


// ---

#[derive(PartialEq, Properties)]
pub struct StringInputProps {
    pub label: String,
    pub onchange: Callback<String>,

    #[prop_or_default]
    pub tooltip: Option<String>,
}

pub struct StringInput {}
impl Component for StringInput {
    type Message = ();
    type Properties = StringInputProps;

    fn create(_ctx: &Context<Self>) -> Self {
	Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
	let label = format!("{}: ", ctx.props().label.clone());

	let callback = ctx.props().onchange.clone();
	let onchange = Callback::from(move |e: web_sys::Event| {
	    if let Some(target) = e.target() {
		if let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>() {
		    callback.emit( input.value() );
		}
	    }
	});

	html! {
	    <p>
		<label>{label}</label>
		<input {onchange} type="text"/>
		</p>
	}
    }
}
