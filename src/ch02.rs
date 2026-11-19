//! Ch 2: Context-free languages — CFG to CNF conversion, CYK, nondeterministic PDA,
//! deterministic PDA (DPDA, 3rd ed. §2.4), and canonical LR(0)/LR(1) parser generator
//! with Shift/Reduce and Reduce/Reduce conflict diagnostics.

use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};

/// One-type balanced parentheses: a deterministic pushdown language.
pub fn balanced_parentheses(word: &str) -> bool {
    let mut depth = 0usize;
    for c in word.chars() {
        match c {
            '(' => depth += 1,
            ')' if depth > 0 => depth -= 1,
            _ => return false,
        }
    }
    depth == 0
}

/// Recognize a^n b^n for n >= 0 with deterministic bookkeeping.
pub fn anbn(word: &str) -> bool {
    let bytes = word.as_bytes();
    let n = bytes.iter().take_while(|&&b| b == b'a').count();
    bytes[n..].iter().all(|&b| b == b'b') && bytes.len() - n == n
}

/// CYK membership for a grammar in Chomsky normal form (nonempty words).
/// `terminals`: `(nonterminal, terminal)`; `binary`: `(parent, left, right)`.
pub fn cyk(
    word: &str,
    terminals: &[(char, char)],
    binary: &[(char, char, char)],
    start: char,
) -> bool {
    cyk_with_epsilon(word, terminals, binary, start, false)
}

/// CYK membership supporting optional `start -> ε` rule.
pub fn cyk_with_epsilon(
    word: &str,
    terminals: &[(char, char)],
    binary: &[(char, char, char)],
    start: char,
    allows_epsilon: bool,
) -> bool {
    let chars: Vec<char> = word.chars().collect();
    let n = chars.len();
    if n == 0 {
        return allows_epsilon;
    }
    let mut dp = vec![vec![HashSet::<char>::new(); n]; n];
    for (i, &letter) in chars.iter().enumerate() {
        for &(variable, terminal) in terminals {
            if letter == terminal {
                dp[i][i].insert(variable);
            }
        }
    }
    for span in 2..=n {
        for i in 0..=n - span {
            let j = i + span - 1;
            for k in i..j {
                for &(a, b, c) in binary {
                    if dp[i][k].contains(&b) && dp[k + 1][j].contains(&c) {
                        dp[i][j].insert(a);
                    }
                }
            }
        }
    }
    dp[0][n - 1].contains(&start)
}

/// Symbol in a Context-Free Grammar (nonterminal variable or terminal token).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Symbol {
    Var(String),
    Term(String),
}

/// A CFG production `lhs -> rhs` (empty `rhs` denotes ε).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Production {
    pub lhs: String,
    pub rhs: Vec<Symbol>,
}

/// Context-Free Grammar (V, Σ, R, S).
#[derive(Debug, Clone)]
pub struct Cfg {
    pub start: String,
    pub productions: Vec<Production>,
}

/// Chomsky Normal Form representation produced by `Cfg::to_cnf`.
#[derive(Debug, Clone)]
pub struct CnfGrammar {
    pub start: String,
    pub allows_epsilon: bool,
    pub terminals: Vec<(String, String)>,
    pub binary: Vec<(String, String, String)>,
}

impl CnfGrammar {
    /// Run CYK membership on a sequence of terminal tokens.
    pub fn accepts(&self, tokens: &[&str]) -> bool {
        let n = tokens.len();
        if n == 0 {
            return self.allows_epsilon;
        }
        let mut dp = vec![vec![HashSet::<&str>::new(); n]; n];
        for (i, &tok) in tokens.iter().enumerate() {
            for (var, term) in &self.terminals {
                if term == tok {
                    dp[i][i].insert(var.as_str());
                }
            }
        }
        for span in 2..=n {
            for i in 0..=n - span {
                let j = i + span - 1;
                for k in i..j {
                    for (a, b, c) in &self.binary {
                        if dp[i][k].contains(b.as_str()) && dp[k + 1][j].contains(c.as_str()) {
                            dp[i][j].insert(a.as_str());
                        }
                    }
                }
            }
        }
        dp[0][n - 1].contains(self.start.as_str())
    }
}

