import unittest

from generate_core_inventory import constructor_properties, top_level_methods


class InventoryTests(unittest.TestCase):
    def test_object_default_does_not_truncate_constructor(self):
        source = '''
        constructor(width = 1, options = {}) {
            super();
            options = Object.assign({ multiview: false }, options);
            this.width = width;
            this.multiview = options.multiview;
        }
        resize(width) { this.width = width; }
        '''
        methods = top_level_methods(source)
        self.assertEqual([m.name for m in methods], ['constructor', 'resize'])
        self.assertEqual(constructor_properties('RenderTarget', source, methods),
                         {'width', 'multiview'})


if __name__ == '__main__':
    unittest.main()
