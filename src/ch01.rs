//! Ch 1: Regular languages — DFA, NFA with epsilon transitions, subset construction,
//! product construction (union/intersection/complement), Myhill–Nerode minimization,
//! and Thompson's regex-to-NFA compiler.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

/// DFA recognizing binary words containing the substring "01".
pub fn contains_01(word: &str) -> bool {
    let mut q = 0;
    for c in word.chars() {
        q = match (q, c) {
            (0, '0') => 1,
            (0, '1') => 0,
            (1, '0') => 1,
            (1, '1') => 2,
            (2, '0' | '1') => 2,
            _ => return false,
        };
    }
    q == 2
}

/// NFA for words ending in "ab". State 0 can guess where the suffix begins.
pub fn ends_ab_nfa(word: &str) -> bool {
    let mut current = HashSet::from([0u8]);
    for c in word.chars() {
        let mut next = HashSet::new();
        for q in current {
            match (q, c) {
                (0, 'a') => {
                    next.insert(0);
                    next.insert(1);
                }
                (0, 'b') => {
                    next.insert(0);
                }
                (1, 'b') => {
                    next.insert(2);
                }
                _ => {}
            }
        }
        current = next;
    }
    current.contains(&2)
}

/// General Deterministic Finite Automaton (Q, Σ, δ, q0, F).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dfa {
    pub num_states: usize,
    pub alphabet: Vec<char>,
    pub transitions: Vec<BTreeMap<char, usize>>,
    pub start: usize,
    pub accept: BTreeSet<usize>,
}

impl Dfa {
    pub fn accepts(&self, word: &str) -> bool {
        let mut q = self.start;
        for c in word.chars() {
            match self.transitions[q].get(&c) {
                Some(&next) => q = next,
                None => return false,
            }
        }
        self.accept.contains(&q)
    }

    /// Complement DFA (assumes total transition function over `alphabet`).
    pub fn complement(&self) -> Self {
        let accept = (0..self.num_states)
            .filter(|s| !self.accept.contains(s))
            .collect();
        Self {
            num_states: self.num_states,
            alphabet: self.alphabet.clone(),
            transitions: self.transitions.clone(),
            start: self.start,
            accept,
        }
    }

    /// Product construction for intersection (if `is_intersection` is true) or union (false).
    pub fn product(&self, other: &Self, is_intersection: bool) -> Self {
        assert_eq!(self.alphabet, other.alphabet, "DFAs must share the same alphabet");
        let n1 = self.num_states;
        let n2 = other.num_states;
        let mut transitions = vec![BTreeMap::new(); n1 * n2];
        let mut accept = BTreeSet::new();
        for q1 in 0..n1 {
            for q2 in 0..n2 {
                let id = q1 * n2 + q2;
                let a1 = self.accept.contains(&q1);
                let a2 = other.accept.contains(&q2);
                if (is_intersection && (a1 && a2)) || (!is_intersection && (a1 || a2)) {
                    accept.insert(id);
                }
                for &sym in &self.alphabet {
                    if let (Some(&t1), Some(&t2)) =
                        (self.transitions[q1].get(&sym), other.transitions[q2].get(&sym))
                    {
                        transitions[id].insert(sym, t1 * n2 + t2);
                    }
                }
            }
        }
        Self {
            num_states: n1 * n2,
            alphabet: self.alphabet.clone(),
            transitions,
            start: self.start * n2 + other.start,
            accept,
        }
    }

    /// Return reachable states from `start`.
    pub fn reachable_states(&self) -> BTreeSet<usize> {
        let mut seen = BTreeSet::new();
        let mut q = VecDeque::from([self.start]);
        seen.insert(self.start);
        while let Some(u) = q.pop_front() {
            for &v in self.transitions[u].values() {
                if seen.insert(v) {
                    q.push_back(v);
                }
            }
        }
        seen
    }

