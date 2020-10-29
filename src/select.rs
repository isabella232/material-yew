use wasm_bindgen::prelude::*;
use yew::prelude::*;
use web_sys::{Node, CustomEvent};
use crate::{add_event_listener, add_event_listener_with_one_param, to_option, to_option_string, NativeValidityState, validity_state::ValidityStateJS, ValidityTransform, ValidityState};
use crate::utils::WeakComponentLink;
use wasm_bindgen::JsCast;
pub use crate::list::{ActionDetail, SelectedDetail, ListIndex};

#[wasm_bindgen(module = "/build/mwc-select.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type Select;

    #[wasm_bindgen(getter, static_method_of = Select)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method)]
    fn select(this: &Select, index: usize);

    #[wasm_bindgen(method, setter = validityTransform)]
    fn set_validity_transform(this: &Select, val: &Closure<dyn Fn(String, NativeValidityState) -> ValidityStateJS>);
}

loader_hack!(Select);

/// The `mwc-select` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/select)
pub struct MatSelect {
    props: Props,
    node_ref: NodeRef,
    validity_transform_closure: Option<Closure<dyn Fn(String, NativeValidityState) -> ValidityStateJS>>,
    opened_closure: Option<Closure<dyn FnMut()>>,
    closed_closure: Option<Closure<dyn FnMut()>>,
    action_closure: Option<Closure<dyn FnMut(JsValue)>>,
    selected_closure: Option<Closure<dyn FnMut(JsValue)>>,
}

/// Props for [`MatSelect`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/select#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/select#events)
#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub natural_menu_width: bool,
    #[prop_or_default]
    pub icon: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub helper: String,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub validation_message: String,
    #[prop_or_default]
    pub items: String,
    #[prop_or(- 1)]
    pub index: i64,
    #[prop_or_default]
    pub validity_transform: Option<ValidityTransform>,
    #[prop_or_default]
    pub validate_on_initial_render: bool,
    #[prop_or_default]
    pub children: Children,
    /// [`WeakComponentLink`] for `MatList` which provides the following methods
    /// - ```select(&self)```
    ///
    /// See [`WeakComponentLink`] documentation for more information
    #[prop_or_default]
    pub select_link: WeakComponentLink<MatSelect>,
    /// Binds to `opened` event on `mwc-select-surface`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onopened: Callback<()>,
    /// Binds to `closed` event on `mwc-select-surface`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onclosed: Callback<()>,
    /// Binds to `action` event on `mwc-list`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onaction: Callback<ActionDetail>,
    /// Binds to `selected` event on `mwc-list`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onselected: Callback<SelectedDetail>,
}

impl Component for MatSelect {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        props.select_link.borrow_mut().replace(link);
        Select::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), validity_transform_closure: None, opened_closure: None, closed_closure: None, action_closure: None, selected_closure: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <mwc-select
                value?=to_option_string(&self.props.value)
                label?=to_option_string(&self.props.label)
                naturalMenuWidth?=to_option(self.props.natural_menu_width)
                icon?=to_option_string(&self.props.icon)
                disabled=self.props.disabled
                outlined?=to_option(self.props.outlined)
                helper?=to_option_string(&self.props.helper)
                required=self.props.required
                validationMessage?=to_option_string(&self.props.validation_message)
                items?=to_option_string(&self.props.items)
                index=self.props.index
                validateOnInitialRender?=to_option(self.props.validate_on_initial_render)
                ref=self.node_ref.clone()
            >
              { self.props.children.clone() }
            </mwc-select>
        }
    }

    //noinspection DuplicatedCode
    fn rendered(&mut self, first_render: bool) {
        if first_render {

            let this = self.node_ref.cast::<Select>().unwrap();
            if let Some(transform) = self.props.validity_transform.clone() {
                self.validity_transform_closure = Some(Closure::wrap(Box::new(move |s: String, v: NativeValidityState| -> ValidityStateJS {
                    transform.0(s, v).into()
                }) as Box<dyn Fn(String, NativeValidityState) -> ValidityStateJS>));
                this.set_validity_transform(&self.validity_transform_closure.as_ref().unwrap());
            }

            let onopened = self.props.onopened.clone();
            add_event_listener(&self.node_ref, "opened", move || { onopened.emit(()) }, &mut self.opened_closure);

            let onclosed = self.props.onclosed.clone();
            add_event_listener(&self.node_ref, "closed", move || { onclosed.emit(()) }, &mut self.closed_closure);

            let on_action = self.props.onaction.clone();
            add_event_listener_with_one_param(&self.node_ref, "action", move |value| {
                let event = value.unchecked_into::<CustomEvent>();
                let details = ActionDetail::from(event.detail());

                on_action.emit(details)
            }, &mut self.action_closure);

            let on_selected = self.props.onselected.clone();
            add_event_listener_with_one_param(&self.node_ref, "selected", move |value| {
                let event = value.unchecked_into::<web_sys::CustomEvent>();
                on_selected.emit(SelectedDetail::from(event.detail()))
            }, &mut self.selected_closure);
        }
    }
}

impl WeakComponentLink<MatSelect> {
    pub fn select(&self, val: usize) {
            let c = (*self.borrow()
                .as_ref()
                .unwrap()
                .get_component()
                .unwrap())
                .node_ref
                .clone();
        let select_element = c.cast::<Select>().unwrap();
        select_element.select(val);
    }
}

impl MatSelect {
    /// Returns [`ValidityTransform`] to be passed to `validity_transform` prop
    pub fn validity_transform<F: Fn(String, NativeValidityState) -> ValidityState + 'static>(func: F) -> ValidityTransform {
        ValidityTransform::new(func)
    }
}
