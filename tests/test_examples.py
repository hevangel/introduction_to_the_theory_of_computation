"""Comprehensive mirrored behavioral tests for Chapters 0–10.
Run: python3 -m unittest discover -s tests -v
"""
import unittest
from python import ch00, ch01, ch02, ch03, ch04, ch05, ch06, ch07, ch08, ch09, ch10


class Examples(unittest.TestCase):
    def test_ch00(self):
        self.assertEqual(ch00.reachable([[1], [2], [], []], 0), [True, True, True, False])
        self.assertFalse(ch00.has_cycle([[1], [2], []]))
        self.assertTrue(ch00.has_cycle([[1], [2], [0]]))
        self.assertEqual(len(ch00.power_set([1, 2, 3])), 8)
        mod2 = [(0, 0), (0, 2), (2, 0), (2, 2), (1, 1), (1, 3), (3, 1), (3, 3)]
        self.assertTrue(ch00.is_equivalence_relation(4, mod2))
        self.assertEqual(ch00.equivalence_classes(4, mod2), [[0, 2], [1, 3]])
        self.assertFalse(ch00.is_equivalence_relation(2, [(0, 0), (1, 1), (0, 1)]))
        diag = ch00.cantor_diagonal_bitstring(["010", "111", "001"])
        self.assertEqual(diag, "100")
        self.assertNotIn(diag, ["010", "111", "001"])

    def test_ch01(self):
        self.assertTrue(ch01.contains_01("11010"))
        self.assertFalse(ch01.contains_01("111"))
        self.assertFalse(ch01.contains_01("02"))
        self.assertTrue(ch01.ends_ab_nfa("bbaab"))
        self.assertFalse(ch01.ends_ab_nfa("aba"))

        nfa = ch01.regex_to_nfa("(0|1)*01(0|1)*")
        dfa = nfa.to_dfa()
        min_dfa = dfa.minimize()
        self.assertEqual(min_dfa.num_states, 3)
        for w in ["", "0", "1", "11100", "01", "11010", "0011"]:
            self.assertEqual(nfa.accepts(w), ch01.contains_01(w))
            self.assertEqual(dfa.accepts(w), ch01.contains_01(w))
            self.assertEqual(min_dfa.accepts(w), ch01.contains_01(w))
        comp = min_dfa.complement()
        self.assertFalse(comp.accepts("01"))
        self.assertTrue(comp.accepts("111000"))
        empty = min_dfa.product(comp, is_intersection=True)
        self.assertFalse(empty.accepts("01"))

    def test_ch02(self):
        self.assertTrue(ch02.balanced_parentheses("(()())"))
        self.assertFalse(ch02.balanced_parentheses("())("))
        self.assertTrue(ch02.anbn("aaabbb"))
        self.assertFalse(ch02.anbn("aab"))
        t, rules = [("A", "a"), ("B", "b")], [("S", "A", "B")]
        self.assertTrue(ch02.cyk("ab", t, rules, "S"))
        self.assertFalse(ch02.cyk("aa", t, rules, "S"))
        self.assertTrue(ch02.cyk_with_epsilon("", t, rules, "S", allows_epsilon=True))

        # CFG -> CNF + CYK for S -> a S b | ε
        cfg = ch02.Cfg(
            start="S",
            productions=[
                ch02.Production("S", (ch02.term("a"), ch02.var("S"), ch02.term("b"))),
                ch02.Production("S", ()),
            ],
        )
        cnf = cfg.to_cnf()
        self.assertTrue(cnf.accepts([]))
        self.assertTrue(cnf.accepts(["a", "a", "b", "b"]))
        self.assertFalse(cnf.accepts(["a", "a", "b"]))

        # Nondeterministic PDA for even palindromes w w^R
        pda = ch02.Pda(
            start=0,
            accept={2},
            transitions=[
                ch02.PdaTransition(0, None, None, 1, ("$",)),
                ch02.PdaTransition(1, "a", None, 1, ("a",)),
                ch02.PdaTransition(1, "b", None, 1, ("b",)),
                ch02.PdaTransition(1, None, None, 3, ()),
                ch02.PdaTransition(3, "a", "a", 3, ()),
                ch02.PdaTransition(3, "b", "b", 3, ()),
                ch02.PdaTransition(3, None, "$", 2, ()),
            ],
        )
        self.assertTrue(pda.accepts("abba"))
        self.assertFalse(pda.accepts("aba"))

        # DPDA for a^n b^n $ (n >= 1)
        dpda = ch02.Dpda(
            start=0,
            accept={3},
            transitions=[
                ch02.PdaTransition(0, None, None, 1, ("Z",)),
                ch02.PdaTransition(1, "a", None, 1, ("A",)),
                ch02.PdaTransition(1, "b", "A", 2, ()),
                ch02.PdaTransition(2, "b", "A", 2, ()),
                ch02.PdaTransition(2, "$", "Z", 3, ()),
            ],
        )
        self.assertTrue(dpda.is_deterministic())
        self.assertTrue(dpda.accepts("aaabbb$"))
        self.assertFalse(dpda.accepts("aab$"))

        # Canonical LR(1) parser: arithmetic expression grammar (conflict-free)
        v, tm = ch02.var, ch02.term
        expr_cfg = ch02.Cfg(
            start="E",
            productions=[
                ch02.Production("E", (v("E"), tm("+"), v("T"))),
                ch02.Production("E", (v("T"),)),
                ch02.Production("T", (v("T"), tm("*"), v("F"))),
                ch02.Production("T", (v("F"),)),
                ch02.Production("F", (tm("("), v("E"), tm(")"))),
                ch02.Production("F", (tm("n"),)),
            ],
        )
        parser = ch02.Lr1Parser.build(expr_cfg)
        self.assertEqual(parser.conflicts, [])
        self.assertTrue(len(parser.parse(["n", "+", "n", "*", "n"])) > 0)
        with self.assertRaises(ValueError):
            parser.parse(["n", "+", "*", "n"])

        # Ambiguous grammar E -> E + E | n must report shift-reduce conflict
        ambig = ch02.Cfg(
            start="E",
            productions=[
                ch02.Production("E", (v("E"), tm("+"), v("E"))),
                ch02.Production("E", (tm("n"),)),
            ],
        )
        ambig_parser = ch02.Lr1Parser.build(ambig)
        self.assertTrue(any(c["type"] == "shift-reduce" for c in ambig_parser.conflicts))

    def test_ch03(self):
        self.assertEqual(ch03.unary_successor("111", 4), (ch03.Outcome.ACCEPT, "1111"))
        self.assertEqual(ch03.unary_successor("111", 3)[0], ch03.Outcome.TIMEOUT)
        m2 = ch03.tm_power_of_two_zeros()
        self.assertEqual(m2.run("0", 50)[0], ch03.Outcome.ACCEPT)
        self.assertEqual(m2.run("0000", 100)[0], ch03.Outcome.ACCEPT)
        self.assertEqual(m2.run("000", 100)[0], ch03.Outcome.REJECT)
        self.assertEqual(ch03.two_tape_ww_hash("0110#0110")[0], ch03.Outcome.ACCEPT)
        self.assertEqual(ch03.two_tape_ww_hash("01#10")[0], ch03.Outcome.REJECT)

    def test_ch04(self):
        parity = [(0, 1), (1, 0)]
        self.assertTrue(ch04.dfa_equivalent(parity, [True, False], parity, [True, False]))
        self.assertFalse(ch04.dfa_equivalent(parity, [True, False], [(0, 0)], [True]))

        d1 = ch01.regex_to_nfa("(a|b)*ab").to_dfa()
        d2 = d1.minimize()
        self.assertTrue(ch04.a_dfa(d1, "bab"))
        self.assertFalse(ch04.e_dfa(d1))
        self.assertTrue(ch04.eq_dfa(d1, d2))
        self.assertTrue(ch04.e_dfa(d1.product(d1.complement(), is_intersection=True)))

        empty_cfg = ch02.Cfg("S", [ch02.Production("S", (ch02.term("a"), ch02.var("S")))])
        self.assertTrue(ch04.e_cfg(empty_cfg))
        anbn_cfg = ch02.Cfg(
            "S",
            [
                ch02.Production("S", (ch02.term("a"), ch02.var("S"), ch02.term("b"))),
                ch02.Production("S", ()),
            ],
        )
        self.assertFalse(ch04.e_cfg(anbn_cfg))
        self.assertTrue(ch04.a_cfg(anbn_cfg, ["a", "a", "b", "b"]))
        self.assertFalse(ch04.a_cfg(anbn_cfg, ["a", "b", "b"]))

    def test_ch05(self):
        self.assertTrue(ch05.brute_sat(ch05.three_color_to_cnf(3, [(0, 1), (1, 2), (0, 2)]), 9))
        self.assertFalse(
            ch05.brute_sat(
                ch05.three_color_to_cnf(4, [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]), 12
            )
        )
        dominoes = [("b", "ca"), ("a", "ab"), ("ca", "a"), ("abc", "c")]
        sol = ch05.bounded_pcp(dominoes, 5)
        self.assertIsNotNone(sol)
        self.assertEqual("".join(dominoes[i][0] for i in sol), "".join(dominoes[i][1] for i in sol))

        looping = ch03.TuringMachine(0, 1, 2, {(0, "0"): (0, "0", ch03.Move.STAY)})
        self.assertFalse(ch05.lba_accepts(looping, "0"))
        self.assertTrue(ch05.lba_accepts(ch03.tm_power_of_two_zeros(), "0000_"))

        n, edges, k = ch05.three_sat_to_clique([(1, 1, 2), (-1, -2, -2), (-1, 2, 2)])
        self.assertEqual((n, k), (9, 3))
        self.assertIn((2, 3), edges)

    def test_ch06(self):
        self.assertEqual(ch06.binary_strings_up_to(2), ["", "0", "1", "00", "01", "10", "11"])
        self.assertEqual(ch06.run_length_encode("aaabb"), [("a", 3), ("b", 2)])
        for x in range(10):
            for y in range(10):
                self.assertEqual(ch06.cantor_unpair(ch06.cantor_pair(x, y)), (x, y))
        a, b = ch06.self_reproducing_string()
        self.assertEqual(a, b)
        has_zero = ch06.ForAll("x", ch06.Exists("y", ch06.EqSum("x", "y", "x")))
        self.assertTrue(ch06.eval_fo_mod(has_zero, 5))
        all_zero = ch06.ForAll("x", ch06.ForAll("y", ch06.EqSum("x", "y", "x")))
        self.assertFalse(ch06.eval_fo_mod(all_zero, 5))

    def test_ch07(self):
        edges = [(0, 1), (1, 2)]
        self.assertTrue(ch07.verifies_vertex_cover(3, edges, [1]))
        self.assertEqual(len(ch07.minimum_vertex_cover(3, edges)), 1)
        self.assertTrue(ch07.subset_sum_nonnegative([3, 5, 7], 8))
        self.assertFalse(ch07.subset_sum_nonnegative([3, 5, 7], 1))
        self.assertEqual(ch07.two_sat_solve(2, [(1, 2), (-1, 2), (1, -2)]), [True, True])
        self.assertIsNone(ch07.two_sat_solve(1, [(1, 1), (-1, -1)]))
        self.assertTrue(ch07.verifies_clique(4, [(0, 1), (1, 2), (0, 2), (2, 3)], [0, 1, 2], 3))
        self.assertTrue(ch07.verifies_hampath(3, [(0, 1), (1, 2)], 0, 2, [0, 1, 2]))
        n, comp_edges, vc_k = ch07.clique_to_vertex_cover(4, [(0, 1), (1, 2), (0, 2), (2, 3)], 3)
        self.assertEqual(vc_k, 1)
        self.assertEqual(len(ch07.minimum_vertex_cover(n, comp_edges)), 1)
        nums, target = ch07.three_sat_to_subset_sum(2, [(1, 1, 2), (-1, -2, -2)])
        self.assertTrue(ch07.subset_sum_nonnegative(nums, target))

    def test_ch08(self):
        graph = [[1], [2], []]
        self.assertFalse(ch08.reach_within(graph, 0, 2, 1))
        self.assertTrue(ch08.reach_within(graph, 0, 2, 2))
        f_true = ch08.QForAll(
            "x",
            ch08.QExists(
                "y",
                ch08.QOr(
                    ch08.QAnd(ch08.QVar("x"), ch08.QVar("y")),
                    ch08.QAnd(ch08.QNot(ch08.QVar("x")), ch08.QNot(ch08.QVar("y"))),
                ),
            ),
        )
        self.assertTrue(ch08.eval_tqbf(f_true))
        g2 = [[1], [2], [], [2]]
        self.assertFalse(ch08.immerman_szelepcsenyi_non_reach(g2, 0, 2))
        self.assertTrue(ch08.immerman_szelepcsenyi_non_reach(g2, 0, 3))

    def test_ch09(self):
        self.assertEqual(ch09.boolean_function_count(3), 256)
        self.assertEqual(ch09.xor_truth_table(), (False, True, True, False))
        self.assertEqual(ch09.xor_circuit().truth_table(), [False, True, True, False])
        self.assertTrue(ch09.oracle_exists_of_length(3, lambda w: w == "101"))
        self.assertFalse(ch09.oracle_exists_of_length(3, lambda w: w == "1010"))

    def test_ch10(self):
        edges = [(0, 1), (1, 2), (2, 3)]
        cover = ch10.two_approx_vertex_cover(edges)
        self.assertTrue(ch07.verifies_vertex_cover(4, edges, cover))
        self.assertLessEqual(len(cover), 2 * len(ch07.minimum_vertex_cover(4, edges)))
        self.assertEqual(ch10.mod_pow(2, 10, 17), 4)
        self.assertTrue(2.8 < ch10.pi_estimate(5000) < 3.5)
        self.assertTrue(ch10.is_probably_prime(1_000_000_007))
        self.assertFalse(ch10.is_probably_prime(561))  # Carmichael number
        nodes = [
            ch10.RobpBranch(0, 1, 2),
            ch10.RobpBranch(1, 3, 4),
            ch10.RobpBranch(1, 4, 3),
            ch10.RobpSink(False),
            ch10.RobpSink(True),
        ]
        self.assertTrue(ch10.eval_robp(nodes, 0, [True, False]))
        self.assertFalse(ch10.eval_robp(nodes, 0, [True, True]))
        self.assertEqual(ch10.parallel_prefix_sum([1, 2, 3, 4, 5, 6, 7, 8]), ([1, 3, 6, 10, 15, 21, 28, 36], 3))
        self.assertTrue(ch10.verify_graph_isomorphism(3, [(0, 1), (1, 2)], [(2, 0), (0, 1)], [2, 0, 1]))
        self.assertEqual(ch10.rsa_roundtrip(61, 53, 17, 65), (2790, 65))


if __name__ == "__main__":
    unittest.main()
