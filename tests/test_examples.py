"""Mirrored behavioral tests. Run: python -m unittest discover -s tests -v"""
import unittest
from python import ch00, ch01, ch02, ch03, ch04, ch05, ch06, ch07, ch08, ch09, ch10

class Examples(unittest.TestCase):
    def test_ch00(self):
        self.assertEqual(ch00.reachable([[1], [2], [], []], 0),
                         [True, True, True, False])
    def test_ch01(self):
        self.assertTrue(ch01.contains_01("11010"))
        self.assertFalse(ch01.contains_01("111"))
        self.assertFalse(ch01.contains_01("02"))
        self.assertTrue(ch01.ends_ab_nfa("bbaab"))
        self.assertFalse(ch01.ends_ab_nfa("aba"))
    def test_ch02(self):
        self.assertTrue(ch02.balanced_parentheses("(()())"))
        self.assertFalse(ch02.balanced_parentheses("())("))
        self.assertTrue(ch02.anbn("aaabbb"))
        self.assertFalse(ch02.anbn("aab"))
        t, rules = [("A","a"),("B","b")], [("S","A","B")]
        self.assertTrue(ch02.cyk("ab",t,rules,"S"))
        self.assertFalse(ch02.cyk("aa",t,rules,"S"))
    def test_ch03(self):
        self.assertEqual(ch03.unary_successor("111",4),
                         (ch03.Outcome.ACCEPT,"1111"))
        self.assertEqual(ch03.unary_successor("111",3)[0], ch03.Outcome.TIMEOUT)
    def test_ch04(self):
        parity = [(0,1),(1,0)]
        self.assertTrue(ch04.dfa_equivalent(parity,[True,False],parity,[True,False]))
        self.assertFalse(ch04.dfa_equivalent(parity,[True,False],[(0,0)],[True]))
    def test_ch05(self):
        self.assertTrue(ch05.brute_sat(ch05.three_color_to_cnf(
            3,[(0,1),(1,2),(0,2)]),9))
        self.assertFalse(ch05.brute_sat(ch05.three_color_to_cnf(
            4,[(0,1),(0,2),(0,3),(1,2),(1,3),(2,3)]),12))
    def test_ch06(self):
        self.assertEqual(ch06.binary_strings_up_to(2),
                         ["","0","1","00","01","10","11"])
        self.assertEqual(ch06.run_length_encode("aaabb"),[("a",3),("b",2)])
    def test_ch07(self):
        edges = [(0,1),(1,2)]
        self.assertTrue(ch07.verifies_vertex_cover(3,edges,[1]))
        self.assertEqual(len(ch07.minimum_vertex_cover(3,edges)),1)
        self.assertTrue(ch07.subset_sum_nonnegative([3,5,7],8))
        self.assertFalse(ch07.subset_sum_nonnegative([3,5,7],1))
    def test_ch08(self):
        graph = [[1],[2],[]]
        self.assertFalse(ch08.reach_within(graph,0,2,1))
        self.assertTrue(ch08.reach_within(graph,0,2,2))
    def test_ch09(self):
        self.assertEqual(ch09.boolean_function_count(3),256)
        self.assertEqual(ch09.xor_truth_table(),(False,True,True,False))
    def test_ch10(self):
        edges = [(0,1),(1,2),(2,3)]
        cover = ch10.two_approx_vertex_cover(edges)
        self.assertTrue(ch07.verifies_vertex_cover(4,edges,cover))
        self.assertLessEqual(len(cover),2*len(ch07.minimum_vertex_cover(4,edges)))
        self.assertEqual(ch10.mod_pow(2,10,17),4)
        self.assertTrue(2.8 < ch10.pi_estimate(5000) < 3.5)

if __name__ == "__main__":
    unittest.main()
