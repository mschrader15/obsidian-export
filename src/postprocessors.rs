//! A collection of officially maintained [postprocessors][crate::Postprocessor].

use crate::Exporter;

use super::{Context, MarkdownEvents, PostprocessorResult};
use pulldown_cmark::Event;
use serde_yaml::Value;

/// This postprocessor converts all soft line breaks to hard line breaks. Enabling this mimics
/// Obsidian's _'Strict line breaks'_ setting.
pub fn softbreaks_to_hardbreaks(
    _context: &mut Context,
    events: &mut MarkdownEvents,
    _exporter: & Exporter,
) -> PostprocessorResult {

    for event in events.iter_mut(){
        *event = if let Event::SoftBreak = event {
            Event::HardBreak
        } else {
            // event
            event.clone()
        }
    };

    PostprocessorResult::Continue
}

// pub async fn some_async_func(arg: &str) {}


/// This postprocessor scans the YAML frontmatter for the desired inclusion tag. If it is found, Postprocessing continues
/// Otherwise, the note should be skipped
pub fn yaml_includer(
    context: &mut Context,
    _events: &mut MarkdownEvents,
    exporter: & Exporter,
) -> PostprocessorResult {
    
    match context.frontmatter.get(&exporter.yaml_inclusion_key) {
        Some(Value::Bool(true)) => return PostprocessorResult::Continue,
        _ => return PostprocessorResult::StopAndSkipNote
    };
    
    // (context, events, res)
}

// pub fn yaml_includer_factory(key: &str) 
//     -> Box<dyn Fn(Context, MarkdownEvents) -> (Context, MarkdownEvents, PostprocessorResult) + Send + Sync>
// {
//     let key = Value::String(key.to_string());
    
//     Box::new(move |context: Context, events: MarkdownEvents| yaml_includer(context, events, key.clone()))
//     // (move |context: Context, events: MarkdownEvents| yaml_includer(context, events, key.clone()))
// }


// pub struct YamlIncluder {
//     key: Value,
// }

// impl<'a> YamlIncluder {
    
//     pub fn new(key: String) -> YamlIncluder {
//         YamlIncluder {
//             key: Value::String(key.to_string())
//         }
//     }

//     pub fn process(
//         self,
//         context: Context,
//         events: MarkdownEvents,
//     ) -> (Context, MarkdownEvents, PostprocessorResult) {
        
//         match context.frontmatter.get(&self.key) {
//             Some(Value::Bool(true)) => return (context, events, PostprocessorResult::Continue),
//             _ => return (context, events, PostprocessorResult::StopAndSkipNote),
//         };
        
//     }
//   }