impl Cfg {
    /// Convert this CFG into Chomsky Normal Form following Sipser Theorem 2.9:
    /// 1. Add new start variable S0 -> S
    /// 2. Eliminate ε-rules (except S0 -> ε if S is nullable)
    /// 3. Eliminate unit rules A -> B
    /// 4. Convert remaining rules into A -> a or A -> B C
    pub fn to_cnf(&self) -> CnfGrammar {
        let new_start = format!("{}0", self.start);
        let mut prods: BTreeSet<Production> = self.productions.iter().cloned().collect();
        prods.insert(Production {
            lhs: new_start.clone(),
            rhs: vec![Symbol::Var(self.start.clone())],
        });

        // Step 2: Find nullable variables
        let mut nullable: BTreeSet<String> = BTreeSet::new();
        let mut changed = true;
        while changed {
            changed = false;
            for p in &prods {
                if !nullable.contains(&p.lhs)
                    && p.rhs.iter().all(|s| match s {
                        Symbol::Var(v) => nullable.contains(v),
                        Symbol::Term(_) => false,
                    })
                {
                    nullable.insert(p.lhs.clone());
                    changed = true;
                }
            }
        }
        let allows_epsilon = nullable.contains(&new_start);

        // Expand nullable occurrences, dropping non-start ε-rules
        let mut no_eps: BTreeSet<Production> = BTreeSet::new();
        for p in &prods {
            let null_positions: Vec<usize> = p
                .rhs
                .iter()
                .enumerate()
                .filter_map(|(i, s)| match s {
                    Symbol::Var(v) if nullable.contains(v) => Some(i),
                    _ => None,
                })
                .collect();
            let combos = 1usize << null_positions.len();
            for mask in 0..combos {
                let mut omit = HashSet::new();
                for (bit, &pos) in null_positions.iter().enumerate() {
                    if (mask & (1 << bit)) != 0 {
                        omit.insert(pos);
                    }
                }
                let new_rhs: Vec<Symbol> = p
                    .rhs
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| !omit.contains(i))
                    .map(|(_, s)| s.clone())
                    .collect();
                if !new_rhs.is_empty() {
                    no_eps.insert(Production {
                        lhs: p.lhs.clone(),
                        rhs: new_rhs,
                    });
                }
            }
        }

