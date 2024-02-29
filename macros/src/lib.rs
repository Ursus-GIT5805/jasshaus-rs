use proc_macro::TokenStream;
use quote::*;
use syn::{
    DeriveInput,
    Field,
    Ident,
    Type, Attribute,
};

mod enums;

fn handle_data_type(label: proc_macro2::TokenStream, path: syn::TypePath, _input_type: String) -> proc_macro2::TokenStream {
    let ty = path.clone().to_token_stream();
    let s = path.to_token_stream().to_string();

    match s.as_str() {
	"bool" => quote! {
	    html! { <BooleanInput label={ #label } {onchange}/> }
	},
	"i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64"| "u64" => {
	    quote! {
		html! {
		    <NumberInput<#ty>
			min={std::#ty::MIN}
		    max={std::#ty::MAX}
		    label={ #label }
		    {onchange}
		    />
		}
	    }
	}
	"String" => quote! {
	    html! {
		<StringInput
		    label={ #label }
		{onchange}
		/>
	    }
	},
	_ => quote! {
	    html! {
		// The given datatype must implements the trait "YewSetting"
		<YewSettingsForm<#ty> {onchange} title={ #label }/>
	    }
	},
    }
}

fn get_attribute_token_stream( path: String, attrs: &Vec<Attribute> ) -> Option< proc_macro2::TokenStream > {

    for att in attrs {
	if att.style != syn::AttrStyle::Outer { continue; }

	if let syn::Meta::List(m) = &att.meta {
	    let s = m.path.to_token_stream().to_string();
	    if s == path {
		return m.tokens.clone().into();
	    }
	}
    }

    None
}

fn handle_struct_field(field: Field) -> proc_macro2::TokenStream {
    let f_ident: Ident = field.ident.unwrap();
    let f_ty: Type = field.ty.clone();

    let label = if let Some(tok) = get_attribute_token_stream("label".into(), &field.attrs) {
	tok
    } else {
	quote! { stringify!{ #f_ident } }
    };

    let input = match f_ty.clone() {
	Type::Array(_) => quote!{},
	Type::Path(p) => {
	    let _input: Option<String> = None;

	    let input_quote = handle_data_type(label, p, String::new());

	    quote! {
		{
		    let arc_clone = arc.clone();
		    let callback = onchange.clone();
		    let onchange = yew::callback::Callback::from(move |val: #f_ty| {
			let mut data = arc_clone.lock().unwrap();
			data.#f_ident = val;
			callback.emit(data.clone());
		    });

		    #input_quote
		}
	    }
	},
	_ => panic!("Unsupported datatype!"),
    };

    quote! {
	{html! {
	    {#input}
	}}
    }
}

fn handle_enum_field(enum_ident: Ident, variant: syn::Variant) -> proc_macro2::TokenStream {
    let ident = variant.ident.clone();

    match variant.fields {
	syn::Fields::Named(_f) => {
	    panic!("YewSettings does not currently support named enum fields")
	},
	syn::Fields::Unnamed(_f) => {
	    panic!("YewSettings does not currently support unnamed enum fields")
	},
	syn::Fields::Unit => {},
    }

    quote! {
	stringify! { #ident } => #enum_ident::#ident,
    }
}


#[proc_macro_derive(YewSetting, attributes(input, label, tooltip, min, max, placeholder))]
pub fn macro_yew_setting(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let ident = ast.ident;

    let out = match ast.data {
	syn::Data::Struct(data) => {
	    let fields: Vec<_> = data.fields.into_iter().map( handle_struct_field ).collect();

	    quote! {
		html! { <> #(#fields)* </> }
	    }
	},
	syn::Data::Enum(data) => {
	    let options: Vec<_> = data.variants.clone().into_iter().map( |v| {
		let ident = v.ident;
		let label = if let Some(tok) = get_attribute_token_stream("label".into(), &v.attrs) { tok }
		else { quote! { stringify! { #ident } } };

		quote! {
		    <option value={ stringify! { #ident } } >{ #label }</option>
		}
	    }).collect();

	    let matches: Vec<_> = data.variants.into_iter().map( |v| handle_enum_field(ident.clone(), v) ).collect();

	    quote! {
		let arc_clone = arc.clone();
		let custom_onchange = onchange;
		let onchange = Callback::from(move |e: web_sys::Event| {
		    if let Some(target) = e.target() {
			if let Some(input) = target.dyn_ref::<web_sys::HtmlSelectElement>() {
			    let value = input.value();
			    let mut data = arc_clone.lock().expect("Could not unwrap!");
			    *data = match value.as_str() {
				#(#matches)*
				_ => #ident::default(),
			    };
			    custom_onchange.emit( data.clone() );
			}
		    }
		});

		html! {
		    <p>
			<select {onchange}>
		            #(#options)*
		        </select>
		    </p>
		}
	    }
	},
	_ => panic!("Invalid data type!"),
    };

    //println!("{}", out);

    quote! {
	impl jasshaus_traits::YewSetting for #ident {
	    fn setting_html(arc: std::sync::Arc< std::sync::Mutex<Self> >, onchange: Callback<Self>) -> Html {
		#out
	    }
	}
    }.into()
}
