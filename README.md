# 🐸 FRAUG
The GitHub repository of the 🐸 **FRAUG** (**F**or **R**ealistic **AUG**mentations) library! 

# 🚧 WIP 

## NLP TODO

|Methods|Sub-method|Sub-submethod|Interest of the method|Pseudo-code for French|Pseudo-code for English|Pseudo-code for multilingual|Rust|Example|
|:----|:----|:----|:----|:----|:----|:----|:----|:----|
|Lexical substitution|Thesaurus|LibreOffice dictionaries| |🟢|🟢|🟢|🔴|🔴|
| | |WordNet nltk| |🟢|🟢|🟢|🔴|🔴|
| | |WordNet non-nltk| |🟢|🔴 (don't know if any resources exist for English, bibliography need to be done)|🔴 (add all available languages on http://globalwordnet.org/resources/wordnets-in-the-world/)|🔴|🔴|
| |Word embedding|Gensim| |🟢|🟢|🔴 (add gensim models on the HF Hub)|🔴|🔴|
| | |FastText| |🟢|🟢|🟢|🔴|🔴|
| |Masked language model (BERT like)|Random| |🟢|🟢|🟠|🔴|🔴|
| | |POS| |🟢|🟢|🟠|🔴|🔴|
| |TD-IDF| | |🔴|🔴|🔴|🔴|🔴|
|Back-translation|Marian (Helsinki-NLP models)| | |🟢|🟢|🟢|🔴|🔴|
| |M2M100| | |🟢|🟢|🟢|🔴|🔴|
| |NLLB| | |🟢|🟢|🟢| | |
| |AutoModel (so that users can use the model of their choice)| | |🟢|🟢|🟢|🔴|🔴|
|Transformation of the text surface|Number to letters (and vice versa)| | |🟢|🟢|🟢|🔴|🔴|
| |You're → You are (and vice versa)| | |Not relevant|🟢|?|🔴|🔴|
|Random noise injection|Spelling mistakes injection| | |🟢|🟢 (Note: the dataset must be cleaned)|🟢|🔴|🔴|
| |Spelling mistakes correction| | |🟢|🟢 (Note: the dataset must be cleaned)|🟢|🔴|🔴|
| |Typing errors injection| | |🟢|🟢 (code ok but data to be completed)|🟢 (code ok but data to be completed)|🔴|🔴|
| |Token noise injection| | |🟢|🟢|🟢|🔴|🔴|
| |Mixed sentences| | |🟢|🟢|🟢|🔴|🔴|
| |Random swap| | |🟢|🟢|🟢|🔴|🔴|
| |Random deletion| | |🟢|🟢|🟢|🔴|🔴|
|Manipulating the syntax tree|Time manipulation| | |🟠|🔴|🔴|🔴|🔴|
| |Gender manipulation| | |🟠|🔴|🔴|🔴|🔴|
| |Number manipulation| | |🟠|🔴|🔴|🔴|🔴|
|Generative methods|Generate paraphrases| | |🟠|🔴|🔴|🔴|🔴|
| |Complexification| | |🟠|🔴|🔴|🔴|🔴|
|Text simplification|Text summary| | |🟠|🔴|🔴|🔴|🔴|
| |Simplification| | |🟠|🔴|🔴|🔴|🔴|

---

If you find the project useful, please consider giving it a star ⭐️.
