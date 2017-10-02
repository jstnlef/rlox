import sys


def generate_AST(output_dir, base_name, types):
    with open(f'{output_dir}/{base_name}.rs', 'w') as f:
        f.write("use scanner::{Literal, Token};\n")
        write_ast_struct(f)
        write_expr_enum(f, types)


def write_ast_struct(file):
    file.write('\n')
    file.write('pub struct AST {\n')
    file.write('    root: Box<Expr>,\n')
    file.write('}\n')


def write_expr_enum(file, types):
    file.write('\n')
    file.write('pub enum Expr {\n')
    for class_name, fields in types:
        file.write(f'    {class_name}({", ".join(fields)}),\n')

    file.write('}\n')


if __name__ == '__main__':
    import argparse

    parser = argparse.ArgumentParser(
        description='Generates the expr.rs file.'
    )
    parser.add_argument('output_dir')

    args = parser.parse_args(sys.argv[1:])

    generate_AST(args.output_dir, 'expr', [
        ('Literal', ['Literal']),
        ('Binary', ['Box<Expr>', 'Token', 'Box<Expr>']),
        ('Unary', ['Token', 'Box<Expr>']),
        ('Grouping', ['Box<Expr>'])
    ])
