import sys


def generate_AST(output_dir, enum_descriptions):
    with open(f'{output_dir}/ast.rs', 'w') as f:
        f.write("use scanner::{Literal, Token};\n")
        write_ast_struct(f)
        for (base_name, types) in enum_descriptions:
            write_enum(f, base_name, types)
            write_visitor(f, base_name)


def write_ast_struct(file):
    file.write('\n')
    file.write('pub struct AST {\n')
    file.write('    pub root: Vec<Box<Stmt>>,\n')
    file.write('}\n')


def write_enum(file, base_name, types):
    file.write('\n')
    file.write(f'pub enum {base_name.capitalize()} {{\n')
    for class_name, fields in types:
        file.write(f'    {class_name}({", ".join(fields)}),\n')

    file.write('}\n')


def write_visitor(file, base_name):
    file.write('\n')
    file.write(f"pub trait {base_name.capitalize()}Visitor<E> {{\n")
    file.write(
        f'    fn visit_{base_name}(&mut self, {base_name.lower()}: &Box<{base_name.capitalize()}>) -> E;\n')
    file.write('}\n')


if __name__ == '__main__':
    import argparse

    parser = argparse.ArgumentParser(
        description='Generates the grammar.rs file.'
    )
    parser.add_argument('output_dir')

    args = parser.parse_args(sys.argv[1:])

    generate_AST(args.output_dir, [
        ('expr', [
            ('Assign', ['Token', 'Box<Expr>']),
            ('Literal', ['Literal']),
            ('Logical', ['Box<Expr>', 'Token', 'Box<Expr>']),
            ('Binary', ['Box<Expr>', 'Token', 'Box<Expr>']),
            ('Call', ['Box<Expr>', 'Token', 'Vec<Box<Expr>>']),
            ('Unary', ['Token', 'Box<Expr>']),
            ('Grouping', ['Box<Expr>']),
            ('Variable', ['Token'])
        ]),
        ('stmt', [
            ('Block', ['Vec<Box<Stmt>>']),
            ('Expression', ['Box<Expr>']),
            ('If', ['Box<Expr>', 'Box<Stmt>', 'Option<Box<Stmt>>']),
            ('Print', ['Box<Expr>']),
            ('Var', ['Token', 'Box<Expr>']),
            ('While', ['Box<Expr>', 'Box<Stmt>'])
        ])
    ])
