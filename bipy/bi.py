from typing import Dict, List, Tuple

# global constant for our corpus, pre-requisite is that corpus
# is delimited by new-lines between sentences
example = 'example.txt'
sentences = 'sentences.txt'

def get_unigrams(corpus: List[str]) -> Dict[str, Tuple[str, float]]:
    
    for snt in corpus:
        wd_prob = Dict[str, str] = {}
        for i in range(len(snt)):
            wd_prob[snt[i]] = snt[i+1]


def main():
    # split corpus by new-line characters
    corpus = open(example).read().split('\n')
    print(corpus)
    
    unigrams = get_unigrams(corpus)
    

if __name__ == '__main__':
    main()
