# Introduction to Language Modelling
- Problems with thinking through storing word data:
	- simply listing words in an array with an index does not clue us in at all to the semantic content of any given word.
		- e.g., attempting to represent a sentence we might result in say "32123", but what does this actually mean?
	- We *know* how to encode and represent characters, but characters by themselves don't actually have any meaning by themselves - rather, *words* are the only things that bear semantic content
		- e.g., plant and tree are more similar to each other than to the word pencil, but there's nothing to really suggest this given the bare content of the phonemes or the characters
- What is semantic content?
	- It may be the representational relationship between a word and something in the world
	- what we can determine, at the very least, is that meaning has something to do with the context which words interact within
		- e.g., saying "yeah, okay" means two different things if you are answering snarkily or if you are answering genuinely
	- Similarly, we can think about how words can have double meanings- Apple can refer to the company, but also apples in general
	- Meanings of words are very complex descriptions, and seems to be very evasive in attempting to describe it
- Some terms:
	- sense: the unique elements that some word which is the concept that forms the semantic content of the word
	- Lemma: the dictionary form of the word
	- Synonym: words that have the same meaning in some or all contexts
- Alors: on peut parler sur les mots comme beaucoup des caractéristiques, mais il y a problèmes avec ça.
- Lisez ces phrases:
	- A wampimunk scurried across the lawn, and burrowed into its hole.
	- Wampimunks are shy creatures, rarely seen during the time of day.
	- Wampimunks are an endangered species.
	- I was bitten by that damn wampimunk.
- Avec ces phrases, on peut penser que le mot "wampimunk" signifie un animal, comme un rat
- L'associationisme:
	- Le signification d'un mot est donnée par le contexte.
	- Les mots sont doc définis par le contexte qui les entoure
- Alors, comment peut-on construire ensuite un encodage? En d'autres termes, comment on répresente les numéros avec signification?

# Construire d'un modèle spatial vectoriel basé sur la co-occurence
1. Choissisez un corpus
2. Tokenization
	- Séparation des mots en contextes
3. Normalization
	- Les formes differents d'un mot
4. Lemmaization
5. L'Élimination de les Stop-Words
6. Limites de fréquence
- Alors, après construire, le modèle a une matrice. Les ranges signifient mots, et les colonnes signifient documents
- E.g.,
	- rye = {1,1,0,0,0,0,0,0}
	- shown = {2,0,0,0,0,0,0,0}
	- appetite = {1,1,0,0,0,0,0}
-   On peut construire la fréquence des termes, et la fréquence inverse des documents (TF-IDF)
	- Cette équation signifie l'importance actuelle d'un mot comparé à l'importance de les autres mots
- Des problèmes:
	- pour corpora actuelles, notre vecteurs seraient enormés, 100K+ dimensions
	- Aussi, ces vecteurs seraient clairsemées

# Distributional hypothesis
- Les mots qui apparaissent dans des contexts similaires ont un sens similaire
- Alors, comment on peut trouver ces sens?
	- en cette case, le mot *context* a un context avec un mot "des" avant saisi et "similaires" après sa
- Avec notre mot "wampimunk":
	1. Wampimunks are cute
	2. I saw a wampimunk scurry.
	3. Wampimunks are endangered.
	- Fênetre contextuelle = 1
		- be = 2, a = 1, scurry = 1
	- Fênetre contextuelle = 2
		- be = 2, a = 1, scurry = 1, endangered = 1, cute = 1