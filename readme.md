# Turkish Word Root Analysis Project

A comprehensive project for analyzing Turkish word roots using Wikipedia articles, TDK dictionary data, and large language models (Gemma/Gemini). The project aims to identify and validate Turkish word roots through various methods.

## Project Overview

This project combines multiple approaches to analyze Turkish word roots:
1. Wikipedia corpus analysis
2. Turkish dictionary (TDK) integration
3. Large Language Model assistance
4. Suffix stripping algorithms

## Components

### 1. Wikipedia Analysis (`wikipedia_kok.ipynb`)
- Loads Turkish Wikipedia dataset (~535K articles)
- Implements suffix stripping algorithm
- Processes text in parallel for efficiency
- Generates root candidates from Wikipedia corpus

### 2. Root Selection with Gemma (`gemma_kokleri_sec.ipynb`)
- Uses Gemma 2.0 (27B) model for root validation
- Implements comprehensive Turkish suffix list (249 suffixes)
- Processes words in chunks for efficient analysis
- Filters non-Turkish words and proper nouns

### 3. Root Correction with Gemini (`gemini_kokleri_duzelt.ipynb`)
- Uses Google's Gemini model for root verification
- Processes and validates root candidates
- Combines results with existing root dictionary
- Saves validated roots to JSON format

### 4. Dictionary Integration (`sozluk/sozluk.ipynb`)
- Integrates TDK dictionary database
- Provides word meanings and etymology
- Contains 8 related tables:
  - madde (words)
  - anlam (meanings)
  - ornek (examples)
  - ozellik (properties)
  - etc.

## Data Files

### Large Files
Due to GitHub's file size limitations, large files have been split into smaller chunks:

#### Dictionary Data
The TDK dictionary data (`sozluk/gts.json`, ~104 MB) is split into three parts:
```
sozluk/gts.json.part_aa
sozluk/gts.json.part_ab
sozluk/gts.json.part_ac
```

To merge these files into the original:

```bash
# On Unix-like systems (Linux/MacOS):
cat sozluk/gts.json.part_* > sozluk/gts.json

# On Windows (PowerShell):
Get-Content sozluk/gts.json.part_* | Set-Content sozluk/gts.json

# On Windows (Command Prompt):
copy /b sozluk\gts.json.part_* sozluk\gts.json
```

For contributors who need to split large files:
```bash
# On Unix-like systems:
split -b 50M large_file.json large_file.json.part_

# On Windows (PowerShell):
$file = [IO.File]::ReadAllBytes("large_file.json")
$size = 50MB
$parts = [Math]::Ceiling($file.Length / $size)
for($i=0; $i -lt $parts; $i++) {
    $start = $i * $size
    $chunk = $file[$start..([Math]::Min($start + $size - 1, $file.Length - 1))]
    [IO.File]::WriteAllBytes("large_file.json.part_$i", $chunk)
}
```

### Regular Files
- `kokler.txt`: Base dictionary of known Turkish roots
- `all_roots.json`: Combined validated roots
- `sorted_remains_words_freq.json`: Frequency analysis of remaining words

## Technical Details

### Suffix List
Contains 249 Turkish suffixes including:
- Case endings
- Possessive markers
- Verb tenses
- Derivational suffixes

### Processing Pipeline
1. Text extraction from Wikipedia
2. Suffix stripping
3. Root candidate generation
4. LLM validation
5. Dictionary verification
6. Frequency analysis

## Results

- Initial known roots: 10,470
- Total discovered forms: ~4 million
- Validated unique roots: ~21,756
- Processing time: ~21 minutes with parallel processing

## Requirements

- Python 3.9+
- Required packages:
  - pandas
  - datasets
  - joblib
  - sqlite3
  - ollama (for Gemma)
  - google.generativeai (for Gemini)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/turkce_kokler.git
```

2. Merge the dictionary data:
```bash
# On Unix-like systems:
cat sozluk/gts.json.part_* > sozluk/gts.json

# On Windows (PowerShell):
Get-Content sozluk/gts.json.part_* | Set-Content sozluk/gts.json
```

3. Install Python dependencies:
```bash
pip install -r requirements.txt
```

## Usage

1. Make sure the dictionary file is properly merged (see Installation step 2)

2. Run the notebooks in order:
   - wikipedia_kok.ipynb
   - gemma_kokleri_sec.ipynb
   - gemini_kokleri_duzelt.ipynb

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to your fork
5. Create a Pull Request

Note: When adding large files (>50MB), please split them into smaller chunks:
```bash
split -b 50M large_file.json large_file.json.part_
```

## License

[Add your license information here]

## Authors

[Add author information here]

## Acknowledgments

- TDK for the Turkish dictionary database
- Wikimedia for the Turkish Wikipedia dataset
- Google and Ollama for the LLM models
