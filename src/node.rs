use crate::input::Input;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Position {
  pub offset: usize,
  pub column: usize,
  pub line: usize,
}

impl Position {
  pub fn new(offset: usize, line: usize, column: usize) -> Self {
    Self {
      offset,
      column,
      line,
    }
  }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Source<'a> {
  #[serde(skip_serializing, skip_deserializing)]
  pub input: Rc<RefCell<Input<'a>>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub start: Option<Position>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub end: Option<Position>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RawValue {
  pub value: String,
  pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Node<'a> {
  Root(Root<'a>),
  AtRule(AtRule<'a>),
  Rule(Rule<'a>),
  Decl(Declaration<'a>),
  Comment(Comment<'a>),
  Document(Document<'a>),
}

impl<'a> Node<'a> {
  pub fn set_source(
    &mut self,
    input: Rc<RefCell<Input<'a>>>,
    start: Option<Position>,
    end: Option<Position>,
  ) {
    match self {
      Node::Root(root) => {
        root.source = Some(Source { input, start, end });
      }
      Node::AtRule(at) => {
        at.source = Some(Source { input, start, end });
      }
      Node::Rule(rule) => {
        rule.source = Some(Source { input, start, end });
      }
      Node::Decl(decl) => {
        decl.source = Some(Source { input, start, end });
      }
      Node::Comment(comment) => {
        comment.source = Some(Source { input, start, end });
      }
      Node::Document(doc) => {
        doc.source = Some(Source { input, start, end });
      }
    }
  }

  pub fn set_source_end(&mut self, end: Option<Position>) {
    match self {
      Node::Root(root) => {
        if let Some(source) = root.source.as_mut() {
          source.end = end;
        };
      }
      Node::AtRule(at) => {
        if let Some(source) = at.source.as_mut() {
          source.end = end;
        };
      }
      Node::Rule(rule) => {
        if let Some(source) = rule.source.as_mut() {
          source.end = end;
        };
      }
      Node::Decl(decl) => {
        if let Some(source) = decl.source.as_mut() {
          source.end = end;
        };
      }
      Node::Comment(comment) => {
        if let Some(source) = comment.source.as_mut() {
          source.end = end;
        };
      }
      Node::Document(doc) => {
        if let Some(source) = doc.source.as_mut() {
          source.end = end;
        };
      }
    }
  }

  pub fn set_raw_before(&mut self, before: String) {
    match self {
      Node::AtRule(at) => {
        at.raws.before = Some(before);
      }
      Node::Rule(rule) => {
        rule.raws.before = Some(before);
      }
      Node::Decl(decl) => {
        decl.raws.before = Some(before);
      }
      Node::Comment(comment) => {
        comment.raws.before = Some(before);
      }
      _ => {
        // root, document raw don't have before
        unimplemented!() // TODO
      }
    }
  }

  pub fn push_child(&mut self, node: Rc<RefCell<Node<'a>>>) {
    match self {
      Node::Root(root) => {
        if let Some(children) = root.nodes.as_mut() {
          children.push(node)
        }
      }
      Node::AtRule(at) => {
        if let Some(children) = at.nodes.as_mut() {
          children.push(node)
        }
      }
      Node::Rule(rule) => {
        if let Some(children) = rule.nodes.as_mut() {
          children.push(node)
        }
      }
      Node::Decl(decl) => {
        if let Some(children) = decl.nodes.as_mut() {
          children.push(node)
        }
      }
      Node::Comment(comment) => {
        if let Some(children) = comment.nodes.as_mut() {
          children.push(node)
        }
      }
      Node::Document(doc) => {
        if let Some(children) = doc.nodes.as_mut() {
          children.push(node)
        }
      }
    }
  }

  pub fn as_root(&self) -> Option<&Root<'a>> {
    if let Self::Root(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_at_rule(&self) -> Option<&AtRule<'a>> {
    if let Self::AtRule(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_rule(&self) -> Option<&Rule<'a>> {
    if let Self::Rule(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_decl(&self) -> Option<&Declaration<'a>> {
    if let Self::Decl(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_comment(&self) -> Option<&Comment<'a>> {
    if let Self::Comment(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_document(&self) -> Option<&Document<'a>> {
    if let Self::Document(v) = self {
      Some(v)
    } else {
      None
    }
  }
  pub fn as_root_mut(&self) -> Option<&Root<'a>> {
    if let Self::Root(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_at_rule_mut(&mut self) -> Option<&mut AtRule<'a>> {
    if let Self::AtRule(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_rule_mut(&mut self) -> Option<&mut Rule<'a>> {
    if let Self::Rule(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_decl_mut(&mut self) -> Option<&mut Declaration<'a>> {
    if let Self::Decl(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_comment_mut(&mut self) -> Option<&mut Comment<'a>> {
    if let Self::Comment(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_document_mut(&mut self) -> Option<&mut Document<'a>> {
    if let Self::Document(v) = self {
      Some(v)
    } else {
      None
    }
  }

  /// Returns `true` if the node is [`Root`].
  ///
  /// [`Root`]: Node::Root
  pub fn is_root(&self) -> bool {
    matches!(self, Self::Root(..))
  }

  /// Returns `true` if the node is [`AtRule`].
  ///
  /// [`AtRule`]: Node::AtRule
  pub fn is_at_rule(&self) -> bool {
    matches!(self, Self::AtRule(..))
  }

  /// Returns `true` if the node is [`Rule`].
  ///
  /// [`Rule`]: Node::Rule
  pub fn is_rule(&self) -> bool {
    matches!(self, Self::Rule(..))
  }

  /// Returns `true` if the node is [`Decl`].
  ///
  /// [`Decl`]: Node::Decl
  pub fn is_decl(&self) -> bool {
    matches!(self, Self::Decl(..))
  }

  /// Returns `true` if the node is [`Comment`].
  ///
  /// [`Comment`]: Node::Comment
  pub fn is_comment(&self) -> bool {
    matches!(self, Self::Comment(..))
  }

  /// Returns `true` if the node is [`Document`].
  ///
  /// [`Document`]: Node::Document
  pub fn is_document(&self) -> bool {
    matches!(self, Self::Document(..))
  }
}

fn is_false(boolean: &bool) -> bool {
  !*boolean
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Declaration<'a> {
  /// The declaration's property name.
  pub prop: String,

  /// The declaration’s value.
  ///
  /// This value will be cleaned of comments. If the source value contained
  /// comments, those comments will be available in the `raws` property.
  /// If you have not changed the value, the result of `decl.toString()`
  /// will include the original raws value (comments and all).
  pub value: String,

  /// `true` if the declaration has an `!important` annotation.
  #[serde(default)]
  #[serde(skip_serializing_if = "is_false")]
  pub important: bool,

  /// `true` if declaration is declaration of CSS Custom Property
  /// or Sass variable.
  #[serde(default)]
  #[serde(skip_serializing_if = "is_false")]
  pub variable: bool,

  /// An array containing the node’s children.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nodes: Option<Vec<Rc<RefCell<Node<'a>>>>>,

  /// The node’s parent node.
  #[serde(skip_serializing, skip_deserializing)]
  pub parent: Option<Weak<Node<'a>>>,

  /// The input source of the node.
  /// The property is used in source map generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<Source<'a>>,

  pub raws: DeclarationRaws,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Rule<'a> {
  /// Selector or selectors of the rule.
  pub selector: String,

  ///  Selectors of the rule represented as an array of strings.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub selectors: Option<Vec<String>>,
  /// An array containing the node’s children.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nodes: Option<Vec<Rc<RefCell<Node<'a>>>>>,

  /// The node’s parent node.
  #[serde(skip_serializing, skip_deserializing)]
  pub parent: Option<Weak<Node<'a>>>,

  /// The input source of the node.
  /// The property is used in source map generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<Source<'a>>,

  pub raws: RuleRaws,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtRule<'a> {
  /// An array containing the node’s children.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nodes: Option<Vec<Rc<RefCell<Node<'a>>>>>,

  /// The node’s parent node.
  #[serde(skip_serializing, skip_deserializing)]
  pub parent: Option<Weak<Node<'a>>>,

  /// The input source of the node.
  /// The property is used in source map generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<Source<'a>>,

  /// used in `atrule` or `document`.
  /// The at-rule's name immediately follows the `@`.
  /// Or the document's name.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,

  /// used in `atrule`.
  /// The at-rule’s parameters, the values that follow the at-rule’s name
  /// but precede any {} block.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub params: Option<String>,

  pub raws: AtRuleRaws,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Comment<'a> {
  /// An array containing the node’s children.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nodes: Option<Vec<Rc<RefCell<Node<'a>>>>>,

  /// The node’s parent node.
  #[serde(skip_serializing, skip_deserializing)]
  pub parent: Option<Weak<Node<'a>>>,

  /// The input source of the node.
  /// The property is used in source map generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<Source<'a>>,

  /// used in `comment`.
  /// The comment's text.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub text: Option<String>,

  pub raws: CommentRaws,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document<'a> {
  /// An array containing the node’s children.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nodes: Option<Vec<Rc<RefCell<Node<'a>>>>>,

  /// The node’s parent node.
  #[serde(skip_serializing, skip_deserializing)]
  pub parent: Option<Weak<Node<'a>>>,

  /// The input source of the node.
  /// The property is used in source map generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<Source<'a>>,

  /// used in `atrule` or `document`.
  /// The at-rule's name immediately follows the `@`.
  /// Or the document's name.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  // document node have no raws
  // pub raws: Document
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root<'a> {
  /// An array containing the node’s children.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nodes: Option<Vec<Rc<RefCell<Node<'a>>>>>,

  /// The node’s parent node.
  #[serde(skip_serializing, skip_deserializing)]
  pub parent: Option<Weak<Node<'a>>>,

  /// The input source of the node.
  /// The property is used in source map generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<Source<'a>>,

  pub raws: RootRaws,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootRaws {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub code_before: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub code_after: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub semicolon: Option<bool>,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtRuleRaws {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub before: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub between: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub semicolon: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub params: Option<RawValue>,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentRaws {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub before: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub left: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub right: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeclarationRaws {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub before: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub between: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub important: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<RawValue>,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleRaws {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub before: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub between: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub semicolon: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub own_semicolon: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub selector: Option<RawValue>,
}
