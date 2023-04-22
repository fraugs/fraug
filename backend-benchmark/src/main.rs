extern crate anyhow;

use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::translation::{Language, TranslationModelBuilder};

fn main() -> anyhow::Result<()> {
    let fwd_scr = Language::French; // source language
    let fwd_trg = Language::English; // target language
    let rev_scr = Language::English; // source language
    let rev_trg = Language::French; // target language

    let text = "Le pongiste français Alexis Lebrun, 19 ans, a réalisé un exploit ce vendredi 21 avril en battant le Chinois Fan Zhendong, numéro 1 mondial, en quarts de finale du tournoi WTT Champions de Macao, en Chine."; // original text

    let forward_translator = TranslationModelBuilder::new()
        .with_model_type(ModelType::Marian)
        .with_source_languages(vec![fwd_scr])
        .with_target_languages(vec![fwd_trg])
        .create_model()?;

    let reverse_translator = TranslationModelBuilder::new()
        .with_model_type(ModelType::Marian)
        .with_source_languages(vec![rev_scr])
        .with_target_languages(vec![rev_trg])
        .create_model()?;

    let forward_translation_output = forward_translator.translate(&[text], None, None)?; // translate to target language
    let reverse_translation_output = reverse_translator.translate(&[&forward_translation_output[0]], None, None)?; // translate back to source language

    println!("{}", text);
    println!("{}", &reverse_translation_output[0]);

    Ok(())
}
