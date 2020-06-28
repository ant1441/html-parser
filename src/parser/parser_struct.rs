use std::{cell::RefCell, fmt, io::prelude::*, rc::Rc, str};

use log::{debug, trace};

use crate::{
    dom::{self, Document},
    parser::{
        states::States, FramesetOkFlag, ListOfActiveFormattingElements, OpenElementsStack,
        ScriptingFlag, TransitionResult,
    },
    tokenizer::{TagName, Token, Tokenizer},
};

pub struct Parser<R>
where
    R: Read + Seek,
{
    pub document: Document,

    tokenizer: Tokenizer<R>,

    insertion_mode: Option<States>,
    reprocess: bool,
    last_token: Option<Token>,

    pub(super) open_elements: OpenElementsStack,
    pub(super) list_of_active_formatting_elements: ListOfActiveFormattingElements,

    // Element pointsers
    head_element_pointer: Option<Rc<RefCell<dom::Element>>>,

    // Other Parsing state flags
    pub(super) scripting: ScriptingFlag,
    pub(super) frameset_ok: FramesetOkFlag,
}

impl<R> Parser<R>
where
    R: Read + Seek,
{
    pub fn new(r: R) -> Self {
        let document = Document::default();
        let tokenizer = Tokenizer::new(r, false);

        Parser {
            document,
            tokenizer,

            insertion_mode: Some(States::new()),
            reprocess: false,
            last_token: None,

            open_elements: OpenElementsStack::new(),
            list_of_active_formatting_elements: ListOfActiveFormattingElements::new(),

            head_element_pointer: None,

            scripting: ScriptingFlag::Disabled,
            frameset_ok: FramesetOkFlag::Ok,
        }
    }

    pub fn run(&mut self) {
        loop {
            let insertion_mode = self.insertion_mode.take().unwrap();

            let num_open_elements = self.open_elements.len();
            let num_active_formatting_elements = self.list_of_active_formatting_elements.len();
            debug!(
                target: "html_parser::parser",
                "State ({}) [Open Elements: {:3}, Active Formatting Elements: {:3}]: {:?}",
                if self.reprocess { "R" } else { "-" },
                num_open_elements,
                num_active_formatting_elements,
                insertion_mode
            );
            let res = match insertion_mode {
                States::Term(_) => return,
                _ => {
                    let token = if !self.reprocess {
                        self.tokenizer.next().unwrap()
                    } else {
                        self.last_token.take().unwrap()
                    };

                    // tree construction dispatcher
                    if self.is_tree_construction_first_case(&token) {
                        trace!(target: "html_parser::parser", "Received token {:?}", token);
                        let ret = insertion_mode.on_token(self, &token);
                        self.last_token = Some(token);
                        ret
                    } else {
                        todo!("Parser: Process the token according to the rules given in the section for parsing tokens in foreign content.");
                    }
                }
            };

            if res.is_err() {
                let next_state_error = res.state().unwrap_err();
                // TODO return err?
                panic!("Parser error: {}", next_state_error);
            }

            trace!(target: "html_parser::parser", "Document: {:#?}", self.document);
            self.reprocess = res.reprocess();
            self.insertion_mode = Some(res.state().unwrap());
        }
    }

    pub(super) fn set_head(&mut self, head_elem: Rc<RefCell<dom::Element>>) {
        self.head_element_pointer = Some(head_elem);
    }

    pub(super) fn insert_html_element(&mut self, elem: Rc<RefCell<dom::Element>>) {
        let (target, pos) = self.appropriate_place_for_inserting_a_node(None).unwrap();
        // TODO: If it is possible to insert element at the adjusted insertion location
        // TODO: custom element stuff
        let mut target = target.borrow_mut();
        target.insert(pos, elem.clone().into());
        self.open_elements.push(elem);
    }

    pub(super) fn insert_character<C: AsRef<str>>(&mut self, data: C) {
        let (target, pos) = self.appropriate_place_for_inserting_a_node(None).unwrap();
        let mut target = target.borrow_mut();
        if pos > 0 {
            if let Some(dom::ElementChildNode::Text(text)) = target.get_mut(pos - 1) {
                trace!(target: "html_parser::parser", "Appending char at position {}", pos - 1);
                let mut text = text.borrow_mut();
                return text.push_str(data.as_ref());
            }
        }
        let node = dom::Text::new(data.as_ref().to_string());
        trace!(target: "html_parser::parser", "Inserting char {:?} at position {}", node, pos);
        target.insert(pos, node.into());
    }

    pub(super) fn generic_raw_text_element_parse(
        &mut self,
        current_state: States,
        token: &Token,
    ) -> TransitionResult {
        let node = dom::Element::new_html(token.tag_name().unwrap().clone());
        self.insert_html_element(node);

        self.tokenizer.switch_to_rawtext_state();

        States::text(Box::new(current_state)).into_transition_result()
    }

    pub(super) fn generic_rcdata_element_parse(
        &mut self,
        current_state: States,
        token: &Token,
    ) -> TransitionResult {
        let node = dom::Element::new_html(token.tag_name().unwrap().clone());
        self.insert_html_element(node);

        self.tokenizer.switch_to_rcdata_state();

        States::text(Box::new(current_state)).into_transition_result()
    }

    pub(super) fn generate_implied_end_tags(&mut self, except: Option<&TagName>) {
        while let Some(node) = self.current_node() {
            let node = node.borrow();
            let name = node.name();
            if !matches!(
                name,
                TagName::Dd
                    | TagName::Dt
                    | TagName::Li
                    | TagName::Optgroup
                    | TagName::Option
                    | TagName::P
                    | TagName::Rb
                    | TagName::Rp
                    | TagName::Rt
                    | TagName::Rtc
            ) || Some(name) == except
            {
                break;
            }
            trace!(target: "html_parser::parser", "generate_implied_end_tags: Popping {} off stack", name);
            let _ = self.open_elements.pop();
        }
    }

    // Returning the parent element and the index to insert at
    //
    // ie. You cancall this then call `ret.0.insert(ret.1, new_elem)`
    fn appropriate_place_for_inserting_a_node(
        &mut self,
        r#override: Option<()>,
    ) -> Option<(Rc<RefCell<dom::Element>>, usize)> {
        if r#override.is_some() {
            todo!("Parser::appropriate_place_for_inserting_a_node with override")
        }
        let target = self.current_node()?;

        // TODO: foster parenting
        if target.borrow().name() == &TagName::Template {
            todo!("Parser::appropriate_place_for_inserting_a_node in template")
        }
        let pos = target.borrow().len();

        Some((target, pos))
    }

    // https://html.spec.whatwg.org/multipage/parsing.html#tree-construction
    fn is_tree_construction_first_case(&self, token: &Token) -> bool {
        // If the stack of open elements is empty
        if self.open_elements.is_empty() {
            return true;
        }

        let adjusted_current_node = self.adjusted_current_node().expect(
            "No adjusted_current_node when there are elements on the stack of open elements",
        );
        let node = adjusted_current_node.borrow();
        let is_token_start_tag = token.is_start_tag();
        let is_token_character = token.is_start_tag();
        let is_token_eof = token.is_eof();
        let token_tag_name = token.tag_name();

        // If the adjusted current node is an element in the HTML namespace
        if node.namespace() == dom::Namespace::HTML {
            return true;
        }

        // If the adjusted current node is a MathML text integration point and
        // the token is a start tag whose tag name is neither "mglyph" nor "malignmark"
        if node.is_mathml_text_integration_point()
            && is_token_start_tag
            && (token_tag_name != Some(&TagName::Mglyph)
                && token_tag_name != Some(&TagName::Malignmark))
        {
            return true;
        }

        // If the adjusted current node is a MathML text integration point and
        // the token is a character token
        if node.is_mathml_text_integration_point() && is_token_character {
            return true;
        }

        // If the adjusted current node is a MathML annotation-xml element and
        // the token is a start tag whose tag name is "svg"
        if node.name() == &TagName::AnnotationXml
            && is_token_start_tag
            && token_tag_name == Some(&TagName::Svg)
        {
            return true;
        }

        // If the adjusted current node is an HTML integration point and
        // the token is a start tag
        if node.is_html_integration_point() && is_token_start_tag {
            return true;
        }

        // If the adjusted current node is an HTML integration point and
        // the token is a character token
        if node.is_html_integration_point() && is_token_character {
            return true;
        }

        // If the token is an end-of-file token
        if is_token_eof {
            return true;
        }

        false
    }

    pub fn adjusted_current_node(&self) -> Option<Rc<RefCell<dom::Element>>> {
        if
        /* self.parsing_algorithm == HTMLFragmentParsing && self.open_elements.len() == 1 */
        false {
            todo!("Parser::adjusted_current_node with HTMLFragmentParsing");
        } else {
            self.current_node()
        }
    }

    pub fn current_node(&self) -> Option<Rc<RefCell<dom::Element>>> {
        self.open_elements.last().map(|e| Rc::clone(e))
    }
}

impl<R> fmt::Debug for Parser<R>
where
    R: Read + Seek + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Parser")
            .field("document", &self.document)
            .field("tokenizer", &self.tokenizer)
            .field("insertion_mode", &self.insertion_mode)
            .field("reprocess", &self.reprocess)
            .field("last_token", &self.last_token)
            .field("open_elements", &self.open_elements)
            .field(
                "list_of_active_formatting_elements",
                &self.list_of_active_formatting_elements,
            )
            .field("head_element_pointer", &self.head_element_pointer)
            .field("scripting", &self.scripting)
            .field("frameset_ok", &self.frameset_ok)
            .finish()
    }
}
