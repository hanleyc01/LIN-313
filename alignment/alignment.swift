// given two strings, produce all possible combinations of each of their elements
func possible_pairs(e: [String], f: [String]) -> [(String, String)] {
    var l = [(String,String)]()
    for s1 in e {
        for s2 in f {
            l.append((s1,s2))
        }
    }
    return l
}

// e for language to translate to
let e1 = [
    "mary", "unlocked","the","car","door"
]

let e2 = [
    "can", "you", "unlock","the","door"
]

// f for language to translate from
let f1 = [
    "maria","shloss","die","autotiir","auf"
]

let f2 = [
    "kannst","du","die","tiir","aufschliessen"
]

// assign first round of pairs
let ef1 = possible_pairs(e: e1, f: f1)
let ef2 = possible_pairs(e: e2, f: f2)