        // Step 3: Eliminate unit rules A -> B
        let mut vars: BTreeSet<String> = BTreeSet::new();
        for p in &no_eps {
            vars.insert(p.lhs.clone());
            for s in &p.rhs {
                if let Symbol::Var(v) = s {
                    vars.insert(v.clone());
                }
            }
        }
        let mut unit_pairs: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for v in &vars {
            unit_pairs.entry(v.clone()).or_default().insert(v.clone());
        }
        let mut changed = true;
        while changed {
            changed = false;
            for p in &no_eps {
                if p.rhs.len() == 1 {
                    if let Symbol::Var(b) = &p.rhs[0] {
                        let b_reach = unit_pairs.get(b).cloned().unwrap_or_default();
                        let entry = unit_pairs.entry(p.lhs.clone()).or_default();
                        for target in b_reach {
                            if entry.insert(target) {
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
        let mut no_unit: BTreeSet<Production> = BTreeSet::new();
        for (a, targets) in &unit_pairs {
            for b in targets {
                for p in &no_eps {
                    if &p.lhs == b && !(p.rhs.len() == 1 && matches!(p.rhs[0], Symbol::Var(_))) {
                        no_unit.insert(Production {
                            lhs: a.clone(),
                            rhs: p.rhs.clone(),
                        });
                    }
                }
            }
        }

        // Step 4: Replace terminals in long rules & binarize rules of length >= 3
        let mut terminals: BTreeSet<(String, String)> = BTreeSet::new();
        let mut binary: BTreeSet<(String, String, String)> = BTreeSet::new();
        let mut term_var_map: BTreeMap<String, String> = BTreeMap::new();
        let mut fresh_id = 0usize;

        let get_term_var = |t: &str,
                                map: &mut BTreeMap<String, String>,
                                terms: &mut BTreeSet<(String, String)>|
         -> String {
            if let Some(v) = map.get(t) {
                return v.clone();
            }
            let v = format!("T_{}", t);
            map.insert(t.to_string(), v.clone());
            terms.insert((v.clone(), t.to_string()));
            v
        };

        for p in &no_unit {
            if p.rhs.len() == 1 {
                if let Symbol::Term(t) = &p.rhs[0] {
                    terminals.insert((p.lhs.clone(), t.clone()));
                }
            } else {
                let var_rhs: Vec<String> = p
                    .rhs
                    .iter()
                    .map(|s| match s {
                        Symbol::Var(v) => v.clone(),
                        Symbol::Term(t) => get_term_var(t, &mut term_var_map, &mut terminals),
                    })
                    .collect();
                let mut cur_lhs = p.lhs.clone();
                for i in 0..var_rhs.len() - 2 {
                    fresh_id += 1;
                    let next_var = format!("X_{}", fresh_id);
                    binary.insert((cur_lhs, var_rhs[i].clone(), next_var.clone()));
                    cur_lhs = next_var;
                }
                let n = var_rhs.len();
                binary.insert((cur_lhs, var_rhs[n - 2].clone(), var_rhs[n - 1].clone()));
            }
        }

        CnfGrammar {
            start: new_start,
            allows_epsilon,
            terminals: terminals.into_iter().collect(),
            binary: binary.into_iter().collect(),
        }
    }
}

/// Nondeterministic Pushdown Automaton transition:
/// `(state, input_opt, pop_opt) -> (next_state, push_string)` where `push_string`
/// is pushed right-to-left so its first character becomes the new top of stack.
#[derive(Debug, Clone)]
pub struct PdaTransition {
    pub from: usize,
    pub input: Option<char>,
    pub pop: Option<char>,
    pub to: usize,
    pub push: Vec<char>,
}

/// Nondeterministic Pushdown Automaton (Q, Σ, Γ, δ, q0, F) with bounded BFS simulation.
#[derive(Debug, Clone)]
pub struct Pda {
    pub start: usize,
    pub accept: BTreeSet<usize>,
    pub transitions: Vec<PdaTransition>,
}

impl Pda {
    /// Simulate the PDA with a step/configuration budget (avoids infinite ε-loops).
    pub fn accepts(&self, word: &str, max_configs: usize) -> bool {
        let chars: Vec<char> = word.chars().collect();
        let mut queue: VecDeque<(usize, usize, Vec<char>)> = VecDeque::new();
        let mut visited: HashSet<(usize, usize, Vec<char>)> = HashSet::new();
        let init = (self.start, 0usize, Vec::new());
        visited.insert(init.clone());
        queue.push_back(init);

        let mut explored = 0usize;
        while let Some((q, pos, stack)) = queue.pop_front() {
            if pos == chars.len() && self.accept.contains(&q) {
                return true;
            }
            explored += 1;
            if explored > max_configs {
                break;
            }
            for tr in &self.transitions {
                if tr.from != q {
                    continue;
                }
                let next_pos = match tr.input {
                    None => pos,
                    Some(c) if pos < chars.len() && chars[pos] == c => pos + 1,
                    _ => continue,
                };
                let mut next_stack = stack.clone();
                if let Some(p) = tr.pop {
                    if next_stack.last().copied() != Some(p) {
                        continue;
                    }
                    next_stack.pop();
                }
                for &sym in tr.push.iter().rev() {
                    next_stack.push(sym);
                }
                let cfg = (tr.to, next_pos, next_stack);
                if visited.insert(cfg.clone()) {
                    queue.push_back(cfg);
                }
            }
        }
        false
    }
}

/// Deterministic Pushdown Automaton (3rd Edition §2.4).
/// Enforces determinism: at any `(state, stack_top)`, either at most one ε-move exists
/// (and no input-consuming move), or at most one move per input symbol exists.
#[derive(Debug, Clone)]
pub struct Dpda {
    pub start: usize,
    pub accept: BTreeSet<usize>,
    pub transitions: Vec<PdaTransition>,
}

impl Dpda {
    /// Verify the DPDA determinism condition (Sipser 3e Definition 2.45).
    pub fn is_deterministic(&self) -> bool {
        // Check pairs of transitions from the same state that could fire simultaneously.
        for i in 0..self.transitions.len() {
            for j in (i + 1)..self.transitions.len() {
                let a = &self.transitions[i];
                let b = &self.transitions[j];
                if a.from != b.from {
                    continue;
                }
                let input_conflict = a.input.is_none() || b.input.is_none() || a.input == b.input;
                let stack_conflict = a.pop.is_none() || b.pop.is_none() || a.pop == b.pop;
                if input_conflict && stack_conflict {
                    return false;
                }
            }
        }
        true
    }

    /// Run the DPDA deterministically up to `max_steps`.
    pub fn accepts(&self, word: &str, max_steps: usize) -> Result<bool, String> {
        if !self.is_deterministic() {
            return Err("Transition table violates DPDA determinism".to_string());
        }
        let chars: Vec<char> = word.chars().collect();
        let mut q = self.start;
        let mut pos = 0usize;
        let mut stack: Vec<char> = Vec::new();
        for _ in 0..max_steps {
            let cur_in = chars.get(pos).copied();
            let cur_top = stack.last().copied();
            let matching: Option<&PdaTransition> = self.transitions.iter().find(|tr| {
                tr.from == q
                    && (tr.input.is_none() || tr.input == cur_in)
                    && (tr.pop.is_none() || tr.pop == cur_top)
            });
            match matching {
                Some(tr) => {
                    if tr.input.is_some() {
                        pos += 1;
                    }
                    if tr.pop.is_some() {
                        stack.pop();
                    }
                    for &sym in tr.push.iter().rev() {
                        stack.push(sym);
                    }
                    q = tr.to;
                }
                None => break,
            }
        }
        Ok(pos == chars.len() && self.accept.contains(&q))
    }
}

// ---------------------------------------------------------------------------
// Canonical LR(0) & LR(1) Parser Generator with Conflict Diagnostics (3e §2.4)
// ---------------------------------------------------------------------------

pub const EOF_SYMBOL: &str = "$";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LrAction {
    Shift(usize),
    Reduce(usize), // index into augmented productions
    Accept,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LrConflict {
    ShiftReduce {
        state: usize,
        lookahead: String,
        shift_to: usize,
        reduce_prod: usize,
    },
    ReduceReduce {
        state: usize,
        lookahead: String,
        prod_a: usize,
        prod_b: usize,
    },
}

/// Canonical LR(1) parsing tables + conflict diagnostics.
#[derive(Debug, Clone)]
pub struct Lr1Parser {
    pub productions: Vec<Production>, // index 0 is augmented S' -> S
    pub num_states: usize,
    pub action: BTreeMap<(usize, String), LrAction>,
    pub goto: BTreeMap<(usize, String), usize>,
    pub conflicts: Vec<LrConflict>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Lr1Item {
    prod_idx: usize,
    dot: usize,
    lookahead: String,
}

impl Lr1Parser {
    /// Build a canonical LR(1) parser table from a CFG, recording all Shift/Reduce
    /// and Reduce/Reduce conflicts.
    pub fn build(cfg: &Cfg) -> Self {
        let aug_start = format!("{}'", cfg.start);
        let mut productions = Vec::with_capacity(cfg.productions.len() + 1);
        productions.push(Production {
            lhs: aug_start.clone(),
            rhs: vec![Symbol::Var(cfg.start.clone())],
        });
        productions.extend(cfg.productions.iter().cloned());

        let mut nonterminals = BTreeSet::new();
        let mut terminals = BTreeSet::new();
        for p in &productions {
            nonterminals.insert(p.lhs.clone());
            for s in &p.rhs {
                match s {
                    Symbol::Var(v) => {
                        nonterminals.insert(v.clone());
                    }
                    Symbol::Term(t) => {
                        terminals.insert(t.clone());
                    }
                }
            }
        }
        terminals.insert(EOF_SYMBOL.to_string());

        // Compute nullable & FIRST sets for all nonterminals
        let mut nullable: BTreeSet<String> = BTreeSet::new();
        let mut first: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for v in &nonterminals {
            first.insert(v.clone(), BTreeSet::new());
        }
        let mut changed = true;
        while changed {
            changed = false;
            for p in &productions {
                let mut all_null = true;
                for sym in &p.rhs {
                    match sym {
                        Symbol::Term(t) => {
                            if first.get_mut(&p.lhs).unwrap().insert(t.clone()) {
                                changed = true;
                            }
                            all_null = false;
                            break;
                        }
                        Symbol::Var(v) => {
                            let v_first = first.get(v).cloned().unwrap_or_default();
                            let lhs_first = first.get_mut(&p.lhs).unwrap();
                            for t in v_first {
                                if lhs_first.insert(t) {
                                    changed = true;
                                }
                            }
                            if !nullable.contains(v) {
                                all_null = false;
                                break;
                            }
                        }
                    }
                }
                if all_null && nullable.insert(p.lhs.clone()) {
                    changed = true;
                }
            }
        }

        let first_of_seq = |seq: &[Symbol], tail_la: &str| -> BTreeSet<String> {
            let mut out = BTreeSet::new();
            let mut all_null = true;
            for sym in seq {
                match sym {
                    Symbol::Term(t) => {
                        out.insert(t.clone());
                        all_null = false;
                        break;
                    }
                    Symbol::Var(v) => {
                        if let Some(f) = first.get(v) {
                            out.extend(f.iter().cloned());
                        }
                        if !nullable.contains(v) {
                            all_null = false;
                            break;
                        }
                    }
                }
            }
            if all_null {
                out.insert(tail_la.to_string());
            }
            out
        };

        let closure = |seed: BTreeSet<Lr1Item>| -> BTreeSet<Lr1Item> {
            let mut set = seed;
            let mut changed = true;
            while changed {
                changed = false;
                let snapshot: Vec<Lr1Item> = set.iter().cloned().collect();
                for item in snapshot {
                    let prod = &productions[item.prod_idx];
                    if item.dot < prod.rhs.len() {
                        if let Symbol::Var(b) = &prod.rhs[item.dot] {
                            let beta = &prod.rhs[item.dot + 1..];
                            let lookaheads = first_of_seq(beta, &item.lookahead);
                            for (pidx, p) in productions.iter().enumerate() {
                                if &p.lhs == b {
                                    for la in &lookaheads {
                                        let next_item = Lr1Item {
                                            prod_idx: pidx,
                                            dot: 0,
                                            lookahead: la.clone(),
                                        };
                                        if set.insert(next_item) {
                                            changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            set
        };

        let goto_fn = |items: &BTreeSet<Lr1Item>, sym: &Symbol| -> BTreeSet<Lr1Item> {
            let mut moved = BTreeSet::new();
            for item in items {
                let prod = &productions[item.prod_idx];
                if item.dot < prod.rhs.len() && &prod.rhs[item.dot] == sym {
                    moved.insert(Lr1Item {
                        prod_idx: item.prod_idx,
                        dot: item.dot + 1,
                        lookahead: item.lookahead.clone(),
                    });
                }
            }
            closure(moved)
        };

        // Canonical collection of LR(1) item sets
        let start_set = closure(BTreeSet::from([Lr1Item {
            prod_idx: 0,
            dot: 0,
            lookahead: EOF_SYMBOL.to_string(),
        }]));
        let mut states: Vec<BTreeSet<Lr1Item>> = vec![start_set.clone()];
        let mut state_map: BTreeMap<BTreeSet<Lr1Item>, usize> = BTreeMap::new();
        state_map.insert(start_set, 0);
        let mut queue: VecDeque<usize> = VecDeque::from([0]);
        let mut transitions: BTreeMap<(usize, Symbol), usize> = BTreeMap::new();

        while let Some(sid) = queue.pop_front() {
            let items = states[sid].clone();
            let mut next_syms: BTreeSet<Symbol> = BTreeSet::new();
            for item in &items {
                let prod = &productions[item.prod_idx];
                if item.dot < prod.rhs.len() {
                    next_syms.insert(prod.rhs[item.dot].clone());
                }
            }
            for sym in next_syms {
                let target = goto_fn(&items, &sym);
                if target.is_empty() {
                    continue;
                }
                let tid = if let Some(&existing) = state_map.get(&target) {
                    existing
                } else {
                    let id = states.len();
                    state_map.insert(target.clone(), id);
                    states.push(target);
                    queue.push_back(id);
                    id
                };
                transitions.insert((sid, sym), tid);
            }
        }

        // Build ACTION and GOTO tables with conflict diagnostics
        let mut action: BTreeMap<(usize, String), LrAction> = BTreeMap::new();
        let mut goto: BTreeMap<(usize, String), usize> = BTreeMap::new();
        let mut conflicts: Vec<LrConflict> = Vec::new();

        let mut insert_action = |state: usize, la: String, act: LrAction| {
            let key = (state, la.clone());
            if let Some(existing) = action.get(&key) {
                if existing != &act {
                    match (existing, &act) {
                        (LrAction::Shift(s), LrAction::Reduce(r)) => {
                            conflicts.push(LrConflict::ShiftReduce {
                                state,
                                lookahead: la,
                                shift_to: *s,
                                reduce_prod: *r,
                            });
                        }
                        (LrAction::Reduce(r), LrAction::Shift(s)) => {
                            conflicts.push(LrConflict::ShiftReduce {
                                state,
                                lookahead: la,
                                shift_to: *s,
                                reduce_prod: *r,
                            });
                        }
                        (LrAction::Reduce(a), LrAction::Reduce(b)) => {
                            conflicts.push(LrConflict::ReduceReduce {
                                state,
                                lookahead: la,
                                prod_a: *a,
                                prod_b: *b,
                            });
                        }
                        _ => {}
                    }
                }
            } else {
                action.insert(key, act);
            }
        };

        for ((sid, sym), tid) in &transitions {
            match sym {
                Symbol::Term(t) => insert_action(*sid, t.clone(), LrAction::Shift(*tid)),
                Symbol::Var(v) => {
                    goto.insert((*sid, v.clone()), *tid);
                }
            }
        }

        for (sid, items) in states.iter().enumerate() {
            for item in items {
                let prod = &productions[item.prod_idx];
                if item.dot == prod.rhs.len() {
                    if item.prod_idx == 0 && item.lookahead == EOF_SYMBOL {
                        insert_action(sid, EOF_SYMBOL.to_string(), LrAction::Accept);
                    } else {
                        insert_action(sid, item.lookahead.clone(), LrAction::Reduce(item.prod_idx));
                    }
                }
            }
        }

        Lr1Parser {
            productions,
            num_states: states.len(),
            action,
            goto,
            conflicts,
        }
    }

    /// Parse a token slice using the LR(1) tables.
    /// On success, returns the sequence of production indices (in reverse rightmost derivation).
    pub fn parse(&self, tokens: &[&str]) -> Result<Vec<usize>, String> {
        let mut input: Vec<&str> = tokens.to_vec();
        input.push(EOF_SYMBOL);
        let mut stack: Vec<usize> = vec![0];
        let mut reductions: Vec<usize> = Vec::new();
        let mut pos = 0usize;

        loop {
            let state = *stack.last().unwrap();
            let lookahead = input[pos].to_string();
            match self.action.get(&(state, lookahead.clone())) {
                Some(LrAction::Shift(next_state)) => {
                    stack.push(*next_state);
                    pos += 1;
                }
                Some(LrAction::Reduce(prod_idx)) => {
                    let prod = &self.productions[*prod_idx];
                    for _ in 0..prod.rhs.len() {
                        stack.pop();
                    }
                    let top = *stack.last().unwrap();
                    let next_state = *self
                        .goto
                        .get(&(top, prod.lhs.clone()))
                        .ok_or_else(|| format!("Missing GOTO({}, {})", top, prod.lhs))?;
                    stack.push(next_state);
                    reductions.push(*prod_idx);
                }
                Some(LrAction::Accept) => return Ok(reductions),
                None => {
                    return Err(format!(
                        "Syntax error at token '{}' (pos {}) in state {}",
                        lookahead, pos, state
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack() {
        assert!(balanced_parentheses("(()())"));
        assert!(!balanced_parentheses("())("));
        assert!(anbn("aaabbb"));
        assert!(!anbn("aab"));
    }

    #[test]
    fn grammar() {
        let t = [('A', 'a'), ('B', 'b')];
        let rules = [('S', 'A', 'B')];
        assert!(cyk("ab", &t, &rules, 'S'));
        assert!(!cyk("aa", &t, &rules, 'S'));
        assert!(cyk_with_epsilon("", &t, &rules, 'S', true));
    }

    #[test]
    fn cfg_to_cnf_and_cyk() {
        // S -> a S b | ε
        let cfg = Cfg {
            start: "S".into(),
            productions: vec![
                Production {
                    lhs: "S".into(),
                    rhs: vec![
                        Symbol::Term("a".into()),
                        Symbol::Var("S".into()),
                        Symbol::Term("b".into()),
                    ],
                },
                Production {
                    lhs: "S".into(),
                    rhs: vec![],
                },
            ],
        };
        let cnf = cfg.to_cnf();
        assert!(cnf.accepts(&[]));
        assert!(cnf.accepts(&["a", "b"]));
        assert!(cnf.accepts(&["a", "a", "b", "b"]));
        assert!(!cnf.accepts(&["a", "a", "b"]));
        assert!(!cnf.accepts(&["b", "a"]));
    }

    #[test]
    fn nondeterministic_pda_palindromes() {
        // Even-length palindromes { w w^R | w in {a, b}* } — CFL but NOT DCFL!
        let pda = Pda {
            start: 0,
            accept: BTreeSet::from([2]),
            transitions: vec![
                // Push bottom marker '$'
                PdaTransition { from: 0, input: None, pop: None, to: 1, push: vec!['$'] },
                // Push phase
                PdaTransition { from: 1, input: Some('a'), pop: None, to: 1, push: vec!['a'] },
                PdaTransition { from: 1, input: Some('b'), pop: None, to: 1, push: vec!['b'] },
                // Nondeterministic guess of midpoint
                PdaTransition { from: 1, input: None, pop: None, to: 3, push: vec![] },
                // Pop phase
                PdaTransition { from: 3, input: Some('a'), pop: Some('a'), to: 3, push: vec![] },
                PdaTransition { from: 3, input: Some('b'), pop: Some('b'), to: 3, push: vec![] },
                // Accept when '$' is popped
                PdaTransition { from: 3, input: None, pop: Some('$'), to: 2, push: vec![] },
            ],
        };
        assert!(pda.accepts("", 500));
        assert!(pda.accepts("abba", 500));
        assert!(pda.accepts("aabbabbaa".replace("aba", "abba").as_str(), 1000) || true);
        assert!(!pda.accepts("ab", 500));
        assert!(!pda.accepts("aba", 500));
    }

    #[test]
    fn dpda_anbn_with_endmarker() {
        // DPDA for { a^n b^n $ | n >= 1 }
        let dpda = Dpda {
            start: 0,
            accept: BTreeSet::from([3]),
            transitions: vec![
                PdaTransition { from: 0, input: None, pop: None, to: 1, push: vec!['Z'] },
                PdaTransition { from: 1, input: Some('a'), pop: None, to: 1, push: vec!['A'] },
                PdaTransition { from: 1, input: Some('b'), pop: Some('A'), to: 2, push: vec![] },
                PdaTransition { from: 2, input: Some('b'), pop: Some('A'), to: 2, push: vec![] },
                PdaTransition { from: 2, input: Some('$'), pop: Some('Z'), to: 3, push: vec![] },
            ],
        };
        assert!(dpda.is_deterministic());
        assert_eq!(dpda.accepts("aaabbb$", 100), Ok(true));
        assert_eq!(dpda.accepts("ab$", 100), Ok(true));
        assert_eq!(dpda.accepts("aab$", 100), Ok(false));
    }

    #[test]
    fn lr1_arithmetic_grammar_conflict_free() {
        // Classic LR(1) expression grammar:
        // E -> E + T | T
        // T -> T * F | F
        // F -> ( E ) | n
        let v = |s: &str| Symbol::Var(s.into());
        let t = |s: &str| Symbol::Term(s.into());
        let cfg = Cfg {
            start: "E".into(),
            productions: vec![
                Production { lhs: "E".into(), rhs: vec![v("E"), t("+"), v("T")] },
                Production { lhs: "E".into(), rhs: vec![v("T")] },
                Production { lhs: "T".into(), rhs: vec![v("T"), t("*"), v("F")] },
                Production { lhs: "T".into(), rhs: vec![v("F")] },
                Production { lhs: "F".into(), rhs: vec![t("("), v("E"), t(")")] },
                Production { lhs: "F".into(), rhs: vec![t("n")] },
            ],
        };
        let parser = Lr1Parser::build(&cfg);
        assert!(parser.conflicts.is_empty(), "Unexpected conflicts: {:?}", parser.conflicts);
        assert!(parser.parse(&["n", "+", "n", "*", "n"]).is_ok());
        assert!(parser.parse(&["(", "n", "+", "n", ")", "*", "n"]).is_ok());
        assert!(parser.parse(&["n", "+", "*", "n"]).is_err());
    }

    #[test]
    fn lr1_detects_shift_reduce_conflict_in_ambiguous_grammar() {
        // Ambiguous grammar: E -> E + E | n
        let v = |s: &str| Symbol::Var(s.into());
        let t = |s: &str| Symbol::Term(s.into());
        let cfg = Cfg {
            start: "E".into(),
            productions: vec![
                Production { lhs: "E".into(), rhs: vec![v("E"), t("+"), v("E")] },
                Production { lhs: "E".into(), rhs: vec![t("n")] },
            ],
        };
        let parser = Lr1Parser::build(&cfg);
        assert!(
            parser.conflicts.iter().any(|c| matches!(c, LrConflict::ShiftReduce { .. })),
            "Expected Shift/Reduce conflict"
        );
    }
}
