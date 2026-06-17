use std::path::PathBuf;

use anyhow::{Context, Result};
use tokenizers::Tokenizer;

const MODEL_DIM: usize = 384;
const MAX_SEQ_LEN: usize = 256;

pub struct EmbeddingModel {
    session: ort::session::Session,
    tokenizer: Tokenizer,
}

impl EmbeddingModel {
    pub fn new(model_path: &std::path::Path) -> Result<Self> {
        let model_file = model_path.join("model.onnx");
        let tokenizer_file = model_path.join("tokenizer.json");

        if !model_file.exists() {
            download_model(&model_file, &tokenizer_file)?;
        }

        let session = ort::session::Session::builder()?.commit_from_file(&model_file)?;

        let tokenizer = Tokenizer::from_file(&tokenizer_file)
            .map_err(|e| anyhow::anyhow!("failed to load tokenizer: {}", e))?;

        Ok(Self { session, tokenizer })
    }

    pub fn embed(&mut self, text: &str) -> Result<Vec<f32>> {
        let encoding = self
            .tokenizer
            .encode(text, true)
            .map_err(|e| anyhow::anyhow!("tokenization failed: {}", e))?;

        let input_ids = encoding.get_ids();
        let attention_mask = encoding.get_attention_mask();
        let token_type_ids = encoding.get_type_ids();

        let seq_len = input_ids.len().min(MAX_SEQ_LEN);

        let mut padded_ids = vec![0i64; MAX_SEQ_LEN];
        let mut padded_mask = vec![0i64; MAX_SEQ_LEN];
        let mut padded_types = vec![0i64; MAX_SEQ_LEN];

        for i in 0..seq_len {
            padded_ids[i] = input_ids[i] as i64;
            padded_mask[i] = attention_mask[i] as i64;
            padded_types[i] = token_type_ids[i] as i64;
        }

        let ids_tensor = ort::value::Tensor::<i64>::from_array((
            [1_usize, MAX_SEQ_LEN],
            padded_ids.into_boxed_slice(),
        ))?;

        let mask_tensor = ort::value::Tensor::<i64>::from_array((
            [1_usize, MAX_SEQ_LEN],
            padded_mask.into_boxed_slice(),
        ))?;

        let types_tensor = ort::value::Tensor::<i64>::from_array((
            [1_usize, MAX_SEQ_LEN],
            padded_types.into_boxed_slice(),
        ))?;

        let outputs = self
            .session
            .run(ort::inputs![ids_tensor, mask_tensor, types_tensor])?;

        let output_value = outputs
            .get("last_hidden_state")
            .unwrap_or_else(|| &outputs[0]);

        let (shape, data) = output_value
            .try_extract_tensor::<f32>()
            .context("failed to extract embedding tensor")?;

        let actual_seq_len = shape.as_ref()[1].min(seq_len as i64) as usize;

        let mut embedding = vec![0.0_f32; MODEL_DIM];
        let mut count = 0;

        for i in 0..actual_seq_len {
            if attention_mask[i] != 0 {
                for j in 0..MODEL_DIM {
                    embedding[j] += data[i * MODEL_DIM + j];
                }
                count += 1;
            }
        }

        if count > 0 {
            for val in embedding.iter_mut() {
                *val /= count as f32;
            }
        }

        normalize(&mut embedding);

        Ok(embedding)
    }

    pub fn dimension() -> usize {
        MODEL_DIM
    }

    pub fn models_dir() -> Result<PathBuf> {
        let cache = dirs::cache_dir()
            .context("no cache directory")?
            .join("lightwisper")
            .join("embedding-models");
        std::fs::create_dir_all(&cache)?;
        Ok(cache)
    }

    pub fn is_available(model_path: &std::path::Path) -> bool {
        model_path.join("model.onnx").exists()
    }
}

fn normalize(v: &mut [f32]) {
    let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for val in v.iter_mut() {
            *val /= norm;
        }
    }
}

fn download_model(model_path: &std::path::Path, tokenizer_path: &std::path::Path) -> Result<()> {
    let base = "https://huggingface.co/Xenova/all-MiniLM-L6-v2/resolve/main/onnx";

    tracing::info!("downloading MiniLM embedding model");

    let model_url = format!("{}/model_quantized.onnx", base);
    let tokenizer_url = format!("{}/tokenizer.json", base);

    let model_resp =
        reqwest::blocking::get(&model_url).context("failed to download embedding model")?;
    let model_bytes = model_resp.bytes().context("failed to read model bytes")?;
    std::fs::write(model_path, &model_bytes).context("failed to write embedding model")?;

    let tok_resp =
        reqwest::blocking::get(&tokenizer_url).context("failed to download tokenizer")?;
    let tok_bytes = tok_resp.bytes().context("failed to read tokenizer bytes")?;
    std::fs::write(tokenizer_path, &tok_bytes).context("failed to write tokenizer")?;

    tracing::info!("MiniLM model downloaded to {:?}", model_path);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        let mut v = vec![3.0, 4.0];
        normalize(&mut v);
        assert!((v[0] - 0.6).abs() < 1e-6);
        assert!((v[1] - 0.8).abs() < 1e-6);
    }

    #[test]
    fn test_normalize_zero_vector() {
        let mut v = vec![0.0, 0.0, 0.0];
        normalize(&mut v);
        assert_eq!(v, vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_model_creation_fails_without_model() {
        let path = std::path::Path::new("/nonexistent/path");
        let result = EmbeddingModel::new(path);
        assert!(result.is_err());
    }
}