    /// Myhill–Nerode table-filling equivalence classes on reachable states.
    /// Two reachable states p, q are indistinguishable iff for every suffix z,
    /// δ*(p, z) ∈ F  <=>  δ*(q, z) ∈ F.
    pub fn myhill_nerode_classes(&self) -> Vec<Vec<usize>> {
        let reachable: Vec<usize> = self.reachable_states().into_iter().collect();
        let m = reachable.len();
        let idx: HashMap<usize, usize> =
            reachable.iter().enumerate().map(|(i, &s)| (s, i)).collect();

        // dist[i][j] = true means reachable[i] and reachable[j] are distinguishable.
        let mut dist = vec![vec![false; m]; m];
        for i in 0..m {
            for j in (i + 1)..m {
                if self.accept.contains(&reachable[i]) != self.accept.contains(&reachable[j]) {
                    dist[i][j] = true;
                    dist[j][i] = true;
                }
            }
        }

        let mut changed = true;
        while changed {
            changed = false;
            for i in 0..m {
                for j in (i + 1)..m {
                    if dist[i][j] {
                        continue;
                    }
                    for &sym in &self.alphabet {
                        let t1 = self.transitions[reachable[i]].get(&sym);
                        let t2 = self.transitions[reachable[j]].get(&sym);
                        let separated = match (t1, t2) {
                            (Some(&u), Some(&v)) => dist[idx[&u]][idx[&v]],
                            (None, None) => false,
                            _ => true,
                        };
                        if separated {
                            dist[i][j] = true;
                            dist[j][i] = true;
                            changed = true;
                            break;
                        }
                    }
                }
            }
        }

        let mut used = vec![false; m];
        let mut classes = Vec::new();
        for i in 0..m {
            if !used[i] {
                let mut cls = Vec::new();
                for j in i..m {
                    if !dist[i][j] {
                        used[j] = true;
                        cls.push(reachable[j]);
                    }
                }
                classes.push(cls);
            }
        }
        classes
    }

    /// Minimize the DFA by collapsing Myhill–Nerode equivalence classes.
    pub fn minimize(&self) -> Self {
        let classes = self.myhill_nerode_classes();
        let mut state_to_class = HashMap::new();
        for (cid, cls) in classes.iter().enumerate() {
            for &s in cls {
                state_to_class.insert(s, cid);
            }
        }
        let mut transitions = vec![BTreeMap::new(); classes.len()];
        let mut accept = BTreeSet::new();
        for (cid, cls) in classes.iter().enumerate() {
            let rep = cls[0];
            if self.accept.contains(&rep) {
                accept.insert(cid);
            }
            for &sym in &self.alphabet {
                if let Some(&target) = self.transitions[rep].get(&sym) {
                    transitions[cid].insert(sym, state_to_class[&target]);
                }
            }
        }
        Self {
            num_states: classes.len(),
            alphabet: self.alphabet.clone(),
            transitions,
            start: state_to_class[&self.start],
            accept,
        }
    }
}

/// General Nondeterministic Finite Automaton with ε-transitions (`None` key).
#[derive(Debug, Clone)]
pub struct Nfa {
    pub num_states: usize,
    pub alphabet: Vec<char>,
    pub transitions: Vec<BTreeMap<Option<char>, BTreeSet<usize>>>,
    pub start: usize,
    pub accept: BTreeSet<usize>,
}

impl Nfa {
    /// Compute ε-closure E(S) of a set of states.
    pub fn epsilon_closure(&self, states: &BTreeSet<usize>) -> BTreeSet<usize> {
        let mut closure = states.clone();
        let mut stack: Vec<usize> = states.iter().copied().collect();
        while let Some(u) = stack.pop() {
            if let Some(eps_targets) = self.transitions[u].get(&None) {
                for &v in eps_targets {
                    if closure.insert(v) {
                        stack.push(v);
                    }
                }
            }
        }
        closure
    }

    pub fn accepts(&self, word: &str) -> bool {
        let mut current = self.epsilon_closure(&BTreeSet::from([self.start]));
        for c in word.chars() {
            let mut next = BTreeSet::new();
            for &u in &current {
                if let Some(targets) = self.transitions[u].get(&Some(c)) {
                    next.extend(targets.iter().copied());
                }
            }
            current = self.epsilon_closure(&next);
        }
        current.iter().any(|s| self.accept.contains(s))
    }

