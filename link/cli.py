"""Command line interface for link."""
import argh


def greet() -> None:
    r"""Say hello, link"""
    print(f'Hello, world!')


def main():
    parser = argh.ArghParser()
    parser.add_commands([greet])
    parser.dispatch()


if __name__=='__main__':
    main()
