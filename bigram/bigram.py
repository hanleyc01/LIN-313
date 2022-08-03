import re
from collections import Counter
from typing import Optional, List

# global constant for corpus name
fname = 'big.txt'

# primitive markov chain
# corpus -> parse to sentences ->
# for each sentence, for each word, calculate up to the trigram of that word ->
# create list of probable n-grams for each word ->
# using list of probable following words, find most probable sequence of
# of following words 

def main() -> int:
    try:
        f = open(fname,'r')
    except IOError:
        print("Could not open/read file: ", fname)
        return -1
    with f:
        f_cont = f.read()
        
        # find only words which match either some combination of words
        # or end with a period
        f_reg: List[str] = re.findall(r'\w+|\.', f_cont)
        
        # create sublists for each sentence; i.e., a 2d list of words UNTIIL a period
        
        print(sentences)
    return 0
    

if __name__ == '__main__':
    exit_code = main()
    if exit_code < 0:
        print('Program exited abnormally with exit code:', exit_code)
    else:
        print('Program exited successfully with exit code:', exit_code)