    /// Subset construction (Sipser Theorem 1.39): convert this NFA into an equivalent DFA.
    pub fn to_dfa(&self) -> Dfa {
        let start_set = self.epsilon_closure(&BTreeSet::from([self.start]));
        let mut subset_to_id: BTreeMap<BTreeSet<usize>, usize> = BTreeMap::new();
        let mut queue: VecDeque<BTreeSet<usize>> = VecDeque::new();
        let mut transitions: Vec<BTreeMap<char, usize>> = Vec::new();
        let mut accept = BTreeSet::new();

        subset_to_id.insert(start_set.clone(), 0);
        queue.push_back(start_set);
        transitions.push(BTreeMap::new());

        while let Some(cur_set) = queue.pop_front() {
            let cur_id = subset_to_id[&cur_set];
            if cur_set.iter().any(|s| self.accept.contains(s)) {
                accept.insert(cur_id);
            }
            for &sym in &self.alphabet {
                let mut moved = BTreeSet::new();
                for &u in &cur_set {
                    if let Some(targets) = self.transitions[u].get(&Some(sym)) {
                        moved.extend(targets.iter().copied());
                    }
                }
                let next_set = self.epsilon_closure(&moved);
                let next_id = if let Some(&id) = subset_to_id.get(&next_set) {
                    id
                } else {
                    let id = transitions.len();
                    subset_to_id.insert(next_set.clone(), id);
                    transitions.push(BTreeMap::new());
                    queue.push_back(next_set);
                    id
                };
                transitions[cur_id].insert(sym, next_id);
            }
        }

        Dfa {
            num_states: transitions.len(),
            alphabet: self.alphabet.clone(),
            transitions,
            start: 0,
            accept,
        }
    }
}

