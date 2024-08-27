import unittest


from pyo3_test import string_operations, math_operations

class TestRustMethods(unittest.TestCase):

    def test_concatenate(self):
        self.assertEqual("FooBar", string_operations.concatenate("Foo", "Bar"))

    def test_add(self):
        self.assertEqual(4, math_operations.add(2, 2))

if __name__ == "__main__":
    unittest.main()
