use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use serde::Serialize;
use std::cmp::Ordering;

#[derive(Serialize)]
struct RootFrequency {
    root: String,
    frequency: usize,
}

/// Reads the roots file efficiently and sorts them by length (descending) and alphabetically
fn read_roots(path: impl AsRef<Path>) -> Result<Vec<String>> {
    let file = File::open(&path)
        .with_context(|| format!("Failed to open roots file: {}", path.as_ref().display()))?;
    let reader = BufReader::new(file);
    let mut roots: Vec<String> = reader
        .lines()
        .filter_map(Result::ok)
        .collect();

    roots.sort_unstable_by(|a, b| {
        match b.len().cmp(&a.len()) {
            Ordering::Equal => a.cmp(b),
            other => other,
        }
    });

    Ok(roots)
}

/// Reads the corpus file and splits words by whitespace
fn read_corpus(path: impl AsRef<Path>) -> Result<Vec<String>> {
    println!("Reading corpus file...");
    let file = File::open(&path)
        .with_context(|| format!("Failed to open corpus file: {}", path.as_ref().display()))?;
    let reader = BufReader::new(file);
    
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    
    let mut word_count = 0;
    let corpus: Vec<String> = reader
        .lines()
        .filter_map(Result::ok)
        .flat_map(|line| {
            let words = line.split_whitespace().map(String::from).collect::<Vec<_>>();
            word_count += words.len();
            pb.set_message(format!("Read {} words", word_count));
            words
        })
        .collect();

    pb.finish_with_message(format!("Loaded {} words from corpus", word_count));
    Ok(corpus)
}

/// Computes the frequency of each root in the corpus
fn count_root_frequencies(roots: &[String], corpus: &[String]) -> HashMap<String, usize> {
    // Initialize frequencies HashMap
    let mut frequencies: HashMap<String, usize> = roots.iter()
        .map(|root| (root.clone(), 0))
        .collect();

    // Create progress bar
    let pb = ProgressBar::new(corpus.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>9}/{len:9} ({percent:>3}%) {msg}")
            .unwrap()
            .progress_chars("##-"),
    );
    pb.set_message("Processing words...");
    pb.enable_steady_tick(Duration::from_millis(100));

    // Create atomic counter for progress
    let processed_count = Arc::new(AtomicUsize::new(0));

    // Process corpus in parallel
    let chunk_size = 100_000;
    let chunks: Vec<_> = corpus.chunks(chunk_size).collect();
    
    println!("Processing {} chunks of approximately {} words each", chunks.len(), chunk_size);
    
    let chunk_results: Vec<HashMap<String, usize>> = chunks.par_iter().map(|chunk| {
        let mut local_freq = HashMap::new();
        
        for word in *chunk {
            for root in roots {
                if word.starts_with(root) {
                    *local_freq.entry(root.clone()).or_insert(0) += 1;
                    break;
                }
            }
        }
        
        // Update progress
        let done = processed_count.fetch_add(chunk.len(), AtomicOrdering::Relaxed);
        pb.set_position(done as u64);
        
        local_freq
    }).collect();

    // Merge results
    pb.set_message("Merging results...");
    for chunk_freq in chunk_results {
        for (root, count) in chunk_freq {
            *frequencies.get_mut(&root).unwrap() += count;
        }
    }

    pb.finish_with_message(format!("Processed {} words", corpus.len()));
    frequencies
}

/// Writes the frequency map to a JSON file
fn write_json_output(frequencies: HashMap<String, usize>, output_path: impl AsRef<Path>) -> Result<()> {
    println!("Sorting results by frequency...");
    let mut freq_vec: Vec<RootFrequency> = frequencies
        .into_iter()
        .map(|(root, frequency)| RootFrequency { root, frequency })
        .collect();

    freq_vec.sort_unstable_by(|a, b| {
        match b.frequency.cmp(&a.frequency) {
            Ordering::Equal => a.root.cmp(&b.root),
            other => other,
        }
    });

    println!("Writing results to JSON file...");
    let json = serde_json::to_string_pretty(&freq_vec)?;
    fs::write(&output_path, json)
        .with_context(|| format!("Failed to write output to {}", output_path.as_ref().display()))?;

    println!("Results written successfully!");
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        return Err(anyhow!("Usage: {} <roots_file> <corpus_file> <output_file>", args[0]));
    }

    let roots_file = &args[1];
    let corpus_file = &args[2];
    let output_file = &args[3];

    println!("Reading roots file...");
    let roots = read_roots(roots_file)?;
    println!("Loaded {} roots", roots.len());

    let corpus = read_corpus(corpus_file)?;

    println!("Starting frequency calculation...");
    let frequencies = count_root_frequencies(&roots, &corpus);

    write_json_output(frequencies, output_file)?;

    println!("Process completed successfully!");
    Ok(())
}