/// Thompson's construction: compile a classical regular expression supporting
/// literals, concatenation, union `|`, Kleene star `*`, parentheses `(...)`, and `ε` (`@` or `ε`)
/// into an equivalent ε-NFA.
pub fn regex_to_nfa(pattern: &str) -> Result<Nfa, String> {
    #[derive(Clone)]
    enum Expr {
        Empty,
        Lit(char),
        Concat(Box<Expr>, Box<Expr>),
        Union(Box<Expr>, Box<Expr>),
        Star(Box<Expr>),
    }

    struct Parser {
        chars: Vec<char>,
        pos: usize,
    }
    impl Parser {
        fn peek(&self) -> Option<char> {
            self.chars.get(self.pos).copied()
        }
        fn parse_union(&mut self) -> Result<Expr, String> {
            let mut left = self.parse_concat()?;
            while self.peek() == Some('|') {
                self.pos += 1;
                let right = self.parse_concat()?;
                left = Expr::Union(Box::new(left), Box::new(right));
            }
            Ok(left)
        }
        fn parse_concat(&mut self) -> Result<Expr, String> {
            let mut items = Vec::new();
            while let Some(c) = self.peek() {
                if c == ')' || c == '|' {
                    break;
                }
                items.push(self.parse_star()?);
            }
            if items.is_empty() {
                return Ok(Expr::Empty);
            }
            let mut acc = items.remove(0);
            for item in items {
                acc = Expr::Concat(Box::new(acc), Box::new(item));
            }
            Ok(acc)
        }
        fn parse_star(&mut self) -> Result<Expr, String> {
            let mut base = self.parse_atom()?;
            while self.peek() == Some('*') {
                self.pos += 1;
                base = Expr::Star(Box::new(base));
            }
            Ok(base)
        }
        fn parse_atom(&mut self) -> Result<Expr, String> {
            match self.peek() {
                Some('(') => {
                    self.pos += 1;
                    let inner = self.parse_union()?;
                    if self.peek() != Some(')') {
                        return Err("Unclosed '(' in regex".to_string());
                    }
                    self.pos += 1;
                    Ok(inner)
                }
                Some('ε') => {
                    self.pos += 1;
                    Ok(Expr::Empty)
                }
                Some(c) if c != ')' && c != '|' && c != '*' => {
                    self.pos += 1;
                    Ok(Expr::Lit(c))
                }
                other => Err(format!("Unexpected token {:?} in regex", other)),
            }
        }
    }

    let mut parser = Parser {
        chars: pattern.chars().collect(),
        pos: 0,
    };
    let expr = parser.parse_union()?;
    if parser.pos != parser.chars.len() {
        return Err("Trailing characters in regex".to_string());
    }

    let mut transitions: Vec<BTreeMap<Option<char>, BTreeSet<usize>>> = Vec::new();
    let mut alphabet = BTreeSet::new();

    fn new_state(t: &mut Vec<BTreeMap<Option<char>, BTreeSet<usize>>>) -> usize {
        let id = t.len();
        t.push(BTreeMap::new());
        id
    }
    fn add_edge(
        t: &mut [BTreeMap<Option<char>, BTreeSet<usize>>],
        u: usize,
        sym: Option<char>,
        v: usize,
    ) {
        t[u].entry(sym).or_default().insert(v);
    }

    fn build(
        e: &Expr,
        t: &mut Vec<BTreeMap<Option<char>, BTreeSet<usize>>>,
        alpha: &mut BTreeSet<char>,
    ) -> (usize, usize) {
        match e {
            Expr::Empty => {
                let s = new_state(t);
                let f = new_state(t);
                add_edge(t, s, None, f);
                (s, f)
            }
            Expr::Lit(c) => {
                alpha.insert(*c);
                let s = new_state(t);
                let f = new_state(t);
                add_edge(t, s, Some(*c), f);
                (s, f)
            }
            Expr::Concat(a, b) => {
                let (s1, f1) = build(a, t, alpha);
                let (s2, f2) = build(b, t, alpha);
                add_edge(t, f1, None, s2);
                (s1, f2)
            }
            Expr::Union(a, b) => {
                let s = new_state(t);
                let (s1, f1) = build(a, t, alpha);
                let (s2, f2) = build(b, t, alpha);
                let f = new_state(t);
                add_edge(t, s, None, s1);
                add_edge(t, s, None, s2);
                add_edge(t, f1, None, f);
                add_edge(t, f2, None, f);
                (s, f)
            }
            Expr::Star(a) => {
                let s = new_state(t);
                let (s1, f1) = build(a, t, alpha);
                let f = new_state(t);
                add_edge(t, s, None, s1);
                add_edge(t, s, None, f);
                add_edge(t, f1, None, s1);
                add_edge(t, f1, None, f);
                (s, f)
            }
        }
    }

    let (start, finish) = build(&expr, &mut transitions, &mut alphabet);
    Ok(Nfa {
        num_states: transitions.len(),
        alphabet: alphabet.into_iter().collect(),
        transitions,
        start,
        accept: BTreeSet::from([finish]),
    })
}

/// Match a word against a classical regular expression via Thompson's NFA construction.
pub fn regex_matches(pattern: &str, word: &str) -> Result<bool, String> {
    Ok(regex_to_nfa(pattern)?.accepts(word))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfa() {
        assert!(contains_01("11010"));
        assert!(!contains_01("111"));
        assert!(!contains_01("02"));
    }

    #[test]
    fn nfa() {
        assert!(ends_ab_nfa("bbaab"));
        assert!(!ends_ab_nfa("aba"));
        assert!(!ends_ab_nfa("acb"));
    }

    #[test]
    fn regex_subset_and_myhill_nerode() {
        let nfa = regex_to_nfa("(0|1)*01(0|1)*").unwrap();
        let dfa = nfa.to_dfa();
        let min_dfa = dfa.minimize();
        // Minimal complete DFA for "contains 01" has exactly 3 Myhill-Nerode classes.
        assert_eq!(min_dfa.num_states, 3);
        for w in ["", "0", "1", "11100", "01", "11010", "0011"] {
            assert_eq!(nfa.accepts(w), contains_01(w));
            assert_eq!(dfa.accepts(w), contains_01(w));
            assert_eq!(min_dfa.accepts(w), contains_01(w));
        }
        // Complement & intersection
        let comp = min_dfa.complement();
        assert!(!comp.accepts("01"));
        assert!(comp.accepts("111000"));
        let empty = min_dfa.product(&comp, true);
        assert!(!empty.accepts("01"));
        assert!(!empty.accepts("111"));
    }
}
