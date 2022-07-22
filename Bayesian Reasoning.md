Learning from [this video](https://www.youtube.com/watch?v=HZGCoVF3YvM&ab_channel=3Blue1Brown).

# Human Judgments
- Steve is very shy and withdrawn, invariably helpful but with very little interest in people or in the world of reality. A meek and tidy soul, he has a need for order and structure, and a passion for detail
	- Given this, what is the probability that Steve is either a Librarian or a Farmer?
- According to the researchers, after given the description, most say he is a librarian rather than a farmer; but according to them, this is an irrationality
	- The ratio of farmers to librarians is 20/1
	- Anyone asked this question are not expected to know the exact ratio, but judgers are expected to make judgments about relevant facts
- Picture a representative sample: 200 farmers to 10 librarians; 40% of the librarians fit the description (4), and 10% farmers (20 farmers); thus even if we think that librarians are more likely to fit the description, but the overwhelming amount of farmers should contradict this
	- Core of Bayes theorem, at least the intuition. New evidence should update probabilities
- Bayes theorem is relevant when we have some hypothesis, and some new evidence, and what is the probability the hypothesis holds given that the evidence is true (so `P(H|E)`, the probability of H given E)
	- Our prior here is P(H) (so 1/21 people are librarians), and our `P(E|H)`, or probability of the evidence given the hypothesis, called the likelihood (here 0.4); and similarly, we need to know what the probability of `P(E|~H)`
	- This all comes down to `P(H)P(E|H) / P(H)P(E|H) + P(~H)P(E|~H)`
		- This is Bayes theorem; we can also write it as `P(H)P(E|H)/P(E)`
		- `P(H|E)` is called the *posterior*, the updated belief
		- Value of the formula here is that it allows us to quantify the update to our beliefs!
- However we end up writing it, the formula, it is very helpful to get a graphical interpretation
- We could think of these probabilities as being geometric: the area of the region of `P(E|H)` an `P(E|~H)`
- The math of probability is merely just the math of proportions, and *is* intricately linked with geometry

# Proof of Bayes Theorem
[From here.](https://www.youtube.com/watch?v=U_85TaXbeIo&ab_channel=3Blue1Brown)

- Quick way to think about how Bayes theorem is true by analyzing the conjunction in probability;
	- The probability of `P(A and B) = P(A)P(A|B)=P(B)P(B|A)'
	- This allows us to do some algebraic manipulation of the formula for whatever is the most useful :)
- The reason these hold is because the events A and B are *independent* of one another; so the probablity of B given A is just the same as P(B|A)